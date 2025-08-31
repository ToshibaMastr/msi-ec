use std::future::pending;

use anyhow::Result;

use msi_ec::service::MsiService;

#[tokio::main]
async fn main() -> Result<()> {
    let _conn = zbus::connection::Builder::system()?
        .name("by.toshibam.MsiEc")?
        .serve_at("/by/toshibam/MsiEc", MsiService {})?
        .build()
        .await?;

    pending::<()>().await;

    Ok(())
}
