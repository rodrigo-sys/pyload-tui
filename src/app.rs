use crossterm::event::KeyCode;
use openapi::models::{FileData, PackageData};
use ratatui::widgets::TableState;

use crate::current_screen::CurrentScreen;

pub struct App {
    pub current_screen: CurrentScreen,
    pub packages: Vec<PackageData>,
    pub files: Vec<FileData>,
    pub packages_table_state: TableState,
    pub files_table_state: TableState,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Packages,
            packages: Vec::new(),
            files: Vec::new(),
            packages_table_state: TableState::new(),
            files_table_state: TableState::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        let state = match self.current_screen {
            CurrentScreen::Packages => &mut self.packages_table_state,
            CurrentScreen::Files => &mut self.files_table_state,
            _ => return,
        };

        match key {
            KeyCode::Char('j') => state.select_next(),
            KeyCode::Char('k') => state.select_previous(),
            _ => (),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
