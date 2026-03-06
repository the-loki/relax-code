use relax_core::{Message, SessionState};
use relax_runtime::SessionStore;

#[test]
fn session_state_roundtrip_to_disk() {
    let store = SessionStore::in_temp_dir();
    let mut session = SessionState::new();
    session.push_message(Message::User("hello".to_string()));

    store.save("demo", &session).unwrap();
    let loaded = store.load("demo").unwrap();

    assert_eq!(loaded.messages(), session.messages());
}

#[test]
fn session_store_rejects_path_traversal_session_id() {
    let store = SessionStore::in_temp_dir();
    let mut session = SessionState::new();
    session.push_message(Message::User("hello".to_string()));

    let result = store.save("../escape", &session);

    assert!(result.is_err());
}
