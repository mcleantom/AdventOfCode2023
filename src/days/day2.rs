use std::collections::HashMap;
use std::cmp;


fn is_valid_game(line: &str) -> bool {
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


fn power(line: &str) -> u32 {
    let mut min_balls = vec![0, 0, 0];

    let (_, games) = line.split_once(": ").unwrap();

    for sub_game in games.split("; ") {
        for chosen_color in sub_game.split(", ") {
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
    min_balls[0] * min_balls[1] * min_balls[2]
}


pub fn part1(lines: &str) -> usize {
    let mut sum = 0;
    for (i, line) in lines.split("\n").enumerate() {
        let is_valid = is_valid_game(line);
        if is_valid {
            sum += i + 1;
        }
    }
    sum
}

pub fn part2(lines: &str) -> u32 {
    let mut sum = 0;
    for line in lines.split("\n") {
        sum += power(line);
    }
    sum
}