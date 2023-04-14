use bluer::Device;

/// Struct, which holds values used by the application
pub struct App {
    pub devices: Vec<Device>,
    pub informations: Vec<String>,
    pub state: u8,
    pub status: bool,
}

impl App {
    /// Creates new instance of App for use
    pub fn new() -> Self {
        App {
            devices: Vec::new(),
            informations: Vec::new(),
            state: 0,
            status: false,
        }
    }

    pub async fn device_information(&mut self) {
        let mut names: Vec<String> = Vec::new();
        let n: usize = self.devices.len();

        for i in 0..n {
            let name = self.devices[i].name().await;
            if !name.is_ok() {
                continue;
            }
            let name = name.unwrap();
            if name.is_some() {
                names.push(name.unwrap())
            }
        }

        self.informations = names;
    }

    pub fn clear_devices(&mut self) {
        self.devices.clear();
    }
}
