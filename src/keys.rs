use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};
use std::sync::Arc;
use tokio::{
    sync::{mpsc::Sender, Mutex},
    time::{sleep, Duration},
};

/// Matches the pressed key to the wanted function.
pub async fn handle_key(
    app_mutex: &Arc<Mutex<App>>,
    key: KeyEvent,
    sender: &Sender<u8>,
) -> Option<u8> {
    match key {
        KeyEvent {
            code: KeyCode::Char('q'),
            ..
        } => Some(0),
        KeyEvent {
            code: KeyCode::Down, ..
        } => change_selection(app_mutex, "down").await,
        KeyEvent {
            code: KeyCode::Char('j'), ..
        } => change_selection(app_mutex, "down").await,
        KeyEvent {
            code: KeyCode::Up, ..
        } => change_selection(app_mutex, "up").await,
        KeyEvent {
            code: KeyCode::Char('k'), ..
        } => change_selection(app_mutex, "up").await,
        KeyEvent {
            code: KeyCode::Tab, ..
        } => toggle(app_mutex, sender).await,
        _ => None,
    }
}

async fn change_selection(app_mutex: &Arc<Mutex<App>>, direction: &str) -> Option<u8> {
    let mut app = app_mutex.lock().await;
    app.change_selection(direction);
    Some(1)
}

/// Call app functions to toggle bluetooth on and off
async fn toggle(app_mutex: &Arc<Mutex<App>>, sender: &Sender<u8>) -> Option<u8> {
    let app = app_mutex.lock().await;
    let current_status: bool = app.status;
    drop(app);
    sender.send(1).await.unwrap();
    loop {
        sleep(Duration::from_millis(20)).await;
        let app = app_mutex.lock().await;
        let status: bool = app.status;
        drop(app);
        if status != current_status {
            break;
        }
        sender.send(2).await.unwrap();
    }
    Some(3)
}
