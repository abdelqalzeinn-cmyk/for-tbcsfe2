use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::ui::get_project_dir;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditorConfig {
    pub use_waydroid_by_default: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to deserialize config: {0}")]
    TomlDe(toml::de::Error),
    #[error("failed to serialize config: {0}")]
    TomlSe(toml::ser::Error),
    #[error("failed to read path: {0} due to: {1}")]
    ReadPath(PathBuf, std::io::Error),
    #[error("failed to write to path: {0} due to: {1}")]
    WriteFile(PathBuf, std::io::Error),
    #[error("failed to determine if path {0} exists: {1}")]
    ExistsFail(PathBuf, std::io::Error),
    #[error("failed to find suitable location for config file")]
    NoHomeDir,
    #[error("failed to create dirs at path: {0} due to {1}")]
    CreateDirs(PathBuf, std::io::Error),
}

impl EditorConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_default_path() -> Option<PathBuf> {
        get_project_dir().map(|v| v.config_dir().join("config.toml"))
    }

    pub fn from_str(data: &str) -> Result<Self, ConfigError> {
        toml::from_str(data).map_err(ConfigError::TomlDe)
    }

    pub fn to_str(&self) -> Result<String, ConfigError> {
        toml::to_string_pretty(self).map_err(ConfigError::TomlSe)
    }

    pub fn from_path(path: &Path) -> Result<Self, ConfigError> {
        Self::from_str(
            &std::fs::read_to_string(path)
                .map_err(|e| ConfigError::ReadPath(path.to_path_buf(), e))?,
        )
    }

    pub fn to_path(&self, path: &Path) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ConfigError::CreateDirs(parent.to_path_buf(), e))?;
        }
        std::fs::write(path, self.to_str()?.as_bytes())
            .map_err(|e| ConfigError::WriteFile(path.to_path_buf(), e))
    }

    pub fn open_path(path: &Path) -> Result<Self, ConfigError> {
        if std::fs::exists(&path).map_err(|e| ConfigError::ExistsFail(path.to_path_buf(), e))? {
            Self::from_path(&path)
        } else {
            let s = Self::new();
            s.to_path(&path)?;
            Ok(s)
        }
    }

    pub fn open(path_override: Option<&Path>) -> Result<Self, ConfigError> {
        let path = path_override
            .map(|v| v.to_path_buf())
            .or(Self::get_default_path());

        if let Some(ref path) = path {
            Self::open_path(path)
        } else {
            // TODO: should let user know
            Ok(Self::new())
        }
    }

    pub fn save(&self, path_override: Option<&Path>) -> Result<(), ConfigError> {
        let path = path_override
            .map(|v| v.to_path_buf())
            .or(Self::get_default_path());
        if let Some(ref path) = path {
            self.to_path(path)
        } else {
            Ok(())
        }
    }
}
