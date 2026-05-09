use crossterm::event::{self, Event, KeyCode};
use pyload_tui::app::App;
use pyload_tui::app_action::AppAction;
use pyload_tui::current_screen::CurrentScreen;
use pyload_tui::files_screen::FilesScreen;
use pyload_tui::packages_screen::PackagesScreen;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new().await;

    let mut terminal = ratatui::init();

    loop {
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
            if key.code == KeyCode::Char('q') {
                break;
            }

            let action = match &mut app.current_screen {
                CurrentScreen::Packages(s) => s.handle_keys(key),
                CurrentScreen::Files(s) => s.handle_keys(key),
                _ => None,
            };

            match action {
                Some(AppAction::Quit) => {}
                Some(AppAction::GoToPackages) => {
                    if !matches!(app.current_screen, CurrentScreen::Packages(_)) {
                        let mut s = PackagesScreen::new().await;
                        app.current_screen = CurrentScreen::Packages(s);
                    }
                }
                Some(AppAction::GoToFiles(pid)) => {
                    if !matches!(app.current_screen, CurrentScreen::Files(_)) {
                        let mut s = FilesScreen::new(pid).await;
                        app.current_screen = CurrentScreen::Files(s);
                    }
                }
                None => {}
            }
        }
    }

    ratatui::restore();

    Ok(())
}
