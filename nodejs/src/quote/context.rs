use std::sync::Arc;

use longbridge::quote::PushEventDetail;
use napi::{
    bindgen_prelude::Either4, threadsafe_function::ThreadsafeFunctionCallMode, JsFunction, Result,
};

use crate::{
    config::Config,
    error::ErrorNewType,
    quote::{
        push::{PushBrokersEvent, PushDepthEvent, PushQuoteEvent, PushTradesEvent},
        types::{
            AdjustType, Candlestick, IntradayLine, IssuerInfo, MarketTradingDays,
            MarketTradingSession, OptionQuote, ParticipantInfo, Period, RealtimeQuote,
            SecurityBrokers, SecurityDepth, SecurityQuote, SecurityStaticInfo, StrikePriceInfo,
            SubType, SubTypes, Trade, WarrantQuote,
        },
    },
    time::NaiveDate,
    types::Market,
    utils::JsCallback,
};

type QuoteCallback =
    JsCallback<Either4<PushQuoteEvent, PushDepthEvent, PushBrokersEvent, PushTradesEvent>>;

/// Quote context
#[napi_derive::napi]
#[derive(Clone)]
pub struct QuoteContext {
    config: longbridge::Config,
    ctx: Option<longbridge::quote::QuoteContext>,
    on_push: Option<QuoteCallback>,
}

#[napi_derive::napi]
impl QuoteContext {
    #[napi(
        ts_args_type = "callback: (err: null | Error, event: PushQuoteEvent | PushDepthEvent | PushBrokersEvent | PushTradesEvent) => void"
    )]
    pub fn new(config: &Config, on_push: Option<JsFunction>) -> Result<QuoteContext> {
        Ok(QuoteContext {
            config: config.0.clone(),
            ctx: None,
            on_push: on_push
                .map(|on_push| on_push.create_threadsafe_function(32, |ctx| Ok(vec![ctx.value])))
                .transpose()?,
        })
    }

    /// Open quote context
    #[napi]
    pub async fn open(&mut self) -> Result<()> {
        check_ctx_exists!(self.ctx);

        let (ctx, mut receiver) =
            longbridge::quote::QuoteContext::try_new(Arc::new(self.config.clone()))
                .await
                .map_err(ErrorNewType)?;
        self.ctx = Some(ctx);

        let handler = self.on_push.take();
        tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                match msg.detail {
                    PushEventDetail::Quote(quote) => {
                        if let Some(handler) = &handler {
                            if let Ok(quote) = quote.try_into() {
                                handler.call(
                                    Ok(Either4::A(PushQuoteEvent {
                                        symbol: msg.symbol,
                                        data: quote,
                                    })),
                                    ThreadsafeFunctionCallMode::Blocking,
                                );
                            }
                        }
                    }
                    PushEventDetail::Depth(depth) => {
                        if let Some(handler) = &handler {
                            if let Ok(depth) = depth.try_into() {
                                handler.call(
                                    Ok(Either4::B(PushDepthEvent {
                                        symbol: msg.symbol,
                                        data: depth,
                                    })),
                                    ThreadsafeFunctionCallMode::Blocking,
                                );
                            }
                        }
                    }
                    PushEventDetail::Brokers(brokers) => {
                        if let Some(handler) = &handler {
                            if let Ok(brokers) = brokers.try_into() {
                                handler.call(
                                    Ok(Either4::C(PushBrokersEvent {
                                        symbol: msg.symbol,
                                        data: brokers,
                                    })),
                                    ThreadsafeFunctionCallMode::Blocking,
                                );
                            }
                        }
                    }
                    PushEventDetail::Trade(trades) => {
                        if let Some(handler) = &handler {
                            if let Ok(trades) = trades.try_into() {
                                handler.call(
                                    Ok(Either4::D(PushTradesEvent {
                                        symbol: msg.symbol,
                                        data: trades,
                                    })),
                                    ThreadsafeFunctionCallMode::Blocking,
                                );
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// Subscribe
    ///
    /// Example
    ///
    /// ```javascript
    /// import { Config, QuoteContext, SubType } from 'longbridge'
    ///
    /// let config = Config.fromEnv()
    /// let ctx = await QuoteContext.new(config);
    ///
    /// ctx.on_quote = (_, data) => console.log(data)
    /// await ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote], true)
    /// ```
    #[napi]
    pub async fn subscribe(
        &self,
        symbols: Vec<String>,
        sub_types: Vec<SubType>,
        is_first_push: bool,
    ) -> Result<()> {
        get_ctx!(self.ctx)
            .subscribe(symbols, SubTypes(sub_types), is_first_push)
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Unsubscribe
    #[napi]
    pub async fn unsubscribe(&self, symbols: Vec<String>, sub_types: Vec<SubType>) -> Result<()> {
        get_ctx!(self.ctx)
            .unsubscribe(symbols, SubTypes(sub_types))
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Get basic information of securities
    #[napi]
    pub async fn static_info(&self, symbols: Vec<String>) -> Result<Vec<SecurityStaticInfo>> {
        get_ctx!(self.ctx)
            .static_info(symbols)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of securities
    #[napi]
    pub async fn quote(&self, symbols: Vec<String>) -> Result<Vec<SecurityQuote>> {
        get_ctx!(self.ctx)
            .quote(symbols)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of option securities
    #[napi]
    pub async fn option_quote(&self, symbols: Vec<String>) -> Result<Vec<OptionQuote>> {
        get_ctx!(self.ctx)
            .option_quote(symbols)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get quote of warrant securities
    #[napi]
    pub async fn warrant_quote(&self, symbols: Vec<String>) -> Result<Vec<WarrantQuote>> {
        get_ctx!(self.ctx)
            .warrant_quote(symbols)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security depth
    #[napi]
    pub async fn depth(&self, symbol: String) -> Result<SecurityDepth> {
        get_ctx!(self.ctx)
            .depth(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get security brokers
    #[napi]
    pub async fn brokers(&self, symbol: String) -> Result<SecurityBrokers> {
        get_ctx!(self.ctx)
            .brokers(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get participants
    #[napi]
    pub async fn participants(&self) -> Result<Vec<ParticipantInfo>> {
        get_ctx!(self.ctx)
            .participants()
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security trades
    #[napi]
    pub async fn trades(&self, symbol: String, count: i32) -> Result<Vec<Trade>> {
        get_ctx!(self.ctx)
            .trades(symbol, count.max(0) as usize)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security intraday
    #[napi]
    pub async fn intraday(&self, symbol: String) -> Result<Vec<IntradayLine>> {
        get_ctx!(self.ctx)
            .intraday(symbol)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get security candlesticks
    #[napi]
    pub async fn candlesticks(
        &self,
        symbol: String,
        period: Period,
        count: i32,
        adjust_type: AdjustType,
    ) -> Result<Vec<Candlestick>> {
        get_ctx!(self.ctx)
            .candlesticks(
                symbol,
                period.into(),
                count.max(0) as usize,
                adjust_type.into(),
            )
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get option chain expiry date list
    #[napi]
    pub async fn option_chain_expiry_date_list(&self, symbol: String) -> Result<Vec<NaiveDate>> {
        Ok(get_ctx!(self.ctx)
            .option_chain_expiry_date_list(symbol)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    /// Get option chain info by date
    #[napi]
    pub async fn option_chain_info_by_date(
        &self,
        symbol: String,
        expiry_date: &NaiveDate,
    ) -> Result<Vec<StrikePriceInfo>> {
        get_ctx!(self.ctx)
            .option_chain_info_by_date(symbol, expiry_date.0)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get warrant issuers
    #[napi]
    pub async fn warrant_issuers(&self) -> Result<Vec<IssuerInfo>> {
        get_ctx!(self.ctx)
            .warrant_issuers()
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get trading session of the day
    #[napi]
    pub async fn trading_session(&self) -> Result<Vec<MarketTradingSession>> {
        get_ctx!(self.ctx)
            .trading_session()
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get trading session of the day
    #[napi]
    pub async fn trading_days(
        &self,
        market: Market,
        begin: &NaiveDate,
        end: &NaiveDate,
    ) -> Result<MarketTradingDays> {
        get_ctx!(self.ctx)
            .trading_days(market.into(), begin.0, end.0)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get real-time quote
    #[napi]
    pub async fn realtime_quote(&self, symbols: Vec<String>) -> Result<Vec<RealtimeQuote>> {
        get_ctx!(self.ctx)
            .realtime_quote(symbols)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get real-time depth
    #[napi]
    pub async fn realtime_depth(&self, symbol: String) -> Result<SecurityDepth> {
        get_ctx!(self.ctx)
            .realtime_depth(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get real-time brokers
    #[napi]
    pub async fn realtime_brokers(&self, symbol: String) -> Result<SecurityBrokers> {
        get_ctx!(self.ctx)
            .realtime_brokers(symbol)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get real-time trades
    #[napi]
    pub async fn realtime_trades(&self, symbol: String, count: i32) -> Result<Vec<Trade>> {
        get_ctx!(self.ctx)
            .realtime_trades(symbol, count.max(0) as usize)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
