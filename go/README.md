### Run Benchmark

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
