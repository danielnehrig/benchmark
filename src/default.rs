use crate::results::BenchmarkResults;
use std::future::Future;

// builder pattern
#[derive(Clone, Debug)]
pub struct BenchmarkBuilder {
    pub passes: i32,
    pub debug: bool,
}

impl Default for BenchmarkBuilder {
    fn default() -> Self {
        Self {
            passes: 10,
            debug: false,
        }
    }
}

pub struct Benchmark(BenchmarkBuilder);

impl BenchmarkBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Amount of times the benchmark closure will be run and measurments are taken.
    pub fn set_passes(&mut self, passes: i32) -> Self {
        self.passes = passes;
        self.to_owned()
    }

    pub fn set_debug(&mut self, debug: bool) -> Self {
        self.debug = debug;
        self.to_owned()
    }

    pub fn done(&self) -> Benchmark {
        Benchmark(self.to_owned())
    }
}

impl Benchmark {
    #[inline]
    pub fn default_run<F, Fut>(closure: F) -> BenchmarkResults
    where
        F: Fn() -> Fut,
        Fut: Future<Output = ()>,
    {
        BenchmarkBuilder::default().done().run(closure)
    }

    pub fn run<F, Fut>(&self, closure: F) -> BenchmarkResults
    where
        F: Fn() -> Fut,
        Fut: Future<Output = ()>,
    {
        // run the benchmark
        let mut times: Vec<i64> = Vec::new();
        for _ in 0..self.0.passes {
            // start timer
            let timer = std::time::Instant::now();
            futures::executor::block_on(closure());
            times.push(timer.elapsed().as_nanos() as i64)
        }

        BenchmarkResults {
            time: times,
            platform: std::env::consts::OS.to_string(),
            additional: None,
        }
    }
}
