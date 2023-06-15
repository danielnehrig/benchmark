#[cfg(windows)]
use super::{Cpu, CpuInfo, Memory};

#[cfg(windows)]
/// read current systems memory usage and max available memory
/// # Example
/// ```
/// use benchmark::sys::read_memory;
/// let mem = read_memory();
/// ```
pub fn read_memory() -> Memory {
    todo!();
}

#[cfg(windows)]
/// get rtsc time from current process
/// # Example
/// ```
/// use benchmark::sys::get_rtsc_time;
/// let rtsc = get_rtsc_time();
/// assert!(rtsc > 0);
/// ```
pub fn get_rtsc_time() -> u64 {
    todo!();
}

#[cfg(windows)]
/// Obtains the CPU name and core amount from /proc/cpuinfo
/// # Example
/// ```
/// use benchmark::sys::get_cpu_info;
/// let cpu_info = get_cpu_info();
/// assert!(cpu_info.cpu_name.len() > 0);
/// assert!(cpu_info.core_amount > 0);
/// assert!(cpu_info.thread_amount > 0);
/// assert!(cpu_info.cpu_mhz > 0.0);
/// ```
pub fn get_cpu_info() -> CpuInfo {
    todo!();
}

#[cfg(windows)]
/// read the current process's memory usage
/// # Example
/// ```
/// use benchmark::sys::read_process_memory;
/// let mem = read_process_memory();
/// ```
pub fn read_process_memory() -> u64 {
    todo!();
}

#[cfg(windows)]
/// read the cpu usage of the current process
fn read_process_cpu() -> Option<u64> {
    todo!();
}

#[cfg(windows)]
/// read the global cpu usage
fn read_global_cpu() -> Option<u64> {
    todo!();
}

#[cfg(windows)]
/// read cpu usage from the current process and the global cpu usage
/// # Example
/// ```
/// use benchmark::sys::read_cpu_usage;
/// let cpu = read_cpu_usage();
/// ```
pub fn read_cpu_usage() -> Cpu {
    todo!();
}
