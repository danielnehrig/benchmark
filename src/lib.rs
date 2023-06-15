pub mod results;

pub mod engine;
pub mod measurements;

pub mod sys;

#[cfg(windows)]
pub mod windows;

pub mod prelude {
    pub use crate::engine::{Benchmark, BenchmarkBuilder};
    pub use crate::results::BenchmarkResults;
}

// Test sync
#[cfg(not(feature = "async"))]
#[cfg(test)]
mod bench_sync {
    use super::*;
    use engine::{Benchmark, BenchmarkBuilder};

    #[test]
    fn benchmark_builder() {
        let mut benchmark = BenchmarkBuilder::default();
        let result = benchmark
            .passes(50)
            .done()
            .run(|| {
                println!("Hello world");
            })
            .add_info("goated".into());
        assert_eq!(result.platform, std::env::consts::OS);
        result.save("sync");
    }

    #[test]
    fn benchmark() {
        let result = Benchmark::default_run(|| {
            println!("Hello world");
        })
        .add_info("default".into());
        assert_eq!(result.platform, std::env::consts::OS);
    }
}

// Test async
#[cfg(feature = "async")]
#[cfg(test)]
mod bench_async {
    use super::*;
    use engine::{Benchmark, BenchmarkBuilder};

    async fn sum(x: i32, y: i32) -> i32 {
        x + y
    }

    #[test]
    fn benchmark_builder() {
        let result = BenchmarkBuilder::default()
            .done()
            .run(|| async {
                sum(5, 5).await;
            })
            .add_info("goated".into());
        assert_eq!(result.platform, std::env::consts::OS);
        result.save("async");
    }

    #[test]
    fn benchmark() {
        let result = Benchmark::default_run(|| async {
            sum(5, 5).await;
        })
        .add_info("goated".into());
        assert_eq!(result.platform, std::env::consts::OS);
    }
}
