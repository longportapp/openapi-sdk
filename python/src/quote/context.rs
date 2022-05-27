use std::sync::Arc;

use longbridge::blocking::QuoteContextSync;
use pyo3::prelude::*;

use crate::{
    config::Config,
    quote::{
        push::handle_push_event,
        types::{
            AdjustType, Candlestick, IntradayLine, IssuerInfo, MarketTradingDays,
            MarketTradingSession, OptionQuote, ParticipantInfo, Period, RealtimeQuote,
            SecuritiyStaticInfo, SecurityBrokers, SecurityDepth, SecurityQuote, StrikePriceInfo,
            SubType, SubTypes, Trade, WarrantQuote,
        },
    },
    time::PyDateWrapper,
    types::Market,
};

#[pyclass]
pub(crate) struct QuoteContext(QuoteContextSync);

#[pymethods]
impl QuoteContext {
    #[new]
    fn new(config: &Config, handler: Option<PyObject>) -> PyResult<Self> {
        let ctx = QuoteContextSync::try_new(Arc::new(config.0.clone()), move |event| {
            if let Some(handler) = &handler {
                handle_push_event(handler, event);
            }
        })?;
        Ok(Self(ctx))
    }

    /// Subscribe
    #[args(is_first_push = false)]
    fn subscribe(
        &self,
        symbols: Vec<String>,
        sub_types: Vec<SubType>,
        is_first_push: bool,
    ) -> PyResult<()> {
        self.0
            .subscribe(symbols, SubTypes(sub_types), is_first_push)?;
        Ok(())
    }

    /// Unsubscribe
    fn unsubscribe(&self, symbols: Vec<String>, sub_types: Vec<SubType>) -> PyResult<()> {
        self.0.unsubscribe(symbols, SubTypes(sub_types))?;
        Ok(())
    }

    /// Get basic information of securities
    fn static_info(&self, symbols: Vec<String>) -> PyResult<Vec<SecuritiyStaticInfo>> {
        self.0
            .static_info(symbols)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of securities
    fn quote(&self, symbols: Vec<String>) -> PyResult<Vec<SecurityQuote>> {
        self.0
            .quote(symbols)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of option securities
    fn option_quote(&self, symbols: Vec<String>) -> PyResult<Vec<OptionQuote>> {
        self.0
            .option_quote(symbols)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of warrant securities
    fn warrant_quote(&self, symbols: Vec<String>) -> PyResult<Vec<WarrantQuote>> {
        self.0
            .warrant_quote(symbols)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security depth
    fn depth(&self, symbol: String) -> PyResult<SecurityDepth> {
        self.0.depth(symbol)?.try_into()
    }

    /// Get security brokers
    fn brokers(&self, symbol: String) -> PyResult<SecurityBrokers> {
        self.0.brokers(symbol)?.try_into()
    }

    /// Get participants
    pub fn participants(&self) -> PyResult<Vec<ParticipantInfo>> {
        self.0
            .participants()?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security trades
    pub fn trades(&self, symbol: String, count: usize) -> PyResult<Vec<Trade>> {
        self.0
            .trades(symbol, count)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security intraday
    pub fn intraday(&self, symbol: String) -> PyResult<Vec<IntradayLine>> {
        self.0
            .intraday(symbol)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security candlesticks
    pub fn candlesticks(
        &self,
        symbol: String,
        period: Period,
        count: usize,
        adjust_type: AdjustType,
    ) -> PyResult<Vec<Candlestick>> {
        self.0
            .candlesticks(symbol, period.into(), count, adjust_type.into())?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get option chain expiry date list
    pub fn option_chain_expiry_date_list(&self, symbol: String) -> PyResult<Vec<PyDateWrapper>> {
        Ok(self
            .0
            .option_chain_expiry_date_list(symbol)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    /// Get option chain info by date
    pub fn option_chain_info_by_date(
        &self,
        symbol: String,
        expiry_date: PyDateWrapper,
    ) -> PyResult<Vec<StrikePriceInfo>> {
        self.0
            .option_chain_info_by_date(symbol, expiry_date.0)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get warrant issuers
    pub fn warrant_issuers(&self) -> PyResult<Vec<IssuerInfo>> {
        self.0
            .warrant_issuers()?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get trading session of the day
    pub fn trading_session(&self) -> PyResult<Vec<MarketTradingSession>> {
        self.0
            .trading_session()?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get trading session of the day
    pub fn trading_days(
        &self,
        market: Market,
        begin: PyDateWrapper,
        end: PyDateWrapper,
    ) -> PyResult<MarketTradingDays> {
        self.0
            .trading_days(market.into(), begin.0, end.0)?
            .try_into()
    }

    /// Get real-time quote
    fn realtime_quote(&self, symbols: Vec<String>) -> PyResult<Vec<RealtimeQuote>> {
        self.0
            .realtime_quote(symbols)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get real-time depth
    fn realtime_depth(&self, symbol: String) -> PyResult<SecurityDepth> {
        self.0.realtime_depth(symbol)?.try_into()
    }

    /// Get real-time brokers
    fn realtime_brokers(&self, symbol: String) -> PyResult<SecurityBrokers> {
        self.0.realtime_brokers(symbol)?.try_into()
    }

    /// Get real-time trades
    #[args(count = 500)]
    fn realtime_trades(&self, symbol: String, count: usize) -> PyResult<Vec<Trade>> {
        self.0
            .realtime_trades(symbol, count)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}