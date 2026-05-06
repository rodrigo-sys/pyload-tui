use crossterm::event::{self, Event};
use pyload_tui::app::App;
use pyload_tui::current_screen::CurrentScreen;
use pyload_tui::table::PackagesTable;
use pyload_tui::utils::fetch_packages;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let packages = fetch_packages().await?;
    let packages_table = PackagesTable::from(packages.clone()).0;

    let mut app = App::new();
    app.packages = packages;
    app.packages_table_state.select_first();

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                match app.current_screen {
                    CurrentScreen::Packages => {
                        frame.render_stateful_widget(
                            &packages_table,
                            frame.area(),
                            &mut app.packages_table_state,
                        )
                    }
                    CurrentScreen::Files => {},
                    _ => {}
                }
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => break,
                    _ => app.handle_key(key.code),
                }
            }
        }
        Ok(())
    })
}
