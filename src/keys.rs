use crossterm::event::{KeyEvent, KeyCode};
use tokio::runtime::Runtime;
use std::sync::{Mutex, MutexGuard};
use tui::{
    Terminal,
    backend::Backend
};

use crate::app::App;

/// Matches the pressed key to the wanted function.
pub fn handle_key<B: Backend>(
    app_mutex: &Mutex<App>, 
    mut terminal: &mut Terminal<B>,
    rt: &Runtime,
    key: KeyEvent
    ) -> u8 {
    match key {
        KeyEvent {
            code: KeyCode::Char('q'),
            ..
        } => quit(app_mutex),
        KeyEvent {
            code: KeyCode::Tab,
            ..
        } => toggle(&app_mutex, &mut terminal, &rt),
        _ => 0
    }
}

/// Tells the application it should quit, by turning its state to 1
fn quit(app_mutex: &Mutex<App>) -> u8 {
    let mut app: std::sync::MutexGuard<App> = match app_mutex.lock() {
        Ok(app) => app,
        Err(app) => app.into_inner()
    };
    app.state = 1;
    1
}

/// Call app functions to toggle bluetooth on and off
fn toggle<B: Backend>(app_mutex: &Mutex<App>,
          terminal: &mut Terminal<B>,
          rt: &Runtime
) -> u8 {
    let mut app: MutexGuard<App> = match app_mutex.lock() {
        Ok(app) => app,
        Err(app) => app.into_inner()
    };
    rt.block_on(app.toggle_bluetooth());
    terminal.draw(|f| crate::ui::draw(f, &mut app)).unwrap();
    0
}
