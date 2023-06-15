use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub process: u64,
    pub global: u64,
    pub max_mem: u64,
}

/// read current systems memory usage and max available memory
/// # Example
/// ```
/// use benchmark::sys::read_memory;
/// let mem = read_memory();
/// ```
pub fn read_memory() -> Memory {
    let path = Path::new("/proc/meminfo");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut global = None;
    let mut max_mem = None;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let key = split.next().unwrap();
        if key == "MemTotal:" {
            max_mem = Some(split.next().unwrap().parse::<u64>().unwrap());
        } else if key == "MemAvailable:" {
            global = Some(split.next().unwrap().parse::<u64>().unwrap());
        }
    }
    Memory {
        process: read_process_memory(),
        global: global.expect("failed to read global memory"),
        max_mem: max_mem.expect("failed to read max memory"),
    }
}

/// read the current process's memory usage
/// # Example
/// ```
/// use benchmark::sys::read_process_memory;
/// let mem = read_process_memory();
/// ```
pub fn read_process_memory() -> u64 {
    let path = Path::new("/proc/self/status");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut mem = None;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let key = split.next().unwrap();
        if key == "VmRSS:" {
            mem = Some(split.next().unwrap().parse::<u64>().unwrap());
        }
    }
    mem.expect("failed to read process memory")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cpu {
    pub cpu_process_usage: u64,
    pub cpu_global_usage: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub cpu_name: String,
    pub core_amount: u64,
}

/// read the cpu usage of the current process
fn read_process_cpu() -> Option<u64> {
    let path = Path::new("/proc/self/stat");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut cpu = None;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let mut i = 0;
        while i < 13 {
            split.next();
            i += 1;
        }
        cpu = Some(split.next().unwrap().parse::<u64>().unwrap());
    }
    cpu
}

/// read the global cpu usage
fn read_global_cpu() -> Option<u64> {
    let path = Path::new("/proc/stat");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut cpu = None;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let key = split.next().unwrap();
        if key == "cpu" {
            let mut i: u64 = 0;
            while i < 8 {
                split.next();
                i += 1;
            }
            cpu = Some(split.next().unwrap().parse::<u64>().unwrap());
        }
    }
    cpu
}

/// read cpu usage from the current process and the global cpu usage
/// # Example
/// ```
/// use benchmark::sys::read_cpu_usage;
/// let cpu = read_cpu_usage();
/// ```
pub fn read_cpu_usage() -> Cpu {
    Cpu {
        cpu_process_usage: read_process_cpu().expect("failed to read process cpu"),
        cpu_global_usage: read_global_cpu().expect("failed to read global cpu"),
    }
}

/// Obtains the CPU name and core amount from /proc/cpuinfo
/// # Example
/// ```
/// use benchmark::sys::get_cpu_info;
/// let cpu_info = get_cpu_info();
/// assert!(cpu_info.cpu_name.len() > 0);
/// assert!(cpu_info.core_amount > 0);
/// ```
pub fn get_cpu_info() -> CpuInfo {
    let path = Path::new("/proc/cpuinfo");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut cpu_name = None;
    let mut core_amount = None;
    for line in reader.lines() {
        let line = line.unwrap();
        let split = line.split_whitespace();
        if line.starts_with("model name") {
            cpu_name = Some(split.collect::<Vec<&str>>()[3..].join(" "));
        } else if line.starts_with("cpu cores") {
            core_amount = Some(split.collect::<Vec<&str>>()[3].parse::<u64>().unwrap());
        }
    }

    CpuInfo {
        cpu_name: cpu_name.expect("failed to read cpu name"),
        core_amount: core_amount.expect("failed to read core amount"),
    }
}

/// get rtsc time from current process
/// # Example
/// ```
/// use benchmark::sys::get_rtsc_time;
/// let rtsc = get_rtsc_time();
/// assert!(rtsc > 0);
/// ```
pub fn get_rtsc_time() -> u64 {
    let path = Path::new("/proc/self/stat");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut rtsc = None;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let mut i = 0;
        while i < 21 {
            split.next();
            i += 1;
        }
        rtsc = Some(split.next().unwrap().parse::<u64>().unwrap());
    }
    rtsc.expect("failed to read rtsc time")
}
