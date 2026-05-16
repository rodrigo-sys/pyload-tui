use crossterm::event::{KeyCode, KeyEvent};
use openapi::models::Destination;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_textarea::TextArea;
use tui_checkbox::Checkbox;

use super::SelectedInput;
use crate::{app_action::AppAction, utils::add_package};

use ratatui::layout::HorizontalAlignment;
use ratatui::text::Line;

#[derive(Clone)]
pub struct AddPackageForm {
    pub name: TextArea<'static>,
    pub links: TextArea<'static>,
    pub password: TextArea<'static>,
    pub selected: SelectedInput,
    pub queue: Checkbox<'static>,
    pub collector: Checkbox<'static>,
    pub queue_checked: bool,
    pub collector_checked: bool,
    pub add_package: Paragraph<'static>,
    pub destination_label: Paragraph<'static>,
}

impl Default for AddPackageForm {
    fn default() -> Self {
        Self {
            name: TextArea::default(),
            links: TextArea::default(),
            password: TextArea::default(),
            selected: SelectedInput::default(),
            queue: Checkbox::new("Queue", false)
                .checked_symbol("\u{f111} ")
                .unchecked_symbol("\u{f10c} "),
            collector: Checkbox::new("collector", true)
                .checked_symbol("\u{f111} ")
                .unchecked_symbol("\u{f10c} "),
            queue_checked: false,
            collector_checked: true,
            add_package: Paragraph::new("Add package")
                .alignment(HorizontalAlignment::Center)
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_style(Color::Green)
                        .style(Style::new().fg(Color::Green)),
                ),
            destination_label: Paragraph::new(Line::from(" Destination")).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Color::Black),
            ),
        }
    }
}

impl AddPackageForm {
    pub async fn handle_keys(&mut self, key: KeyEvent) -> Option<AppAction> {
        match key.code {
            KeyCode::Esc => Some(AppAction::GoToPackages),
            KeyCode::Tab => {
                self.selected = self.selected.next();
                None
            }
            KeyCode::BackTab => {
                self.selected = self.selected.prev();
                None
            }
            KeyCode::Enter => match self.selected {
                SelectedInput::AddPackage => self.submit().await,
                SelectedInput::Name | SelectedInput::Links | SelectedInput::Password => {
                    self.handle_text_input(key);
                    None
                }
                _ => {
                    self.handle_checkbox_toggle();
                    None
                }
            },
            _ => {
                self.handle_text_input(key);
                None
            }
        }
    }

    async fn submit(&self) -> Option<AppAction> {
        let name = self.name.lines().join("\n");

        let links = self
            .links
            .lines()
            .iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let password = {
            let p = self.password.lines().join("");
            if p.is_empty() { None } else { Some(p) }
        };

        let dest = if self.collector_checked {
            Destination::COLLECTOR
        } else {
            Destination::QUEUE
        };

        add_package(name, links, password, dest).await.ok()?;
        Some(AppAction::GoToPackages)
    }

    pub fn handle_paste(&mut self, content: &str) {
        match self.selected {
            SelectedInput::Name => { self.name.insert_str(content); }
            SelectedInput::Links => { self.links.insert_str(content); }
            SelectedInput::Password => { self.password.insert_str(content); }
            _ => {}
        }
    }

    fn handle_text_input(&mut self, key: KeyEvent) {
        match self.selected {
            SelectedInput::Name => {
                self.name.input(key);
            }
            SelectedInput::Links => {
                self.links.input(key);
            }
            SelectedInput::Password => {
                self.password.input(key);
            }
            _ => {}
        }
    }

    fn handle_checkbox_toggle(&mut self) {
        let (primary_checked, primary, other_checked, other) = match self.selected {
            SelectedInput::Queue => (
                &mut self.queue_checked,
                &mut self.queue,
                &mut self.collector_checked,
                &mut self.collector,
            ),
            SelectedInput::Collector => (
                &mut self.collector_checked,
                &mut self.collector,
                &mut self.queue_checked,
                &mut self.queue,
            ),
            _ => return,
        };

        *primary_checked = !*primary_checked;
        *primary = primary.clone().checked(*primary_checked);
        if *primary_checked {
            *other_checked = false;
            *other = other.clone().checked(false);
        }
    }
}
