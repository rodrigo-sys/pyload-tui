use ratatui::{
    layout::{Constraint, Direction, HorizontalAlignment, Layout},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use ratatui_textarea::TextArea;
use tui_checkbox::Checkbox;

use super::SelectedInput;
use crate::add_package_form::AddPackageForm;

fn input_block(title: &'static str, selected: bool) -> Block<'static> {
    let color = if selected { Color::Yellow } else { Color::Blue };
    Block::default()
        .borders(Borders::ALL)
        .border_style(color)
        .title(title)
        .padding(Padding::new(1, 1, 0, 0))
}

fn checkbox_block(selected: bool) -> Block<'static> {
    let color = if selected { Color::Yellow } else { Color::Black };
    Block::default()
        .borders(Borders::ALL)
        .border_style(color)
        .padding(Padding::new(1, 1, 0, 0))
}

fn base_checkbox(label: &'static str, checked: bool) -> Checkbox<'static> {
    Checkbox::new(label, checked)
        .checked_symbol("\u{f111} ")
        .unchecked_symbol("\u{f10c} ")
}

fn textarea_with_block(title: &'static str) -> TextArea<'static> {
    let mut textarea = TextArea::default();
    textarea.set_block(input_block(title, false));
    textarea
}

impl Widget for AddPackageForm {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let container = Block::new()
            .padding(Padding::uniform(3))
            .title(Line::from(" Package Form ").centered());

        let inner = container.inner(area);

        let links_height = (self.links.lines().len() as u16).max(3) + 2;

        let rows = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Percentage(15),
                Constraint::Length(links_height),
                Constraint::Percentage(15),
                Constraint::Length(3),
                Constraint::Percentage(15),
            ],
        )
        .split(inner);

        let checkbox_layout = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Percentage(13),
                Constraint::Percentage(11),
                Constraint::Percentage(15),
            ],
        )
        .split(rows[3]);

        let add_package_layout =
            Layout::new(Direction::Horizontal, vec![Constraint::Percentage(15)]).split(rows[4]);

        container.render(area, buf);

        for (textarea, title, variant) in [
            (&mut self.name,     "name",     SelectedInput::Name),
            (&mut self.links,    "links",    SelectedInput::Links),
            (&mut self.password, "password", SelectedInput::Password),
        ] {
            let active = self.selected == variant;
            textarea.set_block(input_block(title, active));
            if !active {
                textarea.set_cursor_style(Style::default());
            }
        }

        self.queue = base_checkbox("queue", self.queue_checked)
            .block(checkbox_block(self.selected == SelectedInput::Queue));
        self.collector = base_checkbox("collector", self.collector_checked)
            .block(checkbox_block(self.selected == SelectedInput::Collector));

        let add_package_active = self.selected == SelectedInput::AddPackage;
        let (border_color, text_color) = if add_package_active {
            (Color::Yellow, Color::Yellow)
        } else {
            (Color::Blue, Color::Blue)
        };
        self.add_package = Paragraph::new("Add package")
            .alignment(HorizontalAlignment::Center)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_style(border_color)
                    .style(Style::new().fg(text_color)),
            );

        self.name.render(rows[0], buf);
        self.links.render(rows[1], buf);
        self.password.render(rows[2], buf);
        self.destination_label.render(checkbox_layout[0], buf);
        self.queue.render(checkbox_layout[1], buf);
        self.collector.render(checkbox_layout[2], buf);
        self.add_package.render(add_package_layout[0], buf);
    }
}
