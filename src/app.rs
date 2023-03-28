use bluer::Device;

/// Struct, which holds values used by the application
pub struct App {
    pub devices: Vec<Device>,
    pub state: u8,
    pub status: bool,
}

impl App {
    /// Creates new instance of App for use
    pub fn new() -> Self {
        App {
            devices: Vec::new(),
            state: 0,
            status: false,
        }
    }
}
