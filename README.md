# Advent of Code - 2023

RUST based solutions for [Advent of Code 2023](https://adventofcode.com/2023).

Run all days using

```bash
cargo run
```

Run benchmark using

```bash
cargo bench
```

## Performance

```bash
Timer precision: 12 ns
benchmarks              fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ real_input                         │               │               │               │         │
│  ├─ day1_part1        51.13 µs      │ 59 µs         │ 52.76 µs      │ 52.85 µs      │ 100     │ 10000
│  ├─ day1_part2        749.5 µs      │ 809.1 µs      │ 759.9 µs      │ 761.5 µs      │ 100     │ 10000
│  ├─ day2_part1        50.79 µs      │ 64.62 µs      │ 52.19 µs      │ 52.29 µs      │ 100     │ 10000
│  ├─ day2_part2        49.08 µs      │ 54.17 µs      │ 51.15 µs      │ 51.19 µs      │ 100     │ 10000
│  ├─ day3_part1        533.2 µs      │ 554.7 µs      │ 535.2 µs      │ 537 µs        │ 100     │ 10000
│  ├─ day3_part2        602.4 µs      │ 668.5 µs      │ 608.8 µs      │ 614 µs        │ 100     │ 10000
│  ├─ day4_part1        262.7 µs      │ 282.1 µs      │ 269.1 µs      │ 269 µs        │ 100     │ 10000
│  ├─ day4_part2        274.7 µs      │ 327.9 µs      │ 280.5 µs      │ 281.1 µs      │ 100     │ 10000
│  ├─ day5_part1        32.37 µs      │ 35.76 µs      │ 33.38 µs      │ 33.5 µs       │ 100     │ 10000
│  ├─ day5_part2        29.5 µs       │ 41.38 µs      │ 30.45 µs      │ 30.68 µs      │ 100     │ 10000
│  ├─ day6_part1        158.2 ns      │ 229 ns        │ 166 ns        │ 169.4 ns      │ 100     │ 10000
│  ├─ day6_part2        165.5 ns      │ 230.9 ns      │ 171.8 ns      │ 174.7 ns      │ 100     │ 10000
│  ├─ day7_part1        241.7 µs      │ 263.7 µs      │ 244.8 µs      │ 245.9 µs      │ 100     │ 10000
│  ├─ day7_part2        317.3 µs      │ 425.5 µs      │ 325.9 µs      │ 327.2 µs      │ 100     │ 10000
│  ├─ day8_part1        274.8 µs      │ 295.3 µs      │ 279.8 µs      │ 280.2 µs      │ 100     │ 10000
│  ╰─ day8_part2        3.043 ms      │ 3.246 ms      │ 3.095 ms      │ 3.097 ms      │ 100     │ 10000
╰─ test_input                         │               │               │               │         │
   ├─ day1_part1        135.4 ns      │ 191.3 ns      │ 143.4 ns      │ 146.9 ns      │ 100     │ 10000
   ├─ day1_part2        3.927 µs      │ 5.745 µs      │ 4.174 µs      │ 4.296 µs      │ 100     │ 10000
   ├─ day2_part1        1.564 µs      │ 1.752 µs      │ 1.608 µs      │ 1.624 µs      │ 100     │ 10000
   ├─ day2_part2        1.285 µs      │ 1.611 µs      │ 1.333 µs      │ 1.35 µs       │ 100     │ 10000
   ├─ day3_part1        875.2 ns      │ 1.061 µs      │ 920.8 ns      │ 917.1 ns      │ 100     │ 10000
   ├─ day3_part2        1.192 µs      │ 1.417 µs      │ 1.209 µs      │ 1.235 µs      │ 100     │ 10000
   ├─ day4_part1        3.383 µs      │ 4.948 µs      │ 3.534 µs      │ 3.669 µs      │ 100     │ 10000
   ├─ day4_part2        3.451 µs      │ 6.233 µs      │ 3.728 µs      │ 3.929 µs      │ 100     │ 10000
   ├─ day5_part1        2.226 µs      │ 3.293 µs      │ 2.324 µs      │ 2.367 µs      │ 100     │ 10000
   ├─ day5_part2        1.356 µs      │ 2.224 µs      │ 1.406 µs      │ 1.455 µs      │ 100     │ 10000
   ├─ day6_part1        116.4 ns      │ 135.6 ns      │ 119.3 ns      │ 119.3 ns      │ 100     │ 10000
   ├─ day6_part2        122 ns        │ 167.5 ns      │ 124.2 ns      │ 126.6 ns      │ 100     │ 10000
   ├─ day7_part1        316 ns        │ 371.1 ns      │ 317.4 ns      │ 320.9 ns      │ 100     │ 10000
   ├─ day7_part2        730.2 ns      │ 947.7 ns      │ 757.1 ns      │ 768.8 ns      │ 100     │ 10000
   ├─ day8_part1_test1  732.6 ns      │ 812.4 ns      │ 748.6 ns      │ 754.1 ns      │ 100     │ 10000
   ├─ day8_part1_test2  412.2 ns      │ 721 ns        │ 425.8 ns      │ 433.2 ns      │ 100     │ 10000
   ╰─ day8_part2        1.489 µs      │ 2.222 µs      │ 1.548 µs      │ 1.609 µs      │ 100     │ 10000
```