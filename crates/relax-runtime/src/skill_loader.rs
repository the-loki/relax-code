use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SkillLoader {
    root: PathBuf,
}

impl SkillLoader {
    pub fn from_root(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    pub fn from_workspace(workspace: impl AsRef<Path>) -> Self {
        Self::from_root(workspace.as_ref().join("skills"))
    }

    pub fn load(&self, skill_name: &str) -> Result<String, io::Error> {
        validate_skill_name(skill_name)?;
        fs::read_to_string(self.root.join(skill_name).join("SKILL.md"))
    }
}

fn validate_skill_name(skill_name: &str) -> Result<(), io::Error> {
    if skill_name.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "skill name cannot be empty",
        ));
    }

    if skill_name
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
    {
        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "skill name contains invalid characters",
    ))
}
