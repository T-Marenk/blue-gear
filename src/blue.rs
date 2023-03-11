use bluer::Adapter;

pub async fn new_adapter() -> bluer::Result<Adapter> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;

    Ok(adapter)
}
