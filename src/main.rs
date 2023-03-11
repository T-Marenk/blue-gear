mod crossterm;
// mod blue;
mod ui;

use crate::crossterm::start;

fn main() {
    if let Err(e) = start() {
        eprint!("{e}")
    };
}
