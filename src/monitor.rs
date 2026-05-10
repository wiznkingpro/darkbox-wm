use std::fs;

pub fn spawn_system_monitor() {
    println!("System monitor started");
}

pub fn get_system_info() -> String {
    let cpu = get_cpu_usage();
    let (total_ram, used_ram) = get_ram_usage();
    format!("CPU: {:.1}%  RAM: {}/{} MB", cpu, used_ram, total_ram)
}

fn get_cpu_usage() -> f64 {
    0.0
}

fn get_ram_usage() -> (u64, u64) {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total = 0u64;
    let mut available = 0u64;
    
    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace()
                .nth(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        } else if line.starts_with("MemAvailable:") {
            available = line.split_whitespace()
                .nth(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        }
    }
    
    (total / 1024, (total - available) / 1024)
}