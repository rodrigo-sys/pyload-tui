use openapi::models::ServerStatus;
use ratatui::{
    layout::Constraint,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table, Widget},
};

#[derive(Clone)]
pub struct StatusBar {
    server_status: Option<ServerStatus>,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            server_status: None,
        }
    }

    pub fn refresh(&mut self, server_status: ServerStatus) {
        if Some(&server_status) == self.server_status.as_ref() {
            return;
        }

        self.server_status = Some(server_status);
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(status) = self.server_status {
            StatusTable::from(status).0.render(area, buf);
        }
    }
}

pub struct StatusTable(pub Table<'static>);

impl From<ServerStatus> for StatusTable {
    fn from(server_status: ServerStatus) -> Self {
        let (queue_label, queue_color) = if server_status.pause {
            ("PAUSED", Color::Yellow)
        } else {
            ("UNPAUSED", Color::Green)
        };

        let rows = vec![
            Row::new(vec!["QUEUE:", queue_label]).style(Style::new().fg(queue_color).bold()),
            Row::new(vec![
                "ACTIVE:".to_string(),
                server_status.active.to_string(),
            ])
            .style(Style::new().fg(Color::Magenta).bold()),
            Row::new(vec![
                "SPEED:".to_string(),
                if server_status.active > 0 {
                    format_speed(server_status.speed)
                } else {
                    "―".to_string()
                },
            ])
            .style(Style::new().fg(Color::Blue).bold()),
        ];

        let table = Table::new(rows, vec![Constraint::Length(10), Constraint::Length(10)])
            .block(Block::default().borders(Borders::ALL));

        StatusTable(table)
    }
}

pub fn format_speed(speed: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = 1024 * KB;
    const GB: i64 = 1024 * MB;

    if speed >= GB {
        format!("{:.1} GB/s", speed as f64 / GB as f64)
    } else if speed >= MB {
        format!("{:.1} MB/s", speed as f64 / MB as f64)
    } else if speed >= KB {
        format!("{:.1} KB/s", speed as f64 / KB as f64)
    } else {
        format!("{} B/s", speed)
    }
}
