mod results;
mod sync;

#[cfg(feature = "async")]
mod default;

// Test sync
#[cfg(test)]
mod sync_tests {
    use super::*;
    use sync::{Benchmark, BenchmarkBuilder};

    #[test]
    fn benchmark_builder() {
        let mut benchmark = BenchmarkBuilder::default();
        let result = benchmark
            .set_passes(50)
            .set_debug(false)
            .done()
            .run(|| {
                println!("Hello world");
            })
            .add_info("goated".into());
        assert_eq!(result.platform, "linux");
    }

    #[test]
    fn benchmark() {
        let result = Benchmark::default_run(|| {
            println!("Hello world");
        })
        .add_info("default".into());
        assert_eq!(result.platform, "linux");
    }
}

// Test async
#[cfg(test)]
mod async_tests {
    use super::*;
    use default::{Benchmark, BenchmarkBuilder};

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
        assert_eq!(result.platform, "linux");
    }

    #[test]
    fn benchmark() {
        let result = Benchmark::default_run(|| async {
            sum(5, 5).await;
        })
        .add_info("goated".into());
        assert_eq!(result.platform, "linux");
    }
}
