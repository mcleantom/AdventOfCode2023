use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;


fn game_points(line: &String) -> usize {
    let (_, game) = line.split_once(": ").unwrap();
    let (winning_numbers, my_numbers) = game.split_once(" | ").unwrap();

    let winning_numbers: HashSet<i32> = winning_numbers.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let my_numbers: HashSet<i32> = my_numbers.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let matching_numbers: HashSet<_> = my_numbers.intersection(&winning_numbers)
        .cloned()
        .collect();

    let number_of_matches = matching_numbers.len();

    let mut total_points = 0;
    if number_of_matches > 0 {
        total_points = 2_usize.pow(number_of_matches as u32 - 1);
    }

    total_points
}


fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        sum += game_points(&line?);
    }
    println!("{}", sum);
    Ok(())
}