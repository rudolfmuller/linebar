use chrono::{Datelike, Timelike};
use clap::Parser;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use strfmt::strfmt;

mod sysstat;
#[derive(Parser, Debug)]
struct Args {
    format: String,
    #[arg(long)]
    remove_not_listed_disks: bool,
    #[arg(long)]
    interval: u64,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut stat = sysstat::Status::new();
    let mut vars = HashMap::new();
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
        println!("{}", strfmt(&args.format, &vars)?);

        thread::sleep(Duration::from_millis(args.interval));
    }
}
