pub(crate) use http::{header, HeaderValue, Request};
use longbridge_httpcli::{HttpClient, HttpClientConfig};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

use crate::error::Result;

const QUOTE_WS_URL: &str = "wss://openapi-quote.longbridgeapp.com";
const TRADE_WS_URL: &str = "wss://openapi-trade.longbridgeapp.com";

/// Language identifier
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Language {
    /// zh-CN
    ZH_CN,
    /// zh-HK
    ZH_HK,
    /// en
    EN,
}

impl Language {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Language::ZH_CN => "zh-CN",
            Language::ZH_HK => "zh-HK",
            Language::EN => "en",
        }
    }
}

/// Configuration options for Longbridge sdk
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) http_cli_config: HttpClientConfig,
    pub(crate) quote_ws_url: String,
    pub(crate) trade_ws_url: String,
    language: Language,
}

impl Config {
    /// Create a new `Config`
    pub fn new(
        app_key: impl Into<String>,
        app_secret: impl Into<String>,
        access_token: impl Into<String>,
    ) -> Self {
        Self {
            http_cli_config: HttpClientConfig::new(app_key, app_secret, access_token),
            quote_ws_url: QUOTE_WS_URL.to_string(),
            trade_ws_url: TRADE_WS_URL.to_string(),
            language: Language::EN,
        }
    }

    /// Create a new `Config` from the given environment variables
    ///
    /// It first gets the environment variables from the `.env` file in the
    /// current directory.
    ///
    /// # Variables
    ///
    /// - `LONGBRIDGE_APP_KEY` - App key
    /// - `LONGBRIDGE_APP_SECRET` - App secret
    /// - `LONGBRIDGE_ACCESS_TOKEN` - Access token
    /// - `LONGBRIDGE_HTTP_URL` - HTTP endpoint url (Default: `https://openapi.longbridgeapp.com`)
    /// - `LONGBRIDGE_QUOTE_WS_URL` - Quote websocket endpoint url (Default:
    ///   `wss://openapi-quote.longbridgeapp.com`)
    /// - `LONGBRIDGE_TRADE_WS_URL` - Trade websocket endpoint url (Default:
    ///   `wss://openapi-trade.longbridgeapp.com`)
    pub fn from_env() -> Result<Self> {
        let _ = dotenv::dotenv();

        let http_cli_config = HttpClientConfig::from_env()?;
        let mut config = Config {
            http_cli_config,
            quote_ws_url: QUOTE_WS_URL.to_string(),
            trade_ws_url: TRADE_WS_URL.to_string(),
            language: Language::EN,
        };

        if let Ok(http_url) = std::env::var("LONGBRIDGE_HTTP_URL") {
            config.http_cli_config = config.http_cli_config.http_url(http_url);
        }

        if let Ok(quote_ws_url) = std::env::var("LONGBRIDGE_QUOTE_WS_URL") {
            config.quote_ws_url = quote_ws_url;
        }

        if let Ok(trade_ws_url) = std::env::var("LONGBRIDGE_TRADE_WS_URL") {
            config.trade_ws_url = trade_ws_url;
        }

        Ok(config)
    }

    /// Specifies the url of the OpenAPI server.
    ///
    /// Default: `https://openapi.longbridgeapp.com`
    ///
    /// NOTE: Usually you don't need to change it.
    #[must_use]
    pub fn http_url(mut self, url: impl Into<String>) -> Self {
        self.http_cli_config = self.http_cli_config.http_url(url);
        self
    }

    /// Specifies the url of the OpenAPI quote websocket server.
    ///
    /// Default: `wss://openapi-quote.longbridgeapp.com`
    ///
    /// NOTE: Usually you don't need to change it.
    #[must_use]
    pub fn quote_ws_url(self, url: impl Into<String>) -> Self {
        Self {
            quote_ws_url: url.into(),
            ..self
        }
    }

    /// Specifies the url of the OpenAPI trade websocket server.
    ///
    /// Default: `wss://openapi-trade.longbridgeapp.com`
    ///
    /// NOTE: Usually you don't need to change it.
    #[must_use]
    pub fn trade_ws_url(self, url: impl Into<String>) -> Self {
        Self {
            trade_ws_url: url.into(),
            ..self
        }
    }

    /// Specifies the language
    ///
    /// Default: `Language::EN`
    pub fn language(self, language: Language) -> Self {
        Self { language, ..self }
    }

    #[inline]
    pub(crate) fn create_http_client(&self) -> HttpClient {
        HttpClient::new(self.http_cli_config.clone())
            .header(header::ACCEPT_LANGUAGE, self.language.as_str())
    }

    fn create_ws_request(&self, url: &str) -> tokio_tungstenite::tungstenite::Result<Request<()>> {
        let mut request = url.into_client_request()?;
        request.headers_mut().append(
            header::ACCEPT_LANGUAGE,
            HeaderValue::from_str(self.language.as_str()).unwrap(),
        );
        Ok(request)
    }

    #[inline]
    pub(crate) fn create_quote_ws_request(
        &self,
    ) -> tokio_tungstenite::tungstenite::Result<Request<()>> {
        self.create_ws_request(&self.quote_ws_url)
    }

    #[inline]
    pub(crate) fn create_trade_ws_request(
        &self,
    ) -> tokio_tungstenite::tungstenite::Result<Request<()>> {
        self.create_ws_request(&self.trade_ws_url)
    }
}
