use std::sync::Arc;

use anyhow::Result;
use time::Date;

use crate::{
    blocking::runtime::BlockingRuntime,
    quote::{
        AdjustType, Candlestick, IntradayLine, IssuerInfo, MarketTradingDays, MarketTradingSession,
        OptionQuote, ParticipantInfo, Period, PushEvent, RealtimeQuote, SecuritiyStaticInfo,
        SecurityBrokers, SecurityDepth, SecurityQuote, StrikePriceInfo, SubFlags, Trade,
        WarrantQuote,
    },
    Config, Market, QuoteContext,
};

/// Quote context
pub struct QuoteContextSync {
    rt: BlockingRuntime<QuoteContext>,
}

impl QuoteContextSync {
    /// Create a `QuoteContextSync` object
    pub fn try_new<F>(config: Arc<Config>, push_callback: F) -> Result<Self>
    where
        F: FnMut(PushEvent) + Send + 'static,
    {
        let rt = BlockingRuntime::try_new(move || QuoteContext::try_new(config), push_callback)?;
        Ok(Self { rt })
    }

    /// Subscribe quote
    pub fn subscribe<I, T, F>(&self, symbols: I, sub_types: F, is_first_push: bool) -> Result<()>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
        F: Into<SubFlags> + Send + 'static,
    {
        self.rt.call(move |ctx| async move {
            ctx.subscribe(symbols, sub_types.into(), is_first_push)
                .await
        })
    }

    /// Unsubscribe quote
    pub fn unsubscribe<I, T, F>(&self, symbols: I, sub_types: F) -> Result<()>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
        F: Into<SubFlags> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.unsubscribe(symbols, sub_types.into()).await })
    }

    /// Get basic information of securities
    pub fn static_info<I, T>(&self, symbols: I) -> Result<Vec<SecuritiyStaticInfo>>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.static_info(symbols).await })
    }

    /// Get quote of securities
    pub fn quote<I, T>(&self, symbols: I) -> Result<Vec<SecurityQuote>>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.quote(symbols).await })
    }

    /// Get quote of option securities
    pub fn option_quote<I, T>(&self, symbols: I) -> Result<Vec<OptionQuote>>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.option_quote(symbols).await })
    }

    /// Get quote of warrant securities
    pub fn warrant_quote<I, T>(&self, symbols: I) -> Result<Vec<WarrantQuote>>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.warrant_quote(symbols).await })
    }

    /// Get security depth
    pub fn depth(&self, symbol: impl Into<String> + Send + 'static) -> Result<SecurityDepth> {
        self.rt
            .call(move |ctx| async move { ctx.depth(symbol).await })
    }

    /// Get security brokers
    pub fn brokers(&self, symbol: impl Into<String> + Send + 'static) -> Result<SecurityBrokers> {
        self.rt
            .call(move |ctx| async move { ctx.brokers(symbol).await })
    }

    /// Get participants
    pub fn participants(&self) -> Result<Vec<ParticipantInfo>> {
        self.rt
            .call(move |ctx| async move { ctx.participants().await })
    }

    /// Get security trades
    pub fn trades(
        &self,
        symbol: impl Into<String> + Send + 'static,
        count: usize,
    ) -> Result<Vec<Trade>> {
        self.rt
            .call(move |ctx| async move { ctx.trades(symbol, count).await })
    }

    /// Get security intraday
    pub fn intraday(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<Vec<IntradayLine>> {
        self.rt
            .call(move |ctx| async move { ctx.intraday(symbol).await })
    }

    /// Get security candlesticks
    pub fn candlesticks(
        &self,
        symbol: impl Into<String> + Send + 'static,
        period: Period,
        count: usize,
        adjust_type: AdjustType,
    ) -> Result<Vec<Candlestick>> {
        self.rt.call(move |ctx| async move {
            ctx.candlesticks(symbol, period, count, adjust_type).await
        })
    }

    /// Get option chain expiry date list
    pub fn option_chain_expiry_date_list(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<Vec<Date>> {
        self.rt
            .call(move |ctx| async move { ctx.option_chain_expiry_date_list(symbol).await })
    }

    /// Get option chain info by date
    pub fn option_chain_info_by_date(
        &self,
        symbol: impl Into<String> + Send + 'static,
        expiry_date: Date,
    ) -> Result<Vec<StrikePriceInfo>> {
        self.rt.call(
            move |ctx| async move { ctx.option_chain_info_by_date(symbol, expiry_date).await },
        )
    }

    /// Get warrant issuers
    pub fn warrant_issuers(&self) -> Result<Vec<IssuerInfo>> {
        self.rt
            .call(move |ctx| async move { ctx.warrant_issuers().await })
    }

    /// Get trading session of the day
    pub fn trading_session(&self) -> Result<Vec<MarketTradingSession>> {
        self.rt
            .call(move |ctx| async move { ctx.trading_session().await })
    }

    /// Get market trading days
    pub fn trading_days(
        &self,
        market: Market,
        begin: Date,
        end: Date,
    ) -> Result<MarketTradingDays> {
        self.rt
            .call(move |ctx| async move { ctx.trading_days(market, begin, end).await })
    }

    /// Get real-time quote
    pub fn realtime_quote<I, T>(&self, symbols: I) -> Result<Vec<RealtimeQuote>>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.realtime_quote(symbols).await })
    }

    /// Get real-time depth
    pub fn realtime_depth(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<SecurityDepth> {
        self.rt
            .call(move |ctx| async move { ctx.realtime_depth(symbol).await })
    }

    /// Get real-time trades
    pub fn realtime_trades(
        &self,
        symbol: impl Into<String> + Send + 'static,
        count: usize,
    ) -> Result<Vec<Trade>> {
        self.rt
            .call(move |ctx| async move { ctx.realtime_trades(symbol, count).await })
    }

    /// Get real-time broker queue
    pub fn realtime_brokers(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<SecurityBrokers> {
        self.rt
            .call(move |ctx| async move { ctx.realtime_brokers(symbol).await })
    }
}
