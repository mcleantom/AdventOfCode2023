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
Timer precision: 11 ns
benchmarks               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ real_input                          │               │               │               │         │
│  ├─ day01_part1        54.72 µs      │ 59.65 µs      │ 57.78 µs      │ 57.53 µs      │ 100     │ 10000
│  ├─ day01_part2        734.3 µs      │ 808.4 µs      │ 744 µs        │ 746.4 µs      │ 100     │ 10000
│  ├─ day02_part1        47.49 µs      │ 51.41 µs      │ 48.99 µs      │ 49.03 µs      │ 100     │ 10000
│  ├─ day02_part2        47.08 µs      │ 49.98 µs      │ 48.54 µs      │ 48.56 µs      │ 100     │ 10000
│  ├─ day03_part1        529.1 µs      │ 558.6 µs      │ 530.3 µs      │ 532 µs        │ 100     │ 10000
│  ├─ day03_part2        600.4 µs      │ 609.9 µs      │ 601.9 µs      │ 602.3 µs      │ 100     │ 10000
│  ├─ day04_part1        255.1 µs      │ 271 µs        │ 266.2 µs      │ 266.3 µs      │ 100     │ 10000
│  ├─ day04_part2        270.4 µs      │ 277.1 µs      │ 274.4 µs      │ 274.2 µs      │ 100     │ 10000
│  ├─ day05_part1        32.87 µs      │ 35.69 µs      │ 34.28 µs      │ 34.22 µs      │ 100     │ 10000
│  ├─ day05_part2        26.54 µs      │ 29.46 µs      │ 27.86 µs      │ 27.77 µs      │ 100     │ 10000
│  ├─ day06_part1        166.1 ns      │ 524.9 ns      │ 172.4 ns      │ 175.6 ns      │ 100     │ 10000
│  ├─ day06_part2        165.9 ns      │ 212.2 ns      │ 167.8 ns      │ 169.3 ns      │ 100     │ 10000
│  ├─ day07_part1        229.7 µs      │ 238.8 µs      │ 231.2 µs      │ 231.3 µs      │ 100     │ 10000
│  ├─ day07_part2        313.9 µs      │ 332.4 µs      │ 321.5 µs      │ 321.3 µs      │ 100     │ 10000
│  ├─ day08_part1        283.8 µs      │ 353.8 µs      │ 293.3 µs      │ 294.4 µs      │ 100     │ 10000
│  ├─ day08_part2        2.95 ms       │ 3.565 ms      │ 3.001 ms      │ 3.034 ms      │ 100     │ 10000
│  ├─ day09_part1        94.57 µs      │ 99.69 µs      │ 96.86 µs      │ 97.04 µs      │ 100     │ 10000
│  ├─ day09_part2        92.76 µs      │ 98.1 µs       │ 95.99 µs      │ 95.83 µs      │ 100     │ 10000
│  ├─ day11_part1        3.464 ms      │ 5.546 ms      │ 4.664 ms      │ 4.231 ms      │ 100     │ 10000
│  ╰─ day11_part2        4.666 ms      │ 4.983 ms      │ 4.673 ms      │ 4.783 ms      │ 100     │ 10000
╰─ test_input                          │               │               │               │         │
   ├─ day01_part1        270.3 ns      │ 296.7 ns      │ 270.5 ns      │ 271 ns        │ 100     │ 10000
   ├─ day01_part2        6.3 µs        │ 6.653 µs      │ 6.383 µs      │ 6.401 µs      │ 100     │ 10000
   ├─ day02_part1        2.943 µs      │ 3.056 µs      │ 2.965 µs      │ 2.969 µs      │ 100     │ 10000
   ├─ day02_part2        2.285 µs      │ 2.54 µs       │ 2.292 µs      │ 2.306 µs      │ 100     │ 10000
   ├─ day03_part1        1.519 µs      │ 3.115 µs      │ 1.521 µs      │ 1.56 µs       │ 100     │ 10000
   ├─ day03_part2        2.107 µs      │ 2.229 µs      │ 2.127 µs      │ 2.127 µs      │ 100     │ 10000
   ├─ day04_part1        5.582 µs      │ 7.158 µs      │ 5.619 µs      │ 5.64 µs       │ 100     │ 10000
   ├─ day04_part2        5.951 µs      │ 6.29 µs       │ 6.028 µs      │ 6.029 µs      │ 100     │ 10000
   ├─ day05_part1        3.666 µs      │ 3.809 µs      │ 3.676 µs      │ 3.683 µs      │ 100     │ 10000
   ├─ day05_part2        2.18 µs       │ 2.311 µs      │ 2.196 µs      │ 2.199 µs      │ 100     │ 10000
   ├─ day06_part1        205.7 ns      │ 261.8 ns      │ 206 ns        │ 206.8 ns      │ 100     │ 10000
   ├─ day06_part2        216 ns        │ 225 ns        │ 218.8 ns      │ 218.8 ns      │ 100     │ 10000
   ├─ day07_part1        552 ns        │ 587.4 ns      │ 552.8 ns      │ 553.8 ns      │ 100     │ 10000
   ├─ day07_part2        1.241 µs      │ 1.37 µs       │ 1.253 µs      │ 1.259 µs      │ 100     │ 10000
   ├─ day08_part1_test1  1.264 µs      │ 1.412 µs      │ 1.269 µs      │ 1.274 µs      │ 100     │ 10000
   ├─ day08_part1_test2  686.5 ns      │ 735.1 ns      │ 688.6 ns      │ 690.2 ns      │ 100     │ 10000
   ├─ day08_part2        2.545 µs      │ 2.983 µs      │ 2.562 µs      │ 2.583 µs      │ 100     │ 10000
   ├─ day09_part1        607.5 ns      │ 645.1 ns      │ 610.9 ns      │ 611.4 ns      │ 100     │ 10000
   ├─ day09_part2        602.5 ns      │ 629.6 ns      │ 604.2 ns      │ 604.4 ns      │ 100     │ 10000
   ├─ day11_part1        2.868 µs      │ 3.003 µs      │ 2.88 µs       │ 2.885 µs      │ 100     │ 10000
   ╰─ day11_part2        2.827 µs      │ 2.958 µs      │ 2.841 µs      │ 2.854 µs      │ 100     │ 10000
```