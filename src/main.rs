use chrono::Local;
use std::thread;
use std::time::Duration;
use sysinfo::{Disks, System};
fn main() {
    let mut system = System::new_all();
    let mut disks = Disks::new();
    loop {
        let now = Local::now();
        system.refresh_all();
        disks.refresh(false);
        let free_disk = if let Some(disk) = disks.list().first() {
            let total = disk.total_space();
            let available = disk.available_space();

            (available as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        let used_cpu = system.global_cpu_usage();
        let used_memory = (system.used_memory() as f32 / system.total_memory() as f32) * 100.0;

        println!(
            " {:>6.2}󱉸 󰋊 {:>3.0}󱉸  {:>6.2}󱉸 [{}]",
            used_memory,
            free_disk,
            used_cpu,
            now.format("%d-%m.%y %a %H:%M:%S"),
        );
        thread::sleep(Duration::from_millis(1000));
    }
}
