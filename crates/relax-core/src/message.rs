use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Message {
    User(String),
    Assistant(String),
    ToolResult { name: String, output: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssistantBlock {
    Text(String),
    ToolCall { name: String, input: Value },
}
