use crossterm::event::{KeyCode, KeyEvent};
use openapi::{apis::py_load_rest_api::api_get_events_get, models::PackageData};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, TableState},
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
            table_state: TableState::new().with_selected(0),
        }
    }

    pub async fn handle_keys(&mut self, key: KeyEvent) -> Option<AppAction> {
        match key.code {
            KeyCode::Char('q') => Some(AppAction::Quit),
            KeyCode::Char('a') => {
                let index = self.table_state.selected().unwrap();
                let pkg = &self.packages[index];
                Some(AppAction::OpenAppendFilesForm(pkg.pid, pkg.name.clone()))
            }
            KeyCode::Char('d') => {
                let index = self.table_state.selected()?;
                Some(AppAction::DeletePackages(vec![self.packages[index].pid]))
            }
            KeyCode::Char('l') => {
                let index = self.table_state.selected().unwrap();
                let pkg = &self.packages[index];
                Some(AppAction::GoToFiles(pkg.pid, pkg.name.clone()))
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
        Self {
            packages: vec![],
            table_state: TableState::new(),
        }
    }
}

impl StatefulWidget for PackagesScreen {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(
            PackagesTable::from(self.packages.clone()).0,
            area,
            buf,
            state,
        );
    }
}

