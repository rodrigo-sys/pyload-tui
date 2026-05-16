use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use ratatui::layout::HorizontalAlignment;
use ratatui_textarea::TextArea;

use super::SelectedInput;
use crate::{app_action::AppAction, utils::add_links_to_package};

#[derive(Clone)]
pub struct AppendFilesForm {
    pub package_id: i32,
    pub links: TextArea<'static>,
    pub submit: Paragraph<'static>,
    pub selected: SelectedInput,
}

impl AppendFilesForm {
    pub fn new(package_id: i32) -> Self {
        Self {
            package_id,
            links: TextArea::default(),
            submit: Paragraph::new("Add links")
                .alignment(HorizontalAlignment::Center)
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_style(Color::Green)
                        .style(Style::new().fg(Color::Green)),
                ),
            selected: SelectedInput::default(),
        }
    }

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
                SelectedInput::AddLinks => self.submit().await,
                SelectedInput::Links => {
                    self.links.input(key);
                    None
                }
            },
            _ => {
                if self.selected == SelectedInput::Links {
                    self.links.input(key);
                }
                None
            }
        }
    }

    async fn submit(&self) -> Option<AppAction> {
        let links: Vec<String> = self
            .links
            .lines()
            .iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if links.is_empty() {
            return None;
        }

        add_links_to_package(self.package_id, links).await.ok()?;
        Some(AppAction::GoToPackages)
    }

    pub fn handle_paste(&mut self, content: &str) {
        if self.selected == SelectedInput::Links {
            self.links.insert_str(content);
        }
    }
}
