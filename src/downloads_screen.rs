use openapi::models::{DownloadInfo, DownloadStatus};
use ratatui::{
    layout::Constraint,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Row, StatefulWidget, Table, TableState},
};

#[derive(Clone)]
pub struct DownloadsScreen {
    downloads_info: Vec<DownloadInfo>,
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
        Self {
            downloads_info: vec![],
            table_state: TableState::new(),
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
            Row::new(vec![
                d.statusmsg,
                d.name,
                d.plugin,
                d.info,
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
