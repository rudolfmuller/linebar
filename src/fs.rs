use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

const SWAY_CONFIG_DIR_NAME: &str = "sway";
const LINEBAR_TOML_FILE_NAME: &str = "linebar.toml";

#[derive(Error, Debug)]
pub enum FsError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("failed to open directory")]
    DirectoryError,
    #[error("invalid path")]
    InvalidPath,
    #[error("configuration file not found (expect '~/.config/sway/linebar.toml')")]
    ConfigurationFileNotFound,
    #[error("failed to parse toml configuration file")]
    TomlError(#[from] toml::de::Error),
}

// Return sway configuration directory
fn sway_config_dir() -> Result<PathBuf, FsError> {
    let sway_config_dir = dirs::config_dir()
        .ok_or(FsError::DirectoryError)?
        .join(SWAY_CONFIG_DIR_NAME);
    Ok(sway_config_dir)
}

pub fn read_config_file() -> Result<String, FsError> {
    let cfg_path = sway_config_dir()?.join(LINEBAR_TOML_FILE_NAME);
    if !cfg_path.exists() {
        return Err(FsError::ConfigurationFileNotFound);
    }
    let config_contents = fs::read_to_string(cfg_path)?;
    Ok(config_contents)
}
