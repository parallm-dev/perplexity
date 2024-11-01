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
            let mut request = self
                .client
                .post("https://api.perplexity.ai/chat/completions")
                .header("Content-Type", "application/json")
                .json(&json!({
                    "model": self.model,
                    "messages": [{"role": "user", "content": query}],
                    "stream": true,
                }));

            if let Some(api_key) = &self.api_key {
                request = request.header("Authorization", format!("Bearer {}", api_key));
            }

            let mut response = request.send().await?;
            let mut full_content = String::new();
            let mut buffer = String::new();

            while let Some(chunk) = response.chunk().await? {
                buffer.push_str(&String::from_utf8_lossy(&chunk));
                for line in buffer.lines() {
                    if let Some(json) = line.strip_prefix("data: ") {
                        if let Ok(event) = serde_json::from_str::<StreamEvent>(json) {
                            if let Some(choice) = event.choices.first() {
                                full_content.push_str(&choice.delta.content);
                            }
                        }
                    }
                }
                buffer.clear();
            }

            Ok(full_content)
        }
    }

    use serde;

    #[derive(serde::Deserialize, Debug)]
    pub struct StreamEvent {
        pub id: String,
        pub model: String,
        pub created: u64,
        pub usage: Usage,
        pub object: String,
        pub choices: Vec<Choice>,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct Usage {
        pub prompt_tokens: u32,
        pub completion_tokens: u32,
        pub total_tokens: u32,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct Choice {
        pub index: u32,
        pub finish_reason: Option<String>,
        pub message: Message,
        pub delta: Delta,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct Message {
        pub role: String,
        pub content: String,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct Delta {
        pub role: String,
        pub content: String,
    }
}
