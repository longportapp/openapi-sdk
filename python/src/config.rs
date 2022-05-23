use pyo3::{prelude::*, types::PyType};

#[pyclass(name = "Config")]
pub(crate) struct Config(pub(crate) longbridge::Config);

#[pymethods]
impl Config {
    #[new]
    #[args(
        http_url = "\"https://openapi.longbridge.global\"",
        quote_ws_url = "\"https://openapi-quote.longbridge.global\"",
        trade_ws_url = "\"https://openapi-trade.longbridge.global\""
    )]
    fn py_new(
        app_key: String,
        app_secret: String,
        access_token: String,
        http_url: &str,
        quote_ws_url: &str,
        trade_ws_url: &str,
    ) -> Self {
        Self(
            longbridge::Config::new(app_key, app_secret, access_token)
                .http_url(http_url)
                .quote_ws_url(quote_ws_url)
                .trade_ws_url(trade_ws_url),
        )
    }

    #[classmethod]
    fn from_env(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(longbridge::Config::from_env()?))
    }
}
