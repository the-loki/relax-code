use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use relax_core::{Message, SessionState};
use serde::{Deserialize, Serialize};

use crate::RuntimePaths;

type SessionStoreResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct SessionStore {
    paths: RuntimePaths,
}

impl SessionStore {
    pub fn new(workspace: impl AsRef<Path>) -> Self {
        Self {
            paths: RuntimePaths::from_workspace(workspace),
        }
    }

    pub fn in_temp_dir() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let workspace = std::env::temp_dir().join(format!("relax-session-store-{timestamp}"));
        Self::new(workspace)
    }

    pub fn save(&self, session_id: &str, session: &SessionState) -> SessionStoreResult<()> {
        fs::create_dir_all(&self.paths.sessions)?;
        let stored = StoredSession::from_session_state(session);
        let payload = serde_json::to_string_pretty(&stored)?;
        fs::write(self.session_file(session_id)?, payload)?;
        Ok(())
    }

    pub fn load(&self, session_id: &str) -> SessionStoreResult<SessionState> {
        let payload = fs::read_to_string(self.session_file(session_id)?)?;
        let stored: StoredSession = serde_json::from_str(&payload)?;
        Ok(stored.into_session_state())
    }

    pub fn session_file(&self, session_id: &str) -> SessionStoreResult<PathBuf> {
        validate_session_id(session_id)?;
        Ok(self.paths.sessions.join(format!("{session_id}.json")))
    }
}

fn validate_session_id(session_id: &str) -> SessionStoreResult<()> {
    if session_id.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "session id cannot be empty").into());
    }

    if session_id
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
    {
        return Ok(());
    }

    Err(io::Error::new(io::ErrorKind::InvalidInput, "session id contains invalid characters").into())
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredSession {
    messages: Vec<StoredMessage>,
}

impl StoredSession {
    fn from_session_state(session: &SessionState) -> Self {
        Self {
            messages: session
                .messages()
                .iter()
                .cloned()
                .map(StoredMessage::from)
                .collect(),
        }
    }

    fn into_session_state(self) -> SessionState {
        let mut session = SessionState::new();
        for message in self.messages {
            session.push_message(message.into());
        }
        session
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum StoredMessage {
    User { content: String },
    Assistant { content: String },
    ToolResult { name: String, output: String },
}

impl From<Message> for StoredMessage {
    fn from(value: Message) -> Self {
        match value {
            Message::User(content) => Self::User { content },
            Message::Assistant(content) => Self::Assistant { content },
            Message::ToolResult { name, output } => Self::ToolResult { name, output },
        }
    }
}

impl From<StoredMessage> for Message {
    fn from(value: StoredMessage) -> Self {
        match value {
            StoredMessage::User { content } => Self::User(content),
            StoredMessage::Assistant { content } => Self::Assistant(content),
            StoredMessage::ToolResult { name, output } => Self::ToolResult { name, output },
        }
    }
}
