use reqwest::StatusCode;

use relax_providers::openai_compatible::{parse_chat_response, parse_http_response};
use relax_providers::provider::{ProviderBlock, ProviderError};

#[test]
fn parse_text_response_block() {
    let json = r#"{"choices":[{"message":{"content":"hello"}}]}"#;
    let parsed = parse_chat_response(json).unwrap();

    assert_eq!(parsed.blocks.len(), 1);
    assert_eq!(parsed.blocks[0], ProviderBlock::Text("hello".to_string()));
}

#[test]
fn parse_tool_call_response_block() {
    let json = r#"
    {
        "choices": [
            {
                "message": {
                    "content": null,
                    "tool_calls": [
                        {
                            "id": "call_123",
                            "type": "function",
                            "function": {
                                "name": "read_file",
                                "arguments": "{\"path\":\"README.md\"}"
                            }
                        }
                    ]
                }
            }
        ]
    }
    "#;
    let parsed = parse_chat_response(json).unwrap();

    assert_eq!(parsed.blocks.len(), 1);
    assert_eq!(
        parsed.blocks[0],
        ProviderBlock::ToolCall {
            id: "call_123".to_string(),
            name: "read_file".to_string(),
            arguments: "{\"path\":\"README.md\"}".to_string(),
        }
    );
}

#[test]
fn parse_http_response_rejects_error_status() {
    let error = parse_http_response(
        StatusCode::BAD_REQUEST,
        r#"{"error":{"message":"bad request"}}"#,
    )
    .unwrap_err();

    match error {
        ProviderError::HttpStatus { status, body } => {
            assert_eq!(status, StatusCode::BAD_REQUEST);
            assert!(body.contains("bad request"));
        }
        other => panic!("unexpected error: {other}"),
    }
}
