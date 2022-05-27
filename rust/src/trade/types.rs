use rust_decimal::Decimal;
use serde::Deserialize;
use strum_macros::{Display, EnumString};
use time::{Date, OffsetDateTime};

use crate::trade::serde_utils;

/// Order type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
#[allow(clippy::upper_case_acronyms)]
pub enum OrderType {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Limit Order
    #[strum(serialize = "LO")]
    LO,
    /// Enhanced Limit Order
    #[strum(serialize = "ELO")]
    ELO,
    /// Market Order
    #[strum(serialize = "MO")]
    MO,
    /// At-auction Order
    #[strum(serialize = "AO")]
    AO,
    /// At-auction Limit Order
    #[strum(serialize = "ALO")]
    ALO,
    /// Odd Lots
    #[strum(serialize = "ODD")]
    ODD,
    /// Limit If Touched
    #[strum(serialize = "LIT")]
    LIT,
    /// Market If Touched
    #[strum(serialize = "MIT")]
    MIT,
    /// Trailing Limit If Touched (Trailing Amount)
    #[strum(serialize = "TSLPAMT")]
    TSLPAMT,
    /// Trailing Limit If Touched (Trailing Percent)
    #[strum(serialize = "TSLPPCT")]
    TSLPPCT,
    /// Trailing Market If Touched (Trailing Amount)
    #[strum(serialize = "TSMAMT")]
    TSMAMT,
    /// Trailing Market If Touched (Trailing Percent)
    #[strum(serialize = "TSMPCT")]
    TSMPCT,
}

/// Order status
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum OrderStatus {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Not reported
    #[strum(serialize = "NotReported")]
    NotReported,
    /// Not reported (Replaced Order)
    #[strum(serialize = "ReplacedNotReported")]
    ReplacedNotReported,
    /// Not reported (Protected Order)
    #[strum(serialize = "ProtectedNotReported")]
    ProtectedNotReported,
    /// Not reported (Conditional Order)
    #[strum(serialize = "VarietiesNotReported")]
    VarietiesNotReported,
    /// Filled
    #[strum(serialize = "FilledStatus")]
    Filled,
    /// Wait To New
    #[strum(serialize = "WaitToNew")]
    WaitToNew,
    /// New
    #[strum(serialize = "NewStatus")]
    New,
    /// Wait To Replace
    #[strum(serialize = "WaitToReplace")]
    WaitToReplace,
    /// Pending Replace
    #[strum(serialize = "PendingReplaceStatus")]
    PendingReplace,
    /// Replaced
    #[strum(serialize = "ReplacedStatus")]
    Replaced,
    /// Partial Filled
    #[strum(serialize = "PartialFilledStatus")]
    PartialFilled,
    /// Wait To Cancel
    #[strum(serialize = "WaitToCancel")]
    WaitToCancel,
    /// Pending Cancel
    #[strum(serialize = "PendingCancelStatus")]
    PendingCancel,
    /// Rejected
    #[strum(serialize = "RejectedStatus")]
    Rejected,
    /// Canceled
    #[strum(serialize = "CanceledStatus")]
    Canceled,
    /// Expired
    #[strum(serialize = "ExpiredStatus")]
    Expired,
    /// Partial Withdrawal
    #[strum(serialize = "PartialWithdrawal")]
    PartialWithdrawal,
}

/// Execution
#[derive(Debug, Clone, Deserialize)]
pub struct Execution {
    /// Order ID
    pub order_id: String,
    /// Execution ID
    pub trade_id: String,
    /// Security code
    pub symbol: String,
    /// Trade done time
    #[serde(with = "serde_utils::timestamp")]
    pub trade_done_at: OffsetDateTime,
    /// Executed quantity
    pub quantity: Decimal,
    /// Executed price
    pub price: Decimal,
}

/// Order side
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum OrderSide {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Buy
    #[strum(serialize = "Buy")]
    Buy,
    /// Sell
    #[strum(serialize = "Sell")]
    Sell,
}

/// Order trigger price type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum TriggerPriceType {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Limit If Touched
    #[strum(serialize = "LIT")]
    LimitIfTouched,
    /// Market If Touched
    #[strum(serialize = "MIT")]
    MarketIfTouched,
}

/// Order tag
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum OrderTag {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Normal Order
    #[strum(serialize = "Normal")]
    Normal,
    /// Long term Order
    #[strum(serialize = "GTC")]
    LongTerm,
    /// Grey Order
    #[strum(serialize = "Grey")]
    Grey,
}

/// Time in force Type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum TimeInForceType {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Day Order
    #[strum(serialize = "Day")]
    Day,
    /// Good Til Canceled Order
    #[strum(serialize = "GTC")]
    GoodTilCanceled,
    /// Good Til Date Order
    #[strum(serialize = "GTD")]
    GoodTilDate,
}

/// Trigger status
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum TriggerStatus {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Deactive
    #[strum(serialize = "DEACTIVE")]
    Deactive,
    /// Active
    #[strum(serialize = "ACTIVE")]
    Active,
    /// Released
    #[strum(serialize = "RELEASED")]
    Released,
}

/// Enable or disable outside regular trading hours
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum OutsideRTH {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Regular trading hour only
    #[strum(serialize = "RTH_ONLY")]
    RTHOnly,
    /// Any time
    #[strum(serialize = "ANY_TIME")]
    AnyTime,
}

/// Order
#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    /// Order ID
    pub order_id: String,
    /// Order status
    pub status: OrderStatus,
    /// Stock name
    pub stock_name: String,
    /// Submitted quantity
    pub quantity: Decimal,
    /// Executed quantity
    #[serde(with = "serde_utils::decimal_opt_0_is_none")]
    pub executed_quantity: Option<Decimal>,
    /// Submitted price
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub price: Option<Decimal>,
    /// Executed price
    #[serde(with = "serde_utils::decimal_opt_0_is_none")]
    pub executed_price: Option<Decimal>,
    /// Submitted time
    #[serde(with = "serde_utils::timestamp_opt")]
    pub submitted_at: Option<OffsetDateTime>,
    /// Order side
    pub side: OrderSide,
    /// Security code
    pub symbol: String,
    /// Order type
    pub order_type: OrderType,
    /// Last done
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub last_done: Option<Decimal>,
    /// `LIT` / `MIT` Order Trigger Price
    #[serde(with = "serde_utils::decimal_opt_0_is_none")]
    pub trigger_price: Option<Decimal>,
    /// Rejected Message or remark
    pub msg: String,
    /// Order tag
    pub tag: OrderTag,
    /// Time in force type
    pub time_in_force: TimeInForceType,
    /// Long term order expire date
    #[serde(with = "serde_utils::date_opt")]
    pub expire_date: Option<Date>,
    /// Last updated time
    #[serde(with = "serde_utils::timestamp_opt")]
    pub updated_at: Option<OffsetDateTime>,
    /// Conditional order trigger time
    #[serde(with = "serde_utils::timestamp_opt")]
    pub trigger_at: Option<OffsetDateTime>,
    /// `TSMAMT` / `TSLPAMT` order trailing amount
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub trailing_amount: Option<Decimal>,
    /// `TSMPCT` / `TSLPPCT` order trailing percent
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub trailing_percent: Option<Decimal>,
    /// `TSLPAMT` / `TSLPPCT` order limit offset amount
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub limit_offset: Option<Decimal>,
    /// Conditional order trigger status
    #[serde(with = "serde_utils::trigger_status")]
    pub trigger_status: Option<TriggerStatus>,
    /// Currency
    pub currency: String,
    /// Enable or disable outside regular trading hours
    #[serde(with = "serde_utils::outside_rth")]
    pub outside_rth: Option<OutsideRTH>,
}

/// Cash info
#[derive(Debug, Clone, Deserialize)]
pub struct CashInfo {
    /// Withdraw cash
    pub withdraw_cash: Decimal,
    /// Available cash
    pub available_cash: Decimal,
    /// Frozen cash
    pub frozen_cash: Decimal,
    /// Cash to be settled
    pub settling_cash: Decimal,
    /// Currency
    pub currency: String,
}

/// Account balance
#[derive(Debug, Clone, Deserialize)]
pub struct AccountBalance {
    /// Total cash
    pub total_cash: Decimal,
    /// Maximum financing amount
    pub max_finance_amount: Decimal,
    /// Remaining financing amount
    pub remaining_finance_amount: Decimal,
    /// Risk control level
    #[serde(with = "serde_utils::number_str_opt")]
    pub risk_level: Option<i32>,
    /// Margin call
    pub margin_call: Decimal,
    /// Currency
    pub currency: String,
    /// Cash details
    #[serde(default)]
    pub cash_infos: Vec<CashInfo>,
}

/// Balance type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum BalanceType {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Limit Order
    #[strum(serialize = "1")]
    Cash,
    /// Stock
    #[strum(serialize = "2")]
    Stock,
    /// Fund
    #[strum(serialize = "3")]
    Fund,
}

/// Cash flow direction
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum CashFlowDirection {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Out
    #[strum(serialize = "1")]
    Out,
    /// Stock
    #[strum(serialize = "2")]
    In,
}

/// Cash flow
#[derive(Debug, Clone, Deserialize)]
pub struct CashFlow {
    /// Cash flow name
    pub transaction_flow_name: String,
    /// Outflow direction
    pub direction: CashFlowDirection,
    /// Balance type
    pub business_type: BalanceType,
    /// Cash amount
    pub balance: Decimal,
    /// Cash currency
    pub currency: String,
    /// Business time
    #[serde(with = "serde_utils::timestamp")]
    pub business_time: OffsetDateTime,
    /// Associated Stock code information
    #[serde(with = "serde_utils::cash_flow_symbol")]
    pub symbol: Option<String>,
    /// Cash flow description
    pub description: String,
}

/// Fund positions response
#[derive(Debug, Clone, Deserialize)]
pub struct FundPositionsResponse {
    /// Account type
    pub account_channel: String,
    /// Fund positions
    #[serde(default)]
    pub positions: Vec<FundPosition>,
}

/// Fund position
#[derive(Debug, Clone, Deserialize)]
pub struct FundPosition {
    /// Fund ISIN code
    pub symbol: String,
    /// Current equity
    pub current_net_asset_value: Decimal,
    /// Current equity time
    pub net_asset_value_day: Decimal,
    /// Fund name
    pub symbol_name: String,
    /// Currency
    pub currency: String,
    /// Net cost
    pub cost_net_asset_value: Decimal,
}

/// Stock positions response
#[derive(Debug, Clone, Deserialize)]
pub struct StockPositionsResponse {
    /// Account type
    pub account_channel: String,
    /// Stock positions
    #[serde(default)]
    pub positions: Vec<StockPosition>,
}

/// Stock position
#[derive(Debug, Clone, Deserialize)]
pub struct StockPosition {
    /// Stock code
    pub symbol: String,
    /// Stock name
    pub symbol_name: String,
    /// The number of holdings
    pub quality: Decimal,
    /// Available quantity
    pub available_quality: Decimal,
    /// Currency
    pub currency: String,
    /// Cost Price(According to the client's choice of average purchase or
    /// diluted cost)
    pub cost_price: Decimal,
}

impl_serde_for_enum_string!(
    OrderType,
    OrderStatus,
    OrderSide,
    TriggerPriceType,
    OrderTag,
    TimeInForceType,
    TriggerStatus,
    OutsideRTH,
    BalanceType,
    CashFlowDirection
);
impl_default_for_enum_string!(
    OrderType,
    OrderStatus,
    OrderSide,
    TriggerPriceType,
    OrderTag,
    TimeInForceType,
    TriggerStatus,
    OutsideRTH,
    BalanceType,
    CashFlowDirection
);
