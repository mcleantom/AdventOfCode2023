use std::collections::HashMap;

fn decode_line(string: &str) -> u64 {
    let first_num = string.chars().find(|&c| c.is_digit(10)).unwrap();
    let last_num = string.chars().rev().find(|&c| c.is_digit(10)).unwrap();
    let combined = first_num.to_string() + &last_num.to_string();
    combined.parse::<u64>().unwrap()
}

fn decode_line_inc_words(line: &str) -> u64 {
    let str_to_number = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9)
    ]);

    let first = (0..line.len())
        .find_map(|i| {
            str_to_number.keys().find(|key| line[i..].starts_with(*key))
        })
        .unwrap_or_else(|| panic!("Could not find first number in {}", line));
    
    let second = (0..line.len())
        .find_map(|i| {
            str_to_number.keys().find(|key| line[..(line.len()-i)].starts_with(*key))
        })
        .unwrap_or_else(|| panic!("Cound not find second number in {}", line));
    
    str_to_number[first] * 10 + str_to_number[second]
}

pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.split("\n") {
        sum += decode_line(line);
    }
    sum
}

pub fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.split("\n") {
        sum += decode_line_inc_words(line);
    }
    sum
}