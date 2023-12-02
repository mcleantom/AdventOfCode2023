use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn decode_line(string: &String) -> u32 {
    let first_num = string.chars().find(|&c| c.is_digit(10)).unwrap();
    let last_num = string.chars().rev().find(|&c| c.is_digit(10)).unwrap();
    let combined = first_num.to_string() + &last_num.to_string();
    // return the sum of the first and last number
    combined.parse::<u32>().unwrap()
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
