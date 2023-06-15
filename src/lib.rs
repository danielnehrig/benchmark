pub mod results;

pub mod engine;

// Test sync
#[cfg(not(feature = "async"))]
#[cfg(test)]
mod sync_tests {
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
mod async_tests {
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
