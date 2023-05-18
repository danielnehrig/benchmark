use crate::results::BenchmarkResults;

// builder pattern for the benchmarking
#[derive(Clone, Debug)]
pub struct BenchmarkBuilder {
    pub passes: i32,
    debug: bool,
}

impl Default for BenchmarkBuilder {
    fn default() -> Self {
        Self {
            passes: 10,
            debug: false,
        }
    }
}

/// the sync implementation of the benchmark
pub struct Benchmark(BenchmarkBuilder);

impl BenchmarkBuilder {
    /// construct the builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Amount of times the benchmark closure will be run and measurments are taken.
    pub fn set_passes(&mut self, passes: i32) -> Self {
        self.passes = passes;
        self.to_owned()
    }

    /// Debug output for the benchmark lib
    fn set_debug(&mut self, debug: bool) -> Self {
        self.debug = debug;
        self.to_owned()
    }

    /// Build the benchmark
    pub fn done(&self) -> Benchmark {
        Benchmark(self.to_owned())
    }
}

impl Benchmark {
    /// run the benchmark with the default settings without any construction
    ///
    /// # Example
    ///
    /// ```
    /// use benchmark::sync::Benchmark;
    /// Benchmark::default_run(|| {
    ///    // do something
    /// });
    /// ```
    #[inline]
    pub fn default_run<F>(closure: F) -> BenchmarkResults
    where
        F: Fn(),
    {
        BenchmarkBuilder::default().done().run(closure)
    }

    /// run the benchmark with the builder borrows self
    ///
    /// # Example
    ///
    /// ```
    /// use benchmark::sync::BenchmarkBuilder;
    /// BenchmarkBuilder::default()
    ///    .set_passes(10)
    ///    .done()
    ///    .run(|| {
    ///      // do something
    ///    });
    /// ```
    pub fn run<F>(&self, closure: F) -> BenchmarkResults
    where
        F: Fn(),
    {
        // run the benchmark
        let mut times: Vec<i64> = Vec::new();
        for _ in 0..self.0.passes {
            // start timer
            let timer = std::time::Instant::now();
            closure();
            times.push(timer.elapsed().as_nanos() as i64)
        }

        BenchmarkResults {
            time: times,
            platform: std::env::consts::OS.to_string(),
            additional: None,
        }
    }
}
