use std::sync::Arc;

use anyhow::Result;

use crate::{
    blocking::runtime::BlockingRuntime,
    trade::{
        AccountBalance, CashFlow, FundPositionsResponse, GetCashFlowOptions,
        GetHistoryExecutionsOptions, GetHistoryOrdersOptions, GetTodayExecutionsOptions,
        GetTodayOrdersOptions, Order, PushEvent, ReplaceOrderOptions, StockPositionsResponse,
        SubmitOrderOptions, SubmitOrderResponse, Trade, TradeContext,
    },
    Config,
};

/// Trade context
pub struct TradeContextSync {
    rt: BlockingRuntime<TradeContext>,
}

impl TradeContextSync {
    /// Create a `TradeContextSync` object
    pub fn try_new<F>(config: Arc<Config>, push_callback: F) -> Result<Self>
    where
        F: FnMut(PushEvent) + Send + 'static,
    {
        let rt = BlockingRuntime::try_new(move || TradeContext::try_new(config), push_callback)?;
        Ok(Self { rt })
    }

    /// Subscribe topics
    pub fn subscribe<I, T>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        T: Into<String>,
    {
        self.rt
            .call(move |ctx| async move { ctx.subscribe(topics).await })
    }

    /// Unsubscribe topics
    pub fn unsubscribe<I, T>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        T: Into<String>,
    {
        self.rt
            .call(move |ctx| async move { ctx.unsubscribe(topics).await })
    }

    /// Get history executions
    pub fn history_executions(
        &self,
        options: Option<GetHistoryExecutionsOptions>,
    ) -> Result<Vec<Trade>> {
        self.rt
            .call(move |ctx| async move { ctx.history_executions(options).await })
    }

    /// Get today executions
    pub fn today_executions(
        &self,
        options: Option<GetTodayExecutionsOptions>,
    ) -> Result<Vec<Trade>> {
        self.rt
            .call(move |ctx| async move { ctx.today_executions(options).await })
    }

    /// Get history orders
    pub fn history_orders(&self, options: Option<GetHistoryOrdersOptions>) -> Result<Vec<Order>> {
        self.rt
            .call(move |ctx| async move { ctx.history_orders(options).await })
    }

    /// Get today orders
    pub fn today_orders(&self, options: Option<GetTodayOrdersOptions>) -> Result<Vec<Order>> {
        self.rt
            .call(move |ctx| async move { ctx.today_orders(options).await })
    }

    /// Replace order
    pub fn replace_order(&self, options: ReplaceOrderOptions) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.replace_order(options).await })
    }

    /// Submit order
    pub fn submit_order(&self, options: SubmitOrderOptions) -> Result<SubmitOrderResponse> {
        self.rt
            .call(move |ctx| async move { ctx.submit_order(options).await })
    }

    /// Withdraw order
    pub fn withdraw_order(&self, order_id: impl Into<String> + Send + 'static) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.withdraw_order(order_id).await })
    }

    /// Get account balance
    pub fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        self.rt
            .call(move |ctx| async move { ctx.account_balance().await })
    }

    /// Get cash flow
    pub fn cash_flow(&self, options: GetCashFlowOptions) -> Result<Vec<CashFlow>> {
        self.rt
            .call(move |ctx| async move { ctx.cash_flow(options).await })
    }

    /// Get fund positions
    pub fn fund_positions<I, T>(&self, symbols: I) -> Result<FundPositionsResponse>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.fund_positions(symbols).await })
    }

    /// Get stock positions
    pub fn stock_positions<I, T>(&self, symbols: I) -> Result<StockPositionsResponse>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.stock_positions(symbols).await })
    }
}
