use crossterm::event::{KeyCode, KeyEvent};
use openapi::models::{DownloadInfo, DownloadStatus};
use ratatui::{
    layout::Constraint,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Row, StatefulWidget, Table, TableState},
};

use crate::{app_action::AppAction, screens::ScreenHandler, table::format_bytes};

#[derive(Clone)]
pub struct DownloadsScreen {
    pub downloads_info: Vec<DownloadInfo>,
    pub table_state: TableState,
}

impl DownloadsScreen {
    pub fn new() -> Self {
        Self {
            downloads_info: vec![],
            table_state: TableState::new().with_selected(0),
        }
    }
}

impl Default for DownloadsScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenHandler for DownloadsScreen {
    async fn handle_keys(&mut self, key: KeyEvent) -> Option<AppAction> {
        match key.code {
            KeyCode::Char('q') => Some(AppAction::Quit),
            KeyCode::Char('h') => Some(AppAction::GoToPreviousScreen),
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

impl StatefulWidget for DownloadsScreen {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(DownloadsTable::from(self.downloads_info).0, area, buf, state);
    }
}

pub struct DownloadsTable(pub Table<'static>);

impl From<Vec<DownloadInfo>> for DownloadsTable {
    fn from(downloads_info: Vec<DownloadInfo>) -> Self {
        let rows = downloads_info.into_iter().map(|d| {
            let info = match d.status {
                DownloadStatus::DOWNLOADING => {
                    format!("{} @ {}/s", d.format_eta, format_bytes(d.speed))
                }
                DownloadStatus::WAITING => format!("waiting {}", d.format_wait),
                _ => String::new(),
            };

            Row::new(vec![
                d.statusmsg,
                d.name,
                d.plugin,
                info,
                d.format_size,
                d.percent.to_string(),
            ])
        });

        let header =
            Row::new(vec!["Status", "Name", "Hoster", "Info", "Size", "Progress"])
                .style(Style::default().bold());

        let widths = [
            Constraint::Min(5),
            Constraint::Min(5),
            Constraint::Min(5),
            Constraint::Min(5),
            Constraint::Min(5),
            Constraint::Min(5),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(Style::new().bg(Color::Blue).fg(Color::Black).bold());

        DownloadsTable(table)
    }
}
