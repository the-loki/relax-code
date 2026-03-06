use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::fs;

use crate::{Tool, ToolError, ToolResult, ToolSchema};

pub struct WriteFileTool;

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &'static str {
        "write_file"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: "Write content to a file".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "content": { "type": "string" }
                },
                "required": ["path", "content"]
            }),
        }
    }

    async fn invoke(&self, input: Value) -> Result<ToolResult, ToolError> {
        let path = input
            .get("path")
            .and_then(|item| item.as_str())
            .ok_or_else(|| ToolError::InvalidInput("path is required".to_string()))?;
        let content = input
            .get("content")
            .and_then(|item| item.as_str())
            .ok_or_else(|| ToolError::InvalidInput("content is required".to_string()))?;

        fs::write(path, content)
            .await
            .map_err(|error| ToolError::ExecutionFailed(error.to_string()))?;

        Ok(ToolResult::text(path.to_string()))
    }
}
