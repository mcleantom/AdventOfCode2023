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
Timer precision: 100 ns
benchmarks        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ real_input                   │               │               │               │         │
│  ├─ day1_part1  77.78 µs      │ 142.8 µs      │ 83.2 µs       │ 88.7 µs       │ 100     │ 10000
│  ├─ day1_part2  959.5 µs      │ 1.085 ms      │ 997.3 µs      │ 1.002 ms      │ 100     │ 10000
│  ├─ day2_part1  73.39 µs      │ 109.3 µs      │ 78.35 µs      │ 81.26 µs      │ 100     │ 10000
│  ├─ day2_part2  72.21 µs      │ 91.55 µs      │ 76.87 µs      │ 77.1 µs       │ 100     │ 10000
│  ├─ day3_part1  571.7 µs      │ 690.2 µs      │ 584.6 µs      │ 598.8 µs      │ 100     │ 10000
│  ├─ day3_part2  685.5 µs      │ 781.4 µs      │ 701.2 µs      │ 707.3 µs      │ 100     │ 10000
│  ├─ day4_part1  371.8 µs      │ 430.5 µs      │ 378.4 µs      │ 381 µs        │ 100     │ 10000
│  ├─ day4_part2  385.9 µs      │ 447.7 µs      │ 396.8 µs      │ 400 µs        │ 100     │ 10000
│  ├─ day5_part1  46.58 µs      │ 64.85 µs      │ 47.82 µs      │ 48.69 µs      │ 100     │ 10000
│  ├─ day5_part2  45.55 µs      │ 60.53 µs      │ 46.91 µs      │ 47.32 µs      │ 100     │ 10000
│  ├─ day6_part1  209.8 ns      │ 307.8 ns      │ 212.7 ns      │ 214.2 ns      │ 100     │ 10000
│  ╰─ day6_part2  212.7 ns      │ 298.8 ns      │ 218.7 ns      │ 220.3 ns      │ 100     │ 10000
╰─ test_input                   │               │               │               │         │
   ├─ day1_part1  197.7 ns      │ 397.8 ns      │ 199.7 ns      │ 203.3 ns      │ 100     │ 10000
   ├─ day1_part2  5.28 µs       │ 9.791 µs      │ 5.56 µs       │ 5.887 µs      │ 100     │ 10000
   ├─ day2_part1  2.469 µs      │ 4.152 µs      │ 2.487 µs      │ 2.727 µs      │ 100     │ 10000
   ├─ day2_part2  1.69 µs       │ 3.457 µs      │ 1.708 µs      │ 1.951 µs      │ 100     │ 10000
   ├─ day3_part1  1.208 µs      │ 3.376 µs      │ 1.226 µs      │ 1.409 µs      │ 100     │ 10000
   ├─ day3_part2  1.72 µs       │ 4.102 µs      │ 1.841 µs      │ 2.071 µs      │ 100     │ 10000
   ├─ day4_part1  4.719 µs      │ 8.707 µs      │ 4.829 µs      │ 5.091 µs      │ 100     │ 10000
   ├─ day4_part2  5.094 µs      │ 7.736 µs      │ 5.425 µs      │ 5.645 µs      │ 100     │ 10000
   ├─ day5_part1  3.198 µs      │ 6.123 µs      │ 3.23 µs       │ 3.389 µs      │ 100     │ 10000
   ├─ day5_part2  1.965 µs      │ 3.451 µs      │ 1.976 µs      │ 2.104 µs      │ 100     │ 10000
   ├─ day6_part1  155.7 ns      │ 255.8 ns      │ 158.7 ns      │ 159.9 ns      │ 100     │ 10000
   ╰─ day6_part2  162.7 ns      │ 262.8 ns      │ 165.8 ns      │ 168.2 ns      │ 100     │ 10000
```