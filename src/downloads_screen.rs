use openapi::models::DownloadInfo;
use ratatui::{layout::Constraint, widgets::{Row, Table}};

pub struct DownloadsScreen {
}

pub struct DownloadsTable(pub Table<'static>);

impl From<Vec<DownloadInfo>> for DownloadsTable {
    fn from(downloads_info: Vec<DownloadInfo>) -> Self {

        let rows = downloads_info.into_iter().map(|d| {
            Row::new(
                vec![ d.statusmsg, d.name, d.plugin, d.info, d.format_size, d.percent.to_string()]
            )
        });

        let header = Row::new(vec!["Status", "Name", "Hoster", "Info", "Size", "Progress"]);

        let widths = [ Constraint::Length(10), Constraint::Length(10)];

        let table = Table::new(rows, widths).header(header);

        DownloadsTable(table)
    }
}
