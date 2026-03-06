use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use relax_runtime::{Config, RuntimePaths};

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn runtime_paths_default_to_dot_relax() {
    let paths = RuntimePaths::from_workspace("/tmp/project");
    assert!(paths.root.ends_with(".relax"));
    assert!(paths.sessions.ends_with(".relax/sessions"));
    assert!(paths.tasks.ends_with(".relax/tasks"));
}

#[test]
fn config_loads_defaults_when_no_sources_exist() {
    with_clean_env(|| {
        let workspace = create_temp_workspace();
        let config = Config::load_from_workspace(&workspace).unwrap();

        assert_eq!(config.provider, "openai-compatible");
        assert_eq!(config.model, "gpt-4.1-mini");
        assert_eq!(config.config_file, workspace.join("relax.toml"));
    });
}

#[test]
fn config_loads_values_from_relax_toml() {
    with_clean_env(|| {
        let workspace = create_temp_workspace();
        fs::write(
            workspace.join("relax.toml"),
            "provider = \"custom-provider\"\nmodel = \"custom-model\"\n",
        )
        .unwrap();

        let config = Config::load_from_workspace(&workspace).unwrap();

        assert_eq!(config.provider, "custom-provider");
        assert_eq!(config.model, "custom-model");
    });
}

#[test]
fn config_prefers_environment_over_file_values() {
    with_clean_env(|| {
        let workspace = create_temp_workspace();
        fs::write(
            workspace.join("relax.toml"),
            "provider = \"file-provider\"\nmodel = \"file-model\"\n",
        )
        .unwrap();

        std::env::set_var("RELAX_PROVIDER", "env-provider");
        std::env::set_var("RELAX_MODEL", "env-model");

        let config = Config::load_from_workspace(&workspace).unwrap();

        assert_eq!(config.provider, "env-provider");
        assert_eq!(config.model, "env-model");
    });
}

fn with_clean_env(test_fn: impl FnOnce()) {
    let _guard = ENV_LOCK.lock().unwrap();
    let original_provider = std::env::var("RELAX_PROVIDER").ok();
    let original_model = std::env::var("RELAX_MODEL").ok();

    std::env::remove_var("RELAX_PROVIDER");
    std::env::remove_var("RELAX_MODEL");

    test_fn();

    restore_var("RELAX_PROVIDER", original_provider);
    restore_var("RELAX_MODEL", original_model);
}

fn restore_var(name: &str, value: Option<String>) {
    if let Some(value) = value {
        std::env::set_var(name, value);
    } else {
        std::env::remove_var(name);
    }
}

fn create_temp_workspace() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("relax-runtime-test-{unique}"));
    fs::create_dir_all(&path).unwrap();
    path
}
