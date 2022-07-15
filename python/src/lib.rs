mod config;
mod decimal;
mod error;
mod quote;
mod time;
mod trade;
mod types;

use pyo3::prelude::*;

#[pymodule]
fn longbridge(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    let openapi = PyModule::new(py, "openapi")?;

    openapi.add_class::<config::Config>()?;
    openapi.add_class::<types::Language>()?;
    openapi.add_class::<types::Market>()?;
    quote::register_types(openapi)?;
    trade::register_types(openapi)?;

    m.add_submodule(openapi)?;
    Ok(())
}
