use async_trait::async_trait;
use serde_json::{json, Value};

use crate::{Tool, ToolError, ToolResult, ToolSchema};

pub struct UpdatePlanTool;

#[async_trait]
impl Tool for UpdatePlanTool {
    fn name(&self) -> &'static str {
        "update_plan"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: "Update the current plan with a short explanation and step statuses"
                .to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "explanation": { "type": "string" },
                    "steps": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "step": { "type": "string" },
                                "status": { "type": "string" }
                            },
                            "required": ["step", "status"]
                        }
                    }
                },
                "required": ["steps"]
            }),
        }
    }

    async fn invoke(&self, input: Value) -> Result<ToolResult, ToolError> {
        let steps = input
            .get("steps")
            .and_then(|item| item.as_array())
            .ok_or_else(|| ToolError::InvalidInput("steps is required".to_string()))?;

        let mut lines = Vec::new();

        if let Some(explanation) = input.get("explanation").and_then(|item| item.as_str()) {
            lines.push(explanation.to_string());
        }

        for step in steps {
            let step_name = step
                .get("step")
                .and_then(|item| item.as_str())
                .ok_or_else(|| ToolError::InvalidInput("step is required".to_string()))?;
            let status = step
                .get("status")
                .and_then(|item| item.as_str())
                .ok_or_else(|| ToolError::InvalidInput("status is required".to_string()))?;
            lines.push(format!("[{status}] {step_name}"));
        }

        Ok(ToolResult::text(lines.join("\n")))
    }
}
