use itertools::Itertools;

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
struct Maps {
    seeds: Vec<i64>,
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
                [] => {},
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

pub fn part1(lines: &str) -> i64 {
    let lines: Vec<String> = lines
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let map = Maps::new(&lines);
    let min_seed = map.seeds.iter().map(|seed| map.get_location(*seed)).min();
    min_seed.unwrap()
}


#[derive(Debug, Clone)]
struct Range {
    from: i64,
    to: i64,
}

#[derive(Debug, Clone)]
struct Rule {
    destination: i64,
    source: i64,
    range: i64,
}

pub fn part2(lines: &str) -> i64 {
    let (seeds_str, maps_str) = lines.split_once("\n\n").unwrap();
    let seeds = seeds_str.strip_prefix("seeds: ").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .chunks(2);
    let seeds = seeds.into_iter().map(|mut chunk| {
        let from = chunk.next().unwrap();
        let range = chunk.next().unwrap();
        Range {
            from,
            to: from + range,
        }
    });

    let maps: Vec<Vec<Rule>> = maps_str
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.splitn(3, " ");
                    Rule {
                        destination: nums.next().unwrap().parse().unwrap(),
                        source: nums.next().unwrap().parse().unwrap(),
                        range: nums.next().unwrap().parse().unwrap(),
                    }
                })
                .sorted_by(|a, b| a.source.cmp(&b.source))
                .collect()
        })
        .collect();

    let mut curr_ranges: Vec<Range> = seeds.collect();

    for map in &maps {
        let mut new_ranges: Vec<Range> = Vec::new();

        for range in &curr_ranges {
            let mut curr = range.clone();

            for rule in map {
                let offset = rule.destination - rule.source;
                let rule_applies = curr.from <= curr.to  // Current range is valid
                    && curr.from <= rule.source + rule.range  // The starting point is within the range covered by the rule
                    && curr.to >= rule.source;  // And the ending point also within the rule

                if rule_applies {
                    /*
                        If the current range is less than the rule source
                        There is a portion of the range that is less than the source of the rule
                        So we need to split the range appropriately

                        Otherwise, the current range is left unchanged (and may be modified by the next
                        rule)
                    */
                    if curr.from < rule.source {
                        /*
                            Make one new range that goes from the current range start,
                            to just before the start of the rule
                        */
                        new_ranges.push(Range {
                            from: curr.from,
                            to: rule.source - 1,
                        });
                        // After making this rule, the current range now starts at the rule source
                        curr.from = rule.source;
                        /*
                            If the rest of the range is within the rule range, then add one new range
                            that has been mapped according to the rule
                        */
                        if curr.to < rule.source + rule.range {
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: curr.to + offset,
                            });
                            // After applying this rule, the current range has been fully applied so
                            // set the start to past the end position
                            curr.from = curr.to + 1;
                        } else {
                            /*
                            Else, add one new range that covers the rest of the rule 
                            */
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: rule.source + rule.range - 1 + offset,
                            });
                            // And move the start from position of the current range to the edge of the
                            // rule
                            curr.from = rule.source + rule.range;
                        }
                    } else if curr.to < rule.source + rule.range {
                        /*
                        Else, the current range ends before the range covered by the rule
                        i.e. the current range is fully applied by the rule
                        */
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: curr.to + offset,
                        });
                        // And invalidate the current range, as it has been fully applied
                        curr.from = curr.to + 1;
                    } else {
                        /*
                        Otherwise, the current range intersects with the range covered by the rule
                        but does not end before the end of the rule.

                        In this case, we add a new range in the area applied by the rule
                        and also move the start position of the current range to the end position of the
                        rule so that it could be futher modified by another rule in the next iterations
                        */
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: rule.source + rule.range - 1 + offset,
                        });
                        curr.from = rule.source + rule.range;
                    }
                }
            }
            /*
            If the current rule has still not been invalidated, than no rules have been applied to the range
            and as per the instructions: any source numbers that aren't mapped correspond to the same destination number
            */
            if curr.from <= curr.to {
                new_ranges.push(curr);
            }
        }
        curr_ranges = new_ranges;
    }
    
    curr_ranges.iter().map(|range| range.from).min().unwrap()
}
