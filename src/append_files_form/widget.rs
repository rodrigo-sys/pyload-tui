use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    style::Style,
    widgets::{Block, Borders, Padding, Widget},
};

use super::SelectedInput;
use crate::append_files_form::AppendFilesForm;

impl Widget for AppendFilesForm {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let container = Block::new()
            .padding(Padding::uniform(3))
            .title(format!(" Add Links to {} ", self.package_name));

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

        let links_active = self.selected == SelectedInput::Links;
        let links_color = if links_active {
            ratatui::style::Color::Yellow
        } else {
            ratatui::style::Color::Blue
        };
        let mut links = self.links.clone();
        links.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(links_color)
                .title("links")
                .padding(Padding::new(1, 1, 0, 0)),
        );
        if !links_active {
            links.set_cursor_style(Style::default());
        }
        links.render(rows[0], buf);

        let submit_active = self.selected == SelectedInput::AddLinks;
        let (border_color, text_color) = if submit_active {
            (ratatui::style::Color::Yellow, ratatui::style::Color::Yellow)
        } else {
            (ratatui::style::Color::Blue, ratatui::style::Color::Blue)
        };
        self.submit = ratatui::widgets::Paragraph::new("Add links")
            .alignment(ratatui::layout::HorizontalAlignment::Center)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_style(border_color)
                    .style(Style::new().fg(text_color)),
            );
        self.submit.render(add_links_layout[0], buf);
    }
}
