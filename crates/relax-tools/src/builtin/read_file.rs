use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::fs;

use crate::{Tool, ToolError, ToolResult, ToolSchema};

pub struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &'static str {
        "read_file"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: "Read a file from disk".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": ["path"]
            }),
        }
    }

    async fn invoke(&self, input: Value) -> Result<ToolResult, ToolError> {
        let path = input
            .get("path")
            .and_then(|item| item.as_str())
            .ok_or_else(|| ToolError::InvalidInput("path is required".to_string()))?;

        let content = fs::read_to_string(path)
            .await
            .map_err(|error| ToolError::ExecutionFailed(error.to_string()))?;

        Ok(ToolResult::text(content))
    }
}
