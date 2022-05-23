```
Gnuplot not found, using plotters backend
bench_div_round_up      time:   [20.300 ns 20.303 ns 20.307 ns]                                
                        change: [-0.7101% -0.6959% -0.6793%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild

bench_round_up          time:   [10.359 ns 10.363 ns 10.368 ns]                            
                        change: [-0.5699% -0.4445% -0.3653%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) low severe

bench_round_up_div      time:   [10.412 ns 10.412 ns 10.413 ns]                                
                        change: [+7.7506% +7.7635% +7.7760%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking bench_100000000_4_div_round_up: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 37.2s, or reduce sample count to 10.
bench_100000000_4_div_round_up                                                                            
                        time:   [371.58 ms 371.62 ms 371.67 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking bench_100000000_4_round_up: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 22.3s, or reduce sample count to 20.
bench_100000000_4_round_up                                                                            
                        time:   [222.84 ms 222.87 ms 222.90 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

Benchmarking bench_100000000_4_round_up_div: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 18.6s, or reduce sample count to 20.
bench_100000000_4_round_up_div                                                                            
                        time:   [185.92 ms 185.93 ms 185.94 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

Benchmarking bench_100000000_0x1000000_div_round_up: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 37.2s, or reduce sample count to 10.
bench_100000000_0x1000000_div_round_up                                                                            
                        time:   [371.43 ms 371.45 ms 371.48 ms]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

Benchmarking bench_100000000_0x1000000_round_up: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 22.3s, or reduce sample count to 20.
bench_100000000_0x1000000_round_up                                                                            
                        time:   [222.87 ms 222.89 ms 222.91 ms]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

Benchmarking bench_100000000_0x1000000_round_up_div: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 22.3s, or reduce sample count to 20.
bench_100000000_0x1000000_round_up_div                                                                            
                        time:   [222.94 ms 222.98 ms 223.02 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
```
