use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SonarModel {
    /// 8B parameters, 127k context length
    Small,
    /// 70B parameters, 127k context length
    Large,
    /// 405B parameters, 127k context length
    Huge,
}

impl SonarModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            SonarModel::Small => "llama-3.1-sonar-small-128k-online",
            SonarModel::Large => "llama-3.1-sonar-large-128k-online",
            SonarModel::Huge => "llama-3.1-sonar-huge-128k-online",
        }
    }

    pub fn context_length(&self) -> usize {
        127_072 // All models have same context length
    }

    pub fn parameters(&self) -> usize {
        match self {
            SonarModel::Small => 8_000_000_000,
            SonarModel::Large => 70_000_000_000,
            SonarModel::Huge => 405_000_000_000,
        }
    }
}
