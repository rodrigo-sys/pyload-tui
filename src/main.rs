use crossterm::event::{self, Event};
use pyload_tui::app::App;
use pyload_tui::screens::CurrentScreen;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new().await;

    let mut terminal = ratatui::init();

    while !app.quit {
        terminal.draw(|frame| {
            match &mut app.current_screen {
                CurrentScreen::Packages => {
                    let s = app.screens.packages.as_mut().unwrap();
                    frame.render_stateful_widget(s.clone(), frame.area(), &mut s.table_state);
                }
                CurrentScreen::Files => {
                    let s = app.screens.files.as_mut().unwrap();
                    frame.render_stateful_widget(s.clone(), frame.area(), &mut s.table_state);
                }
                _ => {}
            }
        })?;

        if let Event::Key(key) = event::read()? {
            app.handle_key(key).await;
        }
    }

    ratatui::restore();

    Ok(())
}
