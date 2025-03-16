## Hardware Overview

```
Model Name: MacBook Air
Model Identifier: MacBookAir10,1
Model Number: MGN93ID/A
Chip: Apple M1
Total Number of Cores: 8 (4 performance and 4 efficiency)
Memory: 8 GB
System Firmware Version: 11881.81.4
OS Loader Version: 11881.81.4
Serial Number (system): HXJM31NP1WFY
Hardware UUID: 552CF088-B7D7-5E32-893A-631C7989F070
Provisioning UDID: 00008103-000E21DE3A61A01E
Activation Lock Status: Enabled
```

## Benchmark Result

### Go

```
wrk -t12 -c400 -d30s http://localhost:8080/compress

Running 30s test @ http://localhost:8080/compress
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.79ms    3.55ms 167.21ms   88.31%
    Req/Sec     5.92k   539.53    12.42k    84.07%
  2123985 requests in 30.10s, 364.61MB read
  Socket errors: connect 0, read 383, write 0, timeout 0
  Non-2xx or 3xx responses: 2123985
Requests/sec:  70571.25
Transfer/sec:     12.11MB

```

### Rust

```
wrk -t12 -c400 -d30s http://localhost:8080/compress

Running 30s test @ http://localhost:8080/compress
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.96ms   17.13ms 313.86ms   89.92%
    Req/Sec    10.52k     3.10k   31.16k    76.59%
  3764007 requests in 30.10s, 294.35MB read
  Socket errors: connect 0, read 362, write 0, timeout 0
  Non-2xx or 3xx responses: 3764007
Requests/sec: 125036.84
Transfer/sec:      9.78MB
```
