use std::error::Error;
use std::future::Future;

use relax_tools::builtin::read_file::ReadFileTool;
use relax_tools::builtin::shell::ShellTool;
use relax_tools::builtin::update_plan::UpdatePlanTool;
use relax_tools::builtin::write_file::WriteFileTool;
use relax_tools::ToolRegistry;

use crate::{AssistantBlock, Message, SessionState};

pub trait ChatProvider {
    type Error;

    fn respond(
        &self,
        messages: &[Message],
    ) -> impl Future<Output = Result<Vec<AssistantBlock>, Self::Error>> + Send;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AgentLoopResult {
    session: SessionState,
    final_text: String,
}

pub fn build_system_prompt(skill_texts: &[String]) -> String {
    let mut sections = vec![
        "[Base System Prompt]\nYou are a local coding agent.".to_string(),
        "[Repository Rules]\nLoad repository instructions before acting.".to_string(),
        "[Current Task Context]\nUse the current task and session state.".to_string(),
    ];

    if !skill_texts.is_empty() {
        sections.push(format!(
            "[Skills]\n{}",
            skill_texts.join("\n\n---\n\n")
        ));
    }

    sections.push(
        "[Runtime Boundaries]\nRespect tool boundaries and workspace safety rules.".to_string(),
    );

    sections.join("\n\n")
}

impl AgentLoopResult {
    pub fn final_text(&self) -> &str {
        &self.final_text
    }

    pub fn session(&self) -> &SessionState {
        &self.session
    }
}

pub async fn run_agent_loop<P>(
    provider: &P,
) -> Result<AgentLoopResult, Box<dyn Error + Send + Sync>>
where
    P: ChatProvider,
    P::Error: Error + Send + Sync + 'static,
{
    let mut registry = ToolRegistry::new();
    registry.register(ReadFileTool);
    registry.register(WriteFileTool);
    registry.register(ShellTool);
    registry.register(UpdatePlanTool);

    let mut session = SessionState::new();
    let mut final_text = String::new();

    loop {
        let blocks = provider
            .respond(session.messages())
            .await
            .map_err(|error| Box::new(error) as Box<dyn Error + Send + Sync>)?;
        let mut saw_tool_call = false;

        for block in blocks {
            match block {
                AssistantBlock::Text(text) => {
                    final_text.push_str(&text);
                    session.push_message(Message::Assistant(text));
                }
                AssistantBlock::ToolCall { name, input } => {
                    saw_tool_call = true;
                    let output = registry
                        .invoke(&name, input)
                        .await
                        .map_err(|error| Box::new(error) as Box<dyn Error + Send + Sync>)?
                        .output_text()
                        .to_string();
                    session.push_message(Message::ToolResult { name, output });
                }
            }
        }

        if !saw_tool_call {
            break;
        }
    }

    Ok(AgentLoopResult {
        session,
        final_text,
    })
}
