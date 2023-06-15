use std::time::Duration;

use serde::{Deserialize, Serialize};

pub trait MeasurementMethod {
    fn run(&mut self);
    fn stop(&mut self);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MeasurementType {
    TimeMeasurement(TimeMeasurement),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeMeasurement {
    #[serde(skip_serializing, skip_deserializing)]
    pub start: Option<std::time::Instant>,

    pub stop: Option<Duration>,
}

impl TimeMeasurement {
    pub fn new() -> Self {
        Self {
            start: None,
            stop: None,
        }
    }
}

impl Default for TimeMeasurement {
    fn default() -> Self {
        Self::new()
    }
}

impl MeasurementMethod for TimeMeasurement {
    fn run(&mut self) {
        self.start = Some(std::time::Instant::now());
    }

    fn stop(&mut self) {
        self.stop = Some(self.start.expect("Measurement to have started").elapsed());
    }
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

    pub fn run(&mut self) {
        for measurement in self.measurements.iter_mut() {
            match measurement {
                MeasurementType::TimeMeasurement(ref mut time_measurement) => {
                    time_measurement.run();
                }
            }
        }
    }

    pub fn stop(&mut self) {
        for measurement in self.measurements.iter_mut() {
            match measurement {
                MeasurementType::TimeMeasurement(time_measurement) => {
                    time_measurement.stop();
                }
            }
        }
    }
}
