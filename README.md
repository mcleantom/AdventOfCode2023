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
benchmarks              fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ real_input                         │               │               │               │         │
│  ├─ day1_part1        52.14 µs      │ 57.91 µs      │ 54.45 µs      │ 54.51 µs      │ 100     │ 10000
│  ├─ day1_part2        799.9 µs      │ 1.372 ms      │ 1.345 ms      │ 1.152 ms      │ 100     │ 10000
│  ├─ day2_part1        89.54 µs      │ 92.34 µs      │ 89.91 µs      │ 89.96 µs      │ 100     │ 10000
│  ├─ day2_part2        87.2 µs       │ 89.52 µs      │ 87.85 µs      │ 87.93 µs      │ 100     │ 10000
│  ├─ day3_part1        876 µs        │ 882.6 µs      │ 878.8 µs      │ 878.8 µs      │ 100     │ 10000
│  ├─ day3_part2        963.5 µs      │ 986.4 µs      │ 966.2 µs      │ 967.4 µs      │ 100     │ 10000
│  ├─ day4_part1        465.3 µs      │ 487 µs        │ 471.7 µs      │ 471 µs        │ 100     │ 10000
│  ├─ day4_part2        479.4 µs      │ 544.7 µs      │ 505 µs        │ 508.5 µs      │ 100     │ 10000
│  ├─ day5_part1        56.15 µs      │ 61.49 µs      │ 56.34 µs      │ 56.51 µs      │ 100     │ 10000
│  ├─ day5_part2        53.55 µs      │ 57.2 µs       │ 53.9 µs       │ 54.14 µs      │ 100     │ 10000
│  ├─ day6_part1        301 ns        │ 401.8 ns      │ 301 ns        │ 302.1 ns      │ 100     │ 10000
│  ├─ day6_part2        294.4 ns      │ 325.5 ns      │ 294.5 ns      │ 294.9 ns      │ 100     │ 10000
│  ├─ day7_part1        350.9 µs      │ 369 µs        │ 352.4 µs      │ 352.9 µs      │ 100     │ 10000
│  ├─ day7_part2        503.5 µs      │ 512.5 µs      │ 504.3 µs      │ 504.8 µs      │ 100     │ 10000
│  ├─ day8_part1        479.7 µs      │ 520.8 µs      │ 502.4 µs      │ 500.9 µs      │ 100     │ 10000
│  ├─ day8_part2        4.828 ms      │ 5.173 ms      │ 5 ms          │ 4.982 ms      │ 100     │ 10000
│  ├─ day9_part1        154.2 µs      │ 158 µs        │ 154.6 µs      │ 155.6 µs      │ 100     │ 10000
│  ╰─ day9_part2        153.5 µs      │ 158.3 µs      │ 154.5 µs      │ 154.5 µs      │ 100     │ 10000
╰─ test_input                         │               │               │               │         │
   ├─ day1_part1        258 ns        │ 380.2 ns      │ 258.1 ns      │ 260 ns        │ 100     │ 10000
   ├─ day1_part2        6.642 µs      │ 8.359 µs      │ 7.81 µs       │ 7.808 µs      │ 100     │ 10000
   ├─ day2_part1        3.023 µs      │ 3.448 µs      │ 3.067 µs      │ 3.106 µs      │ 100     │ 10000
   ├─ day2_part2        2.27 µs       │ 2.659 µs      │ 2.282 µs      │ 2.323 µs      │ 100     │ 10000
   ├─ day3_part1        1.519 µs      │ 1.775 µs      │ 1.527 µs      │ 1.551 µs      │ 100     │ 10000
   ├─ day3_part2        2.035 µs      │ 2.388 µs      │ 2.057 µs      │ 2.093 µs      │ 100     │ 10000
   ├─ day4_part1        5.719 µs      │ 6.364 µs      │ 5.787 µs      │ 5.83 µs       │ 100     │ 10000
   ├─ day4_part2        5.955 µs      │ 6.768 µs      │ 6.076 µs      │ 6.13 µs       │ 100     │ 10000
   ├─ day5_part1        3.772 µs      │ 4.257 µs      │ 3.815 µs      │ 3.88 µs       │ 100     │ 10000
   ├─ day5_part2        2.253 µs      │ 2.68 µs       │ 2.261 µs      │ 2.326 µs      │ 100     │ 10000
   ├─ day6_part1        204.6 ns      │ 234.6 ns      │ 205.1 ns      │ 205.5 ns      │ 100     │ 10000
   ├─ day6_part2        215.4 ns      │ 303.8 ns      │ 218.5 ns      │ 224.5 ns      │ 100     │ 10000
   ├─ day7_part1        543.6 ns      │ 701.5 ns      │ 548.8 ns      │ 570.9 ns      │ 100     │ 10000
   ├─ day7_part2        1.302 µs      │ 1.488 µs      │ 1.319 µs      │ 1.335 µs      │ 100     │ 10000
   ├─ day8_part1_test1  1.242 µs      │ 1.45 µs       │ 1.258 µs      │ 1.271 µs      │ 100     │ 10000
   ├─ day8_part1_test2  687.4 ns      │ 844.4 ns      │ 702.9 ns      │ 715.1 ns      │ 100     │ 10000
   ├─ day8_part2        2.551 µs      │ 3.04 µs       │ 2.583 µs      │ 2.622 µs      │ 100     │ 10000
   ├─ day9_part1        585.3 ns      │ 681.2 ns      │ 611 ns        │ 613.3 ns      │ 100     │ 10000
   ╰─ day9_part2        615 ns        │ 658.5 ns      │ 617.9 ns      │ 621.7 ns      │ 100     │ 10000
```