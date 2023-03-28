use crate::{app::App, blue::Blue, keys::handle_key, ui::draw};
use crossterm::{
    event::{self, Event},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::pin_mut; // Will be used later
use std::{error::Error, io, sync::Arc};
use tokio::{
    runtime::Runtime,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex, MutexGuard,
    },
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

/// Start the application
/// Creates the runtime, initializes App for usage and created alternate terminal for ui
pub fn start() -> Result<(), Box<dyn Error>> {
    let rt: Runtime = match create_rt() {
        Ok(rt) => rt,
        Err(e) => return Err(e),
    };

    let app_mutex = Arc::new(Mutex::new(App::new()));

    let blue: Blue = rt.block_on(Blue::new())?;

    let mut app = rt.block_on(app_mutex.lock());
    app.status = blue.status;
    drop(app);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let response = run(&mut terminal, app_mutex, blue, rt);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
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
    blue: Blue,
    rt: Runtime,
) -> Result<(), Box<dyn Error>> {
    let (sender, receiver) = create_channels();
    rt.block_on(sender.send(1)).unwrap();
    let sender2 = sender.clone();
    let (b_sender, b_receiver) = create_channels();

    let finder = rt.spawn(bluetooth_finder(
        blue,
        app_mutex.clone(),
        sender,
        b_receiver,
    ));
    let reader = rt.spawn(event_reader(app_mutex.clone(), sender2, b_sender));

    rt.block_on(drawer(terminal, app_mutex, receiver));

    rt.block_on(reader)?;
    rt.block_on(finder)?;

    Ok(())
}

/// Responsible for managing bluetooth, finding bluetooth devices and detecting changes while bluetooth is turned on.
/// While bluetooth is off, waits for signal to turn bluetooth on and start search again
async fn bluetooth_finder(
    mut blue: Blue,
    app_mutex: Arc<Mutex<App>>,
    sender: Sender<u8>,
    mut receiver: Receiver<u8>,
) {
    loop {
        let should_break: bool = match blue.status {
            true => bluetooth_on(&mut blue, &app_mutex, &sender, &mut receiver).await,
            false => bluetooth_off(&mut blue, &app_mutex, &sender, &mut receiver).await,
        };
        if should_break {
            break;
        };
    }
}

/// Responsible for bluetooth when bluetooth is turned on
async fn bluetooth_on(
    blue: &mut Blue,
    app_mutex: &Arc<Mutex<App>>,
    sender: &Sender<u8>,
    receiver: &mut Receiver<u8>,
) -> bool {
    loop {
        if let Some(message) = receiver.recv().await {
            match message {
                0 => return true,
                1 | 2 => {
                    toggle_bluetooth(blue, app_mutex, sender).await;
                    break;
                }
                _ => {}
            }
        } else {
            return true;
        };
    }
    while let Ok(message) = receiver.try_recv() {
        match message {
            0 => break,
            1 => toggle_bluetooth(blue, app_mutex, sender).await,
            _ => {}
        };
    }

    false
}

/// Responsible for bluetooth management when bluetooth is turned off
async fn bluetooth_off(
    blue: &mut Blue,
    app_mutex: &Arc<Mutex<App>>,
    sender: &Sender<u8>,
    receiver: &mut Receiver<u8>,
) -> bool {
    loop {
        if let Some(message) = receiver.recv().await {
            match message {
                0 => return true,
                1 | 2 => {
                    toggle_bluetooth(blue, app_mutex, sender).await;
                    break;
                }
                _ => {}
            }
        } else {
            return true;
        };
    }
    while let Ok(message) = receiver.try_recv() {
        match message {
            0 => return true,
            1 => toggle_bluetooth(blue, app_mutex, sender).await,
            _ => {}
        };
    }

    false
}
async fn toggle_bluetooth(blue: &mut Blue, app_mutex: &Arc<Mutex<App>>, sender: &Sender<u8>) {
    blue.toggle().await.unwrap();
    let mut app = app_mutex.lock().await;
    app.status = blue.status;
    drop(app);
    sender.send(1).await.unwrap();
}

/// Responsible for checking system events and finding relevant keyevents. Once keyevents are
/// found, key handeler is called
async fn event_reader(app_mutex: Arc<Mutex<App>>, sender: Sender<u8>, b_sender: Sender<u8>) {
    loop {
        let response: Option<u8> = match event::read().unwrap() {
            Event::Key(key) => handle_key(&app_mutex, key, &b_sender).await,
            _ => None,
        };
        match response {
            None => {}
            Some(r) => {
                match r {
                    0 => {
                        b_sender.send(0).await.unwrap();
                        break;
                    }
                    1 => {
                        sender.send(1).await.unwrap();
                    }
                    2 => {
                        sender.send(1).await.unwrap();
                        // b_sender.send(1).unwrap();
                    }
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
    mut receiver: Receiver<u8>,
) {
    while (receiver.recv().await).is_some() {
        let mut app: MutexGuard<App> = app_mutex.lock().await;
        terminal.draw(|f| draw(f, &mut app)).unwrap();
    }
}
