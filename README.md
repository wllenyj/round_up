```
Gnuplot not found, using plotters backend
bench_div_round_up      time:   [20.329 ns 20.330 ns 20.332 ns]                                
                        change: [+0.1294% +0.1475% +0.1663%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  2 (2.00%) high severe

bench_round_up          time:   [10.335 ns 10.339 ns 10.343 ns]                            
                        change: [-0.3249% -0.2225% -0.1045%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild

bench_round_up_div      time:   [10.410 ns 10.411 ns 10.412 ns]                                
                        change: [-0.0587% -0.0455% -0.0325%] (p = 0.00 < 0.05)
                        Change within noise threshold.

Benchmarking bench_100000000_4_div_round_up: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 37.2s, or reduce sample count to 10.
bench_100000000_4_div_round_up                                                                            
                        time:   [371.60 ms 371.64 ms 371.71 ms]
                        change: [-0.0134% +0.0055% +0.0264%] (p = 0.59 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) low mild
  1 (1.00%) high severe

Benchmarking bench_100000000_4_round_up: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 22.3s, or reduce sample count to 20.
bench_100000000_4_round_up                                                                            
                        time:   [222.95 ms 222.96 ms 222.97 ms]
                        change: [+0.0241% +0.0428% +0.0557%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking bench_100000000_4_round_up_div: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 18.6s, or reduce sample count to 20.
bench_100000000_4_round_up_div                                                                            
                        time:   [185.78 ms 185.78 ms 185.79 ms]
                        change: [-0.0833% -0.0769% -0.0708%] (p = 0.00 < 0.05)
                        Change within noise threshold.

Benchmarking bench_100000000_0x1000000_div_round_up: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 37.2s, or reduce sample count to 10.
bench_100000000_0x1000000_div_round_up                                                                            
                        time:   [371.62 ms 371.64 ms 371.67 ms]
                        change: [+0.0401% +0.0501% +0.0603%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

Benchmarking bench_100000000_0x1000000_round_up: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 22.3s, or reduce sample count to 20.
bench_100000000_0x1000000_round_up                                                                            
                        time:   [222.96 ms 222.98 ms 223.00 ms]
                        change: [+0.0283% +0.0412% +0.0546%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

Benchmarking bench_100000000_0x1000000_round_up_div: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 22.3s, or reduce sample count to 20.
bench_100000000_0x1000000_round_up_div                                                                            
                        time:   [223.01 ms 223.02 ms 223.03 ms]
                        change: [-0.0024% +0.0204% +0.0379%] (p = 0.04 < 0.05)
                        Change within noise threshold.
```
