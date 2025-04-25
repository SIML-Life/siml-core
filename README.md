# SIML Core

This is the SIML Core library and serves the TCP handshaking between the environment and the agent. This is an alpha release and more will come shortly.

### Scaling Test Results

| Metric                 | 500 Agents  | 1000 Agents | 2500 Agents
|-------------------------|---------| ---------| ---------|
| Baseline memory         | 4 MB    | 5 MB  | 6 MB
| Peak memory             | 6 MB    | 8 MB | 13 MB
| Memory per agent        | ~4 KB   | ~4 KB | ~4 KB
| Peak CPU usage          | 15.62%  |  37.47% | 78.05%
| Idle CPU after connection | 0%   | 0% | 0%

### Estimates on a Mac Studio w/ 512GB Ram and M3 Ultra

| Resource | Estimate | Notes |
|:---------|:---------|:------|
| Baseline memory | 4–6 MB | Initial server memory footprint |
| Memory per agent | ~4 KB | Linear growth, lightweight agents |
| Total available RAM | ~88 GB | (96GB minus system reservations) |
| Maximum agents (by RAM) | ~22 million agents | Memory is not the limiting factor |
| CPU cores | 32 performance cores | Apple M3 Ultra specs |
| Maximum agents (by CPU) | ~6400 agents | CPU becomes bottleneck first |
| Peak CPU usage (500 agents) | ~15.62% | From Intel i9 16-core test |
| Idle CPU usage (after connect) | ~0% | Agents become idle after handshake |


✅ Summary:
On an $5500 Apple M3 Ultra with 92GB RAM, the server could theoretically support around 6400 agents concurrently.
Memory usage is negligible; CPU limits scale first under load.


### Running the Server
```
cargo run --bin siml-core
```
### Generate FBS files
```
flatc --rust -o src/generated fbs/*.fbs
```
### Run Tests
To test the client:
```
cargo run --bin siml-core
---
Then in another tab
---
cargo run --bin client
```
To run all the tests:
```
cargo test
```
To run an individual test, and get memory outputs:
```
cargo test --test scaling_test -- --nocapture
```