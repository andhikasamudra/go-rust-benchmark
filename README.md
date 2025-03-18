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

```
Use Case Worker Pooling Process
Running 30s test @ http://localhost:8080/compress 12 threads and 400 connections

Lang    Thread Stats   Avg      Stdev     Max   +/- Stdev
Rust    Latency     4.62ms    7.27ms 174.15ms   95.75%
Go      Latency     2.81ms    1.53ms  49.23ms   97.21%
Rust    Req/Sec     9.02k     1.06k   27.97k    79.82%
Go      Req/Sec    12.14k     1.37k   14.61k    82.33%

Rust   Requests/sec: 107490.15 Transfer/sec:     15.07MB 3233804 requests in 30.08s, 453.24MB read
Go     Requests/sec: 144942.61 Transfer/sec:     20.32MB 4348852 requests in 30.00s, 609.55MB read


Use Case High CPU processing (Image Compressing)
Running 30s test @ http://localhost:8080/compress
  12 threads and 400 connections

Lang    Thread Stats   Avg      Stdev     Max   +/- Stdev
Rust    Latency     8.96ms   17.13ms 313.86ms   89.92%
Go      Latency     5.79ms    3.55ms 167.21ms   88.31%
Rust    Req/Sec     10.52k     3.10k   31.16k    76.59%
Go      Req/Sec     5.92k   539.53    12.42k    84.07%

Rust   Requests/sec: 125036.84 Transfer/sec:      9.78MB 3764007 requests in 30.10s, 294.35MB read
Go     Requests/sec:  70571.25 Transfer/sec:     12.11MB 2123985 requests in 30.10s, 364.61MB read
```
