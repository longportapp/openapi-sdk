use std::sync::Arc;

use anyhow::Result;
use longbridge::{Config, QuoteContext};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(Config::from_env()?);
    let (ctx, _) = QuoteContext::try_new(config).await?;
    let resp = ctx
        .quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
        .await?;
    println!("{:?}", resp);
    Ok(())
}
