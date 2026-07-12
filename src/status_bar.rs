use ratatui::{
    layout::Constraint,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table, Widget},
};

#[derive(Clone)]
pub struct StatusBar {
    table: Table<'static>,
}

impl StatusBar {
    pub fn new() -> Self {
        let rows = vec![
            Row::new(vec!["QUEUE:", "PAUSED"]).style(Style::new().fg(Color::Yellow).bold()),
            Row::new(vec!["ACTIVE:", "0"]).style(Style::new().fg(Color::Magenta).bold()),
            Row::new(vec!["SPEED:", "-"]).style(Style::new().fg(Color::Blue).bold()),
        ];

        let table = Table::new(rows, vec![Constraint::Length(10), Constraint::Length(10)])
            .block(Block::default().borders(Borders::ALL));

        Self { table }
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.table.render(area, buf);
    }
}
