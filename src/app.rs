use crossterm::event::KeyCode;
use openapi::models::{FileData, PackageData};
use ratatui::widgets::TableState;

use crate::{current_screen::CurrentScreen, packages_screen::PackagesScreen};

pub struct App {
    pub current_screen: CurrentScreen,
}

impl App {
    pub async fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Packages(PackagesScreen::new().await),
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {}
}

impl Default for App {
    fn default() -> Self {
        Self { current_screen: CurrentScreen::Packages(PackagesScreen::default()) }
    }
}
