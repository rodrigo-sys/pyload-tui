use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::{
    add_package_form::AddPackageForm,
    app_action::AppAction,
    files_screen::FilesScreen,
    packages_screen::PackagesScreen,
    screens::{CurrentScreen, Screens},
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
            },
            current_screen: CurrentScreen::Packages,
            quit: false,
        }
    }

    pub async fn handle_events(&mut self, event: Event) {
        match event {
            Event::Key(key) => self.handle_key(key).await,
            Event::Paste(content) => {
                if let CurrentScreen::AddPackageForm = self.current_screen
                    && let Some(f) = self.screens.add_package_form.as_mut()
                {
                    f.handle_paste(&content);
                }
            }
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
            _ => None,
        };

        if action.is_none() {
            action = match key.code {
                KeyCode::Char('q') => {
                    (!matches!(self.current_screen, CurrentScreen::AddPackageForm))
                        .then_some(AppAction::Quit)
                }
                KeyCode::Char('a') => Some(AppAction::OpenAddPackageForm),
                _ => None,
            };
        }

        match action {
            Some(AppAction::Quit) => self.quit = true,
            Some(AppAction::OpenAddPackageForm) => self.go_to_add_package_form(),
            Some(AppAction::GoToPackages) => self.go_to_packages(),
            Some(AppAction::GoToFiles(pid)) => self.go_to_files(pid).await,
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

        if self.screens.files.is_none() {
            self.screens.files = Some(FilesScreen::new(pid).await);
        }

        self.current_screen = CurrentScreen::Files;
    }

    pub fn get_bindings(&self) -> BTreeMap<&'static str, &'static str> {
        let mut table_screen_binds = BTreeMap::from([
            ("l", "enter"),
            ("j", "next item"),
            ("k", "prev item"),
            ("q", "quit"),
        ]);

        match self.current_screen {
            CurrentScreen::Packages => table_screen_binds,
            CurrentScreen::Files => {
                table_screen_binds.insert("h", "go back");
                table_screen_binds
            }
            CurrentScreen::AddPackageForm => todo!(),
            CurrentScreen::AppendFilesForm => todo!(),
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
            },
            current_screen: CurrentScreen::Packages,
            quit: false,
        }
    }
}
