#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}


fn number_of_ways_to_beat_race_distance(t_run: u32, dist_record: i64, v:u8) -> i64 {
    /*
    The inequality is:
    distance = wait_time * (race_time - wait_time)
    d = w * (t - w)
    d = wt - w^2
    w^2 - wt + d = 0

    a = 1
    b = -t
    c = d

    w = (-b +- sqrt(b^2 - 4ac)) / 2a
    */
    let a: i64 = -1;
    let b: i64 = i64::from(v as u32 * t_run);
    let c: i64 = -dist_record;

    let b_squared = b.pow(2);
    let term = b_squared - 4 * a * c;
    let sq = f64::sqrt(term as f64);

    let a_doubled = 2 * a;

    let x_1 = (-b as f64 + sq) / a_doubled as f64;
    let x_2 = (-b as f64 - sq) / a_doubled as f64;

    let (x_1, x_2) = (x_1.floor() as i64 + 1, x_2.ceil() as i64 - 1);

    let (a, b) = (x_1, x_2);
    b - a + 1
}


pub fn part1(lines: &str) -> i64 {
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

    let number_of_ways: Vec<i64> = races
        .iter()
        .map(|race| number_of_ways_to_beat_race_distance(race.time as u32, race.distance as i64, 1))
        .collect();
    
    let product = number_of_ways
        .iter()
        .fold(1, |acc, &n| acc * n);

    product
}


pub fn part2(lines: &str) -> i64 {
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
    let number_of_ways = number_of_ways_to_beat_race_distance(race.time as u32, race.distance as i64, 1);
    number_of_ways
}