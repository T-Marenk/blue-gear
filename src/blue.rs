use bluer::{Adapter, AdapterEvent, Address, Device};
use futures::Stream;

/// Holds the data used to interact with bluetooth devices
pub struct Blue {
    adapter: Adapter,
    pub status: bool,
}

impl Blue {
    /// Creates new instance of Blue
    /// creates new session and adapter in order to communicate with devices
    /// as well as checks the status of the adapter
    pub async fn new() -> bluer::Result<Blue> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        let status = adapter.is_powered().await?;

        Ok(Blue { adapter, status })
    }

    /// Toggle bluetooth adapter on and off
    pub async fn toggle(&mut self) -> bluer::Result<()> {
        self.adapter.set_powered(!self.status).await?;
        self.status = !self.status;
        Ok(())
    }

    /// Start searching for bluetooth devices
    ///
    /// # Panics
    ///
    /// Function panics if there is an error while starting search for devices
    pub async fn start_search(&self) -> impl Stream<Item = AdapterEvent> {
        match self.adapter.discover_devices().await {
            Ok(device_events) => device_events,
            Err(e) => panic!("There was an error while starting bluetooth serach {e}"),
        }
    }
    
    /// Get the corresponding device from a bluetooth address
    pub async fn device(&self, addr: Address) -> Option<Device> {
        match self.adapter.device(addr) {
            Ok(device) => Some(device),
            Err(_) => None,
        }
    }
}
