use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app_action::AppAction, screens::{CurrentScreen, Screens}, files_screen::FilesScreen,
    packages_screen::PackagesScreen,
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
            },
            current_screen: CurrentScreen::Packages,
            quit: false,
        }
    }

    pub async fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('q') {
            self.quit = true;
            return;
        }

        let action = match &mut self.current_screen {
            CurrentScreen::Packages => {
                let s = self.screens.packages.as_mut().unwrap();
                s.handle_keys(key)
            }
            CurrentScreen::Files => {
                let s = self.screens.files.as_mut().unwrap();
                s.handle_keys(key)
            }
            _ => return,
        };

        match action {
            Some(AppAction::GoToPackages) => {
                self.go_to_packages();
            }
            Some(AppAction::GoToFiles(pid)) => {
                self.go_to_files(pid).await;
            }
            _ => {}
        }
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
}

impl Default for App {
    fn default() -> Self {
        let packages = PackagesScreen::default();
        Self {
            screens: Screens {
                packages: Some(packages),
                files: None,
            },
            current_screen: CurrentScreen::Packages,
            quit: false,
        }
    }
}
