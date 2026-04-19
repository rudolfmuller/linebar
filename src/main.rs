use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use strfmt::strfmt;
use thiserror::Error;

mod format_scope;
mod stat;
const SWAY_CONFIG_DIR_NAME: &str = "sway";
const LINEBAR_TOML_FILE_NAME: &str = "linebar.toml";
const DEFAULT_INTERVAL_VALUE: u64 = 1000;
// TOML Config
#[derive(Deserialize)]
struct Config {
    general: General,
}

#[derive(Deserialize)]
struct General {
    format: String,
    interval: Option<u64>,
    remove_not_listed_disks: Option<bool>,
}

#[derive(Error, Debug)]
pub enum LineBarError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("failed to open directory")]
    DirectoryError,
    #[error("invalid path")]
    InvalidPath,
    #[error("configuration file not found")]
    ConfigurationFileNotFound,
    #[error("failed to parse toml configuration file")]
    TomlError(#[from] toml::de::Error),
    #[error("string formatting error")]
    FormatError(#[from] strfmt::FmtError),
}
// Return sway configuration directory
fn sway_config_dir() -> Result<PathBuf, LineBarError> {
    let sway_config_dir = dirs::config_dir()
        .ok_or(LineBarError::DirectoryError)?
        .join(SWAY_CONFIG_DIR_NAME);
    Ok(sway_config_dir)
}

fn read_config_file() -> Result<Config, LineBarError> {
    let cfg_path = sway_config_dir()?.join(LINEBAR_TOML_FILE_NAME);
    if !cfg_path.exists() {
        return Err(LineBarError::ConfigurationFileNotFound);
    }
    let config_contents = &fs::read_to_string(cfg_path)?;
    let config = toml::from_str(config_contents)?;
    Ok(config)
}

fn main() -> anyhow::Result<()> {
    let mut stat = stat::Status::new();

    let config: Config = read_config_file()?;
    stat.remove_not_listed_disks = config.general.remove_not_listed_disks.unwrap_or_default();

    loop {
        stat.refresh();
        let fmt_scope = format_scope::build_format_scope(&stat);
        let formated = strfmt(&config.general.format, &fmt_scope)?;
        println!("{formated}");
        io::stdout().flush()?;

        thread::sleep(Duration::from_millis(
            config.general.interval.unwrap_or(DEFAULT_INTERVAL_VALUE),
        ));
    }
}
