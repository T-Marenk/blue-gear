use bluer::Device;
use tui::{style::Color, widgets::TableState};

/// Struct, which holds values used by the application
pub struct App {
    pub devices: Vec<Device>,
    pub informations: Vec<String>,
    pub state: u8,
    pub selected_device: TableState,
    pub status: bool,
}

impl App {
    /// Creates new instance of App for use
    pub fn new() -> Self {
        App {
            devices: Vec::new(),
            informations: Vec::new(),
            state: 0,
            selected_device: TableState::default(),
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

    pub fn change_selection(&mut self, direction: &str) {
        match direction {
            "up" => self.selection_up(),
            "down" => self.selection_down(),
            _ => {},
        }
    }
    
    fn selection_up(&mut self) {
        if self.state == 1 {
            let i = match self.selected_device.selected() {
                Some(i) => i,
                None => 0,
            };
            if i == 0 {
                self.state = 0;
                self.selected_device.select(None);
            } else {
                self.selected_device.select(Some(i - 1));
            }
        }
    }
    fn selection_down(&mut self) {
        if self.state == 0 {
            self.state = 1;
            self.selected_device.select(Some(0));
        } else {
            let i = match self.selected_device.selected() {
                Some(i) => {
                    (i + 1) % (self.informations.len())
                }
                None => 0,
            };
            self.selected_device.select(Some(i));
        }
    }
}
