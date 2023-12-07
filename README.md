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
benchmarks        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ real_input                   │               │               │               │         │
│  ├─ day1_part1  52.58 µs      │ 70.01 µs      │ 55.75 µs      │ 56.16 µs      │ 100     │ 10000
│  ├─ day1_part2  818.1 µs      │ 1.02 ms       │ 839.7 µs      │ 844.9 µs      │ 100     │ 10000
│  ├─ day2_part1  49.95 µs      │ 52.02 µs      │ 50.82 µs      │ 50.9 µs       │ 100     │ 10000
│  ├─ day2_part2  50.58 µs      │ 53.14 µs      │ 51.36 µs      │ 51.44 µs      │ 100     │ 10000
│  ├─ day3_part1  546.9 µs      │ 709 µs        │ 555.3 µs      │ 558.9 µs      │ 100     │ 10000
│  ├─ day3_part2  632.8 µs      │ 745.2 µs      │ 642.4 µs      │ 645.6 µs      │ 100     │ 10000
│  ├─ day4_part1  267.9 µs      │ 288.2 µs      │ 270.2 µs      │ 270.6 µs      │ 100     │ 10000
│  ├─ day4_part2  279.7 µs      │ 288.2 µs      │ 282.2 µs      │ 282.3 µs      │ 100     │ 10000
│  ├─ day5_part1  33.75 µs      │ 36.16 µs      │ 34.56 µs      │ 34.61 µs      │ 100     │ 10000
│  ├─ day5_part2  32.31 µs      │ 34.84 µs      │ 33.64 µs      │ 33.65 µs      │ 100     │ 10000
│  ├─ day6_part1  162.4 ns      │ 234.3 ns      │ 164.2 ns      │ 166.8 ns      │ 100     │ 10000
│  ├─ day6_part2  162.9 ns      │ 186.5 ns      │ 166 ns        │ 168.3 ns      │ 100     │ 10000
│  ├─ day7_part1  235.7 µs      │ 250.3 µs      │ 239 µs        │ 239 µs        │ 100     │ 10000
│  ╰─ day7_part2  326.6 µs      │ 334.3 µs      │ 328.5 µs      │ 328.6 µs      │ 100     │ 10000
╰─ test_input                   │               │               │               │         │
   ├─ day1_part1  145.7 ns      │ 205.4 ns      │ 150 ns        │ 150.7 ns      │ 100     │ 10000
   ├─ day1_part2  4.486 µs      │ 4.893 µs      │ 4.657 µs      │ 4.656 µs      │ 100     │ 10000
   ├─ day2_part1  1.563 µs      │ 1.781 µs      │ 1.683 µs      │ 1.677 µs      │ 100     │ 10000
   ├─ day2_part2  1.239 µs      │ 1.379 µs      │ 1.299 µs      │ 1.297 µs      │ 100     │ 10000
   ├─ day3_part1  906.1 ns      │ 1.266 µs      │ 917.6 ns      │ 924.2 ns      │ 100     │ 10000
   ├─ day3_part2  1.199 µs      │ 1.349 µs      │ 1.23 µs       │ 1.236 µs      │ 100     │ 10000
   ├─ day4_part1  3.397 µs      │ 3.7 µs        │ 3.496 µs      │ 3.502 µs      │ 100     │ 10000
   ├─ day4_part2  3.692 µs      │ 4.055 µs      │ 3.806 µs      │ 3.796 µs      │ 100     │ 10000
   ├─ day5_part1  2.165 µs      │ 2.391 µs      │ 2.232 µs      │ 2.237 µs      │ 100     │ 10000
   ├─ day5_part2  1.453 µs      │ 1.733 µs      │ 1.508 µs      │ 1.518 µs      │ 100     │ 10000
   ├─ day6_part1  115.3 ns      │ 178.3 ns      │ 117 ns        │ 118.1 ns      │ 100     │ 10000
   ├─ day6_part2  119.1 ns      │ 171.3 ns      │ 121.8 ns      │ 122.9 ns      │ 100     │ 10000
   ├─ day7_part1  302.1 ns      │ 348 ns        │ 310.7 ns      │ 312.1 ns      │ 100     │ 10000
   ╰─ day7_part2  747.8 ns      │ 863.5 ns      │ 772.2 ns      │ 776.8 ns      │ 100     │ 10000
```