use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:4000")?;

    hc.do_post("/products/1/buy", json!({"quantity": 1})).await?.print().await?;

    hc.do_get("/products/1").await?.print().await?;

    Ok(())
}
