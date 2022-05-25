use std::sync::Arc;

use anyhow::Result;

use crate::{
    blocking::runtime::BlockingRuntime,
    trade::{
        AccountBalance, CashFlow, Execution, FundPositionsResponse, GetCashFlowOptions,
        GetFundPositionsOptions, GetHistoryExecutionsOptions, GetHistoryOrdersOptions,
        GetStockPositionsOptions, GetTodayExecutionsOptions, GetTodayOrdersOptions, Order,
        PushEvent, ReplaceOrderOptions, StockPositionsResponse, SubmitOrderOptions,
        SubmitOrderResponse, TopicType, TradeContext,
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
    pub fn subscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.subscribe(topics).await })
    }

    /// Unsubscribe topics
    pub fn unsubscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.unsubscribe(topics).await })
    }

    /// Get history executions
    pub fn history_executions(
        &self,
        options: impl Into<Option<GetHistoryExecutionsOptions>> + Send + 'static,
    ) -> Result<Vec<Execution>> {
        self.rt
            .call(move |ctx| async move { ctx.history_executions(options).await })
    }

    /// Get today executions
    pub fn today_executions(
        &self,
        options: impl Into<Option<GetTodayExecutionsOptions>> + Send + 'static,
    ) -> Result<Vec<Execution>> {
        self.rt
            .call(move |ctx| async move { ctx.today_executions(options).await })
    }

    /// Get history orders
    pub fn history_orders(
        &self,
        options: impl Into<Option<GetHistoryOrdersOptions>> + Send + 'static,
    ) -> Result<Vec<Order>> {
        self.rt
            .call(move |ctx| async move { ctx.history_orders(options).await })
    }

    /// Get today orders
    pub fn today_orders(
        &self,
        options: impl Into<Option<GetTodayOrdersOptions>> + Send + 'static,
    ) -> Result<Vec<Order>> {
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
    ///
    /// If `symbols` is empty, it means to get all fund positions.
    pub fn fund_positions(
        &self,
        opts: impl Into<Option<GetFundPositionsOptions>> + Send + 'static,
    ) -> Result<FundPositionsResponse> {
        self.rt
            .call(move |ctx| async move { ctx.fund_positions(opts).await })
    }

    /// Get stock positions
    ///
    /// If `symbols` is empty, it means to get all stock positions.
    pub fn stock_positions(
        &self,
        opts: impl Into<Option<GetStockPositionsOptions>> + Send + 'static,
    ) -> Result<StockPositionsResponse> {
        self.rt
            .call(move |ctx| async move { ctx.stock_positions(opts).await })
    }
}
