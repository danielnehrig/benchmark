use serde::{Deserialize, Serialize};

/// TODO: Add CPU Load over time
/// Add system CPU cores and MEM capacity as well as frequency
#[derive(Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    /// stores the time in nanoseconds on how long the closure needed to run
    pub time: Vec<i64>,
    /// platform system name
    pub platform: String,
    /// lib controlled additional info
    pub additional: Option<Vec<String>>,
}

impl BenchmarkResults {
    pub fn add_info(&mut self, info: String) -> Self {
        if let Some(additional) = &mut self.additional {
            additional.push(info);
        } else {
            self.additional = Some(vec![info]);
        }

        self.to_owned()
    }

    pub fn save(&self, filename: &str) {
        let json = serde_json::to_string(&self).unwrap();
        std::fs::write(format!("{}.json", filename), json).expect("unable to write");
    }
}
