use ratatui::{
    prelude::{Buffer, Rect}, style::{Color, Style}, text::Span, widgets::Widget,
};

#[derive(Clone)]
pub struct StatusBar {
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Span::styled("PAUSED",
            Style::new().bg(Color::Yellow).fg(Color::Black).bold()
        ).render(area, buf);
    }
}
