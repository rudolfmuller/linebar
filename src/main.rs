use chrono::{Datelike, Timelike};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use strfmt::strfmt;
mod sysstat;

const SWAY_CONFIG_DIR_NAME: &str = "sway";

#[derive(Deserialize)]
struct Config {
    general: General,
}

#[derive(Deserialize)]
struct General {
    format: String,
    interval: u64,
}
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LineBarError {
    #[error("failed to access filesystem")]
    IoError(#[from] io::Error),
    #[error("failed to open directory")]
    DirectoryError,
    #[error("failed to access path")]
    InvalidPath,
}

fn sway_config_dir() -> Result<PathBuf, LineBarError> {
    let sway_config_dir = dirs::config_dir()
        .ok_or(LineBarError::DirectoryError)?
        .join(SWAY_CONFIG_DIR_NAME);
    Ok(sway_config_dir)
}

fn main() -> anyhow::Result<()> {
    let mut stat = sysstat::Status::new();
    let mut vars = HashMap::new();
    let cfg_path = sway_config_dir()?.join("linebar.toml");
    let config: Config = toml::from_str(&fs::read_to_string(cfg_path)?)?;
    loop {
        stat.refresh();
        vars.insert(
            "disk.free".to_string(),
            format!("{:>3.0}", stat.free_disk()),
        );
        vars.insert(
            "disk.used".to_string(),
            format!("{:>3.0}", stat.used_disk()),
        );

        vars.insert(
            "cpu.used".to_string(),
            format!("{:>6.2}", stat.global_used_cpu()),
        );
        vars.insert(
            "memory.used".to_string(),
            format!("{:>6.2}", stat.used_memory()),
        );
        vars.insert(
            "memory.free".to_string(),
            format!("{:>6.2}", stat.free_memory()),
        );

        vars.insert("date.year".to_string(), stat.now.year().to_string());
        vars.insert(
            "date.year.short".to_string(),
            format!("{:02}", stat.now.year() % 100),
        );
        vars.insert("date.month".to_string(), format!("{:02}", stat.now.month()));
        vars.insert("date.day".to_string(), format!("{:02}", stat.now.day()));

        vars.insert("time.hour".to_string(), format!("{:02}", stat.now.hour()));
        vars.insert("time.min".to_string(), format!("{:02}", stat.now.minute()));
        vars.insert("time.sec".to_string(), format!("{:02}", stat.now.second()));

        vars.insert(
            "date.weekday".to_string(),
            format!("{:?}", stat.now.weekday()),
        );

        //let fmt = " {memory.used}󱉸 󰋊 {disk.free}󱉸  {cpu.used}󱉸 [{date.day}-{date.month}.{date.year.short} {date.weekday} {time.hour}:{time.min}:{time.sec}]".to_string();
        println!("{}", strfmt(&config.general.format, &vars)?);
        io::stdout().flush()?;

        thread::sleep(Duration::from_millis(config.general.interval));
    }
}
