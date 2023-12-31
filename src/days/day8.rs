use std::collections::HashMap;
use num_integer::lcm;


pub fn part1(lines: &str) -> u64 {
    let lines: Vec<&str> = lines.split("\n").collect();
    let left_right_instructions: Vec<char> = lines[0].chars().collect();
    let mut digraph = HashMap::<&str, [&str; 2]>::new();

    for line in lines.iter().skip(2) {
        let (fr_str, to_str) = line.split_once(" = ").unwrap();
        let fr_str = fr_str.trim();
        let to_str = to_str.trim_start_matches("(").trim_end_matches(")");
        let (left, right) = to_str.split_once(", ").unwrap();
        let arr = [left, right];
        digraph.insert(fr_str, arr);
    }

    let first_key = "AAA";
    let mut current_key = first_key;
    let mut current_instruction = 0;
    let mut count = 0;

    while current_key != "ZZZ" {
        let arr = digraph.get(&current_key).unwrap();
        let left_or_right = left_right_instructions[current_instruction];
        let next_key = match left_or_right {
            'L' => arr[0],
            'R' => arr[1],
            _ => unreachable!(),
        };
        current_key = next_key;
        current_instruction = (current_instruction + 1) % left_right_instructions.len();
        count += 1;
    }

    count
}



pub fn part2(lines: &str) -> u64 {
    let lines: Vec<&str> = lines.split("\n").collect();
    let left_right_instructions: Vec<char> = lines[0].chars().collect();
    let mut digraph = HashMap::<String, [String; 2]>::new();
    let mut nodes_ending_with_a: Vec<String> = Vec::new();

    for line in lines.iter().skip(2) {
        let (fr_str, to_str) = line.split_once(" = ").unwrap();
        let fr_str = fr_str.trim();
        let to_str = to_str.trim_start_matches("(").trim_end_matches(")");
        let (left, right) = to_str.split_once(", ").unwrap();
        let arr = [left.to_string(), right.to_string()];
        digraph.insert(fr_str.to_string(), arr);
        if fr_str.ends_with("A") {
            nodes_ending_with_a.push(fr_str.to_string());
        }
    }

    let mut totals: Vec<u64> = vec![0; nodes_ending_with_a.len()];
    
    for (i, pos) in nodes_ending_with_a.iter_mut().enumerate() {
        let mut cycle = left_right_instructions.iter().cycle();
        while !pos.ends_with("Z") {
            totals[i] += 1;
            let tmp_arr_ref = digraph.get(pos).unwrap();
            let index = match cycle.next().unwrap() {
                'L' => 0,
                'R' => 1,
                _ => unreachable!(),
            };
            *pos = tmp_arr_ref[index].clone();
        }
    }

    let least_common_multiple = totals.iter().fold(1, |acc, &x| lcm(acc, x));
    least_common_multiple
}