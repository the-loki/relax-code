use serde::{Deserialize, Serialize};

use crate::provider::{ChatProvider, ProviderBlock, ProviderError, ProviderRequest, ProviderResponse};

#[derive(Clone, Debug)]
pub struct OpenAiCompatibleClient {
    http_client: reqwest::Client,
    endpoint: String,
}

impl OpenAiCompatibleClient {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            endpoint: endpoint.into(),
        }
    }

    pub async fn send(&self, request: &ProviderRequest) -> Result<ProviderResponse, ProviderError> {
        let response = self
            .http_client
            .post(&self.endpoint)
            .json(&OpenAiCompatibleRequest::from(request))
            .send()
            .await?;
        let status = response.status();
        let response_text = response.text().await?;

        parse_http_response(status, &response_text)
    }
}

impl ChatProvider for OpenAiCompatibleClient {
    async fn complete(&self, request: &ProviderRequest) -> Result<ProviderResponse, ProviderError> {
        self.send(request).await
    }
}

pub fn parse_chat_response(json: &str) -> Result<ProviderResponse, ProviderError> {
    let parsed: OpenAiCompatibleResponse = serde_json::from_str(json)?;
    let mut blocks = Vec::new();

    for choice in parsed.choices {
        if let Some(content) = choice.message.content {
            if !content.is_empty() {
                blocks.push(ProviderBlock::Text(content));
            }
        }

        if let Some(tool_calls) = choice.message.tool_calls {
            for tool_call in tool_calls {
                if tool_call.kind == "function" {
                    blocks.push(ProviderBlock::ToolCall {
                        id: tool_call.id,
                        name: tool_call.function.name,
                        arguments: tool_call.function.arguments,
                    });
                }
            }
        }
    }

    Ok(ProviderResponse { blocks })
}

pub fn parse_http_response(
    status: reqwest::StatusCode,
    body: &str,
) -> Result<ProviderResponse, ProviderError> {
    if !status.is_success() {
        return Err(ProviderError::HttpStatus {
            status,
            body: body.to_string(),
        });
    }

    parse_chat_response(body)
}

#[derive(Clone, Debug, Serialize)]
struct OpenAiCompatibleRequest {
    model: String,
    messages: Vec<OpenAiCompatibleMessage>,
}

impl From<&ProviderRequest> for OpenAiCompatibleRequest {
    fn from(value: &ProviderRequest) -> Self {
        Self {
            model: value.model.clone(),
            messages: value
                .messages
                .iter()
                .map(|message| OpenAiCompatibleMessage {
                    role: message.role.clone(),
                    content: message.content.clone(),
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct OpenAiCompatibleMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAiCompatibleResponse {
    choices: Vec<OpenAiCompatibleChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiCompatibleChoice {
    message: OpenAiCompatibleAssistantMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAiCompatibleAssistantMessage {
    content: Option<String>,
    tool_calls: Option<Vec<OpenAiCompatibleToolCall>>,
}

#[derive(Debug, Deserialize)]
struct OpenAiCompatibleToolCall {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    function: OpenAiCompatibleToolFunction,
}

#[derive(Debug, Deserialize)]
struct OpenAiCompatibleToolFunction {
    name: String,
    arguments: String,
}
