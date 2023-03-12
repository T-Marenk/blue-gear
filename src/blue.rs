use bluer::Adapter;

pub struct Blue {
    adapter: Adapter,
    pub status: bool,
}

impl Blue {
    pub async fn new() -> bluer::Result<Blue> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        
        Ok(Blue{
            adapter,
            status: true
        })
    }

    pub async fn get_bluetooth_status(&mut self) -> bool {
        self.update_status().await.unwrap();
         
        self.status.clone()
    }

    async fn update_status(&mut self) -> bluer::Result<()> {
        self.status = self.adapter.is_powered().await?;

        Ok(())
    }
}

