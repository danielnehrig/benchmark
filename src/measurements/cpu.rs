use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::sys::Cpu;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMeasurement {
    pub data: Vec<Cpu>,
    #[serde(skip)]
    pub handle: Rc<Option<tokio::task::JoinHandle<()>>>,
}

impl CpuMeasurement {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            handle: Rc::new(None),
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
