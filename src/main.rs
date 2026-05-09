use crossterm::event::{self, Event, KeyCode};
use pyload_tui::app::App;
use pyload_tui::current_screen::CurrentScreen;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new().await;

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| match &mut app.current_screen {
                CurrentScreen::Packages(s) => {
                    frame.render_widget(&s.clone(), frame.area());
                }
                CurrentScreen::Files => {}
                _ => {}
            })?;

            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }

                match &mut app.current_screen {
                    CurrentScreen::Packages(s) => s.handle_keys(key),
                    CurrentScreen::Files => None,
                    _ => None,
                };
            }
        }
        Ok(())
    })
}
