from datetime import datetime, date, time
from decimal import Decimal
from typing import List, Optional, Type
from typing_extensions import Protocol

from longbridge import Config, Market


class PushQuote:
    """
    Quote message
    """

    last_done: Decimal
    """
    Latest price
    """

    open: Decimal
    """
    Open
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    trade_status: Type[TradeStatus]
    """
    Security trading status
    """

    trade_session: Type[TradeSession]
    """
    Trade session
    """


class PushDepth:
    """
    Depth message
    """

    asks: List[Depth]
    """
    Ask depth
    """

    bids: List[Depth]
    """
    Bid depth
    """


class PushBrokers:
    """
    Brokers message
    """

    ask_brokers: List[Brokers]
    """
    Ask brokers
    """

    bid_brokers: List[Brokers]
    """
    Bid brokers
    """


class PushTrades:
    """
    Trades message
    """

    trades: List[Trade]
    """
    Trades data
    """


class QuoteHandler(Protocol):
    """
    Quote push message handler
    """

    def on_push(self, symbol: str, msg: PushQuote | PushDepth | PushBrokers | PushTrades) -> None:
        """
        Push message callback
        """


class SubType:
    """
    Subscription flags
    """

    class Quote(SubType):
        """
        Quote
        """

    class Depth(SubType):
        """
        Depth
        """

    class Brokers(SubType):
        """
        Broker
        """

    class Trade(SubType):
        """
        Trade
        """


class DerivativeType:
    """
    Derivative type
    """

    class Option(DerivativeType):
        """
        US stock options
        """

    class Warrant(DerivativeType):
        """
        HK warrants
        """


class SecuritiyStaticInfo:
    """
    The basic information of securities
    """

    symbol: str
    """
    Security code
    """

    name_cn: str
    """
    Security name (zh-CN)
    """

    name_en: str
    """
    Security name (en)
    """

    name_hk: str
    """
    Security name (zh-HK)
    """

    exchange: str
    """
    Exchange which the security belongs to
    """

    currency: str
    """
    Trading currency
    """

    lot_size: int
    """
    Lot size
    """

    total_shares: int
    """
    Total shares
    """

    circulating_shares: int
    """
    Circulating shares
    """

    hk_shares: int
    """
    HK shares (only HK stocks)
    """

    eps: Decimal
    """
    Earnings per share
    """

    eps_ttm: Decimal
    """
    Earnings per share (TTM)
    """

    bps: Decimal
    """
    Net assets per share
    """

    dividend_yield: Decimal
    """
    Dividend yield
    """

    stock_derivatives: List[Type[DerivativeType]]
    """
    Types of supported derivatives
    """


class TradeStatus:
    """
    Security Status
    """

    class Normal(TradeStatus):
        """
        Normal
        """

    class Halted(TradeStatus):
        """
        Suspension
        """

    class Delisted(TradeStatus):
        """
        Delisted
        """

    class Fuse(TradeStatus):
        """
        Fuse
        """

    class PrepareList(TradeStatus):
        """
        Prepare List
        """

    class CodeMoved(TradeStatus):
        """
        Code Moved
        """

    class ToBeOpened(TradeStatus):
        """
        To Be Opened
        """

    class SplitStockHalts(TradeStatus):
        """
        Split Stock Halts
        """

    class Expired(TradeStatus):
        """
        Expired
        """

    class WarrantPrepareList(TradeStatus):
        """
        Warrant To BeListed
        """

    class SuspendTrade(TradeStatus):
        """
        Suspend
        """


class PrePostQuote:
    """
    Quote of US pre/post market
    """

    last_done: Decimal
    """
    Latest price
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    prev_close: Decimal
    """
    Close of the last trade session
    """


class SecurityQuote:
    """
    Quote of securitity
    """

    symbol: str
    """
    Security code
    """

    last_done: Decimal
    """
    Latest price
    """

    prev_close: Decimal
    """
    Yesterday's close
    """

    open: Decimal
    """
    Open
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    trade_status: Type[TradeStatus]
    """
    Security trading status
    """

    pre_market_quote: Optional[PrePostQuote]
    """
    Quote of US pre market
    """

    post_market_quote: Optional[PrePostQuote]
    """
    Quote of US post market
    """


class OptionType:
    """
    Option type
    """

    class Unknown(OptionType):
        """
        Unknown
        """

    class American(OptionType):
        """
        American
        """

    class Europe(OptionType):
        """
        Europe
        """


class OptionDirection:
    """
    Option direction
    """

    class Unknown(OptionDirection):
        """
        Unknown
        """

    class Put(OptionDirection):
        """
        Put
        """

    class Call(OptionDirection):
        """
        Call
        """


class OptionQuote:
    """
    Quote of option
    """

    symbol: str
    """
    Security code
    """

    last_done: Decimal
    """
    Latest price
    """

    prev_close: Decimal
    """
    Yesterday's close
    """

    open: Decimal
    """
    Open
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    trade_status: Type[TradeStatus]
    """
    Security trading status
    """

    implied_volatility: Decimal
    """
    Implied volatility
    """

    open_interest: int
    """
    Number of open positions
    """

    expiry_date: date
    """
    Exprity date
    """

    strike_price: Decimal
    """
    Strike price
    """

    contract_multiplier: Decimal
    """
    Contract multiplier
    """

    contract_type: Type[OptionType]
    """
    Option type
    """

    contract_size: Decimal
    """
    Contract size
    """

    direction: Type[OptionDirection]
    """
    Option direction
    """

    historical_volatility: Decimal
    """
    Underlying security historical volatility of the option
    """

    underlying_symbol: str
    """
    Underlying security symbol of the option
    """


class WarrantType:
    """
    Warrant type
    """

    class Unknown(WarrantType):
        """
        Unknown
        """

    class Call(WarrantType):
        """
        Call
        """

    class Put(WarrantType):
        """
        Put
        """

    class Bull(WarrantType):
        """
        Bull
        """

    class Bear(WarrantType):
        """
        Bear
        """

    class Inline(WarrantType):
        """
        Inline
        """


class WarrantQuote:
    """
    Quote of warrant
    """

    symbol: str
    """
    Security code
    """

    last_done: Decimal
    """
    Latest price
    """

    prev_close: Decimal
    """
    Yesterday's close
    """

    open: Decimal
    """
    Open
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    trade_status: Type[TradeStatus]
    """
    Security trading status
    """

    implied_volatility: Decimal
    """
    Implied volatility
    """

    expiry_date: date
    """
    Exprity date
    """

    last_trade_date: date
    """
    Last tradalbe date
    """

    outstanding_ratio: Decimal
    """
    Outstanding ratio
    """

    outstanding_qty: int
    """
    Outstanding quantity
    """

    conversion_ratio: Decimal
    """
    Conversion ratio
    """

    category: Type[WarrantType]
    """
    Warrant type
    """

    strike_price: Decimal
    """
    Strike price
    """

    upper_strike_price: Decimal
    """
    Upper bound price
    """

    lower_strike_price: Decimal
    """
    Lower bound price
    """

    call_price: Decimal
    """
    Call price
    """

    underlying_symbol: str
    """
    Underlying security symbol of the warrant
    """


class Depth:
    """
    Depth
    """

    position: int
    """
    Position
    """

    price: Decimal
    """
    Price
    """

    volume: int
    """
    Volume
    """

    order_num: int
    """
    Number of orders
    """


class SecuritiyDepth:
    """
    Security depth
    """

    asks: List[Depth]
    """
    Ask depth
    """

    asks: List[Depth]
    """
    Bid depth
    """


class Brokers:
    """
    Brokers
    """

    position: int
    """
    Position
    """

    broker_ids: List[int]
    """
    Broker IDs
    """


class SecurityBrokers:
    """
    Security brokers
    """

    ask_brokers: List[Brokers]
    """
    Ask brokers
    """

    bid_brokers: List[Brokers]
    """
    Bid brokers
    """


class ParticipantInfo:
    """
    Participant info
    """

    broker_ids: List[int]
    """
    Broker IDs
    """

    name_cn: str
    """
    Participant name (zh-CN)
    """

    name_en: str
    """
    Participant name (en)
    """

    name_hk: str
    """
    Participant name (zh-HK)
    """


class TradeDirection:
    """
    Trade direction
    """

    class Nature(TradeDirection):
        """
        Nature
        """

    class Down(TradeDirection):
        """
        Down
        """

    class Up(TradeDirection):
        """
        Up
        """


class TradeSession:
    """
    Trade session
    """

    class Normal(TradeSession):
        """
        Trading
        """

    class Pre(TradeSession):
        """
        Pre-Trading
        """

    class Post(TradeSession):
        """
        Post-Trading
        """


class Trade:
    """
    Trade
    """

    price: Decimal
    """
    Price
    """

    volume: int
    """
    Volume
    """

    timestamp: datetime
    """
    Time of trading
    """

    trade_type: str
    """
    Trade type
    """

    direction: Type[TradeDirection]
    """
    Trade direction
    """

    trade_session: Type[TradeSession]
    """
    Trade session
    """


class IntradayLine:
    """
    Intraday line
    """

    price: Decimal
    """
    Close price of the minute
    """

    timestamp: datetime
    """
    Start time of the minute
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    avg_price: Decimal
    """
    Average price
    """


class Candlestick:
    """
    Candlestick
    """

    close: Decimal
    """
    Close price
    """

    open: Decimal
    """
    Open price
    """

    low: Decimal
    """
    Low price
    """

    high: Decimal
    """
    High price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    timestamp: datetime
    """
    Timestamp
    """


class AdjustType:
    """
    Candlestick adjustment type
    """

    class NoAdjust(AdjustType):
        """
        Actual
        """

    class ForwardAdjust(AdjustType):
        """
        Adjust forward
        """


class Period:
    """
    Candlestick period
    """

    class Min_1(Period):
        """
        One Minute
        """

    class Min_5(Period):
        """
        Five Minutes
        """

    class Min_15(Period):
        """
        Fifteen Minutes
        """

    class Min_30(Period):
        """
        Thirty Minutes
        """

    class Min_60(Period):
        """
        Sixty Minutes
        """

    class Day(Period):
        """
        One Days
        """

    class Week(Period):
        """
        One Week
        """

    class Month(Period):
        """
        One Month
        """

    class Year(Period):
        """
        One Year
        """


class StrikePriceInfo:
    """
    Strike price info
    """

    price: Decimal
    """
    Strike price
    """

    call_symbol: str
    """
    Security code of call option
    """

    put_symbol: str
    """
    Security code of put option
    """

    standard: bool
    """
    Is standard
    """


class IssuerInfo:
    """
    Issuer info
    """

    issuer_id: int
    """
    Issuer ID
    """

    name_cn: str
    """
    Issuer name (zh-CN)
    """

    name_en: str
    """
    Issuer name (en)
    """

    name_hk: str
    """
    Issuer name (zh-HK)
    """


class TradingSessionInfo:
    """
    The information of trading session
    """

    begin_time: time
    """
    Being trading time
    """

    end_time: time
    """
    End trading time
    """

    trade_session: Type[TradeSession]
    """
    Trading session
    """


class MarketTradingSession:
    """
    Market trading session
    """

    market: Type[Market]
    """
    Market
    """

    trade_session: List[TradingSessionInfo]
    """
    Trading session
    """


class MarketTradingDays:
    trading_days: List[date]
    half_trading_days: List[date]


class RealtimeQuote:
    """
    Real-time quote
    """

    symbol: str
    """
    Security code
    """

    last_done: Decimal
    """
    Latest price
    """

    open: Decimal
    """
    Open
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    trade_status: Type[TradeStatus]
    """
    Security trading status
    """


class QuoteContext:
    """
    Quote context
    """

    def __init__(
        self,
        config: Config,
        handler: Optional[QuoteHandler] = None,
    ) -> None: ...

    def subscribe(self, symbols: List[str], sub_types: List[Type[SubType]], is_first_push=False) -> None:
        """
        Subscribe quote
        """

    def unsubscribe(self, symbols: List[str], sub_types: List[Type[SubType]]) -> None:
        """
        Unsubscribe quote
        """

    def static_info(self, symbols: List[str]) -> List[SecuritiyStaticInfo]:
        """
        Get basic information of securities
        """

    def quote(self, symbols: List[str]) -> List[SecurityQuote]:
        """
        Get quote of securities
        """

    def option_quote(self, symbols: List[str]) -> List[OptionQuote]:
        """
        Get quote of option securities
        """

    def warrant_quote(self, symbols: List[str]) -> List[WarrantQuote]:
        """
        Get quote of warrant securities
        """

    def depth(self, symbol: str) -> SecuritiyDepth:
        """
        Get security depth
        """

    def brokers(self, symbol: str) -> SecurityBrokers:
        """
        Get security brokers
        """

    def participants(self) -> List[ParticipantInfo]:
        """
        Get participants
        """

    def trades(self, symbol: str, count: int) -> List[Trade]:
        """
        Get security trades
        """

    def intraday(self, symbol: str) -> List[IntradayLine]:
        """
        Get security intraday
        """

    def candlesticks(self, symbol: str, period: Type[Period], count: int, adjust_type: Type[AdjustType]) -> List[Candlestick]:
        """
        Get security candlesticks
        """

    def option_chain_expiry_date_list(self, symbol: str) -> List[date]:
        """
        Get option chain expiry date list
        """

    def option_chain_info_by_date(self, symbol: str, expiry_date: date) -> List[StrikePriceInfo]:
        """
        Get option chain info by date
        """

    def warrant_issuers(self) -> List[IssuerInfo]:
        """
        Get warrant issuers
        """

    def trading_session(self) -> List[MarketTradingSession]:
        """
        Get trading session of the day
        """

    def trading_days(self) -> MarketTradingDays:
        """
        Get trading session of the day
        """

    def realtime_quote(self, symbols: List[str]) -> List[RealtimeQuote]:
        """
        Get real-time quote
        """

    def realtime_depth(self) -> SecuritiyDepth:
        """
        Get real-time depth
        """

    def realtime_brokers(self) -> SecurityBrokers:
        """
        Get real-time brokers
        """

    def realtime_trades(self, symbol: str, count: int) -> List[Trade]:
        """
        Get real-time trades
        """
