use chrono::{DateTime, Local};
use sysinfo::{Disks, System};

#[derive(Debug)]
pub struct Status {
    system: System,
    pub now: DateTime<Local>,
    pub remove_not_listed_disks: bool,
}
impl Status {
    pub fn new() -> Self {
        let system = System::new_all();
        let now = Local::now();
        Self {
            system,
            now,
            remove_not_listed_disks: false,
        }
    }
    pub fn refresh(&mut self) {
        self.system.refresh_memory();
        self.system.refresh_cpu_all();
        self.now = Local::now();
    }
    pub fn free_disk(&self) -> f32 {
        let disks = Disks::new_with_refreshed_list();

        if let Some(disk) = disks.list().first() {
            let total = disk.total_space() as f32;
            let available = disk.available_space() as f32;

            (available / total) * 100.0
        } else {
            0.0
        }
    }
    pub fn used_disk(&self) -> f32 {
        let disks = Disks::new_with_refreshed_list();

        if let Some(disk) = disks.list().first() {
            let total = disk.total_space() as f32;
            let available = disk.available_space() as f32;

            ((total - available) / total) * 100.0
        } else {
            0.0
        }
    }
    pub fn global_used_cpu(&self) -> f32 {
        self.system.global_cpu_usage()
    }
    pub fn used_memory(&self) -> f32 {
        (self.system.used_memory() as f32 / self.system.total_memory() as f32) * 100.0
    }

    pub fn free_memory(&self) -> f32 {
        (self.system.free_memory() as f32 / self.system.total_memory() as f32) * 100.0
    }
}
