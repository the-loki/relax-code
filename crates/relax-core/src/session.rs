use crate::Message;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionState {
    messages: Vec<Message>,
}

impl SessionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn push_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
