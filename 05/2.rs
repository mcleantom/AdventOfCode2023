use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp::min;

#[derive(Debug)]
struct MapRange {
    source: i64,
    destination: i64,
    range: i64
}

impl MapRange {
    fn new(source: i64, destination: i64, range: i64) -> MapRange {
        MapRange {
            source,
            destination,
            range
        }
    }

    fn get(&self, x: i64) -> Option<i64> {
        if x >= self.source && x <= self.source + self.range {
            let diff = x - self.source;
            return Some(self.destination + diff);
        }
        None
    }
}


#[derive(Debug)]
struct MapRanges {
    ranges: Vec<MapRange>
}

impl MapRanges {
    fn new() -> MapRanges {
        MapRanges {
            ranges: vec![]
        }
    }

    fn get(&self, x: i64) -> i64 {
        for range in self.ranges.iter() {
            let v = range.get(x);
            match v {
                Some(k) => return k,
                None => {}
            }
        }
        x
    }
}


#[derive(Debug)]
struct SeedRange {
    start_number: i64,
    number_of_seeds: i64
}

impl SeedRange {
    fn new(start_number: i64, number_of_seeds: i64) -> SeedRange {
        SeedRange {
            start_number,
            number_of_seeds
        }
    }
}


#[derive(Debug)]
struct Maps {
    seeds: Vec<SeedRange>,
    seed_to_soil: MapRanges,
    soil_to_fertilizer: MapRanges,
    fertilizer_to_water: MapRanges,
    water_to_light: MapRanges,
    light_to_temperature: MapRanges,
    temperature_to_humidity: MapRanges,
    humidity_to_location: MapRanges,
}

impl Maps {
    fn new(lines: &Vec<String>) -> Maps {
        let mut maps = Maps {
            seeds: vec![],
            seed_to_soil: MapRanges::new(),
            soil_to_fertilizer: MapRanges::new(),
            fertilizer_to_water: MapRanges::new(),
            water_to_light: MapRanges::new(),
            light_to_temperature: MapRanges::new(),
            temperature_to_humidity: MapRanges::new(),
            humidity_to_location: MapRanges::new(),
        };

        let mut current_hashmap: Option<&mut MapRanges> = None;

        for line in lines {
            let words: Vec<&str> = line.split_whitespace().collect();

            match words.as_slice() {
                [] => println!("Ignoring newline"),
                ["seeds:", rest @ ..] => {
                    let numbers: Vec<i64> = rest.iter().map(|&c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                    for chunk in numbers.chunks_exact(2) {
                        let start = chunk[0];
                        let n = chunk[1];
                        maps.seeds.push(SeedRange::new(start, n));
                    }
                }
                ["seed-to-soil", "map:"] => {
                    current_hashmap = Some(&mut maps.seed_to_soil);
                }
                ["soil-to-fertilizer", "map:"] => {
                    current_hashmap = Some(&mut maps.soil_to_fertilizer);
                }
                ["fertilizer-to-water", "map:"] => {
                    current_hashmap = Some(&mut maps.fertilizer_to_water);
                }
                ["water-to-light", "map:"] => {
                    current_hashmap = Some(&mut maps.water_to_light);
                }
                ["light-to-temperature", "map:"] => {
                    current_hashmap = Some(&mut maps.light_to_temperature);
                }
                ["temperature-to-humidity", "map:"] => {
                    current_hashmap = Some(&mut maps.temperature_to_humidity);
                }
                ["humidity-to-location", "map:"] => {
                    current_hashmap = Some(&mut maps.humidity_to_location);
                }
                _ => {
                    let values: Vec<i64> = words.iter().map(|&s| s.parse().unwrap()).collect();
                    let destination_range_start = values[0];
                    let source_range_start = values[1];
                    let range_length = values[2];
                    current_hashmap.as_mut().unwrap().ranges.push(MapRange::new(source_range_start, destination_range_start, range_length));
                }
            }
        }

        maps
    }

    fn get_location(&self, seed: i64) -> i64 {
        let soil = self.seed_to_soil.get(seed);
        let fertilizer = self.soil_to_fertilizer.get(soil);
        let water = self.fertilizer_to_water.get(fertilizer);
        let light = self.water_to_light.get(water);
        let temperature = self.light_to_temperature.get(light);
        let humidity = self.temperature_to_humidity.get(temperature);
        let location = self.humidity_to_location.get(humidity);
        location
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("could not parse line")).collect();
    println!("Building map.");
    let map = Maps::new(&lines);
    println!("Finding minimum location");
    let mut min_seed = 1000000000000000000;

    println!("{:?}", map.seeds);
    for seed_range in map.seeds.iter() {
        let start_seed = seed_range.start_number;
        let n_seeds = seed_range.number_of_seeds;
        for seed in start_seed..start_seed+n_seeds {
            min_seed = min(min_seed, map.get_location(seed));
            println!("Min seed: {}", min_seed);
        }
    }
    println!("{:?}", min_seed);

    Ok(())
}
