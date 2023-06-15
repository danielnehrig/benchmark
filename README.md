# Benchmark RS

__A lib for micro benchmarking 🚀🚀🚀__  
__with support vor various crossplatform measureing methods__  

## Measure

- ⏲️Time (DONE)
- 💾Memory usage (TODO)
- 💻Cpu usage (TODO)
- ⌚RTSC (Clock Cycles) (TODO)

## Features

1. benchmark async and sync code  
2. concurrent execution support with tokio/futures 🏯
3. Dump results to json.
4. graph creation (TODO)
5. Simple API

# Setup
Add to cargo.toml
```toml
benchmark = { git = "https://github.com/danielnehrig/benchmark", features = ["async"] }
```
