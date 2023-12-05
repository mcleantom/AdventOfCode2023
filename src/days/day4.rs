use std::collections::HashSet;


fn game_points(line: &str) -> usize {
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


pub fn part1(lines: &str) -> usize {
    let mut sum = 0;
    for line in lines.split("\n") {
        sum += game_points(line);
    }
    sum
}


struct Game {
    number: i32,
    number_matching_numbers: i32
}


impl Game {
    fn new(line: &String) -> Game {
        let (card, game) = line.split_once(": ").unwrap();
        let number = card.split_whitespace().nth(1).unwrap();
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
        let my_number = number.parse::<i32>().unwrap() - 1;

        Game {
            number: my_number,
            number_matching_numbers: number_of_matches as i32
        }
    }
}


fn calc_winning_scratchcards(games: &Vec<String>) -> i32 {
    let games = games.iter().map(|line| Game::new(line)).collect::<Vec<Game>>();
    let mut copies = vec![1; games.len()];

    for game in games.iter() {
        for i in (game.number+1)..(game.number+game.number_matching_numbers+1) {
            copies[i as usize] += copies[game.number as usize];
        }
    }

    copies.into_iter().sum()
}

pub fn part2(lines: &str) -> i32 {
    let lines: Vec<String> = lines
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    calc_winning_scratchcards(&lines)
}