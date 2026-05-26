use crossterm::event::{Event, KeyCode, KeyEvent};
use openapi::models::EventInfo;

use crate::{
    add_package_form::AddPackageForm,
    app_action::AppAction,
    append_files_form::AppendFilesForm,
    files_screen::FilesScreen,
    packages_screen::PackagesScreen,
    screens::{Screen, Screens},
    utils::{
        fetch_file_data, fetch_files, fetch_package_data, fetch_packages,
        remove_files_from_package, remove_packages,
    },
};

pub struct App {
    pub screens: Screens,
    pub current_screen: Screen,
    pub previous_screen: Option<Screen>,
    pub quit: bool,
}

impl App {
    pub async fn new() -> Self {
        let packages = PackagesScreen::new().await;
        Self {
            screens: Screens {
                packages: Some(packages),
                files: None,
                add_package_form: None,
                append_files_form: None,
            },
            current_screen: Screen::Packages,
            quit: false,
            previous_screen: None,
        }
    }

    pub async fn handle_events(&mut self, event: Event) {
        match event {
            Event::Key(key) => self.handle_key(key).await,
            Event::Paste(content) => match self.current_screen {
                Screen::AddPackageForm => {
                    if let Some(f) = self.screens.add_package_form.as_mut() {
                        f.handle_paste(&content);
                    }
                }
                Screen::AppendFilesForm => {
                    if let Some(f) = self.screens.append_files_form.as_mut() {
                        f.handle_paste(&content);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub async fn handle_key(&mut self, key: KeyEvent) {
        let mut action = match self.current_screen {
            Screen::Packages => {
                let s = self.screens.packages.as_mut().unwrap();
                s.handle_keys(key).await
            }
            Screen::Files => {
                let s = self.screens.files.as_mut().unwrap();
                s.handle_keys(key).await
            }
            Screen::AddPackageForm => {
                let s = self.screens.add_package_form.as_mut().unwrap();
                s.handle_keys(key).await
            }
            Screen::AppendFilesForm => {
                let s = self.screens.append_files_form.as_mut().unwrap();
                s.handle_keys(key).await
            }
        };

        if action.is_none() {
            action = match key.code {
                KeyCode::Char('q') => (!matches!(self.current_screen, Screen::AddPackageForm))
                    .then_some(AppAction::Quit),
                KeyCode::Char('A') => Some(AppAction::OpenAddPackageForm),
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
            _ => {}
        }
    }

    pub async fn handle_pyload_events(&mut self, event: EventInfo) {
        let event_id = event.id.flatten();
        let event_type = event.r#type.flatten();

        match event.eventname.as_str() {
            "update" => match (event_type, event_id) {
                (Some(0), Some(pid)) => {
                    let Some(screen) = &mut self.screens.packages else {
                        return;
                    };
                    let Ok(package) = fetch_package_data(pid).await else {
                        return;
                    };
                    let Some(position) = screen.packages.iter().position(|p| p.pid == pid) else {
                        return;
                    };

                    screen.packages[position] = package;
                }
                (Some(1), Some(fid)) => {
                    let Ok(file) = fetch_file_data(fid).await else {
                        return;
                    };
                    let Some(files_screen) = &mut self.screens.files else {
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
                }
                _ => {}
            },
            "remove" => match (event_type, event_id) {
                (Some(0), Some(pid)) => {
                    let Some(screen) = &mut self.screens.packages else {
                        return;
                    };
                    let Some(position) = screen.packages.iter().position(|p| p.pid == pid) else {
                        return;
                    };

                    screen.packages.remove(position);
                }
                (Some(1), Some(fid)) => {
                    let Some(files_screen) = &mut self.screens.files else {
                        return;
                    };
                    let Some(position) = files_screen.files.iter().position(|f| f.fid == fid) else {
                        return;
                    };

                    files_screen.files.remove(position);
                }
                _ => {}
            },
            "insert" => match (event_type, event_id) {
                (Some(0), Some(pid)) => {
                    let Some(screen) = &mut self.screens.packages else {
                        return;
                    };
                    let Ok(package) = fetch_package_data(pid).await else {
                        return;
                    };
                    let position = screen.packages.partition_point(|p| p.order < package.order);

                    screen.packages.insert(position, package);
                }
                (Some(1), Some(fid)) => {
                    let Ok(file) = fetch_file_data(fid).await else {
                        return;
                    };
                    let Some(files_screen) = &mut self.screens.files else {
                        return;
                    };
                    if files_screen.package_id != file.package_id {
                        return;
                    }
                    let Ok(files) = fetch_files(file.package_id).await else {
                        return;
                    };

                    files_screen.files = files;
                }
                _ => {}
            },
            "reload" => {
                if let Some(screen) = &mut self.screens.packages
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
        if self
            .previous_screen
            .as_ref()
            .is_none_or(|s| &self.current_screen == s)
        {
            return;
        }

        let prev = self.previous_screen.take().unwrap();
        self.previous_screen = Some(self.current_screen);
        self.current_screen = prev;
    }

    fn go_to_add_package_form(&mut self) {
        if matches!(self.current_screen, Screen::AddPackageForm) {
            return;
        }

        if self.screens.add_package_form.is_none() {
            self.screens.add_package_form = Some(AddPackageForm::default());
        }

        self.previous_screen = Some(self.current_screen);
        self.current_screen = Screen::AddPackageForm;
    }

    fn go_to_packages(&mut self) {
        if matches!(self.current_screen, Screen::Packages) {
            return;
        }

        if self.screens.packages.is_none() {
            self.screens.packages = Some(PackagesScreen::default());
        }

        self.previous_screen = Some(self.current_screen);
        self.current_screen = Screen::Packages;
    }

    async fn go_to_files(&mut self, pid: i32, name: String) {
        if matches!(self.current_screen, Screen::Files) {
            return;
        }

        if self
            .screens
            .files
            .as_ref()
            .is_none_or(|s| s.package_id != pid)
        {
            self.screens.files = Some(FilesScreen::new(pid, name).await);
        }

        self.previous_screen = Some(self.current_screen);
        self.current_screen = Screen::Files;
    }

    fn go_to_append_files_form(&mut self, pid: i32, name: String) {
        if matches!(self.current_screen, Screen::AppendFilesForm) {
            return;
        }

        self.screens.append_files_form = Some(AppendFilesForm::new(pid, name));
        self.previous_screen = Some(self.current_screen);
        self.current_screen = Screen::AppendFilesForm;
    }

    pub fn get_bindings(&self) -> Vec<(&'static str, &'static str)> {
        match self.current_screen {
            Screen::AddPackageForm => vec![
                ("Esc", "back"),
                ("Tab", "next"),
                ("Shift+Tab", "prev"),
                ("Enter", "newline/toggle/submit"),
            ],
            Screen::AppendFilesForm => vec![
                ("Esc", "back"),
                ("Tab", "next"),
                ("Shift+Tab", "prev"),
                ("Enter", "newline/submit"),
            ],
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
                if matches!(self.current_screen, Screen::Files) {
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
            screens: Screens {
                packages: Some(packages),
                files: None,
                add_package_form: None,
                append_files_form: None,
            },
            current_screen: Screen::Packages,
            previous_screen: None,
            quit: false,
        }
    }
}
