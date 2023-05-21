use std::time::Duration;

use serde::{Deserialize, Serialize};

/// TODO: Add CPU Load over time
/// Add system CPU cores and MEM capacity as well as frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    /// stores the time in nanoseconds on how long the closure needed to run
    pub time: Vec<Duration>,
    /// platform system name
    pub platform: String,
    /// lib controlled additional info
    pub additional: Option<Vec<String>>,
}

impl BenchmarkResults {
    /// Adds additional information that can be dumped to the json file
    ///
    /// # Example
    ///
    /// ```
    /// use benchmark::results::BenchmarkResults;
    /// let mut results = BenchmarkResults { time: vec![], platform: "test".to_string(), additional: None };
    /// results.add_info("test".to_string());
    /// ```
    pub fn add_info(&mut self, info: String) -> Self {
        match &mut self.additional {
            Some(additional) => {
                additional.push(info);
            }
            _ => {
                self.additional = Some(vec![info]);
            }
        }

        self.to_owned()
    }

    /// Saves the Benchmark result to disk as a json file
    ///
    /// # Example
    ///
    /// ```
    /// use benchmark::results::BenchmarkResults;
    /// let results = BenchmarkResults { time: vec![], platform: "test".to_string(), additional: None };
    /// results.save("test");
    /// ```
    pub fn save(&self, filename: &str) {
        let json = serde_json::to_string(&self).unwrap();
        std::fs::write(format!("{}.json", filename), json).expect("unable to write");
    }
}
