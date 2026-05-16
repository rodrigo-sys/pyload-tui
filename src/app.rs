use std::collections::BTreeMap;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
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
            },
            current_screen: CurrentScreen::Packages,
            quit: false,
        }
    }

    pub async fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('q') {
            self.quit = true;
            return;
    pub async fn handle_events(&mut self, event: Event) {
        match event {
            Event::Key(key) => self.handle_key(key).await,
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
            }
            _ => None,
        };

        if action.is_none() {
            action = match key.code {
                KeyCode::Char('q') => Some(AppAction::Quit),
                _ => None,
            };
        }

        match action {
            Some(AppAction::Quit) => self.quit = true,
                self.go_to_packages();
            Some(AppAction::GoToPackages) => self.go_to_packages(),
            Some(AppAction::GoToFiles(pid)) => self.go_to_files(pid).await,
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
            },
            current_screen: CurrentScreen::Packages,
            quit: false,
        }
    }
}
