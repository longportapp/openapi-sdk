use crate::WsClientError;

/// The event type of Websocket client
#[derive(Debug)]
pub enum WsEvent {
    /// When a error occurred
    Error(WsClientError),
    /// When the server push a new message
    Push {
        /// Command code
        command_code: u8,
        /// Message body
        body: Vec<u8>,
    },
}
