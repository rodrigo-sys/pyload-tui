use crossterm::event::{self, Event};
use pyload_tui::app::App;
use pyload_tui::current_screen::CurrentScreen;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new().await;

    let mut terminal = ratatui::init();

    while !app.quit {
        terminal.draw(|frame| match &mut app.current_screen {
            CurrentScreen::Packages(s) => {
                frame.render_widget(&s.clone(), frame.area());
            }
            CurrentScreen::Files(s) => {
                frame.render_widget(&s.clone(), frame.area());
            }
            _ => {}
        })?;

        if let Event::Key(key) = event::read()? {
            app.handle_key(key).await;
        }
    }

    ratatui::restore();

    Ok(())
}
