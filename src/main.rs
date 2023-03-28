mod app;
mod blue;
mod crossterm;
mod keys;
mod ui;

use crate::crossterm::start;

fn main() {
    if let Err(e) = start() {
        eprint!("{e}")
    };
}
