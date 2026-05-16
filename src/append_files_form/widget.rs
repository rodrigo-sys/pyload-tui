use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    style::Style,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

use crate::append_files_form::AppendFilesForm;

fn links_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(ratatui::style::Color::Yellow)
        .title("links")
        .padding(Padding::new(1, 1, 0, 0))
}

impl Widget for AppendFilesForm {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let container = Block::new()
            .padding(Padding::uniform(3))
            .title(" Add Links to Package ");

        let inner = container.inner(area);

        let links_height = (self.links.lines().len() as u16).max(3) + 2;

        let rows = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Length(links_height),
                Constraint::Length(3),
            ],
        )
        .split(inner);

        let add_links_layout =
            Layout::new(Direction::Horizontal, vec![Constraint::Percentage(15)]).split(rows[1]);

        container.render(area, buf);

        let mut links = self.links.clone();
        links.set_block(links_block());
        links.render(rows[0], buf);
        self.submit.render(add_links_layout[0], buf);
    }
}
