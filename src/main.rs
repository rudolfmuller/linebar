use serde::Deserialize;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use strfmt::strfmt;
use thiserror::Error;

mod format_scope;
mod fs;
mod stat;

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
    #[error("failed to parse toml configuration file")]
    TomlError(#[from] toml::de::Error),
    #[error("string formatting error")]
    FormatError(#[from] strfmt::FmtError),
}

fn main() -> anyhow::Result<()> {
    let mut stat = stat::Status::new();

    let config_contents = fs::read_config_file()?;
    let config: Config = toml::from_str(&config_contents)?;

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
