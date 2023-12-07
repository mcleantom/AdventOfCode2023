#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}


#[derive(Debug)]
struct Hand {
    cards: String,
    hand_type: HandType,
    bid: i64,
}


impl Hand {
    fn new(hand_str: &str) -> Hand {
        let (card_str, bid_str) = hand_str.split_once(" ").unwrap();
        let bid = bid_str.parse::<i64>().unwrap();
        
        let mut card_counts = [0; 13];
        
        for c in card_str.chars() {
            let index = match c {
                '2' => 0,
                '3' => 1,
                '4' => 2,
                '5' => 3,
                '6' => 4,
                '7' => 5,
                '8' => 6,
                '9' => 7,
                'T' => 8,
                'J' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                _ => panic!("Invalid card: {}", c),
            };
            card_counts[index] += 1;
        }   

        let mut hand_type: Option<HandType> = None;
        if card_counts.contains(&5) {
            hand_type = Some(HandType::FiveOfAKind);
        } else if card_counts.contains(&4) {
            hand_type = Some(HandType::FourOfAKind);
        } else if card_counts.contains(&3) && card_counts.contains(&2) {
            hand_type = Some(HandType::FullHouse);
        } else if card_counts.contains(&3) {
            hand_type = Some(HandType::ThreeOfAKind);
        } else if card_counts.iter().filter(|&x| *x == 2).count() == 2 {
            hand_type = Some(HandType::TwoPair);
        } else if card_counts.contains(&2) {
            hand_type = Some(HandType::OnePair);
        }
        if hand_type.is_none() {
            hand_type = Some(HandType::HighCard);
        }

        Hand {
            cards: card_str.to_string(),
            hand_type: hand_type.unwrap(),
            bid: bid,
        }
    }
}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let card_to_rank = |c: char| match c {
            '2' => 12,
            '3' => 11,
            '4' => 10,
            '5' => 9,
            '6' => 8,
            '7' => 7,
            '8' => 6,
            '9' => 5,
            'T' => 4,
            'J' => 3,
            'Q' => 2,
            'K' => 1,
            'A' => 0,
            _ => panic!("Invalid card: {}", c),
        };

        if self.hand_type == other.hand_type {
            for (my_card, other_card) in self.cards.chars().zip(other.cards.chars()) {
                let my_rank = card_to_rank(my_card);
                let other_rank = card_to_rank(other_card);
                if my_rank != other_rank {
                    return Some(my_rank.cmp(&other_rank));
                }
            }
        }
        Some(self.hand_type.cmp(&other.hand_type))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}


pub fn part1(lines: &str) -> i64 {
    let mut hands = lines
        .lines()
        .map(|line| Hand::new(line))
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
    hands.reverse();

    let mut winnings = 0;
    for (game, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (game + 1) as i64;
    }

    winnings
}