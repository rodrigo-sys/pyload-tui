use std::time::Duration;

use crossterm::event::{self, DisableBracketedPaste, EnableBracketedPaste};
use crossterm::execute;
use openapi::apis::py_load_rest_api::api_get_events_get;
use pyload_tui::utils::{ensure_app_config_exists, get_pyload_config};
use pyload_tui::{app::App, key_hints::KeyHints, screens::Screen};
use ratatui::layout::{Constraint, Direction, Layout};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ensure_app_config_exists()?;

    let mut app = App::new().await;
    let tick_rate = Duration::from_millis(100);

    let (tx, mut rx) = mpsc::channel(64);
    tokio::spawn(async move {
        loop {
            if let Ok(events_info) = api_get_events_get(
                get_pyload_config(),
                Some(std::process::id().to_string().as_ref()),
            )
            .await
            {
                for event in events_info {
                    tx.send(event).await.expect("Error sending pyload events");
                }
            }

            let _ = tokio::time::sleep(Duration::from_secs(3));
        }
    });

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
                Screen::Packages(s) => {
                    frame.render_stateful_widget(s.clone(), areas[0], &mut s.table_state);
                }
                Screen::Files(s) => {
                    frame.render_stateful_widget(s.clone(), areas[0], &mut s.table_state);
                }
                Screen::AddPackageForm(s) => {
                    frame.render_widget(s.clone(), areas[0]);
                }
                Screen::AppendFilesForm(s) => {
                    frame.render_widget(s.clone(), areas[0]);
                }
            }

            frame.render_widget(KeyHints::new(&app.get_bindings()), areas[1]);
            frame.render_widget(app.status_bar.clone(), areas[1]);
        })?;

        if event::poll(tick_rate)? {
            app.handle_events(event::read()?).await;
        }

        while let Ok(pyload_event) = rx.try_recv() {
            app.handle_pyload_events(pyload_event).await;
        }
    }

    execute!(std::io::stdout(), DisableBracketedPaste)?;
    ratatui::restore();

    Ok(())
}
