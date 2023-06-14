use crate::results::BenchmarkResults;
use std::{future::Future, time::Duration};

/// builder pattern for the benchmarking system
/// the default settings are:
/// - passes: 10
///
/// # Example
///
/// ```
/// use benchmark::prelude::BenchmarkBuilder;
/// BenchmarkBuilder::default()
///   .passes(10)
///   .done();
/// ```
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

/// the benchmark struct
/// # Example
/// ```
/// use benchmark::prelude::Benchmark;
/// Benchmark::default_run(|| async {
///   // do something
/// });
/// ```
pub struct Benchmark(BenchmarkBuilder);

impl BenchmarkBuilder {
    /// create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Amount of times the benchmark closure will be run and measurments are taken.
    pub fn passes(&mut self, passes: i32) -> Self {
        self.passes = passes;
        self.to_owned()
    }

    /// enable debug output for the benchmark lib
    #[allow(dead_code)]
    fn set_debug(&mut self, debug: bool) -> Self {
        self.debug = debug;
        self.to_owned()
    }

    /// finish the builder and return the benchmark
    /// # Example
    /// ```
    /// use benchmark::prelude::BenchmarkBuilder;
    /// BenchmarkBuilder::default().done();
    /// ```
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
    /// use benchmark::prelude::Benchmark;
    /// Benchmark::default_run(|| async {
    ///    // do something
    /// });
    /// ```
    #[inline]
    pub fn default_run<F, Fut>(closure: F) -> BenchmarkResults
    where
        F: Fn() -> Fut,
        Fut: Future<Output = ()>,
    {
        BenchmarkBuilder::default().done().run(closure)
    }

    /// run the benchmark with the builder borrows self
    ///
    /// # Example
    ///
    /// ```
    /// use benchmark::prelude::BenchmarkBuilder;
    /// BenchmarkBuilder::default()
    ///    .passes(10)
    ///    .done()
    ///    .run(|| async {
    ///      // do something
    ///    });
    /// ```
    pub fn run<F, Fut>(&self, closure: F) -> BenchmarkResults
    where
        F: Fn() -> Fut,
        Fut: Future<Output = ()>,
    {
        // run the benchmark
        let mut times: Vec<Duration> = Vec::new();
        for _ in 0..self.0.passes {
            // start timer
            let timer = std::time::Instant::now();
            futures::executor::block_on(closure());
            times.push(timer.elapsed())
        }

        BenchmarkResults {
            time: times,
            platform: std::env::consts::OS.to_string(),
            additional: None,
        }
    }
}
