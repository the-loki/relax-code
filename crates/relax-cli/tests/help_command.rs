use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn help_command_prints_chat_subcommand() {
    let mut cmd = cargo_bin_cmd!("relax");
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("chat"));
}
