mod crossterm;
mod blue;
mod ui;
mod app;
mod keys;

use crate::crossterm::start;

fn main() {
    if let Err(e) = start() {
        eprint!("{e}")
    };
}
