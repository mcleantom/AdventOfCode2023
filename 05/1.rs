use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct Maps {
    seeds: Vec<i64>,
    seed_to_soil: HashMap<i64, i64>,
    soil_to_fertilizer: HashMap<i64, i64>,
    fertilizer_to_water: HashMap<i64, i64>,
    water_to_light: HashMap<i64, i64>,
    light_to_temperature: HashMap<i64, i64>,
    temperature_to_humidity: HashMap<i64, i64>,
    humidity_to_location: HashMap<i64, i64>,
}

impl Maps {
    fn new(lines: &Vec<String>) -> Maps {
        let mut maps = Maps {
            seeds: vec![],
            seed_to_soil: HashMap::new(),
            soil_to_fertilizer: HashMap::new(),
            fertilizer_to_water: HashMap::new(),
            water_to_light: HashMap::new(),
            light_to_temperature: HashMap::new(),
            temperature_to_humidity: HashMap::new(),
            humidity_to_location: HashMap::new(),
        };

        let mut current_hashmap: Option<&mut HashMap<i64, i64>> = None;

        for line in lines {
            let words: Vec<&str> = line.split_whitespace().collect();

            match words.as_slice() {
                [] => println!("Ignoring newline"),
                ["seeds:", rest @ ..] => {
                    maps.seeds = rest.iter().map(|&c| c.parse().unwrap()).collect();
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
                    for i in 0..range_length {
                        let source = source_range_start + i;
                        let destination = destination_range_start + i;
                        if let Some(ref mut map) = current_hashmap {
                            map.insert(source, destination);
                        }
                    }
                }
            }
        }

        maps
    }

    fn get_location(&self, seed: i64) -> i64 {
        let soil = self.seed_to_soil.get(&seed).cloned().unwrap_or(seed);
        let fertilizer = self.soil_to_fertilizer.get(&soil).cloned().unwrap_or(soil);
        let water = self.fertilizer_to_water.get(&fertilizer).cloned().unwrap_or(fertilizer);
        let light = self.water_to_light.get(&water).cloned().unwrap_or(water);
        let temperature = self.light_to_temperature.get(&light).cloned().unwrap_or(light);
        let humidity = self.temperature_to_humidity.get(&temperature).cloned().unwrap_or(temperature);
        let location = self.humidity_to_location.get(&humidity).cloned().unwrap_or(humidity);
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
    let min_seed = map.seeds.iter().map(|seed| map.get_location(*seed)).min();
    println!("{:?}", min_seed);

    Ok(())
}
