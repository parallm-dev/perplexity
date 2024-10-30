use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, PerplexityError>;

#[derive(Debug)]
pub enum PerplexityError {
    ApiKeyNotSet,
    RequestError(reqwest::Error),
    InvalidResponse(String),
}

impl fmt::Display for PerplexityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PerplexityError::ApiKeyNotSet => write!(f, "API key not set"),
            PerplexityError::RequestError(e) => write!(f, "Request error: {}", e),
            PerplexityError::InvalidResponse(e) => write!(f, "Invalid response: {}", e),
        }
    }
}

impl Error for PerplexityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PerplexityError::RequestError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for PerplexityError {
    fn from(err: reqwest::Error) -> Self {
        PerplexityError::RequestError(err)
    }
}

