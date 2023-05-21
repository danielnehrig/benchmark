# Benchmark RS

A lib for basic benchmarking dump your results to Json
you probably should much rather be using criterion

# Setup
Add to cargo.toml
```toml
benchmark = { git = "https://github.com/danielnehrig/benchmark", features = ["async"] }
```

# Example

## Sync
```rust
use benchmark::sync::{BenchmarkBuilder};
let mut benchmark = BenchmarkBuilder::default();
let result = benchmark
    .passes(50)
    .done()
    .run(|| {
        println!("Hello world");
    })
    .add_info("goated".into());
```

## Async
```rust
use benchmark::default::{BenchmarkBuilder};
let mut benchmark = BenchmarkBuilder::default();
let result = benchmark
    .passes(50)
    .done()
    .run(|| async {
        println!("Hello world");
    })
    .add_info("goated".into()).save("some_hello_world_test");
```
