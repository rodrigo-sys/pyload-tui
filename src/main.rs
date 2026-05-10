use crossterm::event::{self, Event};
use pyload_tui::{app::App, key_hints::KeyHints};
use pyload_tui::screens::CurrentScreen;
use ratatui::layout::{Constraint, Direction, Layout};
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new().await;

    let mut terminal = ratatui::init();

    let layout = Layout::new(
        Direction::Vertical,
        vec![Constraint::Percentage(90), Constraint::Percentage(10)],
    );

    while !app.quit {
        terminal.draw(|frame| {

            let areas = layout.split(frame.area());

            match &mut app.current_screen {
                CurrentScreen::Packages => {
                    let s = app.screens.packages.as_mut().unwrap();
                    frame.render_stateful_widget(s.clone(), areas[0], &mut s.table_state);
                }
                CurrentScreen::Files => {
                    let s = app.screens.files.as_mut().unwrap();
                    frame.render_stateful_widget(s.clone(), areas[0], &mut s.table_state);
                }
                _ => {}
            }

            frame.render_widget(KeyHints::new(app.get_bindings()), areas[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            app.handle_key(key).await;
        }
    }

    ratatui::restore();

    Ok(())
}
