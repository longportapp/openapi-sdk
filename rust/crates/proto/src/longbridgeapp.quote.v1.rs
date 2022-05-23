#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityRequest {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiSecurityRequest {
    #[prost(string, repeated, tag="1")]
    pub symbol: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityStaticInfoResponse {
    #[prost(message, repeated, tag="1")]
    pub secu_static_info: ::prost::alloc::vec::Vec<StaticInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticInfo {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name_cn: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name_en: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub name_hk: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub listing_date: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub exchange: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub currency: ::prost::alloc::string::String,
    #[prost(int32, tag="8")]
    pub lot_size: i32,
    #[prost(int64, tag="9")]
    pub total_shares: i64,
    #[prost(int64, tag="10")]
    pub circulating_shares: i64,
    #[prost(int64, tag="11")]
    pub hk_shares: i64,
    #[prost(string, tag="12")]
    pub eps: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub eps_ttm: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub bps: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub dividend_yield: ::prost::alloc::string::String,
    #[prost(int32, repeated, tag="16")]
    pub stock_derivatives: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityQuoteResponse {
    #[prost(message, repeated, tag="1")]
    pub secu_quote: ::prost::alloc::vec::Vec<SecurityQuote>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityQuote {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub last_done: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub prev_close: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub open: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub high: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub low: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub timestamp: i64,
    #[prost(int64, tag="8")]
    pub volume: i64,
    #[prost(string, tag="9")]
    pub turnover: ::prost::alloc::string::String,
    #[prost(enumeration="TradeStatus", tag="10")]
    pub trade_status: i32,
    #[prost(message, optional, tag="11")]
    pub pre_market_quote: ::core::option::Option<PrePostQuote>,
    #[prost(message, optional, tag="12")]
    pub post_market_quote: ::core::option::Option<PrePostQuote>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrePostQuote {
    #[prost(string, tag="1")]
    pub last_done: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(int64, tag="3")]
    pub volume: i64,
    #[prost(string, tag="4")]
    pub turnover: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub high: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub low: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub prev_close: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionQuoteResponse {
    #[prost(message, repeated, tag="1")]
    pub secu_quote: ::prost::alloc::vec::Vec<OptionQuote>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionQuote {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub last_done: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub prev_close: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub open: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub high: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub low: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub timestamp: i64,
    #[prost(int64, tag="8")]
    pub volume: i64,
    #[prost(string, tag="9")]
    pub turnover: ::prost::alloc::string::String,
    #[prost(enumeration="TradeStatus", tag="10")]
    pub trade_status: i32,
    #[prost(message, optional, tag="11")]
    pub option_extend: ::core::option::Option<OptionExtend>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionExtend {
    #[prost(string, tag="1")]
    pub implied_volatility: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub open_interest: i64,
    #[prost(string, tag="3")]
    pub expiry_date: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub strike_price: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub contract_multiplier: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub contract_type: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub contract_size: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub direction: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub historical_volatility: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub underlying_symbol: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarrantQuoteResponse {
    #[prost(message, repeated, tag="2")]
    pub secu_quote: ::prost::alloc::vec::Vec<WarrantQuote>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarrantQuote {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub last_done: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub prev_close: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub open: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub high: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub low: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub timestamp: i64,
    #[prost(int64, tag="8")]
    pub volume: i64,
    #[prost(string, tag="9")]
    pub turnover: ::prost::alloc::string::String,
    #[prost(enumeration="TradeStatus", tag="10")]
    pub trade_status: i32,
    #[prost(message, optional, tag="11")]
    pub warrant_extend: ::core::option::Option<WarrantExtend>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarrantExtend {
    #[prost(string, tag="1")]
    pub implied_volatility: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub expiry_date: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub last_trade_date: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub outstanding_ratio: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub outstanding_qty: i64,
    #[prost(string, tag="6")]
    pub conversion_ratio: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub strike_price: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub upper_strike_price: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub lower_strike_price: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub call_price: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub underlying_symbol: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityDepthResponse {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub ask: ::prost::alloc::vec::Vec<Depth>,
    #[prost(message, repeated, tag="3")]
    pub bid: ::prost::alloc::vec::Vec<Depth>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Depth {
    #[prost(int32, tag="1")]
    pub position: i32,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub volume: i64,
    #[prost(int64, tag="4")]
    pub order_num: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityBrokersResponse {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub ask_brokers: ::prost::alloc::vec::Vec<Brokers>,
    #[prost(message, repeated, tag="3")]
    pub bid_brokers: ::prost::alloc::vec::Vec<Brokers>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brokers {
    #[prost(int32, tag="1")]
    pub position: i32,
    #[prost(int32, repeated, tag="2")]
    pub broker_ids: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantBrokerIdsResponse {
    #[prost(message, repeated, tag="1")]
    pub participant_broker_numbers: ::prost::alloc::vec::Vec<ParticipantInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantInfo {
    #[prost(int32, repeated, tag="1")]
    pub broker_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag="2")]
    pub participant_name_cn: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub participant_name_en: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub participant_name_hk: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityTradeRequest {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub count: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityTradeResponse {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub trades: ::prost::alloc::vec::Vec<Trade>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub volume: i64,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
    #[prost(string, tag="4")]
    pub trade_type: ::prost::alloc::string::String,
    #[prost(int32, tag="5")]
    pub direction: i32,
    #[prost(enumeration="TradeSession", tag="6")]
    pub trade_session: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityIntradayRequest {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityIntradayResponse {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub lines: ::prost::alloc::vec::Vec<Line>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Line {
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(int64, tag="3")]
    pub volume: i64,
    #[prost(string, tag="4")]
    pub turnover: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub avg_price: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityCandlestickRequest {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(enumeration="Period", tag="2")]
    pub period: i32,
    #[prost(int32, tag="3")]
    pub count: i32,
    #[prost(enumeration="AdjustType", tag="4")]
    pub adjust_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityCandlestickResponse {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub candlesticks: ::prost::alloc::vec::Vec<Candlestick>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candlestick {
    #[prost(string, tag="1")]
    pub close: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub open: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub low: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub high: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub volume: i64,
    #[prost(string, tag="6")]
    pub turnover: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionChainDateListResponse {
    #[prost(string, repeated, tag="1")]
    pub expiry_date: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionChainDateStrikeInfoRequest {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub expiry_date: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionChainDateStrikeInfoResponse {
    #[prost(message, repeated, tag="1")]
    pub strike_price_info: ::prost::alloc::vec::Vec<StrikePriceInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrikePriceInfo {
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub call_symbol: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub put_symbol: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub standard: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssuerInfoResponse {
    #[prost(message, repeated, tag="1")]
    pub issuer_info: ::prost::alloc::vec::Vec<IssuerInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssuerInfo {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(string, tag="2")]
    pub name_cn: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name_en: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub name_hk: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarrantFilterListRequest {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub filter_config: ::core::option::Option<FilterConfig>,
    #[prost(int32, tag="3")]
    pub language: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfig {
    #[prost(int32, tag="1")]
    pub sort_by: i32,
    #[prost(int32, tag="2")]
    pub sort_order: i32,
    #[prost(int32, tag="3")]
    pub sort_offset: i32,
    #[prost(int32, tag="4")]
    pub sort_count: i32,
    #[prost(int32, repeated, tag="5")]
    pub r#type: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag="6")]
    pub issuer: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag="7")]
    pub expiry_date: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag="8")]
    pub price_type: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag="9")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarrantFilterListResponse {
    #[prost(message, repeated, tag="1")]
    pub warrant_list: ::prost::alloc::vec::Vec<FilterWarrant>,
    #[prost(int32, tag="2")]
    pub total_count: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterWarrant {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub last_done: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub change_rate: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub change_val: ::prost::alloc::string::String,
    #[prost(int64, tag="6")]
    pub volume: i64,
    #[prost(string, tag="7")]
    pub turnover: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub expiry_date: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub strike_price: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub upper_strike_price: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub lower_strike_price: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub outstanding_qty: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub outstanding_ratio: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub premium: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub itm_otm: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub implied_volatility: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub delta: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub call_price: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub to_call_price: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub effective_leverage: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub leverage_ratio: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub conversion_ratio: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub balance_point: ::prost::alloc::string::String,
    #[prost(string, tag="24")]
    pub state: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketTradePeriodResponse {
    #[prost(message, repeated, tag="1")]
    pub market_trade_session: ::prost::alloc::vec::Vec<MarketTradePeriod>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketTradePeriod {
    #[prost(string, tag="1")]
    pub market: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub trade_session: ::prost::alloc::vec::Vec<TradePeriod>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradePeriod {
    #[prost(int32, tag="1")]
    pub beg_time: i32,
    #[prost(int32, tag="2")]
    pub end_time: i32,
    #[prost(enumeration="TradeSession", tag="3")]
    pub trade_session: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionResponse {
    #[prost(message, repeated, tag="1")]
    pub sub_list: ::prost::alloc::vec::Vec<SubTypeList>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubTypeList {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(enumeration="SubType", repeated, tag="2")]
    pub sub_type: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRequest {
    #[prost(string, repeated, tag="1")]
    pub symbol: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="SubType", repeated, tag="2")]
    pub sub_type: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, tag="3")]
    pub is_first_push: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubscribeRequest {
    #[prost(string, repeated, tag="1")]
    pub symbol: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="SubType", repeated, tag="2")]
    pub sub_type: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, tag="3")]
    pub unsub_all: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubscribeResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushQuote {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub sequence: i64,
    #[prost(string, tag="3")]
    pub last_done: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub open: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub high: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub low: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub timestamp: i64,
    #[prost(int64, tag="8")]
    pub volume: i64,
    #[prost(string, tag="9")]
    pub turnover: ::prost::alloc::string::String,
    #[prost(enumeration="TradeStatus", tag="10")]
    pub trade_status: i32,
    #[prost(enumeration="TradeSession", tag="11")]
    pub trade_session: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushDepth {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub sequence: i64,
    #[prost(message, repeated, tag="3")]
    pub ask: ::prost::alloc::vec::Vec<Depth>,
    #[prost(message, repeated, tag="4")]
    pub bid: ::prost::alloc::vec::Vec<Depth>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushBrokers {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub sequence: i64,
    #[prost(message, repeated, tag="3")]
    pub ask_brokers: ::prost::alloc::vec::Vec<Brokers>,
    #[prost(message, repeated, tag="4")]
    pub bid_brokers: ::prost::alloc::vec::Vec<Brokers>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushTrade {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub sequence: i64,
    #[prost(message, repeated, tag="3")]
    pub trade: ::prost::alloc::vec::Vec<Trade>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketTradeDayRequest {
    #[prost(string, tag="1")]
    pub market: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub beg_day: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub end_day: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketTradeDayResponse {
    #[prost(string, repeated, tag="1")]
    pub trade_day: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub half_trade_day: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 协议指令定义
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Command {
    UnknownCommand = 0,
    ///心跳
    HeartBeat = 1,
    ///鉴权
    Auth = 2,
    ///重新连接
    Reconnect = 3,
    ///查询连接的已订阅数据
    Subscription = 5,
    ///订阅行情数据
    Subscribe = 6,
    ///取消订阅行情数据
    Unsubscribe = 7,
    ///查询各市场的当日交易时段
    QueryMarketTradePeriod = 8,
    ///查询交易日
    QueryMarketTradeDay = 9,
    ///查询标的基础信息
    QuerySecurityStaticInfo = 10,
    ///查询标的行情(所有标的通用行情)
    QuerySecurityQuote = 11,
    ///查询期权行情(仅支持期权)
    QueryOptionQuote = 12,
    ///查询轮证行情(仅支持轮证)
    QueryWarrantQuote = 13,
    ///查询盘口
    QueryDepth = 14,
    ///查询经纪队列
    QueryBrokers = 15,
    ///查询券商经纪席位
    QueryParticipantBrokerIds = 16,
    ///查询成交明细
    QueryTrade = 17,
    ///查询当日分时
    QueryIntraday = 18,
    ///查询k线
    QueryCandlestick = 19,
    ///查询标的期权链日期列表
    QueryOptionChainDate = 20,
    ///查询标的期权链某日的行权价信息
    QueryOptionChainDateStrikeInfo = 21,
    ///查询轮证发行商对应Id
    QueryWarrantIssuerInfo = 22,
    ///查询轮证筛选列表
    QueryWarrantFilterList = 23,
    ///推送行情
    PushQuoteData = 101,
    ///推送盘口
    PushDepthData = 102,
    ///推送经纪队列
    PushBrokersData = 103,
    ///推送成交明细
    PushTradeData = 104,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeStatus {
    Normal = 0,
    Halted = 1,
    Delisted = 2,
    Fuse = 3,
    PrepareList = 4,
    CodeMoved = 5,
    ToBeOpened = 6,
    SplitStockHalts = 7,
    Expired = 8,
    WarrantPrepareList = 9,
    SuspendTrade = 10,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeSession {
    NormalTrade = 0,
    PreTrade = 1,
    PostTrade = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdjustType {
    NoAdjust = 0,
    ForwardAdjust = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Period {
    UnknownPeriod = 0,
    OneMinute = 1,
    FiveMinute = 5,
    FifteenMinute = 15,
    ThirtyMinute = 30,
    SixtyMinute = 60,
    Day = 1000,
    Week = 2000,
    Month = 3000,
    Year = 4000,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubType {
    UnknownType = 0,
    Quote = 1,
    Depth = 2,
    Brokers = 3,
    Trade = 4,
}
