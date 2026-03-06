use relax_core::{run_agent_loop, AssistantBlock, ChatProvider, Message};

struct FakeProvider {
    response_text: String,
}

impl FakeProvider {
    fn text(value: &str) -> Self {
        Self {
            response_text: value.to_string(),
        }
    }
}

impl ChatProvider for FakeProvider {
    type Error = std::convert::Infallible;

    fn respond(
        &self,
        _messages: &[Message],
    ) -> impl std::future::Future<Output = Result<Vec<AssistantBlock>, Self::Error>> + Send {
        let blocks = vec![AssistantBlock::Text(self.response_text.clone())];

        async move { Ok(blocks) }
    }
}

#[tokio::test]
async fn agent_loop_stops_when_provider_returns_text() {
    let provider = FakeProvider::text("done");
    let result = run_agent_loop(&provider).await.unwrap();

    assert_eq!(result.final_text(), "done");
}
