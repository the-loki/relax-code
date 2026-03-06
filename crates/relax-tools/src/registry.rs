use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::{Tool, ToolError, ToolResult, ToolSchema};

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self, tool: T)
    where
        T: Tool + 'static,
    {
        self.tools.insert(tool.name().to_string(), Arc::new(tool));
    }

    pub fn schemas(&self) -> Vec<ToolSchema> {
        self.tools.values().map(|tool| tool.schema()).collect()
    }

    pub async fn invoke(&self, name: &str, input: Value) -> Result<ToolResult, ToolError> {
        let tool = self
            .tools
            .get(name)
            .ok_or_else(|| ToolError::NotFound(name.to_string()))?;

        tool.invoke(input).await
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
