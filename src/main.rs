mod crossterm;
mod blue;
mod ui;

use tokio;
use crate::crossterm::start;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(e) = start().await {
        eprint!("{e}")
    };
}
