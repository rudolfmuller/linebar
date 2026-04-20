use crate::stat::Status;
use chrono::{Datelike, Timelike};
use std::collections::HashMap;

pub fn build_format_scope(stat: &Status) -> HashMap<String, String> {
    let mut vars = HashMap::new();

    insert_var(&mut vars, "disk.free", format!("{:>3.0}", stat.free_disk()));
    insert_var(&mut vars, "disk.used", format!("{:>3.0}", stat.used_disk()));

    insert_var(
        &mut vars,
        "cpu.used",
        format!("{:>6.2}", stat.global_used_cpu()),
    );
    insert_var(
        &mut vars,
        "memory.used",
        format!("{:>6.2}", stat.used_memory()),
    );
    insert_var(
        &mut vars,
        "memory.free",
        format!("{:>6.2}", stat.free_memory()),
    );

    insert_var(&mut vars, "date.year", stat.now.year().to_string());
    insert_var(
        &mut vars,
        "date.year.short",
        format!("{:02}", stat.now.year() % 100),
    );
    insert_var(&mut vars, "date.month", format!("{:02}", stat.now.month()));
    insert_var(&mut vars, "date.day", format!("{:02}", stat.now.day()));

    insert_var(&mut vars, "time.hour", format!("{:02}", stat.now.hour()));
    insert_var(&mut vars, "time.min", format!("{:02}", stat.now.minute()));
    insert_var(&mut vars, "time.sec", format!("{:02}", stat.now.second()));

    insert_var(
        &mut vars,
        "date.weekday",
        format!("{:?}", stat.now.weekday()),
    );

    vars
}

fn insert_var(vars: &mut HashMap<String, String>, key: &str, value: String) {
    vars.insert(key.to_string(), value);
}
