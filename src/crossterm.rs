use std::{
    error::Error,
    io, thread, time::Duration
};
use crossterm::{
    execute, 
    terminal::{EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen, enable_raw_mode}, 
    event::{EnableMouseCapture, DisableMouseCapture}
};
use tui::{
    backend::{CrosstermBackend, Backend},
    Terminal,
};
use crate::{
    app::App,
    ui::draw,
};


// Start the application by creating the terminal and stopping it when done
pub fn start() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let response = run(&mut terminal, &mut app);
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(e) = response {
        eprint!("Error while running application {:?}", e)
    }

    Ok(())
}

// Runs the application
fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: &mut App
) -> io::Result<()> {
    terminal.draw(|f| draw(f, &mut app))?;

    thread::sleep(Duration::from_millis(5000));

    Ok(())
}
