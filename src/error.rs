use thiserror::Error;

#[derive(Debug, Error)]
pub enum ComputeError {
    #[error("invalid graph: {0}")]
    InvalidGraph(String),
    #[error("unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),
    #[error("execution error: {0}")]
    Execution(String),
}
