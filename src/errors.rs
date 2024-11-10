pub type Result<T> = std::result::Result<T, PerplexityError>;

#[derive(Debug, thiserror::Error)]
pub enum PerplexityError {
    #[error("API key not set")]
    ApiKeyNotSet,
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("Invalid response format")]
    InvalidResponseFormat,
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Server error: {0}")]
    ServerError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
