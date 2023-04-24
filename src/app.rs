use bluer::Device;
use tui::style::Color;

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
            if name.is_err() {
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
        self.informations.clear();
    }

    pub fn status_color(&self, status: &str) -> Color {
        match status {
            "On" => Color::Green,
            _ => Color::Red,
        }
    }

    pub fn toggle_selected(&self) -> Color {
        match self.state {
            0 => Color::Magenta,
            _ => Color::White,
        }
    }
}
