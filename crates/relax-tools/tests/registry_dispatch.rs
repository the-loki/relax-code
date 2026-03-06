use async_trait::async_trait;
use serde_json::json;

use relax_tools::{Tool, ToolRegistry, ToolResult, ToolSchema};

struct EchoTool;

#[async_trait]
impl Tool for EchoTool {
    fn name(&self) -> &'static str {
        "echo_tool"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: "Echo test tool".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "value": { "type": "string" }
                },
                "required": ["value"]
            }),
        }
    }

    async fn invoke(&self, input: serde_json::Value) -> Result<ToolResult, relax_tools::ToolError> {
        let value = input
            .get("value")
            .and_then(|item| item.as_str())
            .unwrap_or_default()
            .to_string();

        Ok(ToolResult::text(value))
    }
}

#[tokio::test]
async fn registry_dispatches_tool_by_name() {
    let mut registry = ToolRegistry::new();
    registry.register(EchoTool);

    let result = registry.invoke("echo_tool", json!({ "value": "ok" })).await.unwrap();

    assert_eq!(result.output_text(), "ok");
}
