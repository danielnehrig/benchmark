# Benchmark RS

*A lib for micro benchmarking 🚀🚀🚀*
__with support vor various crossplatform measureing methods such as:__

- ⏲️Time (DONE)
- 💾Memory usage (TODO)
- 💻Cpu usage (TODO)
- ⌚RTSC (Clock Cycles) (TODO)


1. benchmark async and sync code  
2. concurrent execution support with tokio 🏯
3. Dump results to json.
4. graph creation (TODO)

# Setup
Add to cargo.toml
```toml
benchmark = { git = "https://github.com/danielnehrig/benchmark", features = ["async"] }
```
