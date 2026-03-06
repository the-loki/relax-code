use std::convert::Infallible;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use relax_core::{run_agent_loop, AssistantBlock, ChatProvider, Message};
use serde_json::json;

struct FakeProvider {
    file_path: PathBuf,
    final_text: String,
}

impl FakeProvider {
    fn tool_then_text(file_path: PathBuf, final_text: &str) -> Self {
        Self {
            file_path,
            final_text: final_text.to_string(),
        }
    }
}

impl ChatProvider for FakeProvider {
    type Error = Infallible;

    fn respond(
        &self,
        messages: &[Message],
    ) -> impl std::future::Future<Output = Result<Vec<AssistantBlock>, Self::Error>> + Send {
        let has_tool_result = messages
            .iter()
            .any(|message| matches!(message, Message::ToolResult { .. }));

        let blocks = if has_tool_result {
            vec![AssistantBlock::Text(self.final_text.clone())]
        } else {
            vec![AssistantBlock::ToolCall {
                name: "read_file".to_string(),
                input: json!({
                    "path": self.file_path,
                }),
            }]
        };

        async move { Ok(blocks) }
    }
}

fn unique_temp_file() -> PathBuf {
    let mut path = std::env::temp_dir();
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    path.push(format!("relax-core-tool-loop-{suffix}.txt"));
    path
}

#[tokio::test]
async fn agent_loop_executes_tool_then_returns_text() {
    let file_path = unique_temp_file();
    fs::write(&file_path, "hello from tool").unwrap();

    let provider = FakeProvider::tool_then_text(file_path.clone(), "done");
    let result = run_agent_loop(&provider).await.unwrap();

    assert_eq!(result.final_text(), "done");
    assert!(result
        .session()
        .messages()
        .iter()
        .any(|message| matches!(message, Message::ToolResult { name, output } if name == "read_file" && output == "hello from tool")));

    let _ = fs::remove_file(file_path);
}

#[tokio::test]
async fn agent_loop_returns_error_when_tool_execution_fails() {
    let missing_file = unique_temp_file();
    let provider = FakeProvider::tool_then_text(missing_file, "done");

    let result = run_agent_loop(&provider).await;

    assert!(result.is_err());
}
