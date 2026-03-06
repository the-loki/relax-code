use std::future::Future;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderMessage {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderBlock {
    Text(String),
    ToolCall {
        id: String,
        name: String,
        arguments: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRequest {
    pub model: String,
    pub messages: Vec<ProviderMessage>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderResponse {
    pub blocks: Vec<ProviderBlock>,
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("failed to parse provider response: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("http request failed with status {status}: {body}")]
    HttpStatus {
        status: reqwest::StatusCode,
        body: String,
    },
}

pub trait ChatProvider {
    fn complete(
        &self,
        request: &ProviderRequest,
    ) -> impl Future<Output = Result<ProviderResponse, ProviderError>> + Send;
}
