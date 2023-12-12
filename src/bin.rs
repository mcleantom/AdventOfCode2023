#![allow(dead_code)]
#![allow(unused_variables)]

use lib::days::*;

const INPUT_D1_PART_1_TEST: &str = include_str!("../res/day1_part1_test.txt");
const INPUT_D1_PART_2_TEST: &str = include_str!("../res/day1_part2_test.txt");
const INPUT_D2_TEST: &str = include_str!("../res/day2_test.txt");
const INPUT_D3_TEST: &str = include_str!("../res/day3_test.txt");
const INPUT_D4_TEST: &str = include_str!("../res/day4_test.txt");
const INPUT_D5_TEST: &str = include_str!("../res/day5_test.txt");
const INPUT_D6_TEST: &str = include_str!("../res/day6_test.txt");
const INPUT_D7_TEST: &str = include_str!("../res/day7_test.txt");
const INPUT_D8_PART1_TEST1: &str = include_str!("../res/day8_part1_test1.txt");
const INPUT_D8_PART1_TEST2: &str = include_str!("../res/day8_part1_test2.txt");
const INPUT_D8_PART2_TEST1: &str = include_str!("../res/day8_part2_test1.txt");
const INPUT_D9_TEST: &str = include_str!("../res/day9_test.txt");
const INPUT_D11_TEST: &str = include_str!("../res/day11_test.txt");
const INPUT_D12_TEST: &str = include_str!("../res/day12_test.txt");

const INPUT_D1: &str = include_str!("../res/day1.txt");
const INPUT_D2: &str = include_str!("../res/day2.txt");
const INPUT_D3: &str = include_str!("../res/day3.txt");
const INPUT_D4: &str = include_str!("../res/day4.txt");
const INPUT_D5: &str = include_str!("../res/day5.txt");
const INPUT_D6: &str = include_str!("../res/day6.txt");
const INPUT_D7: &str = include_str!("../res/day7.txt");
const INPUT_D8: &str = include_str!("../res/day8.txt");
const INPUT_D9: &str = include_str!("../res/day9.txt");
const INPUT_D11: &str = include_str!("../res/day11.txt");
const INPUT_D12: &str = include_str!("../res/day12.txt");


fn main() {
    assert_eq!(day1::part1(INPUT_D1_PART_1_TEST), 142);
    assert_eq!(day1::part1(INPUT_D1), 55538);
    assert_eq!(day1::part2(INPUT_D1_PART_2_TEST), 281);
    assert_eq!(day1::part2(INPUT_D1), 54875);

    assert_eq!(day2::part1(INPUT_D2_TEST), 8);
    assert_eq!(day2::part1(INPUT_D2), 2_207);
    assert_eq!(day2::part2(INPUT_D2_TEST), 2_286);
    assert_eq!(day2::part2(INPUT_D2), 622_41);

    assert_eq!(day3::part1(INPUT_D3_TEST), 4_361);
    assert_eq!(day3::part1(INPUT_D3), 532_331);
    assert_eq!(day3::part2(INPUT_D3_TEST), 467_835);
    assert_eq!(day3::part2(INPUT_D3), 82_301_120);

    assert_eq!(day4::part1(INPUT_D4_TEST), 13);
    assert_eq!(day4::part1(INPUT_D4), 21_485);
    assert_eq!(day4::part2(INPUT_D4_TEST), 30);
    assert_eq!(day4::part2(INPUT_D4), 11_024_379);

    assert_eq!(day5::part1(INPUT_D5_TEST), 35);
    assert_eq!(day5::part1(INPUT_D5), 178_159_714);
    assert_eq!(day5::part2(INPUT_D5_TEST), 46);
    assert_eq!(day5::part2(INPUT_D5), 100_165_128);

    assert_eq!(day6::part1(INPUT_D6_TEST), 288);
    assert_eq!(day6::part1(INPUT_D6), 440_000);
    assert_eq!(day6::part2(INPUT_D6_TEST), 71_503);
    assert_eq!(day6::part2(INPUT_D6), 26_187_338);

    assert_eq!(day7::part1(INPUT_D7_TEST), 6440);
    assert_eq!(day7::part1(INPUT_D7), 250347426);
    assert_eq!(day7::part2(INPUT_D7_TEST), 5905);
    assert_eq!(day7::part2(INPUT_D7), 251224870);

    assert_eq!(day8::part1(INPUT_D8_PART1_TEST1), 2);
    assert_eq!(day8::part1(INPUT_D8_PART1_TEST2), 6);
    assert_eq!(day8::part1(INPUT_D8), 11911);
    assert_eq!(day8::part2(INPUT_D8_PART2_TEST1), 6);
    assert_eq!(day8::part2(INPUT_D8), 10151663816849);

    assert_eq!(day9::part1(INPUT_D9_TEST), 114);
    assert_eq!(day9::part1(INPUT_D9), 1955513104);
    assert_eq!(day9::part2(INPUT_D9_TEST), 2);
    assert_eq!(day9::part2(INPUT_D9), 1131);

    assert_eq!(day11::part1(INPUT_D11_TEST), 374);
    assert_eq!(day11::part1(INPUT_D11), 10231178);
    assert_eq!(day11::calc(INPUT_D11_TEST, 10), 1030);
    assert_eq!(day11::calc(INPUT_D11_TEST, 100), 8410);
    assert_eq!(day11::part2(INPUT_D11), 622120986954);

    assert_eq!(day12::part1(INPUT_D12_TEST), 21);
    assert_eq!(day12::part1(INPUT_D12), 7169);
    assert_eq!(day12::part2(INPUT_D12_TEST), 525152);
    assert_eq!(day12::part2(INPUT_D12), 0);
}