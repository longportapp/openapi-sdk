from datetime import datetime, date
from decimal import Decimal
from typing import Any, List, Optional
from typing_extensions import Protocol
from longbridge_sdk import Config, Market


class OrderSide:
    class Unknown(OrderSide):
        ...

    class Buy(OrderSide):
        ...

    class Sell(OrderSide):
        ...


class OrderType:
    class Unknown(OrderType):
        ...

    class Limit(OrderType):
        ...

    class EnhancedLimit(OrderType):
        ...

    class Market(OrderType):
        ...

    class AtAuction(OrderType):
        ...

    class AtAuctionLimit(OrderType):
        ...

    class OddLots(OrderType):
        ...

    class LimitIfTouched(OrderType):
        ...

    class MarketIfTouched(OrderType):
        ...

    class TrailingLimitIfTouchedAmount(OrderType):
        ...

    class TrailingLimitIfTouchedPercent(OrderType):
        ...

    class TrailingMarketIfTouchedAmount(OrderType):
        ...

    class TrailingMarketIfTouchedPercent(OrderType):
        ...


class OrderStatus:
    class Unknown(OrderStatus):
        ...

    class NotReported(OrderStatus):
        ...

    class ReplacedNotReported(OrderStatus):
        ...

    class ProtectedNotReported(OrderStatus):
        ...

    class VarietiesNotReported(OrderStatus):
        ...

    class Filled(OrderStatus):
        ...

    class WaitToNew(OrderStatus):
        ...

    class New(OrderStatus):
        ...

    class WaitToReplace(OrderStatus):
        ...

    class PendingReplace(OrderStatus):
        ...

    class Replaced(OrderStatus):
        ...

    class PartialFilled(OrderStatus):
        ...

    class WaitToCancel(OrderStatus):
        ...

    class PendingCancel(OrderStatus):
        ...

    class RejectedStatus(OrderStatus):
        ...

    class CanceledStatus(OrderStatus):
        ...

    class ExpiredStatus(OrderStatus):
        ...

    class PartialWithdrawal(OrderStatus):
        ...


class OrderTag:
    class Unknown(OrderTag):
        ...

    class Buy(OrderTag):
        ...

    class Sell(OrderTag):
        ...


class TriggerStatus:
    class Unknown(TriggerStatus):
        ...

    class NotActive(TriggerStatus):
        ...

    class Deactive(TriggerStatus):
        ...

    class Active(TriggerStatus):
        ...

    class Released(TriggerStatus):
        ...


class Trade:
    order_id: str
    trade_id: str
    symbol: str
    trade_done_at: datetime
    quantity: Decimal
    price: Decimal


class PushOrderChanged:
    side: OrderSide
    stock_name: str
    quantity: str
    symbol: str
    order_type: OrderType
    price: Decimal
    executed_quantity: int
    executed_price: Decimal
    order_id: str
    currency: str
    status: OrderStatus
    submitted_at: datetime
    updated_at: datetime
    trigger_price: Optional[Decimal]
    msg: Optional[str]
    tag: OrderTag
    trigger_status: Optional[TriggerStatus]
    trigger_at: Optional[datetime]
    trailing_amount: Optional[Decimal]
    trailing_percent: Optional[Decimal]
    limit_offset: Optional[Decimal]
    account_no: str


class TimeInForceType:
    class Unknown(TimeInForceType):
        ...

    class Day(TimeInForceType):
        ...

    class GoodTilCanceled(TimeInForceType):
        ...

    class GoodTilDate(TimeInForceType):
        ...


class OutsideRTH:
    class Unknown(OutsideRTH):
        ...

    class RTHOnly(OutsideRTH):
        ...

    class AnyTime(OutsideRTH):
        ...


class Order:
    order_id: str
    status: OrderStatus
    stock_name: str
    quantity: Decimal
    executed_qty: Decimal
    price: Optional[Decimal]
    executed_price: Optional[Decimal]
    submitted_at: Optional[datetime]
    side: OrderSide
    symbol: str
    order_type: OrderType
    last_done: Optional[Decimal]
    trigger_price: Optional[Decimal]
    msg: Optional[str]
    tag: OrderTag
    time_in_force: TimeInForceType
    expire_date: Optional[date]
    updated_at: Optional[datetime]
    trigger_at: Optional[datetime]
    trailing_amount: Optional[Decimal]
    trailing_percent: Optional[Decimal]
    limit_offset: Optional[Decimal]
    trigger_status: Optional[TriggerStatus]
    currency: str
    outside_rth: Optional[OutsideRTH]


class TradeHandler(Protocol):
    """
    Trade push message handler
    """

    def on_push(self, symbol: str, msg: PushOrderChanged) -> None:
        """
        Push message callback
        """


class SubmitOrderResponse:
    order_id: str


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
