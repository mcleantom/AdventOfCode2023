use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn decode_line(line: &String) -> u32 {
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

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        sum += decode_line(&line?);
    }
    println!("Sum is {}", sum);
    Ok(())
}