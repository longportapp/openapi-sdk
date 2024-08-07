use std::collections::HashMap;

use longport_candlesticks::InputCandlestick;
use longport_proto::quote::Period;

use crate::quote::{
    push_types::{PushEventDetail, PushQuote},
    Brokers, Candlestick, Depth, PushBrokers, PushDepth, PushEvent, PushTrades, SecurityBoard,
    Trade,
};

const MAX_TRADES: usize = 500;

macro_rules! merge_decimal {
    ($prev:expr, $new:expr, $field:ident) => {
        if !$new.$field.is_zero() {
            $new.$field
        } else {
            $prev.$field
        }
    };
}

macro_rules! merge_i64 {
    ($prev:expr, $new:expr, $field:ident) => {
        if $new.$field != 0 {
            $new.$field
        } else {
            $prev.$field
        }
    };
}

#[derive(Debug)]
pub(crate) struct Candlesticks {
    pub(crate) candlesticks: Vec<Candlestick>,
    pub(crate) confirmed: bool,
}

impl Candlesticks {
    #[inline]
    pub(crate) fn merge_input(&self) -> InputCandlestick {
        match (self.candlesticks.last(), self.confirmed) {
            (None, true) => unreachable!(),
            (None, false) => InputCandlestick::None,
            (Some(prev), true) => InputCandlestick::Confirmed((*prev).into()),
            (Some(prev), false) => InputCandlestick::Normal((*prev).into()),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct SecuritiesData {
    pub(crate) quote: PushQuote,

    pub(crate) asks: Vec<Depth>,
    pub(crate) bids: Vec<Depth>,

    pub(crate) ask_brokers: Vec<Brokers>,
    pub(crate) bid_brokers: Vec<Brokers>,

    pub(crate) trades: Vec<Trade>,

    pub(crate) board: SecurityBoard,
    pub(crate) candlesticks: HashMap<Period, Candlesticks>,
}

#[derive(Debug, Default)]
pub(crate) struct Store {
    pub(crate) securities: HashMap<String, SecuritiesData>,
}

impl Store {
    pub(crate) fn handle_push(&mut self, event: &mut PushEvent) {
        let data = self.securities.entry(event.symbol.clone()).or_default();

        match &mut event.detail {
            PushEventDetail::Quote(quote) => merge_quote(data, quote),
            PushEventDetail::Depth(depth) => merge_depth(data, depth),
            PushEventDetail::Brokers(brokers) => merge_brokers(data, brokers),
            PushEventDetail::Trade(trade) => merge_trades(data, trade),
            PushEventDetail::Candlestick(_) => unreachable!(),
        }
    }
}

fn merge_quote(data: &mut SecuritiesData, quote: &mut PushQuote) {
    let prev_quote = &data.quote;
    let new_quote = PushQuote {
        last_done: merge_decimal!(prev_quote, quote, last_done),
        open: merge_decimal!(prev_quote, quote, open),
        high: merge_decimal!(prev_quote, quote, high),
        low: merge_decimal!(prev_quote, quote, low),
        timestamp: quote.timestamp,
        volume: merge_i64!(prev_quote, quote, volume),
        turnover: merge_decimal!(prev_quote, quote, turnover),
        trade_status: quote.trade_status,
        trade_session: quote.trade_session,
    };
    data.quote = new_quote.clone();
    *quote = new_quote;
}

fn merge_depth(data: &mut SecuritiesData, depth: &PushDepth) {
    replace(&mut data.asks, depth.asks.clone(), |v| v.position);
    replace(&mut data.bids, depth.bids.clone(), |v| v.position);
}

fn merge_brokers(data: &mut SecuritiesData, brokers: &PushBrokers) {
    replace(&mut data.ask_brokers, brokers.ask_brokers.clone(), |v| {
        v.position
    });
    replace(&mut data.bid_brokers, brokers.bid_brokers.clone(), |v| {
        v.position
    });
}

fn merge_trades(data: &mut SecuritiesData, trades: &PushTrades) {
    data.trades.extend(trades.trades.clone());
    if data.trades.len() > MAX_TRADES * 2 {
        data.trades.drain(..MAX_TRADES);
    }
}

fn replace<T, B, F>(elements: &mut Vec<T>, others: Vec<T>, f: F)
where
    F: Fn(&T) -> B,
    B: Ord,
{
    for v in others {
        match elements.binary_search_by_key(&f(&v), &f) {
            Ok(index) => elements[index] = v,
            Err(index) => elements.insert(index, v),
        }
    }
}
