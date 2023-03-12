use futures::executor::block_on;
use crate::blue::Blue;

pub struct App {
    pub bluetooth: Blue,
    pub selected: u8,
}

impl Default for App {
    fn default() -> Self {
        App {
            bluetooth: block_on(Blue::new()).unwrap(),
            selected: 0
        }
    }
}

impl App {
    pub fn get_bluetooth_status(&mut self) -> &str {
        let status = match self.bluetooth.get_bluetooth_status() {
            true => "On",
            false => "Off"
        };
        
        status
    }
}
