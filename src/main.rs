use std::time::Duration;

use crossterm::event::{self, DisableBracketedPaste, EnableBracketedPaste};
use crossterm::execute;
use pyload_tui::utils::ensure_app_config_exists;
use pyload_tui::{app::App, key_hints::KeyHints, screens::Screen};
use ratatui::layout::{Constraint, Direction, Layout};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ensure_app_config_exists()?;

    let mut app = App::new().await;
    let tick_rate = Duration::from_millis(100);

    let mut terminal = ratatui::init();
    execute!(std::io::stdout(), EnableBracketedPaste)?;

    let layout = Layout::new(
        Direction::Vertical,
        vec![Constraint::Percentage(90), Constraint::Percentage(10)],
    );

    while !app.quit {
        terminal.draw(|frame| {
            let areas = layout.split(frame.area());

            match &mut app.current_screen {
                Screen::Packages => {
                    let s = app.screens.packages.as_mut().unwrap();
                    frame.render_stateful_widget(s.clone(), areas[0], &mut s.table_state);
                }
                Screen::Files => {
                    let s = app.screens.files.as_mut().unwrap();
                    frame.render_stateful_widget(s.clone(), areas[0], &mut s.table_state);
                }
                Screen::AddPackageForm => {
                    let s = app.screens.add_package_form.as_mut().unwrap();
                    frame.render_widget(s.clone(), areas[0]);
                }
                Screen::AppendFilesForm => {
                    let s = app.screens.append_files_form.as_mut().unwrap();
                    frame.render_widget(s.clone(), areas[0]);
                }
            }

            frame.render_widget(KeyHints::new(&app.get_bindings()), areas[1]);
        })?;

        if event::poll(tick_rate)? {
            app.handle_events(event::read()?).await;
        }
    }

    execute!(std::io::stdout(), DisableBracketedPaste)?;
    ratatui::restore();

    Ok(())
}
