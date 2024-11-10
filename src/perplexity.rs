use crate::errors::{PerplexityError, Result};
use crate::sonar::SonarModel;
use futures::{Stream, StreamExt};
use reqwest;
use serde_json::{self, json};
use std::env;
use tokio::runtime::Runtime;

/// The main struct for interacting with the Perplexity API.
#[derive(Debug)]
pub struct Perplexity {
    api_key: Option<String>,
    client: reqwest::Client,
    model: SonarModel,
}

impl Perplexity {
    /// Sends a query to the Perplexity API and returns the response.
    pub async fn query(&self, query: &str) -> impl Stream<Item = Result<String>> {
        let mut request = self
            .client
            .post("https://api.perplexity.ai/chat/completions")
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": self.model,
                "messages": [{"role": "user", "content": query}],
                "stream": true
            }));

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let stream = async_stream::try_stream! {
            let mut response = request.send().await?;
            let mut buffer = String::new();

            while let Some(chunk) = response.chunk().await? {
                buffer.push_str(&String::from_utf8_lossy(&chunk));
                for line in buffer.lines() {
                    if let Some(json) = line.strip_prefix("data: ") {
                        if let Ok(event) = serde_json::from_str::<StreamEvent>(json) {
                            if let Some(choice) = event.choices.first() {
                                yield choice.delta.content.clone();
                            }
                        }
                    }
                }
                buffer.clear();
            }
        };

        stream
    }

    /// Sends a synchronous query to the Perplexity API and returns the response.
    pub fn result(&self, query: &str) -> Result<String> {
        match Runtime::new() {
            Ok(rt) => {
                let stream = rt.block_on(self.query(query));
                let mut result = String::new();
                rt.block_on(async {
                    let mut stream = Box::pin(stream);
                    while let Some(chunk) = stream.next().await {
                        if let Ok(content) = chunk {
                            result.push_str(&content);
                        }
                    }
                });
                Ok(result)
            }
            Err(e) => Err(PerplexityError::IoError(e)),
        }
    }
}

/// Represents a streaming event from the Perplexity API.
#[derive(serde::Deserialize, Debug)]
pub struct StreamEvent {
    pub id: String,
    pub model: String,
    pub created: u64,
    pub usage: Usage,
    pub object: String,
    pub choices: Vec<Choice>,
}

/// Represents usage information for a Perplexity API request.
#[derive(serde::Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Represents a choice in the Perplexity API response.
#[derive(serde::Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub finish_reason: Option<String>,
    pub message: Message,
    pub delta: Delta,
}

/// Represents a message in the Perplexity API response.
#[derive(serde::Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Represents a delta in the Perplexity API streaming response.
#[derive(serde::Deserialize, Debug)]
pub struct Delta {
    pub role: String,
    pub content: String,
}

#[derive(Debug)]
pub struct PerplexityBuilder {
    api_key: Option<String>,
    model: SonarModel,
}

impl PerplexityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn model(mut self, model: SonarModel) -> Self {
        self.model = model;
        self
    }

    pub fn build(self) -> Result<Perplexity> {
        let api_key = self
            .api_key
            .or_else(|| env::var("PERPLEXITY_API_KEY").ok())
            .ok_or(PerplexityError::ApiKeyNotSet)?;

        Ok(Perplexity {
            api_key: Some(api_key),
            client: reqwest::Client::new(),
            model: self.model,
        })
    }
}

impl Default for PerplexityBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            model: SonarModel::Small,
        }
    }
}
