use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePaths {
    pub root: PathBuf,
    pub sessions: PathBuf,
    pub tasks: PathBuf,
    pub cache: PathBuf,
    pub skills_index: PathBuf,
}

impl RuntimePaths {
    pub fn from_workspace(workspace: impl AsRef<Path>) -> Self {
        let root = workspace.as_ref().join(".relax");
        Self {
            sessions: root.join("sessions"),
            tasks: root.join("tasks"),
            cache: root.join("cache"),
            skills_index: root.join("skills-index.json"),
            root,
        }
    }
}
