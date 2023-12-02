use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;


fn is_valid_game(line: &String) -> bool {
    let color_to_max_num_balls = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    match line.split_once(": ") {
        Some((_, games)) => {
            for sub_game in games.split("; ") {
                for chosen_color in sub_game.split(", ") {
                    match chosen_color.split_once(" ") {
                        Some((n_balls, color)) => {
                            match color_to_max_num_balls.get(&color) {
                                Some(max_n_balls) => {
                                    if n_balls.parse::<i32>().unwrap() > *max_n_balls {
                                        println!("{}, {}", color, n_balls);
                                        return false
                                    }
                                },
                                None => {
                                    panic!("Color was not one of green, red or blue");
                                }
                            }
                        },
                        None => {
                            panic!("Was expecting the string to split games with a ', ': {}", line);
                        }
                    }
                }
            }
        },
        None => {
            panic!("Was expecting the string to start with game n: {}", line);
        }
    }
    true
}


fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for (i, line) in reader.lines().enumerate() {
        let is_valid = is_valid_game(&line?);
        if is_valid {
            sum += i + 1;
        }
    }
    println!("Sum is {}", sum);
    Ok(())
}