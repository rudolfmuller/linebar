use chrono::{DateTime, Local};
use sysinfo::{Disks, System};

#[derive(Debug)]
pub struct Status {
    system: System,
    disks: Disks,
    pub now: DateTime<Local>,
    pub remove_not_listed_disks: bool,
}
impl Status {
    pub fn new() -> Self {
        let system = System::new_all();
        let disks = Disks::new();
        let now = Local::now();
        Self {
            system,
            disks,
            now,
            remove_not_listed_disks: false,
        }
    }
    pub fn refresh(&mut self) {
        self.system.refresh_all();
        self.disks.refresh(self.remove_not_listed_disks);
        self.now = Local::now();
    }
    pub fn free_disk(&self) -> f32 {
        if let Some(disk) = self.disks.list().first() {
            let total = disk.total_space();
            let available = disk.available_space();

            return (available as f32 / total as f32) * 100.0;
        } else {
            return 0.0;
        };
    }
    pub fn used_disk(&self) -> f32 {
        if let Some(disk) = self.disks.list().first() {
            let total = disk.total_space();
            let available = disk.available_space();

            return (total as f32 - available as f32) / total as f32 * 100.0;
        } else {
            return 0.0;
        };
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
