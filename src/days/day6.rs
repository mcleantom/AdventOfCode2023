#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}


fn race_distance(waiting_time: u64, race_time: &u64) -> u64 {
    let speed = waiting_time;
    let remaining_time = race_time - waiting_time;
    let distance = speed * remaining_time;
    distance
}


fn number_of_ways_to_beat_race_distance(race: &Race) -> u64 {
    let mut ways = 0;
    for waiting_time in 0..race.time {
        let distance = race_distance(waiting_time, &race.time);
        if distance > race.distance {
            ways += 1;
        }
    }
    ways
}


pub fn part1(lines: &str) -> u64 {
    let (time_str, distance_str) = lines.split_once("\n").unwrap();

    let times = time_str
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    
    let distances = distance_str
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    
    let races: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| {
            Race {
                time: *time,
                distance: *distance,
            
            }
        })
        .collect();

    let number_of_ways: Vec<u64> = races
        .iter()
        .map(|race| number_of_ways_to_beat_race_distance(&race))
        .collect();
    
    let product = number_of_ways
        .iter()
        .fold(1, |acc, &n| acc * n);

    product
}


pub fn part2(lines: &str) -> u64 {
    let (time_str, distance_str) = lines.split_once("\n").unwrap();
    let time = time_str
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance = distance_str
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let race = Race {
        time,
        distance,
    };
    let number_of_ways = number_of_ways_to_beat_race_distance(&race);
    number_of_ways
}