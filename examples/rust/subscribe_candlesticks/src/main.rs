use std::sync::Arc;

use longport::{
    quote::{Period, PushCandlestick, PushEvent, PushEventDetail, QuoteContext, SubFlags},
    Config, PushCandlestickMode,
};
use time::OffsetDateTime;
use time_tz::{timezones::db::HONGKONG, OffsetDateTimeExt};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Arc::new(Config::from_env()?);
    let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
    println!("member id: {}", ctx.member_id());
    // ctx.subscribe(["700.HK"], SubFlags::QUOTE | SubFlags::TRADE, false)
    //     .await?;
    ctx.subscribe_candlesticks("700.HK", Period::OneMinute)
        .await?;

    while let Some(event) = receiver.recv().await {
        match event.detail {
            PushEventDetail::Candlestick(PushCandlestick { candlestick, .. }) => {
                println!(
                    "CANDLESTICK {} | {}",
                    OffsetDateTime::now_utc().to_timezone(HONGKONG),
                    candlestick.timestamp.to_timezone(HONGKONG)
                );
            }
            PushEventDetail::Quote(quote) => {
                println!(
                    "QUOTE {} | {}",
                    quote.timestamp.to_timezone(HONGKONG),
                    quote.last_done
                );
            }
            PushEventDetail::Trade(trade) => {
                println!(
                    "TRADE {} | {}",
                    trade.trades.last().unwrap().timestamp.to_timezone(HONGKONG),
                    trade.trades.last().unwrap().price
                );
            }
            _ => {}
        }
    }
    Ok(())
}
