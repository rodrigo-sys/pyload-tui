use openapi::models::{DownloadInfo, FileData, PackageData};
use ratatui::layout::Constraint;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Cell, Row, Table};

const HIGHLIGHT_STYLE: Style = Style::new().bg(Color::LightBlue).fg(Color::Black).bold();

pub struct PackagesTable(pub Table<'static>);
pub struct FilesTable(pub Table<'static>);

impl From<Vec<PackageData>> for PackagesTable {
    fn from(packages: Vec<PackageData>) -> Self {
        let rows: Vec<Row<'static>> = packages
            .into_iter()
            .map(|p| {
                let links = match (&p.linksdone, &p.linkstotal) {
                    (Some(Some(done)), Some(Some(total))) => format!("{}/{}", done, total),
                    _ => "0/0".to_string(),
                };
                let size = match (&p.sizedone, &p.sizetotal) {
                    (Some(Some(done)), Some(Some(total))) => format_size(*done, *total),
                    _ => "0 B / 0 B".to_string(),
                };
                Row::new(vec![
                    Cell::from(p.order.to_string()),
                    Cell::from(p.name),
                    Cell::from(links),
                    Cell::from(size),
                    Cell::from(p.folder),
                    Cell::from(match p.dest {
                        0 => "COLLECTOR",
                        _ => "QUEUE",
                    }),
                ])
            })
            .collect();

        let header = Row::new(vec![
            Cell::from("#"),
            Cell::from("Name"),
            Cell::from("Links"),
            Cell::from("Size"),
            Cell::from("Folder"),
            Cell::from("Dest"),
        ])
        .style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD));

        let widths = [
            Constraint::Length(5),
            Constraint::Length(40),
            Constraint::Length(10),
            Constraint::Length(15),
            Constraint::Length(30),
            Constraint::Length(10),
        ];

        let table = Table::new(rows, widths).header(header).row_highlight_style(HIGHLIGHT_STYLE);

        PackagesTable(table)
    }
}

impl From<(Vec<FileData>, Vec<DownloadInfo>)> for FilesTable {
    fn from((files, downloads): (Vec<FileData>, Vec<DownloadInfo>)) -> Self {
        let rows: Vec<Row<'static>> = files
            .into_iter()
            .map(|f| {
                let dl = downloads.iter().find(|d| d.fid == f.fid);

                let size = f.format_size.clone();

                let speed = dl
                    .filter(|d| d.speed > 0)
                    .map(|d| format!("{}/s", format_bytes(d.speed)))
                    .unwrap_or_default();

                let progress = dl
                    .filter(|d| d.percent > 0)
                    .map(|d| format!("{:.1}%", d.percent))
                    .unwrap_or_default();

                let eta = dl
                    .map(|d| d.format_eta.clone())
                    .filter(|s| !s.is_empty())
                    .unwrap_or_default();

                Row::new(vec![
                    Cell::from(f.fid.to_string()),
                    Cell::from(f.name),
                    Cell::from(size),
                    Cell::from(format!("{:?}", f.status)),
                    Cell::from(speed),
                    Cell::from(progress),
                    Cell::from(eta),
                ])
            })
            .collect();

        let header = Row::new(vec![
            Cell::from("FID"),
            Cell::from("Name"),
            Cell::from("Size"),
            Cell::from("Status"),
            Cell::from("Speed"),
            Cell::from("%"),
            Cell::from("ETA"),
        ])
        .style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD));

        let widths = [
            Constraint::Length(5),
            Constraint::Min(25),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(8),
            Constraint::Length(10),
        ];

        let table = Table::new(rows, widths).header(header).row_highlight_style(HIGHLIGHT_STYLE);

        FilesTable(table)
    }
}

fn format_size(done: i64, total: i64) -> String {
    let done_str = format_bytes(done);
    let total_str = format_bytes(total);
    format!("{} / {}", done_str, total_str)
}

fn format_bytes(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
