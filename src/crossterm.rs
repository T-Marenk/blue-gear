use std::{
    error::Error,
    io,
    sync::Arc
};
use crossterm::{
    event::{self, Event},
    execute, 
    terminal::{EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen, enable_raw_mode}, 
    event::{EnableMouseCapture, DisableMouseCapture}
};
use futures::pin_mut; // Will be used later
use tokio::{
    runtime::Runtime,
    sync::{Mutex, MutexGuard, broadcast::{Sender, Receiver, channel}}
};
use tui::{
    backend::{CrosstermBackend, Backend},
    Terminal,
};
use crate::{
    app::App,
    ui::draw,
    keys::handle_key
};


/// Start the application
/// Creates the runtime, initializes App for usage and created alternate terminal for ui
pub fn start() -> Result<(), Box<dyn Error>> {
    let rt: Runtime = match create_rt() {
        Ok(rt) => rt,
        Err(e) => return Err(e)
    };

    let app_mutex = Arc::new(Mutex::new(rt.block_on(App::new())?));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let response = run(&mut terminal, app_mutex, &rt);
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(e) = response {
        eprint!("Error while running application {:?}", e)
    }

    Ok(())
}

/// Creates tokio runtime for application
/// Uses single threaded runtime
fn create_rt() -> Result<Runtime, Box<dyn Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()?;

    Ok(rt)
}

fn create_channels() -> (Sender<u8>, Receiver<u8>) {
    let (sender, receiver): (Sender<u8>, Receiver<u8>) = channel(16);
    (sender, receiver)
}

/// Runs the application with the created terminal, app and runtime. Calls the appropriate
/// functions depending on the state of the application, which it gets from app. Always calls
/// either with bluetooth or without it, depending on bluetooth status.
fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    app_mutex: Arc<Mutex<App>>,
    rt: &Runtime
) -> Result<(), Box<dyn Error>> {
    let (sender, receiver) = create_channels();
    sender.send(1).unwrap();
    let sender2 = sender.clone();
    let (b_sender, b_receiver) = create_channels();
    let finder = rt.spawn(bluetooth_finder(app_mutex.clone(), sender, b_receiver));
    let reader = rt.spawn(event_reader(app_mutex.clone(), sender2, b_sender)); 
    rt.block_on(drawer(terminal, app_mutex, receiver));
    
    rt.block_on(reader)?;
    rt.block_on(finder)?;
    Ok(())
}

/// Responsible for finding bluetooth devices and detecting changes
async fn bluetooth_finder(
    app_mutex: Arc<Mutex<App>>,
    sender: Sender<u8>,
    mut receiver: Receiver<u8>,
) {
    let mut app = app_mutex.lock().await;
    let mut status = app.get_bluetooth_status();
    drop(app);
    loop {
        if status {
            let mut should_break = false;
            while status {
                if let Ok(message) = receiver.recv().await {
                    match message {
                        0 => { should_break = true },
                        1 => { status = app_mutex.lock().await.get_bluetooth_status(); }
                        _ => {}
                    }
                } else {  break };
                if should_break { break };
            };
        } else {
            if let Ok(message) = receiver.recv().await {
                match message {
                    0 => break,
                    1 => {  status = app_mutex.lock().await.get_bluetooth_status(); }
                    _ => {}
                }
            } else {  break };
        };
    };
}
/// Responsible for checking system events and finding relevant keyevents. Once keyevents are
/// found, key handeler is called 
async fn event_reader(
    app_mutex: Arc<Mutex<App>>,
    sender: Sender<u8>,
    b_sender: Sender<u8>,
) {
    loop {
        let response: Option<u8> = match event::read().unwrap() {
            Event::Key(key) => handle_key(&app_mutex, key).await,
            _ => None
        };
        match response {
            None => {},
            Some(r) => {
                match r {
                    0 => {
                        b_sender.send(0).unwrap();
                        break
                    },
                    1 => {  sender.send(1).unwrap(); }
                    2 => {
                        sender.send(1).unwrap();
                        b_sender.send(1).unwrap();
                    },
                    _ => {}
                }
            }
        }
    }
}

/// Call draw function in order to render the ui with changes to the terminal
async fn drawer<B: Backend>(
    terminal: &mut Terminal<B>,
    app_mutex: Arc<Mutex<App>>,
    mut receiver: Receiver<u8>
) {
    while (receiver.recv().await).is_ok() {
        let mut app: MutexGuard<App> = app_mutex.lock().await;
        terminal.draw(|f| draw(f, &mut app)).unwrap(); 
    }
}
