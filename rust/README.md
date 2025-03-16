## Run Benchmark

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
