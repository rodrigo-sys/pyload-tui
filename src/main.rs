use std::time::Duration;

use crossterm::event::{self, DisableBracketedPaste, EnableBracketedPaste};
use crossterm::execute;
use openapi::apis::py_load_rest_api::api_get_events_get;
use openapi::models::{DownloadInfo, ServerStatus};
use pyload_tui::utils::{
    ensure_app_config_exists, fetch_downloads_info, fetch_server_status, get_pyload_config,
};
use pyload_tui::{app::App, key_hints::KeyHints, screens::Screen};
use ratatui::layout::{Constraint, Direction, Layout};
use tokio::sync::{mpsc, watch};

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

    let (status_tx, status_rx) = watch::channel(ServerStatus::default());
    let (dl_tx, dl_rx) = watch::channel(Vec::<DownloadInfo>::new());

    tokio::spawn(async move {
        loop {
            let Ok(server_status) = fetch_server_status().await else {
                continue;
            };

            status_tx
                .send(server_status.clone())
                .expect("Error sending status server");

            let downloads_info = if server_status.active > 0 {
                fetch_downloads_info().await
            } else {
                Vec::new()
            };
            dl_tx.send(downloads_info).expect("Error sending downloads info");

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    let mut terminal = ratatui::init();
    execute!(std::io::stdout(), EnableBracketedPaste)?;

    let layout = Layout::new(
        Direction::Vertical,
        vec![Constraint::Percentage(90), Constraint::Length(5)],
    );
    let bottom_layout = Layout::new(
        Direction::Horizontal,
        vec![
            Constraint::Length(1),
            Constraint::Length(20),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(15),
        ],
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
                Screen::Downloads(s) => {
                    frame.render_stateful_widget(s.clone(), areas[0], &mut s.table_state);
                }
                Screen::AddPackageForm(s) => {
                    frame.render_widget(s.clone(), areas[0]);
                }
                Screen::AppendFilesForm(s) => {
                    frame.render_widget(s.clone(), areas[0]);
                }
            }

            let bottom_area = bottom_layout.split(areas[1]);
            frame.render_widget(app.status_card.clone(), bottom_area[1]);
            frame.render_widget(KeyHints::new(&app.get_bindings()), bottom_area[3]);
        })?;

        if event::poll(tick_rate)? {
            app.handle_events(event::read()?).await;
        }

        while let Ok(pyload_event) = rx.try_recv() {
            app.handle_pyload_events(pyload_event).await;
        }

        if status_rx.has_changed().unwrap_or(false) {
            app.update_status(status_rx.borrow().clone());
        }

        if dl_rx.has_changed().unwrap_or(false) {
            app.update_downloads_info(dl_rx.borrow().clone());
        }
    }

    execute!(std::io::stdout(), DisableBracketedPaste)?;
    ratatui::restore();

    Ok(())
}
