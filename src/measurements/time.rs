use serde::{Deserialize, Serialize};
use std::time::Duration;

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

impl super::MeasurementMethod for TimeMeasurement {
    fn run(&mut self) {
        self.start = Some(std::time::Instant::now());
    }

    fn stop(&mut self) {
        self.stop = Some(self.start.expect("Measurement to have started").elapsed());
    }
}
