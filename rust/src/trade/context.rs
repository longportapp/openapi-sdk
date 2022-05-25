use std::sync::Arc;

use anyhow::Result;
use longbridge_httpcli::{HttpClient, Method};
use longbridge_wscli::WsClientError;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

use crate::{
    trade::{
        core::{Command, Core},
        AccountBalance, CashFlow, Execution, FundPositionsResponse, GetCashFlowOptions,
        GetHistoryExecutionsOptions, GetHistoryOrdersOptions, GetTodayExecutionsOptions,
        GetTodayOrdersOptions, Order, PushEvent, ReplaceOrderOptions, StockPositionsResponse,
        SubmitOrderOptions,
    },
    Config,
};

/// Response for withdraw order request
#[derive(Debug, Deserialize)]
pub struct SubmitOrderResponse {
    /// Order id
    pub order_id: String,
}

/// Trade context
#[derive(Clone)]
pub struct TradeContext {
    command_tx: mpsc::UnboundedSender<Command>,
    http_cli: HttpClient,
}

impl TradeContext {
    /// Create a `TradeContext`
    pub async fn try_new(
        config: Arc<Config>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<PushEvent>)> {
        let http_cli = config.create_http_client();
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (push_tx, push_rx) = mpsc::unbounded_channel();
        tokio::spawn(Core::try_new(config, command_rx, push_tx).await?.run());
        Ok((
            TradeContext {
                http_cli,
                command_tx,
            },
            push_rx,
        ))
    }

    /// Subscribe topics
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/trade-push#subscribe>
    pub async fn subscribe<I, T>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.command_tx
            .send(Command::Subscribe {
                topics: topics.into_iter().map(Into::into).collect(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Unsubscribe topics
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/trade-push#cancel-subscribe>
    pub async fn unsubscribe<I, T>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.command_tx
            .send(Command::Unsubscribe {
                topics: topics.into_iter().map(Into::into).collect(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Get history executions
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/execution/history_executions>
    pub async fn history_executions(
        &self,
        options: Option<GetHistoryExecutionsOptions>,
    ) -> Result<Vec<Execution>> {
        #[derive(Deserialize)]
        struct Response {
            trades: Vec<Execution>,
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/trade/execution/history")
            .query_params(options.unwrap_or_default())
            .response::<Response>()
            .send()
            .await?
            .trades)
    }

    /// Get today executions
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/execution/today_executions>
    pub async fn today_executions(
        &self,
        options: Option<GetTodayExecutionsOptions>,
    ) -> Result<Vec<Execution>> {
        #[derive(Deserialize)]
        struct Response {
            trades: Vec<Execution>,
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/trade/execution/today")
            .query_params(options.unwrap_or_default())
            .response::<Response>()
            .send()
            .await?
            .trades)
    }

    /// Get history orders
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/order/history_orders>
    pub async fn history_orders(
        &self,
        options: Option<GetHistoryOrdersOptions>,
    ) -> Result<Vec<Order>> {
        #[derive(Deserialize)]
        struct Response {
            orders: Vec<Order>,
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/trade/order/history")
            .query_params(options.unwrap_or_default())
            .response::<Response>()
            .send()
            .await?
            .orders)
    }

    /// Get today orders
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/trade/order/today_orders>
    pub async fn today_orders(&self, options: Option<GetTodayOrdersOptions>) -> Result<Vec<Order>> {
        #[derive(Deserialize)]
        struct Response {
            orders: Vec<Order>,
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/trade/order/today")
            .query_params(options.unwrap_or_default())
            .response::<Response>()
            .send()
            .await?
            .orders)
    }

    /// Replace order
    pub async fn replace_order(&self, options: ReplaceOrderOptions) -> Result<()> {
        Ok(self
            .http_cli
            .request(Method::PUT, "/v1/trade/order")
            .body(options)
            .send()
            .await?)
    }

    /// Submit order
    pub async fn submit_order(&self, options: SubmitOrderOptions) -> Result<SubmitOrderResponse> {
        Ok(self
            .http_cli
            .request(Method::POST, "/v1/trade/order")
            .body(options)
            .response::<_>()
            .send()
            .await?)
    }

    /// Withdraw order
    pub async fn withdraw_order(&self, order_id: impl Into<String>) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct Request {
            order_id: String,
        }

        Ok(self
            .http_cli
            .request(Method::DELETE, "/v1/trade/order")
            .query_params(Request {
                order_id: order_id.into(),
            })
            .send()
            .await?)
    }

    /// Get account balance
    pub async fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            list: Vec<AccountBalance>,
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/asset/account")
            .response::<Response>()
            .send()
            .await?
            .list)
    }

    /// Get cash flow
    pub async fn cash_flow(&self, options: GetCashFlowOptions) -> Result<Vec<CashFlow>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            list: Vec<CashFlow>,
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/asset/cashflow")
            .query_params(options)
            .response::<Response>()
            .send()
            .await?
            .list)
    }

    /// Get fund positions
    pub async fn fund_positions<I, T>(&self, symbols: I) -> Result<FundPositionsResponse>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        #[derive(Debug, Serialize)]
        struct Request {
            symbols: Vec<String>,
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/asset/fund")
            .query_params(Request {
                symbols: symbols.into_iter().map(Into::into).collect(),
            })
            .response::<FundPositionsResponse>()
            .send()
            .await?)
    }

    /// Get stock positions
    pub async fn stock_positions<I, T>(&self, symbols: I) -> Result<StockPositionsResponse>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        #[derive(Debug, Serialize)]
        struct Request {
            symbols: Vec<String>,
        }

        Ok(self
            .http_cli
            .request(Method::GET, "/v1/asset/stock")
            .query_params(Request {
                symbols: symbols.into_iter().map(Into::into).collect(),
            })
            .response::<StockPositionsResponse>()
            .send()
            .await?)
    }
}
