use crossterm::event::{KeyEvent, KeyCode};
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use crate::app::App;

/// Matches the pressed key to the wanted function.
pub async fn handle_key(
    app_mutex: &Arc<Mutex<App>>, 
    key: KeyEvent
    ) -> Option<bool> {
    match key {
        KeyEvent {
            code: KeyCode::Char('q'),
            ..
        } => quit(app_mutex).await,
        KeyEvent {
            code: KeyCode::Tab,
            ..
        } => toggle(app_mutex).await,
        _ => None
    }
}

/// Tells the application it should quit, by turning its state to 1
async fn quit(app_mutex: &Arc<Mutex<App>>) -> Option<bool> {
    let mut app: MutexGuard<App> = app_mutex.lock().await;
    app.state = 1;
    Some(true)
}

/// Call app functions to toggle bluetooth on and off
async fn toggle(app_mutex: &Arc<Mutex<App>>,
) -> Option<bool> {
    let mut app: MutexGuard<App> = app_mutex.lock().await;
    app.toggle_bluetooth().await;
    Some(false)
}
