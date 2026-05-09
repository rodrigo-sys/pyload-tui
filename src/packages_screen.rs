use crossterm::event::{KeyCode, KeyEvent};
use openapi::models::PackageData;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, TableState, Widget},
};

use crate::{app_action::AppAction, table::PackagesTable, utils::fetch_packages};

#[derive(Clone)]
pub struct PackagesScreen {
    pub packages: Vec<PackageData>,
    pub table_state: TableState,
}

impl PackagesScreen {
    pub async fn new() -> Self {
        let packages = fetch_packages().await.unwrap_or_default();
        Self {
            packages,
            table_state: TableState::new(),
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent) -> Option<AppAction> {
        match key.code {
            KeyCode::Char('q') => Some(AppAction::Quit),
            KeyCode::Char('l') => {
                let index = self.table_state.selected().unwrap();
                let pid = self.packages[index].pid;
                Some(AppAction::GoToFiles(pid))
            }
            KeyCode::Char('j') => {
                self.table_state.select_next();
                None
            }
            KeyCode::Char('k') => {
                self.table_state.select_previous();
                None
            }
            _ => None,
        }
    }
}

impl Default for PackagesScreen {
    fn default() -> Self {
        Self { packages: vec![], table_state: TableState::new() }
    }
}

impl Widget for &PackagesScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.table_state;
        StatefulWidget::render(PackagesTable::from(self.packages.clone()).0, area, buf, &mut state);
    }
}
