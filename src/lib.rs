pub mod errors;
pub mod models;
pub mod perplexity;

pub use crate::errors::{PerplexityError, Result};
pub use crate::models::Model;
pub use crate::perplexity::{Choice, Delta, Message, Perplexity, StreamEvent, Usage};

#[derive(Debug)]
pub struct PerplexityBuilder {
    api_key: Option<String>,
    model: String,
}

impl Default for PerplexityBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            model: "llama-3.1-sonar-large-128k-online".to_string(),
        }
    }
}

impl PerplexityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn build(self) -> Result<Perplexity> {
        let api_key = self.api_key
            .or_else(|| env::var("PERPLEXITY_API_KEY").ok())
            .ok_or(PerplexityError::ApiKeyNotSet)?;

        Ok(Perplexity::new(Some(api_key), Some(self.model)))
    }
}

impl Perplexity {
    pub fn builder() -> PerplexityBuilder {
        PerplexityBuilder::new()
    }
}
