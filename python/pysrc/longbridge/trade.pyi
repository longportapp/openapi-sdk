from datetime import datetime, date
from decimal import Decimal
from typing import Any, List, Optional
from typing_extensions import Protocol
from longbridge import Config, Market


class OrderSide:
    """
    Order side
    """

    class Unknown(OrderSide):
        """
        Unknown
        """

    class Buy(OrderSide):
        """
        Buy
        """

    class Sell(OrderSide):
        """
        Sell
        """


class OrderType:
    """
    Order type
    """

    class Unknown(OrderType):
        """
        Unknown
        """

    class Limit(OrderType):
        """
        Limit Order
        """

    class EnhancedLimit(OrderType):
        """
        Enhanced Limit Order
        """

    class Market(OrderType):
        """
        Market Order
        """

    class AtAuction(OrderType):
        """
        At-auction Order
        """

    class AtAuctionLimit(OrderType):
        """
        At-auction Limit Order
        """

    class OddLots(OrderType):
        """
        Odd Lots
        """

    class LimitIfTouched(OrderType):
        """
        Limit If Touched
        """

    class MarketIfTouched(OrderType):
        """
        Market If Touched
        """

    class TrailingLimitIfTouchedAmount(OrderType):
        """
        Trailing Limit If Touched (Trailing Amount)
        """

    class TrailingLimitIfTouchedPercent(OrderType):
        """
        Trailing Limit If Touched (Trailing Percent)
        """

    class TrailingMarketIfTouchedAmount(OrderType):
        """
        Trailing Market If Touched (Trailing Amount)
        """

    class TrailingMarketIfTouchedPercent(OrderType):
        """
        Trailing Market If Touched (Trailing Percent)
        """


class OrderStatus:
    """
    Order status
    """

    class Unknown(OrderStatus):
        """
        Unknown
        """

    class NotReported(OrderStatus):
        """
        Not reported
        """

    class ReplacedNotReported(OrderStatus):
        """
        Not reported (Replaced Order)
        """

    class ProtectedNotReported(OrderStatus):
        """
        Not reported (Protected Order)
        """

    class VarietiesNotReported(OrderStatus):
        """
        Not reported (Conditional Order)
        """

    class Filled(OrderStatus):
        """
        Filled
        """

    class WaitToNew(OrderStatus):
        """
        Wait To New
        """

    class New(OrderStatus):
        """
        New
        """

    class WaitToReplace(OrderStatus):
        """
        Wait To Replace
        """

    class PendingReplace(OrderStatus):
        """
        Pending Replace
        """

    class Replaced(OrderStatus):
        """
        Replaced
        """

    class PartialFilled(OrderStatus):
        """
        Partial Filled
        """

    class WaitToCancel(OrderStatus):
        """
        Wait To Cancel
        """

    class PendingCancel(OrderStatus):
        """
        Pending Cancel
        """

    class Rejected(OrderStatus):
        """
        Rejected
        """

    class Canceled(OrderStatus):
        """
        Canceled
        """

    class Expired(OrderStatus):
        """
        ExpiredStatus
        """

    class PartialWithdrawal(OrderStatus):
        """
        PartialWithdrawal
        """


class OrderTag:
    """
    Order tag
    """

    class Unknown(OrderTag):
        """
        Unknown
        """

    class Buy(OrderTag):
        """
        Buy
        """

    class Sell(OrderTag):
        """
        Sell
        """


class TriggerStatus:
    """
    Trigger status
    """

    class Unknown(TriggerStatus):
        """
        Unknown
        """

    class NotActive(TriggerStatus):
        """
        Not active
        """

    class Deactive(TriggerStatus):
        """
        Deactive
        """

    class Active(TriggerStatus):
        """
        Active
        """

    class Released(TriggerStatus):
        """
        Released
        """


class Trade:
    """
    Trade
    """

    order_id: str
    """
    Order ID
    """

    trade_id: str
    """
    Execution ID
    """

    symbol: str
    """
    Security code
    """

    trade_done_at: datetime
    """
    Trade done time
    """

    quantity: Decimal
    """
    Executed quantity
    """

    price: Decimal
    """
    Executed price
    """


class PushOrderChanged:
    """
    Order changed message
    """

    side: OrderSide
    """
    Order side
    """

    stock_name: str
    """
    Stock name
    """

    quantity: str
    """
    Submitted quantity
    """

    symbol: str
    """
    Order symbol
    """

    order_type: OrderType
    """
    Order type
    """

    price: Decimal
    """
    Submitted price
    """

    executed_quantity: int
    """
    Executed quantity
    """

    executed_price: Decimal
    """
    Executed price
    """

    order_id: str
    """
    Order ID
    """

    currency: str
    """
    Currency
    """

    status: OrderStatus
    """
    Order status
    """

    submitted_at: datetime
    """
    Submitted time
    """

    updated_at: datetime
    """
    Last updated time
    """

    trigger_price: Optional[Decimal]
    """
    Order trigger price
    """

    msg: Optional[str]
    """
    Rejected message or remark
    """

    tag: OrderTag
    """
    Order tag
    """

    trigger_status: Optional[TriggerStatus]
    """
    Conditional order trigger status
    """

    trigger_at: Optional[datetime]
    """
    Conditional order trigger time
    """

    trailing_amount: Optional[Decimal]
    """
    Trailing amount
    """

    trailing_percent: Optional[Decimal]
    """
    Trailing percent
    """

    limit_offset: Optional[Decimal]
    """
    Limit offset amount
    """

    account_no: str
    """
    Account no
    """


class TimeInForceType:
    """
    Time in force type
    """

    class Unknown(TimeInForceType):
        """
        Unknown
        """

    class Day(TimeInForceType):
        """
        Day Order
        """

    class GoodTilCanceled(TimeInForceType):
        """
        Good Til Canceled Order
        """

    class GoodTilDate(TimeInForceType):
        """
        Good Til Date Order
        """


class OutsideRTH:
    """
    Enable or disable outside regular trading hours
    """

    class Unknown(OutsideRTH):
        """
        Unknown
        """

    class RTHOnly(OutsideRTH):
        """
        Regular trading hour only
        """

    class AnyTime(OutsideRTH):
        """
        Any time
        """


class Order:
    """
    Order
    """

    order_id: str
    """
    Order ID
    """

    status: OrderStatus
    """
    Order status
    """

    stock_name: str
    """
    Stock name
    """

    quantity: Decimal
    """
    Submitted quantity
    """

    executed_qty: Decimal
    """
    Executed quantity
    """

    price: Optional[Decimal]
    """
    Submitted price
    """

    executed_price: Optional[Decimal]
    """
    Executed price
    """

    submitted_at: Optional[datetime]
    """
    Submitted time
    """

    side: OrderSide
    """
    Order side
    """

    symbol: str
    """
    Security code
    """

    order_type: OrderType
    """
    Order type
    """

    last_done: Optional[Decimal]
    """
    Last done
    """

    trigger_price: Optional[Decimal]
    """
    `LIT` / `MIT` Order Trigger Price
    """

    msg: Optional[str]
    """
    Rejected Message or remark
    """

    tag: OrderTag
    """
    Order tag
    """

    time_in_force: TimeInForceType
    """
    Time in force type
    """

    expire_date: Optional[date]
    """
    Long term order expire date
    """

    updated_at: Optional[datetime]
    """
    Last updated time
    """

    trigger_at: Optional[datetime]
    """
    Conditional order trigger time
    """

    trailing_amount: Optional[Decimal]
    """
    `TSMAMT` / `TSLPAMT` order trailing amount
    """

    trailing_percent: Optional[Decimal]
    """
    `TSMPCT` / `TSLPPCT` order trailing percent
    """

    limit_offset: Optional[Decimal]
    """
    `TSLPAMT` / `TSLPPCT` order limit offset amount
    """

    trigger_status: Optional[TriggerStatus]
    """
    Conditional order trigger status
    """

    currency: str
    """
    Currency
    """

    outside_rth: Optional[OutsideRTH]
    """
    Enable or disable outside regular trading hours
    """


class TradeHandler(Protocol):
    """
    Trade push message handler
    """

    def on_event(self, symbol: str, msg: PushOrderChanged) -> None:
        """
        Called when a new message is received
        """


class SubmitOrderResponse:
    """
    Response for withdraw order request
    """

    order_id: str
    """
    Order id
    """


class CashInfo:
    """
    CashInfo
    """

    withdraw_cash: Decimal
    """
    Withdraw cash
    """
    available_cash: Decimal
    """
    Available cash
    """
    frozen_cash: Decimal
    """
    Frozen cash
    """
    settling_cash: Decimal
    """
    Cash to be settled
    """
    currency: str
    """
    Currency
    """


class AccountBalance:
    """
    Account balance
    """

    total_cash: Decimal
    """
    Total cash
    """
    max_finance_amount: Decimal
    """
    Maximum financing amount
    """
    remaining_finance_amount: Decimal
    """
    Remaining financing amount
    """
    risk_level: int
    """
    Risk control level
    """
    margin_call: Decimal
    """
    Margin call
    """
    currency: str
    """
    Currency
    """
    cash_infos: List[CashInfo]
    """
    Cash details
    """


class BalanceType:
    class Unknown(BalanceType):
        ...

    class Cash(BalanceType):
        ...

    class Stock(BalanceType):
        ...

    class Fund(BalanceType):
        ...


class CashFlowDirection:
    """
    Cash flow direction
    """

    class Unknown(CashFlowDirection):
        """
        Unknown
        """
        ...

    class Out(CashFlowDirection):
        """
        Out
        """
        ...

    class In(CashFlowDirection):
        """
        Stock
        """
        ...


class CashFlow:
    """
    Cash flow
    """

    transaction_flow_name: str
    """
    Cash flow name
    """

    direction: CashFlowDirection
    """
    Outflow direction
    """

    business_type: BalanceType
    """
    Balance type
    """

    balance: Decimal
    """
    Cash amount
    """

    currency: str
    """
    Cash currency
    """

    business_time: datetime
    """
    Business time
    """

    symbol: Optional[str]
    """
    Associated Stock code information
    """

    description: Optional[str]
    """
    Cash flow description
    """


class FundPosition:
    """
    Fund position
    """

    symbol: str
    """
    Fund ISIN code
    """

    current_net_asset_value: Decimal
    """
    Current equity
    """

    net_asset_value_day: Decimal
    """
    Current equity PyDecimal
    """

    symbol_name: str
    """
    Fund name
    """

    currency: str
    """
    Currency
    """

    cost_net_asset_value: Decimal
    """
    Net cost
    """


class FundPositionsResponse:
    """
    Fund positions response
    """

    account_channel: str
    """
    Account type
    """

    positions: List[FundPosition]
    """
    Fund positions
    """


class StockPosition:
    """
    Stock position
    """

    symbol: str
    """
    Stock code
    """

    symbol_name: str
    """
    Stock name
    """

    quality: Decimal
    """
    The number of holdings
    """

    available_quality: Optional[Decimal]
    """
    Available quantity
    """

    currency: str
    """
    Currency
    """

    cost_price: Decimal
    """
    Cost Price(According to the client's choice of average purchase or diluted cost)
    """


class StockPositionsResponse:
    """
    Stock positions response
    """

    account_channel: str
    """
    Account type
    """

    positions: List[StockPosition]
    """
    Stock positions
    """


class TradeContext:
    """
    Trade context
    """

    def __init__(
        self,
        config: Config,
        handler: Optional[TradeHandler] = None,
    ) -> None: ...

    def subscribe(self, topics: List[str]) -> None:
        """
        Subscribe topics
        """

    def unsubscribe(self, topics: List[str]) -> None:
        """
        Unsubscribe topics
        """

    def history_executions(self, symbol: Optional[str] = None, start_at: Optional[date] = None, end_at: Optional[date] = None) -> List[Trade]:
        """
        Get history executions
        """

    def today_executions(self, symbol: Optional[str] = None, order_id: Optional[str] = None) -> List[Trade]:
        """
        Get today executions
        """

    def history_orders(self, symbol: Optional[str] = None, status: List[OrderStatus] = [], side: Optional[OrderSide] = None, market: Optional[Market] = None, start_at: Optional[date] = None, end_at: Optional[date] = None) -> List[Order]:
        """
        Get history orders
        """

    def today_orders(self, symbol: Optional[str] = None, status: List[OrderStatus] = [], side: Optional[OrderSide] = None, market: Optional[Market] = None) -> List[Order]:
        """
        Get today orders
        """

    def replace_order(self, order_id: str, quantity: Decimal, price: Optional[Decimal] = None, trigger_price: Optional[Decimal] = None, limit_offset: Optional[Decimal] = None, trailing_amount: Optional[Decimal] = None, trailing_percent: Optional[Decimal] = None, remark: Optional[str] = None) -> None:
        """
        Replace order
        """

    def submit_order(self, symbol: str, order_type: OrderType, side: OrderSide, submitted_quantity: Decimal, time_in_force: TimeInForceType, submitted_price: Optional[Decimal] = None,  trigger_price: Optional[Decimal] = None, limit_offset: Optional[Decimal] = None, trailing_amount: Optional[Decimal] = None, trailing_percent: Optional[Decimal] = None, expire_date: Optional[date] = None,  outside_rth: Optional[OutsideRTH] = None,  remark: Optional[str] = None) -> SubmitOrderResponse:
        """
        Submit order
        """

    def withdraw_order(self, order_id: str) -> None:
        """
        Withdraw order
        """

    def account_balance(self) -> List[AccountBalance]:
        """
        Get account balance
        """

    def cash_flow(self, start_at: datetime, end_at: datetime, business_type: Optional[BalanceType] = None, symbol: Optional[str] = None, page: Optional[int] = None, size: Optional[int] = None) -> List[CashFlow]:
        """
        Get cash flow
        """

    def fund_positions(self, symbols: List[str] = []) -> FundPositionsResponse:
        """
        Get fund positions
        """

    def stock_positions(self, symbols: List[str] = []) -> StockPositionsResponse:
        """
        Get fund positions
        """
