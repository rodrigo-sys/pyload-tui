use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Buffer, Color, Line, Rect, Span, Style},
    widgets::{Paragraph, Widget},
};

pub struct KeyHints<'a> {
    pub bindings: &'a [(&'a str, &'a str)],
}

impl<'a> KeyHints<'a> {
    pub const fn new(bindings: &'a [(&'a str, &'a str)]) -> Self {
        Self { bindings }
    }
}

impl Widget for KeyHints<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut line = Line::default();

        for (key, action) in self.bindings {
            let key_style = Style::default().fg(Color::Cyan);
            let action_style = Style::default().fg(Color::Gray);

            line.spans.push(Span::raw("["));
            line.spans.push(Span::styled(*key, key_style));
            line.spans.push(Span::raw("] "));
            line.spans.push(Span::styled(*action, action_style));
            line.spans.push(Span::raw("  "));
        }

        let width = line.width() as u16;
        let layout = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .split(area);

        Paragraph::new(line)
            .style(Style::default().fg(Color::DarkGray))
            .render(layout[0], buf);
    }
}
