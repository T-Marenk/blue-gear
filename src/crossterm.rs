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
use bluer::Adapter;

use crate::ui::draw;
use crate::blue::new_adapter;

//Start the application by creating the terminal and stopping it when done
pub async fn start() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let adapter = new_adapter().await?;
    let response = run(&mut terminal, &adapter).await;
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(e) = response {
        eprint!("Error while running application {:?}", e)
    }

    Ok(())
}

//Runs the application
async fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    adapter: &Adapter
) -> io::Result<()> {
    let status = adapter.is_powered().await?;
    terminal.draw(|f| draw(f, status))?;

    thread::sleep(Duration::from_millis(5000));

    Ok(())
}
