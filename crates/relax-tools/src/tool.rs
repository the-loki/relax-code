use async_trait::async_trait;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ToolSchema {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Clone)]
pub struct ToolResult {
    output: String,
}

impl ToolResult {
    pub fn text(output: String) -> Self {
        Self { output }
    }

    pub fn output_text(&self) -> &str {
        &self.output
    }
}

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("tool input is invalid: {0}")]
    InvalidInput(String),
    #[error("tool execution failed: {0}")]
    ExecutionFailed(String),
    #[error("tool not found: {0}")]
    NotFound(String),
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;

    fn schema(&self) -> ToolSchema;

    async fn invoke(&self, input: Value) -> Result<ToolResult, ToolError>;
}
