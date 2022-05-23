mod config;
mod decimal;
mod quote;
mod time;
mod trade;
mod types;

use pyo3::prelude::*;

#[pymodule]
fn longbridge(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<config::Config>()?;
    m.add_class::<types::Market>()?;

    quote::register_module(py, m)?;
    trade::register_module(py, m)?;
    Ok(())
}
