use serde::{Deserialize, Serialize};

#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod linux;
#[cfg_attr(windows, path = "windows.rs")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(windows)]
pub use windows::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub process: u64,
    pub global: u64,
    pub max_mem: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cpu {
    pub cpu_process_usage: u64,
    pub cpu_global_usage: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub cpu_name: String,
    pub core_amount: i16,
    pub thread_amount: i16,
    pub cpu_mhz: f64,
}
