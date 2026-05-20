use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::{
    add_package_form::AddPackageForm,
    app_action::AppAction,
    append_files_form::AppendFilesForm,
    files_screen::FilesScreen,
    packages_screen::PackagesScreen,
    screens::{Screen, Screens},
    utils::{remove_files_from_package, remove_packages},
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
                if let Some(&(index, package_id)) = packages.first() {
                    // remove package itself
                    let _ = remove_packages(vec![package_id]).await;
                    // remove corresponding item from UI table
                    if let Some(packages_screen) = self.screens.packages.as_mut() {
                        packages_screen.packages.remove(index);
                    }
                }
            }
            Some(AppAction::DeleteFiles(files)) => {
                if let Some(&(index, file_id)) = files.first() {
                    let _ = remove_files_from_package(vec![file_id]).await;
                    if let Some(files_screen) = self.screens.files.as_mut() {
                        files_screen.files.remove(index);
                    }
                }
            }
            Some(AppAction::GoToPreviousScreen) => {
                self.go_to_previous_screen();
            }
            _ => {}
        }
    }

    fn go_to_previous_screen(&mut self) {
        if self.previous_screen.as_ref().is_none_or(|s| &self.current_screen == s){
            return
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

        if self.screens.files.as_ref().is_none_or(|s| s.package_id != pid) {
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
