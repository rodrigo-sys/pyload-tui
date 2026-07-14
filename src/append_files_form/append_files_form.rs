use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::HorizontalAlignment;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_textarea::TextArea;

use super::SelectedInput;
use crate::{app_action::AppAction, screens::ScreenHandler, utils::add_links_to_package};

#[derive(Clone)]
pub struct AppendFilesForm {
    pub package_id: i32,
    pub package_name: String,
    pub links: TextArea<'static>,
    pub submit: Paragraph<'static>,
    pub selected: SelectedInput,
}

impl ScreenHandler for AppendFilesForm {
    async fn handle_keys(&mut self, key: KeyEvent) -> Option<AppAction> {
        match key.code {
            KeyCode::Esc => Some(AppAction::GoToPreviousScreen),
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

    fn handle_paste(&mut self, content: &str) {
        if self.selected == SelectedInput::Links {
            self.links.insert_str(content);
        }
    }
}

impl AppendFilesForm {
    pub fn new(package_id: i32, package_name: String) -> Self {
        Self {
            package_id,
            package_name,
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

    async fn submit(&mut self) -> Option<AppAction> {
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
        self.reset();
        Some(AppAction::GoToPreviousScreen)
    }

    fn reset(&mut self) {
        self.links.clear();
        self.selected = SelectedInput::default();
    }
}
