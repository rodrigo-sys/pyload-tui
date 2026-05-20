use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
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
        let mut lines = Vec::new();

        for chunks in self.bindings.chunks(5) {
            let mut line = Line::default();

            for (key, action) in chunks {
                let key_style = Style::default().fg(Color::Cyan);
                let action_style = Style::default().fg(Color::Gray);

                line.spans.push(Span::raw("["));
                line.spans.push(Span::styled(*key, key_style));
                line.spans.push(Span::raw("] "));
                line.spans.push(Span::styled(*action, action_style));
                line.spans.push(Span::raw("  "));
            }

            lines.push(line);
        }

        let layout = Layout::vertical(vec![Constraint::Length(1); lines.len()]).spacing(1);
        let areas = layout.split(area);

        for (line, area) in lines.into_iter().zip(areas.iter()) {

            let centered_layout = Layout::new(Direction::Horizontal, vec![Constraint::Length(line.width() as u16)])
                .flex(Flex::Center)
            .split(*area);

            Paragraph::new(line)
                .style(Style::default().fg(Color::DarkGray))
                .render(centered_layout[0], buf);
        }
    }

}
