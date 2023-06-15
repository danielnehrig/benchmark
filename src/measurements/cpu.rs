use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuMeasurement {
    pub cpu_process_usage: u64,
    pub cpu_global_usage: u64,
}

impl CpuMeasurement {
    pub fn new() -> Self {
        Self {
            cpu_process_usage: 0,
            cpu_global_usage: 0,
        }
    }
}

impl Default for CpuMeasurement {
    fn default() -> Self {
        Self::new()
    }
}

impl super::MeasurementMethod for CpuMeasurement {
    fn run(&mut self) {
        todo!()
    }
    fn stop(&mut self) {
        todo!()
    }
}
