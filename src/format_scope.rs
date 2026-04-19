use crate::stat::Status;
use chrono::{Datelike, Timelike};
use std::collections::HashMap;

pub fn build_format_scope(stat: &Status) -> HashMap<String, String> {
    let mut vars = HashMap::new();

    insert(&mut vars, "disk.free", format!("{:>3.0}", stat.free_disk()));
    insert(&mut vars, "disk.used", format!("{:>3.0}", stat.used_disk()));

    insert(
        &mut vars,
        "cpu.used",
        format!("{:>6.2}", stat.global_used_cpu()),
    );
    insert(
        &mut vars,
        "memory.used",
        format!("{:>6.2}", stat.used_memory()),
    );
    insert(
        &mut vars,
        "memory.free",
        format!("{:>6.2}", stat.free_memory()),
    );

    insert(&mut vars, "date.year", stat.now.year());
    insert(
        &mut vars,
        "date.year.short",
        format!("{:02}", stat.now.year() % 100),
    );
    insert(&mut vars, "date.month", format!("{:02}", stat.now.month()));
    insert(&mut vars, "date.day", format!("{:02}", stat.now.day()));

    insert(&mut vars, "time.hour", format!("{:02}", stat.now.hour()));
    insert(&mut vars, "time.min", format!("{:02}", stat.now.minute()));
    insert(&mut vars, "time.sec", format!("{:02}", stat.now.second()));

    insert(
        &mut vars,
        "date.weekday",
        format!("{:?}", stat.now.weekday()),
    );

    vars
}

fn insert<T: ToString>(vars: &mut HashMap<String, String>, key: &str, value: T) {
    vars.insert(key.to_string(), value.to_string());
}
