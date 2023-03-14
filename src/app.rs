use std::error::Error;

use crate::blue::Blue;

/// Struct, which holds values used by the application
pub struct App {
    pub bluetooth: Blue,
    pub state: u8
}

impl App {
    /// Creates new instance of App for use
    /// async function in order to create new instance of Blue
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let bluetooth = Blue::new().await?;

        Ok(App {
            bluetooth,
            state: 0
        })
    }
    
    /// Returns bluetooth status to use in UI
    pub fn get_bluetooth_status(&mut self) -> &str {
        let status = match self.bluetooth.status {
            true => "On",
            false => "Off"
        };

        status
    }
}
