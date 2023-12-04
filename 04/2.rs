use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    number: i32,
    winning_numbers: HashSet<i32>,
    my_numbers: HashSet<i32>,
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
            winning_numbers,
            my_numbers,
            number_matching_numbers: number_of_matches as i32
        }
    }

    fn clone(&self) -> Game {
        Game {
            number: self.number,
            winning_numbers: self.winning_numbers.clone(),
            my_numbers: self.my_numbers.clone(),
            number_matching_numbers: self.number_matching_numbers
        }
    }
}


fn calc_winning_scratchcards(games: &Vec<String>) -> i32 {
    let mut games = games.iter().map(|line| Game::new(line)).collect::<Vec<Game>>();
    let mut copies = vec![1; games.len()];

    for game in games.iter() {
        for i in (game.number+1)..(game.number+game.number_matching_numbers+1) {
            copies[i as usize] += copies[game.number as usize];
        }
    }

    copies.into_iter().sum()
}


fn main() -> io::Result<()> {
    /*
    Specifically, you win copies of the scratchcards below the winning card equal to the number of matches. So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.

    Copies of scratchcards are scored like normal scratchcards and have the same card number as the card they copied. So, if you win a copy of card 10 and it has 5 matching numbers, it would then win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This process repeats until none of the copies cause you to win any more cards. (Cards will never make you copy a card past the end of the table.)

    This time, the above example goes differently:

    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
    Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
    Your copy of card 2 also wins one copy each of cards 3 and 4.
    Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
    Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
    Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
    Your one instance of card 6 (one original) has no matching numbers and wins no more cards.
    Once all of the originals and copies have been processed, you end up with 1 instance of card 1, 2 instances of card 2, 4 instances of card 3, 8 instances of card 4, 14 instances of card 5, and 1 instance of card 6. In total, this example pile of scratchcards causes you to ultimately have 30 scratchcards!
    */
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let sum = calc_winning_scratchcards(&lines);
    println!("{}", sum);
    Ok(())
}