use std::sync::Arc;

use reqwest::{Client, Method};
use serde::Deserialize;

use crate::{HttpClientConfig, HttpClientResult, RequestBuilder};

/// Longbridge HTTP client
#[derive(Clone)]
pub struct HttpClient {
    pub(crate) http_cli: Client,
    pub(crate) config: Arc<HttpClientConfig>,
}

impl HttpClient {
    /// Create a new `HttpClient`
    pub fn new(config: HttpClientConfig) -> Self {
        Self {
            http_cli: Client::new(),
            config: Arc::new(config),
        }
    }

    /// Create a new request builder
    #[inline]
    pub fn request(&self, method: Method, path: impl Into<String>) -> RequestBuilder<(), (), ()> {
        RequestBuilder::new(self.clone(), method, path)
    }

    /// Get the socket OTP(One Time Password)
    ///
    /// Reference: <https://open.longbridgeapp.com/en/docs/socket-token-api>
    pub async fn get_otp(&self) -> HttpClientResult<String> {
        #[derive(Debug, Deserialize)]
        struct Response {
            otp: String,
        }

        Ok(self
            .request(Method::GET, "/v1/socket/token")
            .response::<Response>()
            .send()
            .await?
            .otp)
    }
}
