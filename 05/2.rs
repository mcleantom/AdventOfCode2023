use std::fs;
use std::io::{Result};
use itertools::Itertools;

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

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
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
    
    let x = curr_ranges.iter().map(|range| range.from).min().unwrap();
    println!("{:?}", x);

    Ok(())
}
