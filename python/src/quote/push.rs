use longbridge::quote::{
    PushBrokers, PushDepth, PushEvent, PushEventDetail, PushQuote, PushTrades,
};
use pyo3::prelude::*;

pub(crate) fn handle_push_event(handler: &PyObject, event: PushEvent) {
    match event.detail {
        PushEventDetail::Quote(quote) => handle_quote(handler, event.symbol, quote),
        PushEventDetail::Depth(depth) => handle_depth(handler, event.symbol, depth),
        PushEventDetail::Brokers(brokers) => handle_brokers(handler, event.symbol, brokers),
        PushEventDetail::Trade(trades) => handle_trades(handler, event.symbol, trades),
    }
}

fn handle_quote(handler: &PyObject, symbol: String, quote: PushQuote) {
    let _ = Python::with_gil(|py| {
        handler.call_method1(
            py,
            "on_event",
            (symbol, crate::quote::types::PushQuote::try_from(quote)?),
        )?;
        Ok::<_, PyErr>(())
    });
}

fn handle_depth(handler: &PyObject, symbol: String, depth: PushDepth) {
    let _ = Python::with_gil(|py| {
        handler.call_method1(
            py,
            "on_event",
            (symbol, crate::quote::types::PushDepth::try_from(depth)?),
        )?;
        Ok::<_, PyErr>(())
    });
}

fn handle_brokers(handler: &PyObject, symbol: String, brokers: PushBrokers) {
    let _ = Python::with_gil(|py| {
        handler.call_method1(
            py,
            "on_event",
            (symbol, crate::quote::types::PushBrokers::try_from(brokers)?),
        )?;
        Ok::<_, PyErr>(())
    });
}

fn handle_trades(handler: &PyObject, symbol: String, trades: PushTrades) {
    let _ = Python::with_gil(|py| {
        handler.call_method1(
            py,
            "on_event",
            (symbol, crate::quote::types::PushTrades::try_from(trades)?),
        )?;
        Ok::<_, PyErr>(())
    });
}
