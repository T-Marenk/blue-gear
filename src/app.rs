use std::error::Error;

use bluer::Device;

use crate::blue::Blue;

/// Struct, which holds values used by the application
pub struct App {
    pub bluetooth: Blue,
    pub devices: Vec<Device>,
    pub state: u8,
}

impl App {
    /// Creates new instance of App for use
    /// async function in order to create new instance of Blue
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let bluetooth = Blue::new().await?;

        Ok(App {
            bluetooth,
            devices: Vec::new(),
            state: 0
        })
    }
    
    /// Returns bluetooth status to use in UI
    pub fn get_bluetooth_status(&mut self) -> &str {
         match self.bluetooth.status {
            true => "On",
            false => "Off"
        }
    }
    
    /// Toggle bluetooth on and off
    ///
    /// # Panics
    ///
    /// Function panics if an error occurs while toggling bluetooth
    pub async fn toggle_bluetooth(&mut self) {
        match self.bluetooth.toggle().await {
            Ok(_) => (),
            Err(e) => panic!("Error while toggling bluetooth {}", e)
        };
    }
}
