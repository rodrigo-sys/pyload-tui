use crossterm::event::{KeyCode, KeyEvent};
use openapi::models::FileData;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, TableState, Widget},
};

use crate::{app_action::AppAction, table::FilesTable, utils::fetch_files};

#[derive(Clone)]
pub struct FilesScreen {
    pub package_id: i32,
    pub files: Vec<FileData>,
    pub table_state: TableState,
}

impl FilesScreen {
    pub async fn new(package_id: i32) -> Self {
        let files = fetch_files(package_id).await.unwrap_or_default();
         Self {
            package_id,
            files,
            table_state: TableState::new().with_selected(0),
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent) -> Option<AppAction> {
        match key.code {
            // KeyCode::Char('q') | KeyCode::Esc => Some(AppAction::GoToPackages),
            KeyCode::Char('h') => Some(AppAction::GoToPackages),
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

impl Default for FilesScreen {
    fn default() -> Self {
        Self { package_id: 0, files: vec![], table_state: TableState::new() }
    }
}

impl Widget for &FilesScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.table_state;
        StatefulWidget::render(FilesTable::from(self.files.clone()).0, area, buf, &mut state);
    }
}
