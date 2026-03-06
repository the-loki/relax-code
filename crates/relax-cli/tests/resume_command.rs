use std::time::{SystemTime, UNIX_EPOCH};


use predicates::str::contains;
use relax_core::{Message, SessionState};
use relax_runtime::SessionStore;

#[test]
fn resume_command_loads_existing_session() {
    let workspace = std::env::temp_dir().join(format!(
        "relax-cli-resume-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    let store = SessionStore::new(&workspace);
    let mut session = SessionState::new();
    session.push_message(Message::User("hello".to_string()));
    store.save("demo", &session).unwrap();

    let mut command = assert_cmd::cargo::cargo_bin_cmd!("relax");
    command
        .arg("resume")
        .arg("--session")
        .arg("demo")
        .arg("--workspace")
        .arg(&workspace);

    command
        .assert()
        .success()
        .stdout(contains("Loaded session demo"));
}
