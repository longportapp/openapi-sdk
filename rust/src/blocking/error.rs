/// Blocking error
#[derive(Debug, thiserror::Error)]
pub enum BlockingError {
    /// Context closed
    #[error("context closed")]
    Closed,
}
