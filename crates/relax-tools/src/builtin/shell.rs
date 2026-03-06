use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::process::Command;

use crate::{Tool, ToolError, ToolResult, ToolSchema};

pub struct ShellTool;

#[async_trait]
impl Tool for ShellTool {
    fn name(&self) -> &'static str {
        "shell"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: "Run a shell command".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string" }
                },
                "required": ["command"]
            }),
        }
    }

    async fn invoke(&self, input: Value) -> Result<ToolResult, ToolError> {
        let command = input
            .get("command")
            .and_then(|item| item.as_str())
            .ok_or_else(|| ToolError::InvalidInput("command is required".to_string()))?;

        #[cfg(target_os = "windows")]
        let output = Command::new("powershell.exe")
            .arg("-Command")
            .arg(command)
            .output()
            .await
            .map_err(|error| ToolError::ExecutionFailed(error.to_string()))?;

        #[cfg(not(target_os = "windows"))]
        let output = Command::new("sh")
            .arg("-lc")
            .arg(command)
            .output()
            .await
            .map_err(|error| ToolError::ExecutionFailed(error.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(ToolResult::text(stdout.trim_end().to_string()))
        } else {
            Err(ToolError::ExecutionFailed(stderr.trim_end().to_string()))
        }
    }
}
