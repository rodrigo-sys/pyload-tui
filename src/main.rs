use crossterm::event::{self, DisableBracketedPaste, EnableBracketedPaste};
use crossterm::execute;
use pyload_tui::{app::App, key_hints::KeyHints, screens::CurrentScreen};
use ratatui::layout::{Constraint, Direction, Layout};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new().await;

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
                CurrentScreen::Packages => {
                    let s = app.screens.packages.as_mut().unwrap();
                    frame.render_stateful_widget(s.clone(), areas[0], &mut s.table_state);
                }
                CurrentScreen::Files => {
                    let s = app.screens.files.as_mut().unwrap();
                    frame.render_stateful_widget(s.clone(), areas[0], &mut s.table_state);
                }
                CurrentScreen::AddPackageForm => {
                    let s = app.screens.add_package_form.as_mut().unwrap();
                    frame.render_widget(s.clone(), areas[0]);
                }
                _ => {}
            }

            frame.render_widget(KeyHints::new(&app.get_bindings()), areas[1]);
        })?;

        app.handle_events(event::read()?).await;
    }

    execute!(std::io::stdout(), DisableBracketedPaste)?;
    ratatui::restore();

    Ok(())
}
