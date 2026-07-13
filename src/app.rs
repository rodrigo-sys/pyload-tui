use crossterm::event::{Event, KeyCode, KeyEvent};
use openapi::models::{EventInfo, ServerStatus};

use crate::{
    add_package_form::AddPackageForm,
    app_action::AppAction,
    append_files_form::AppendFilesForm,
    downloads_screen::DownloadsScreen,
    files_screen::FilesScreen,
    packages_screen::PackagesScreen,
    screens::{Screen, ScreenHandler},
    status_bar::StatusBar,
    utils::{
        fetch_file_data, fetch_files, fetch_package_data, fetch_packages, fetch_server_status,
        move_package, pause_server, remove_files_from_package, remove_packages, reorder_file,
        reorder_package, restart_failed, restart_file, restart_package, stop_all_downloads,
        stop_downloads, toggle_pause, unpause_server,
    },
};

macro_rules! find_screen {
    ($self:expr, $variant:ident) => {
        if let Screen::$variant(s) = &mut $self.current_screen {
            Some(s)
        } else if let Some(Screen::$variant(s)) = &mut $self.previous_screen {
            Some(s)
        } else {
            None
        }
    };
}

pub struct App {
    pub current_screen: Screen,
    pub previous_screen: Option<Screen>,
    pub status_bar: StatusBar,
    pub quit: bool,
}

impl App {
    pub async fn new() -> Self {
        let packages = PackagesScreen::new().await;
        Self {
            current_screen: Screen::Packages(packages),
            previous_screen: None,
            quit: false,
            status_bar: StatusBar::new(),
        }
    }

    pub async fn handle_events(&mut self, event: Event) {
        match event {
            Event::Key(key) => self.handle_key(key).await,
            Event::Paste(content) => self.current_screen.handle_paste(&content),
            _ => {}
        }
    }

    pub fn update_status(&mut self, server_status: ServerStatus) {
        self.status_bar.refresh(server_status);
    }

    pub async fn handle_key(&mut self, key: KeyEvent) {
        let mut action = self.current_screen.handle_keys(key).await;

        if action.is_none() {
            action = match key.code {
                KeyCode::Char('q') => (!matches!(&self.current_screen, Screen::AddPackageForm(_)))
                    .then_some(AppAction::Quit),
                KeyCode::Char('A') => Some(AppAction::OpenAddPackageForm),
                KeyCode::Char('R') => Some(AppAction::RestartFailed),
                KeyCode::Char('S') => Some(AppAction::AbortActive),
                KeyCode::Char('X') => Some(AppAction::AbortAndPause),
                KeyCode::Char('P') => Some(AppAction::PauseServer),
                KeyCode::Char('U') => Some(AppAction::UnpauseServer),
                KeyCode::Char('T') => Some(AppAction::TogglePause),
                KeyCode::Char('D') => Some(AppAction::GoToDownloads),
                _ => None,
            };
        }

        match action {
            Some(AppAction::Quit) => self.quit = true,
            Some(AppAction::OpenAddPackageForm) => self.go_to_add_package_form(),
            Some(AppAction::OpenAppendFilesForm(pid, name)) => {
                self.go_to_append_files_form(pid, name)
            }
            Some(AppAction::GoToPackages) => self.go_to_packages(),
            Some(AppAction::GoToDownloads) => self.go_to_downloads(),
            Some(AppAction::GoToFiles(pid, name)) => self.go_to_files(pid, name).await,
            Some(AppAction::DeletePackages(packages)) => {
                if let Some(package_id) = packages.first() {
                    let _ = remove_packages(vec![*package_id]).await;
                }
            }
            Some(AppAction::DeleteFiles(files)) => {
                if let Some(file_id) = files.first() {
                    let _ = remove_files_from_package(vec![*file_id]).await;
                }
            }
            Some(AppAction::GoToPreviousScreen) => {
                self.go_to_previous_screen();
            }
            Some(AppAction::StopDownloads(files)) => {
                let _ = stop_downloads(files).await;
            }
            Some(AppAction::RestartFile(file)) => {
                let _ = restart_file(file).await;
            }
            Some(AppAction::RestartPackage(pid)) => {
                let _ = restart_package(pid).await;
            }
            Some(AppAction::RestartFailed) => {
                let _ = restart_failed().await;
            }
            Some(AppAction::AbortActive) => {
                let _ = stop_all_downloads().await;
            }
            Some(AppAction::AbortAndPause) => {
                if let Ok(status) = fetch_server_status().await {
                    if status.pause {
                        let _ = restart_failed().await;
                        let _ = unpause_server().await;
                    } else {
                        let _ = pause_server().await;
                        let _ = stop_all_downloads().await;
                        let _ = restart_failed().await;
                    }
                }
            }
            Some(AppAction::PauseServer) => {
                let _ = pause_server().await;
            }
            Some(AppAction::UnpauseServer) => {
                let _ = unpause_server().await;
            }
            Some(AppAction::TogglePause) => {
                let _ = toggle_pause().await;
            }
            Some(AppAction::ReorderPackage(pid, position)) => {
                let _ = reorder_package(pid, position).await;
            }
            Some(AppAction::ReorderFile(fid, position)) => {
                let _ = reorder_file(fid, position).await;
            }
            Some(AppAction::MovePackage(destination, pid)) => {
                let _ = move_package(destination, pid).await;
            }
            _ => {}
        }
    }

    pub async fn handle_pyload_events(&mut self, event: EventInfo) {
        let event_id = event.id.flatten();
        let event_type = event.r#type.flatten();

        match event.eventname.as_str() {
            "update" => match (event_type, event_id) {
                (Some(0), Some(pid)) => {
                    let Some(screen) = find_screen!(self, Packages) else {
                        return;
                    };
                    let Ok(package) = fetch_package_data(pid).await else {
                        return;
                    };
                    let Some(position) = screen.packages.iter().position(|p| p.pid == pid) else {
                        return;
                    };

                    let refetch = screen.packages[position].linkstotal.flatten()
                        != package.linkstotal.flatten();
                    screen.packages[position] = package;
                    let _ = screen;

                    if refetch
                        && let Some(files_screen) = find_screen!(self, Files)
                        && files_screen.package_id == pid
                        && let Ok(files) = fetch_files(pid).await
                    {
                        files_screen.files = files;
                    }
                }
                (Some(1), Some(fid)) => {
                    let Some(files_screen) = find_screen!(self, Files) else {
                        return;
                    };
                    let Ok(file) = fetch_file_data(fid).await else {
                        return;
                    };
                    if files_screen.package_id != file.package_id {
                        return;
                    }
                    let Some(position) = files_screen.files.iter().position(|f| f.fid == fid)
                    else {
                        return;
                    };

                    files_screen.files[position] = file;
                    files_screen.refresh_downloads_info().await;
                }
                _ => {}
            },
            "remove" => match (event_type, event_id) {
                (Some(0), Some(pid)) => {
                    let Some(screen) = find_screen!(self, Packages) else {
                        return;
                    };
                    let Some(position) = screen.packages.iter().position(|p| p.pid == pid) else {
                        return;
                    };

                    screen.packages.remove(position);
                }
                (Some(1), Some(fid)) => {
                    let Some(files_screen) = find_screen!(self, Files) else {
                        return;
                    };
                    let Some(position) = files_screen.files.iter().position(|f| f.fid == fid)
                    else {
                        return;
                    };

                    files_screen.files.remove(position);
                }
                _ => {}
            },
            "insert" => match (event_type, event_id) {
                (Some(0), Some(pid)) => {
                    let Some(screen) = find_screen!(self, Packages) else {
                        return;
                    };
                    let Ok(package) = fetch_package_data(pid).await else {
                        return;
                    };

                    let position = screen
                        .packages
                        .iter()
                        .position(|p| {
                            p.dest < package.dest
                                || (p.dest == package.dest && p.order > package.order)
                        })
                        .unwrap_or(screen.packages.len());

                    screen.packages.insert(position, package);
                    screen.table_state.select(Some(position));
                }
                (Some(1), Some(fid)) => {
                    let Some(files_screen) = find_screen!(self, Files) else {
                        return;
                    };
                    let Ok(file) = fetch_file_data(fid).await else {
                        return;
                    };
                    if files_screen.package_id != file.package_id {
                        return;
                    }
                    let Ok(files) = fetch_files(file.package_id).await else {
                        return;
                    };

                    files_screen.files = files;
                    let Some(pos) = files_screen.files.iter().position(|f| f.fid == fid) else {
                        return;
                    };
                    files_screen.table_state.select(Some(pos));
                }
                _ => {}
            },
            "reload" => {
                if let Some(screen) = find_screen!(self, Packages)
                    && let Ok(packages) = fetch_packages().await
                {
                    screen.packages = packages;
                    screen.table_state.select_first();
                }
            }
            _ => {}
        }
    }

    fn go_to_previous_screen(&mut self) {
        if self.previous_screen.as_ref().is_none_or(|prev| {
            std::mem::discriminant(&self.current_screen) == std::mem::discriminant(prev)
        }) {
            return;
        }

        let prev = self.previous_screen.take().unwrap();
        let old = std::mem::replace(&mut self.current_screen, prev);
        self.previous_screen = Some(old);
    }

    fn go_to_add_package_form(&mut self) {
        if matches!(&self.current_screen, Screen::AddPackageForm(_)) {
            return;
        }

        if let Some(Screen::AddPackageForm(_)) = &self.previous_screen {
            let prev = self.previous_screen.take().unwrap();
            let old = std::mem::replace(&mut self.current_screen, prev);
            self.previous_screen = Some(old);
            return;
        }

        let old = std::mem::replace(
            &mut self.current_screen,
            Screen::AddPackageForm(AddPackageForm::default()),
        );
        self.previous_screen = Some(old);
    }

    fn go_to_packages(&mut self) {
        if matches!(&self.current_screen, Screen::Packages(_)) {
            return;
        }

        if let Some(Screen::Packages(_)) = &self.previous_screen {
            let prev = self.previous_screen.take().unwrap();
            let old = std::mem::replace(&mut self.current_screen, prev);
            self.previous_screen = Some(old);
            return;
        }

        let old = std::mem::replace(
            &mut self.current_screen,
            Screen::Packages(PackagesScreen::default()),
        );
        self.previous_screen = Some(old);
    }

    fn go_to_downloads(&mut self) {
        if matches!(&self.current_screen, Screen::Downloads(_)) {
            return;
        }

        if let Some(Screen::Downloads(_)) = &self.previous_screen {
            let prev = self.previous_screen.take().unwrap();
            let old = std::mem::replace(&mut self.current_screen, prev);
            self.previous_screen = Some(old);
            return;
        }

        let old = std::mem::replace(
            &mut self.current_screen,
            Screen::Downloads(DownloadsScreen::default()),
        );
        self.previous_screen = Some(old);
    }

    async fn go_to_files(&mut self, pid: i32, name: String) {
        if let Screen::Files(screen) = &self.current_screen {
            if screen.package_id == pid {
                return;
            }
        }

        if let Some(Screen::Files(screen)) = &self.previous_screen {
            if screen.package_id == pid {
                let prev = self.previous_screen.take().unwrap();
                let old = std::mem::replace(&mut self.current_screen, prev);
                self.previous_screen = Some(old);
                return;
            }
        }

        let old = std::mem::replace(
            &mut self.current_screen,
            Screen::Files(FilesScreen::new(pid, name).await),
        );
        self.previous_screen = Some(old);
    }

    fn go_to_append_files_form(&mut self, pid: i32, name: String) {
        if matches!(&self.current_screen, Screen::AppendFilesForm(_)) {
            return;
        }

        let old = std::mem::replace(
            &mut self.current_screen,
            Screen::AppendFilesForm(AppendFilesForm::new(pid, name)),
        );
        self.previous_screen = Some(old);
    }

    pub fn get_bindings(&self) -> Vec<(&'static str, &'static str)> {
        match &self.current_screen {
            Screen::AddPackageForm(_) => {
                vec![
                    ("Esc", "back"),
                    ("Tab", "next"),
                    ("Shift+Tab", "prev"),
                    ("Enter", "newline/toggle/submit"),
                ]
            }
            Screen::AppendFilesForm(_) => {
                vec![
                    ("Esc", "back"),
                    ("Tab", "next"),
                    ("Shift+Tab", "prev"),
                    ("Enter", "newline/submit"),
                ]
            }
            _ => {
                let mut binds = vec![
                    ("j", "next item"),
                    ("k", "prev item"),
                    ("l", "enter"),
                    ("q", "quit"),
                    ("A", "add package"),
                    ("a", "add links"),
                    ("d", "delete"),
                ];
                if matches!(&self.current_screen, Screen::Files(_)) {
                    binds.insert(0, ("h", "go back"));
                }
                if matches!(&self.current_screen, Screen::Downloads(_)) {
                    binds.insert(0, ("h", "go back"));
                }
                binds
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let packages = PackagesScreen::default();
        Self {
            current_screen: Screen::Packages(packages),
            previous_screen: None,
            quit: false,
            status_bar: StatusBar::new(),
        }
    }
}
