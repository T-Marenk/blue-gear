use std::{
    error::Error,
    io,
    time::Duration, sync::Mutex
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute, 
    terminal::{EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen, enable_raw_mode}, 
    event::{EnableMouseCapture, DisableMouseCapture, KeyEvent}
};
use futures::pin_mut; // Will be used later
use tokio::runtime::Runtime;
use tui::{
    backend::{CrosstermBackend, Backend},
    Terminal,
};
use crate::{
    app::App,
    ui::draw,
};


/// Start the application
/// Creates the runtime, initializes App for usage and created alternate terminal for ui
pub fn start() -> Result<(), Box<dyn Error>> {
    let rt: Runtime = match create_rt() {
        Ok(rt) => rt,
        Err(e) => return Err(e)
    };

    let app_mutex = Mutex::new(rt.block_on(App::new())?);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let response = run(&mut terminal, &app_mutex, &rt);
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(e) = response {
        eprint!("Error while running application {:?}", e)
    }

    Ok(())
}

// Runs the application
// rt.task.spawn(suorita_etsintä)
// rt.task.spawn(älä suorita etsintää vaan näppäimet)
// nuo loopiin, ja suorita ja älä suorita ovat whileja

/// Creates tokio runtime for application
/// Uses single threaded runtime
fn create_rt() -> Result<Runtime, Box<dyn Error>> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        Ok(rt)
}

fn run<B: Backend>(
    mut terminal: &mut Terminal<B>,
    app_mutex: &Mutex<App>,
    rt: &Runtime
) -> Result<(), Box<dyn Error>> {
    loop {
        let state = match app_mutex.lock() {
            Ok(app) => app.state.clone(),
            Err(e) => e.get_ref().state.clone()
        };

        match state {
            0 => bluetooth_off(&mut terminal, &app_mutex, &rt)?,
            1 => break,
            _ => {}
        }

 
    }

    Ok(())
}

/// Run the applicatin without bluetooth discovery when bluetooth is Off
/// This way, the bluetooth can still be turned on, but there wont be detection for devices
fn bluetooth_off<B: Backend>(
    mut terminal: &mut Terminal<B>,
    app_mutex: &Mutex<App>,
    rt: &Runtime // Will be used later on
) -> Result<(), Box<dyn Error>> {
    drawer(&mut terminal, &app_mutex);

    loop {
        let response: u8 = match event::read()? {
            Event::Key(key) => handle_key(&app_mutex, &mut terminal, key),
            _ => 0
        };
        if response == 1 {
            break
        }
    }

    Ok(())
}

fn handle_key<B: Backend>(app_mutex: &Mutex<App>, mut terminal: &mut Terminal<B>, key: KeyEvent) -> u8 {
        match key.code {
                KeyCode::Char('q') => test(&app_mutex),
                KeyCode::Char('d') => drawer(&mut terminal, &app_mutex),
                _ => 0
            }
}

fn test(app_mutex: &Mutex<App>) -> u8 {
    let mut app = app_mutex.lock().unwrap();
    app.state = 1;
    1
}
/// Call draw function in order to render the ui with changes to the terminal
fn drawer<B: Backend>(terminal: &mut Terminal<B>, app_mutex: &Mutex<App>) -> u8 {
    let mut app = match app_mutex.lock() {
        Ok(app) => app,
        Err(e) => e.into_inner()
    };

    terminal.draw(|f| draw(f, &mut app)).unwrap();

    0
}
