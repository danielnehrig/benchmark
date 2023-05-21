use std::time::Duration;

use crate::results::BenchmarkResults;

/// builder pattern for the benchmarking system
/// the default settings are:
/// - passes: 10
///
/// # Example
///
/// ```
/// use benchmark::sync::BenchmarkBuilder;
/// BenchmarkBuilder::default()
///   .passes(100)
///   .concurrent(false)
///   .done();
/// ```
#[derive(Clone, Debug)]
pub struct BenchmarkBuilder {
    pub passes: i32,
    debug: bool,
    concurrent: bool,
}

impl Default for BenchmarkBuilder {
    fn default() -> Self {
        Self {
            passes: 10,
            debug: false,
            concurrent: false,
        }
    }
}

/// the benchmark struct
/// # Example
/// ```
/// use benchmark::sync::Benchmark;
/// Benchmark::default_run(|| {
///   // do something
/// });
/// ```
pub struct Benchmark(BenchmarkBuilder);

impl BenchmarkBuilder {
    /// construct the builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Amount of times the benchmark closure will be run and measurments are taken.
    pub fn passes(&mut self, passes: i32) -> Self {
        self.passes = passes;
        self.to_owned()
    }

    /// run passes concurrently
    #[allow(dead_code)]
    pub fn concurrent(&mut self, is: bool) -> Self {
        self.concurrent = is;
        self.to_owned()
    }

    /// Debug output for the benchmark lib
    #[allow(dead_code)]
    fn debug(&mut self, debug: bool) -> Self {
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
    ///    .passes(10)
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
        let mut times: Vec<Duration> = Vec::new();
        for _ in 0..self.0.passes {
            // start timer
            let timer = std::time::Instant::now();
            closure();
            times.push(timer.elapsed())
        }

        BenchmarkResults {
            time: times,
            platform: std::env::consts::OS.to_string(),
            additional: None,
        }
    }
}
