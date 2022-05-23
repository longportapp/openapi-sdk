from datetime import datetime, date, time
from decimal import Decimal
from typing import List, Optional, Type
from typing_extensions import Protocol

from longbridge import Config, Market


class PushQuote:
    """
    Push real-time quote
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
    Push real-time depth
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
    Push real-time brokers
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
    Push real-time trades
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
    class Unknown(WarrantType):
        ...

    class Call(WarrantType):
        ...

    class Put(WarrantType):
        ...

    class Bull(WarrantType):
        ...

    class Bear(WarrantType):
        ...

    class Inline(WarrantType):
        ...


class WarrantQuote:
    symbol: str
    last_done: Decimal
    prev_close: Decimal
    open: Decimal
    high: Decimal
    low: Decimal
    timestamp: datetime
    volume: int
    turnover: Decimal
    trade_status: Type[TradeStatus]
    implied_volatility: Decimal
    expiry_date: date
    last_trade_date: date
    outstanding_ratio: Decimal
    outstanding_qty: int
    conversion_ratio: Decimal
    category: Type[WarrantType]
    strike_price: Decimal
    upper_strike_price: Decimal
    lower_strike_price: Decimal
    call_price: Decimal
    underlying_symbol: str


class Depth:
    position: int
    price: Decimal
    volume: int
    order_num: int


class SecuritiyDepth:
    asks: List[Depth]
    asks: List[Depth]


class Brokers:
    position: int
    broker_ids: List[int]


class SecurityBrokers:
    ask_brokers: List[Brokers]
    bid_brokers: List[Brokers]


class ParticipantInfo:
    broker_ids: List[int]
    name_cn: str
    name_en: str
    name_hk: str


class TradeDirection:
    class Nature(TradeDirection):
        ...

    class Down(TradeDirection):
        ...

    class Up(TradeDirection):
        ...


class TradeSession:
    class Normal(TradeSession):
        ...

    class Pre(TradeSession):
        ...

    class Post(TradeSession):
        ...


class Trade:
    price: Decimal
    volume: int
    timestamp: datetime
    trade_type: str
    direction: Type[TradeDirection]
    trade_session: Type[TradeSession]


class IntradayLine:
    price: Decimal
    timestamp: datetime
    volume: int
    turnover: Decimal
    avg_price: Decimal


class Candlestick:
    close: Decimal
    open: Decimal
    low: Decimal
    high: Decimal
    volume: int
    turnover: Decimal
    timestamp: datetime


class AdjustType:
    class NoAdjust(AdjustType):
        ...

    class ForwardAdjust(AdjustType):
        ...


class Period:
    class Min_1(Period):
        ...

    class Min_5(Period):
        ...

    class Min_15(Period):
        ...

    class Min_30(Period):
        ...

    class Min_60(Period):
        ...

    class Day(Period):
        ...

    class Week(Period):
        ...

    class Month(Period):
        ...

    class Year(Period):
        ...


class StrikePriceInfo:
    price: Decimal
    call_symbol: str
    put_symbol: str
    standard: bool


class IssuerInfo:
    issuer_id: int
    name_cn: str
    name_en: str
    name_hk: str


class TradingSessionInfo:
    begin_time: time
    end_time: time
    trade_session: Type[TradeSession]


class MarketTradingSession:
    market: Type[Market]
    trade_session: List[TradingSessionInfo]


class MarketTradingDays:
    trading_days: List[date]
    half_trading_days: List[date]


class RealtimeQuote:
    symbol: str
    last_done: Decimal
    open: Decimal
    high: Decimal
    low: Decimal
    timestamp: datetime
    volume: int
    turnover: Decimal
    trade_status: Type[TradeStatus]


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
