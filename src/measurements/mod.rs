use super::measurements::cpu::CpuMeasurement;
use super::measurements::time::TimeMeasurement;
use serde::{Deserialize, Serialize};
mod cpu;
mod time;

pub trait MeasurementMethod {
    fn run(&mut self);
    fn stop(&mut self);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MeasurementType {
    TimeMeasurement(TimeMeasurement),
    CpuMeasurement(CpuMeasurement),
}

#[derive(Debug, Clone)]
pub struct Measurement {
    pub measurements: Vec<MeasurementType>,
}

impl Default for Measurement {
    fn default() -> Self {
        Self::new()
    }
}

impl Measurement {
    pub fn new() -> Self {
        Self {
            measurements: vec![MeasurementType::TimeMeasurement(TimeMeasurement::new())],
        }
    }

    pub fn add_measurement(&mut self, measurement: MeasurementType) {
        self.measurements.push(measurement);
    }

    /// run all measurements
    pub fn run(&mut self) {
        for measurement in self.measurements.iter_mut() {
            match measurement {
                MeasurementType::TimeMeasurement(ref mut time_measurement) => {
                    time_measurement.run();
                }
                MeasurementType::CpuMeasurement(_) => todo!(),
            }
        }
    }

    /// stop all measurements
    /// this will panic if the measurement has not been started
    pub fn stop(&mut self) {
        for measurement in self.measurements.iter_mut() {
            match measurement {
                MeasurementType::TimeMeasurement(time_measurement) => {
                    if time_measurement.start.is_none() {
                        panic!("Measurement has not been started");
                    }
                    time_measurement.stop();
                }
                MeasurementType::CpuMeasurement(_) => todo!(),
            }
        }
    }
}
