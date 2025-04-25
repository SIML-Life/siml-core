# SIML Core

This is the SIML Core library and serves the TCP handshaking between the environment and the agent. This is an alpha release and more will come shortly.

### Scaling Test Results (500 Agents)

| Metric                 | 500 Agents  | 1000 Agents | 2500 Agents
|-------------------------|---------| ---------| ---------|
| Baseline memory         | 4 MB    | 5 MB  | 6 MB
| Peak memory             | 6 MB    | 8 MB | 13 MB
| Memory per agent        | ~4 KB   | ~4 KB | ~4 KB
| Peak CPU usage          | 15.62%  |  37.47% | 78.05%
| Idle CPU after connection | 0%   | 0% | 0%


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