use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::{
    add_package_form::AddPackageForm,
    app_action::AppAction,
    append_files_form::AppendFilesForm,
    files_screen::FilesScreen,
    packages_screen::PackagesScreen,
    screens::{CurrentScreen, Screens},
    utils::remove_packages,
};

pub struct App {
    pub screens: Screens,
    pub current_screen: CurrentScreen,
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
            current_screen: CurrentScreen::Packages,
            quit: false,
        }
    }

    pub async fn handle_events(&mut self, event: Event) {
        match event {
            Event::Key(key) => self.handle_key(key).await,
            Event::Paste(content) => match self.current_screen {
                CurrentScreen::AddPackageForm => {
                    if let Some(f) = self.screens.add_package_form.as_mut() {
                        f.handle_paste(&content);
                    }
                }
                CurrentScreen::AppendFilesForm => {
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
            CurrentScreen::Packages => {
                let s = self.screens.packages.as_mut().unwrap();
                s.handle_keys(key).await
            }
            CurrentScreen::Files => {
                let s = self.screens.files.as_mut().unwrap();
                s.handle_keys(key).await
            }
            CurrentScreen::AddPackageForm => {
                let s = self.screens.add_package_form.as_mut().unwrap();
                s.handle_keys(key).await
            }
            CurrentScreen::AppendFilesForm => {
                let s = self.screens.append_files_form.as_mut().unwrap();
                s.handle_keys(key).await
            }
        };

        if action.is_none() {
            action = match key.code {
                KeyCode::Char('q') => {
                    (!matches!(self.current_screen, CurrentScreen::AddPackageForm))
                        .then_some(AppAction::Quit)
                }
                KeyCode::Char('A') => Some(AppAction::OpenAddPackageForm),
                _ => None,
            };
        }

        match action {
            Some(AppAction::Quit) => self.quit = true,
            Some(AppAction::OpenAddPackageForm) => self.go_to_add_package_form(),
            Some(AppAction::OpenAppendFilesForm(pid, name)) => self.go_to_append_files_form(pid, name),
            Some(AppAction::GoToPackages) => self.go_to_packages(),
            Some(AppAction::GoToFiles(pid)) => self.go_to_files(pid).await,
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
            _ => {}
        }
    }

    fn go_to_add_package_form(&mut self) {
        if matches!(self.current_screen, CurrentScreen::AddPackageForm) {
            return;
        }

        if self.screens.add_package_form.is_none() {
            self.screens.add_package_form = Some(AddPackageForm::default());
        }

        self.current_screen = CurrentScreen::AddPackageForm;
    }

    fn go_to_packages(&mut self) {
        if matches!(self.current_screen, CurrentScreen::Packages) {
            return;
        }

        if self.screens.packages.is_none() {
            self.screens.packages = Some(PackagesScreen::default());
        }

        self.current_screen = CurrentScreen::Packages;
    }

    async fn go_to_files(&mut self, pid: i32) {
        if matches!(self.current_screen, CurrentScreen::Files) {
            return;
        }

        if self.screens.files.as_ref().is_none_or(|s| s.package_id != pid) {
            self.screens.files = Some(FilesScreen::new(pid).await);
        }

        self.current_screen = CurrentScreen::Files;
    }

    fn go_to_append_files_form(&mut self, pid: i32, name: String) {
        if matches!(self.current_screen, CurrentScreen::AppendFilesForm) {
            return;
        }

        self.screens.append_files_form = Some(AppendFilesForm::new(pid, name));
        self.current_screen = CurrentScreen::AppendFilesForm;
    }

    pub fn get_bindings(&self) -> Vec<(&'static str, &'static str)> {
        match self.current_screen {
            CurrentScreen::AddPackageForm => vec![
                ("Esc", "back"),
                ("Tab", "next"),
                ("Shift+Tab", "prev"),
                ("Enter", "newline/toggle/submit"),
            ],
            CurrentScreen::AppendFilesForm => vec![
                ("Esc", "back"),
                ("Tab", "next"),
                ("Shift+Tab", "prev"),
                ("Enter", "newline/submit"),
            ],
            _ => {
                let mut binds = vec![
                    ("a", "add links"),
                    ("A", "add package"),
                    ("l", "enter"),
                    ("j", "next item"),
                    ("k", "prev item"),
                    ("q", "quit"),
                ];
                if matches!(self.current_screen, CurrentScreen::Files) {
                    binds.push(("h", "go back"));
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
            current_screen: CurrentScreen::Packages,
            quit: false,
        }
    }
}
