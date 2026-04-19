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
    #[error("failed to access filesystem")]
    IoError(#[from] io::Error),
    #[error("failed to open directory")]
    DirectoryError,
    #[error("failed to access path")]
    InvalidPath,
    #[error("toml parse error")]
    TomlError(#[from] toml::de::Error),
    #[error("format error")]
    FormatError(#[from] strfmt::FmtError),
}
// Return sway configuration directory
fn sway_config_dir() -> Result<PathBuf, LineBarError> {
    let sway_config_dir = dirs::config_dir()
        .ok_or(LineBarError::DirectoryError)?
        .join(SWAY_CONFIG_DIR_NAME);
    Ok(sway_config_dir)
}

fn main() -> Result<(), LineBarError> {
    let cfg_path = sway_config_dir()?.join(LINEBAR_TOML_FILE_NAME);

    let mut stat = stat::Status::new();
    let config: Config = toml::from_str(&fs::read_to_string(cfg_path)?)?;

    stat.remove_not_listed_disks = config.general.remove_not_listed_disks.unwrap_or_default();

    loop {
        stat.refresh();
        let fmt_scope = format_scope::build_format_scope(&stat);
        let formated = strfmt(&config.general.format, &fmt_scope)?;
        println!("{formated}");
        io::stdout().flush()?;

        thread::sleep(Duration::from_millis(
            config.general.interval.unwrap_or(1000),
        ));
    }
}
