use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};
use std::sync::Arc;
use tokio::{
    sync::{mpsc::Sender, Mutex, MutexGuard},
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
        } => quit(app_mutex).await,
        KeyEvent {
            code: KeyCode::Tab, ..
        } => toggle(app_mutex, sender).await,
        _ => None,
    }
}

/// Tells the application it should quit, by turning its state to 1
async fn quit(app_mutex: &Arc<Mutex<App>>) -> Option<u8> {
    let mut app: MutexGuard<App> = app_mutex.lock().await;
    app.state = 1;
    Some(0)
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
