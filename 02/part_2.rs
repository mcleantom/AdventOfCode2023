use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::cmp;

fn power(line: &String) -> u32 {
    let mut min_balls = vec![0, 0, 0];

    let (_, games) = line.split_once(": ").unwrap();

    for sub_game in games.split("; ") {
        for chosen_color in sub_game.split(", ") {
            println!("{}", chosen_color);
            let (n_balls, color) = chosen_color.split_once(" ").unwrap();
            let index = match color.chars().next().unwrap() {
                'r' => 0,
                'b' => 1,
                'g' => 2,
                _ => panic!("unrecognized color")
            };
            min_balls[index] = cmp::max(
                min_balls[index],
                n_balls.parse::<u32>().unwrap()
            );
        }
    }
    println!("{}, {:?}", line, min_balls);
    min_balls[0] * min_balls[1] * min_balls[2]
}


fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        sum += power(&line?);
    }
    println!("Sum is {}", sum);
    Ok(())
}