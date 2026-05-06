use crossterm::event::{self, Event, KeyCode};
use pyload_tui::table::PackagesTable;
use pyload_tui::utils::fetch_packages;
use ratatui::widgets::{Table, TableState};
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let packages = fetch_packages().await?;
    let table: Table = PackagesTable::from(packages).0;

    let mut table_state = TableState::new();
    table_state.select_first();

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                frame.render_stateful_widget(&table, frame.area(), &mut table_state)
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') => table_state.select_next(),
                    KeyCode::Char('k') => table_state.select_previous(),
                    _ => (),
                }
            }
        }
        Ok(())
    })
}
