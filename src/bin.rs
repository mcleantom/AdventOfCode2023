#![allow(dead_code)]
#![allow(unused_variables)]

use lib::days::*;

const INPUT_D1_PART_1_TEST: &str = include_str!("../res/day1_part1_test.txt");
// const INPUT_D1_PART_2_TEST: &str = include_str!("../res/day1_part2_test.txt");
const INPUT_D2_TEST: &str = include_str!("../res/day2_test.txt");
const INPUT_D3_TEST: &str = include_str!("../res/day3_test.txt");
const INPUT_D4_TEST: &str = include_str!("../res/day4_test.txt");
const INPUT_D5_TEST: &str = include_str!("../res/day5_test.txt");
const INPUT_D6_TEST: &str = include_str!("../res/day6_test.txt");

// const INPUT_D1: &str = include_str!("../res/day1.txt");
const INPUT_D2: &str = include_str!("../res/day2.txt");
const INPUT_D3: &str = include_str!("../res/day3.txt");
const INPUT_D4: &str = include_str!("../res/day4.txt");
const INPUT_D5: &str = include_str!("../res/day5.txt");
const INPUT_D6: &str = include_str!("../res/day6.txt");

fn main() {
    assert_eq!(day1::part1(INPUT_D1_PART_1_TEST), 142);
    // assert_eq!(day1::part1(INPUT_D1), 55538);
    // assert_eq!(day1::part2(INPUT_D1_PART_2_TEST), 281);
    // assert_eq!(day1::part2(INPUT_D1), 54875);

    assert_eq!(day2::part1(INPUT_D2_TEST), 8);
    assert_eq!(day2::part1(INPUT_D2), 2207);
    assert_eq!(day2::part2(INPUT_D2_TEST), 2286);
    assert_eq!(day2::part2(INPUT_D2), 62241);

    assert_eq!(day3::part1(INPUT_D3_TEST), 4361);
    assert_eq!(day3::part1(INPUT_D3), 532331);
    assert_eq!(day3::part2(INPUT_D3_TEST), 467835);
    assert_eq!(day3::part2(INPUT_D3), 82301120);

    assert_eq!(day4::part1(INPUT_D4_TEST), 13);
    assert_eq!(day4::part1(INPUT_D4), 21485);
    assert_eq!(day4::part2(INPUT_D4_TEST), 30);
    assert_eq!(day4::part2(INPUT_D4), 11024379);

    assert_eq!(day5::part1(INPUT_D5_TEST), 35);
    assert_eq!(day5::part1(INPUT_D5), 178159714);
    assert_eq!(day5::part2(INPUT_D5_TEST), 46);
    assert_eq!(day5::part2(INPUT_D5), 100165128);

    assert_eq!(day6::part1(INPUT_D6_TEST), 288);
    assert_eq!(day6::part1(INPUT_D6), 440000);
    assert_eq!(day6::part2(INPUT_D6_TEST), 71503);
    assert_eq!(day6::part2(INPUT_D6), 313007);
}