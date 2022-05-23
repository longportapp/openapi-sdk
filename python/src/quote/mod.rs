mod context;
mod push;
mod types;

use pyo3::prelude::*;

pub(crate) fn register_module(py: Python<'_>, parent: &PyModule) -> PyResult<()> {
    let quote = PyModule::new(py, "quote")?;

    quote.add_class::<types::DerivativeType>()?;
    quote.add_class::<types::TradeStatus>()?;
    quote.add_class::<types::TradeSession>()?;
    quote.add_class::<types::SubType>()?;
    quote.add_class::<types::TradeDirection>()?;
    quote.add_class::<types::OptionType>()?;
    quote.add_class::<types::Period>()?;
    quote.add_class::<types::AdjustType>()?;
    quote.add_class::<types::SecuritiyStaticInfo>()?;
    quote.add_class::<types::PrePostQuote>()?;
    quote.add_class::<types::SecurityQuote>()?;
    quote.add_class::<types::OptionQuote>()?;
    quote.add_class::<types::WarrantQuote>()?;
    quote.add_class::<types::Depth>()?;
    quote.add_class::<types::SecurityDepth>()?;
    quote.add_class::<types::Brokers>()?;
    quote.add_class::<types::SecurityBrokers>()?;
    quote.add_class::<types::ParticipantInfo>()?;
    quote.add_class::<types::Trade>()?;
    quote.add_class::<types::IntradayLine>()?;
    quote.add_class::<types::Candlestick>()?;
    quote.add_class::<types::StrikePriceInfo>()?;
    quote.add_class::<types::IssuerInfo>()?;
    quote.add_class::<types::TradingSessionInfo>()?;
    quote.add_class::<types::MarketTradingSession>()?;
    quote.add_class::<types::RealtimeQuote>()?;
    quote.add_class::<types::PushQuote>()?;
    quote.add_class::<types::PushDepth>()?;
    quote.add_class::<types::PushBrokers>()?;
    quote.add_class::<types::PushTrades>()?;

    quote.add_class::<context::QuoteContext>()?;

    parent.add_submodule(quote)?;
    Ok(())
}
