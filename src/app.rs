use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app_action::AppAction, current_screen::CurrentScreen, files_screen::FilesScreen,
    packages_screen::PackagesScreen,
};

pub struct App {
    pub current_screen: CurrentScreen,
    pub quit: bool,
}

impl App {
    pub async fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Packages(PackagesScreen::new().await),
            quit: false,
        }
    }

    pub async fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('q') {
            self.quit = true;
            return;
        }

        let action = match &mut self.current_screen {
            CurrentScreen::Packages(s) => s.handle_keys(key),
            CurrentScreen::Files(s) => s.handle_keys(key),
            _ => return,
        };

        match action {
            Some(AppAction::GoToPackages) => {
                if !matches!(self.current_screen, CurrentScreen::Packages(_)) {
                    self.current_screen = CurrentScreen::Packages(PackagesScreen::new().await);
                }
            }
            Some(AppAction::GoToFiles(pid)) => {
                if !matches!(self.current_screen, CurrentScreen::Files(_)) {
                    self.current_screen = CurrentScreen::Files(FilesScreen::new(pid).await);
                }
            }
            _ => {}
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_screen: CurrentScreen::Packages(PackagesScreen::default()),
            quit: false,
        }
    }
}
