use crossterm::event::{KeyCode, KeyEvent};
use openapi::models::FileData;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, TableState},
};

use crate::{app_action::AppAction, table::FilesTable, utils::fetch_files};

#[derive(Clone)]
pub struct FilesScreen {
    pub package_id: i32,
    pub package_name: String,
    pub files: Vec<FileData>,
    pub table_state: TableState,
}

impl FilesScreen {
    pub async fn new(package_id: i32, package_name: String) -> Self {
        let files = fetch_files(package_id).await.unwrap_or_default();
        Self {
            package_id,
            package_name,
            files,
            table_state: TableState::new().with_selected(0),
        }
    }

    pub async fn handle_keys(&mut self, key: KeyEvent) -> Option<AppAction> {
        match key.code {
            KeyCode::Char('a') => Some(AppAction::OpenAppendFilesForm(self.package_id, self.package_name.clone())),
            KeyCode::Char('q') => Some(AppAction::Quit),
            KeyCode::Char('h') => Some(AppAction::GoToPackages),
            KeyCode::Char('j') => {
                self.table_state.select_next();
                None
            }
            KeyCode::Char('k') => {
                self.table_state.select_previous();
                None
            }
            KeyCode::Char('d') => {
                let file_index = self.table_state.selected()?;
                let file = &self.files[file_index];
                Some(AppAction::DeleteFiles(vec![(file_index, file.fid)]))
            }
            _ => None,
        }
    }
}

impl Default for FilesScreen {
    fn default() -> Self {
        Self { package_id: 0, package_name: String::new(), files: vec![], table_state: TableState::new() }
    }
}

impl StatefulWidget for FilesScreen {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(FilesTable::from(self.files.clone()).0, area, buf, state);
    }
}
