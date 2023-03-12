use tokio::runtime::Runtime;
use crate::blue::Blue;

pub struct App {
    pub bluetooth: Blue,
    pub selected: u8,
    pub rt: Runtime
}

impl App {
    pub fn new() -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build().unwrap();

        let bluetooth = rt.block_on(Blue::new()).unwrap();

        App {
            bluetooth,
            selected: 0,
            rt
        } 
    }
    pub fn get_bluetooth_status(&mut self) -> &str {
        let status = match self.rt.block_on(self.bluetooth.get_bluetooth_status()) {
            true => "On",
            false => "Off"
        };
        
        status
    }
}
