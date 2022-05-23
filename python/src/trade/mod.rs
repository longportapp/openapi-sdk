mod context;
mod push;
mod types;

use pyo3::prelude::*;

pub(crate) fn register_module(py: Python<'_>, parent: &PyModule) -> PyResult<()> {
    let trade = PyModule::new(py, "trade")?;

    trade.add_class::<types::Trade>()?;
    trade.add_class::<types::OrderStatus>()?;
    trade.add_class::<types::OrderSide>()?;
    trade.add_class::<types::OrderType>()?;
    trade.add_class::<types::OrderTag>()?;
    trade.add_class::<types::TimeInForceType>()?;
    trade.add_class::<types::TriggerStatus>()?;
    trade.add_class::<types::OutsideRTH>()?;
    trade.add_class::<types::Order>()?;
    trade.add_class::<types::PushOrderChanged>()?;

    trade.add_class::<context::TradeContext>()?;

    parent.add_submodule(trade)?;
    Ok(())
}
