use std::collections::HashMap;


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
    let mut digraph = HashMap::<&str, [&str; 2]>::new();
    let mut nodes_ending_with_a: Vec<&str> = Vec::new();

    for line in lines.iter().skip(2) {
        let (fr_str, to_str) = line.split_once(" = ").unwrap();
        let fr_str = fr_str.trim();
        let to_str = to_str.trim_start_matches("(").trim_end_matches(")");
        let (left, right) = to_str.split_once(", ").unwrap();
        let arr = [left, right];
        digraph.insert(fr_str, arr);
        if fr_str.ends_with("A") {
            nodes_ending_with_a.push(fr_str);
        }
    }


    let current_nodes = nodes_ending_with_a;
    let mut count = 0;
    while current_nodes.map(|node| node.ends_with("Z")).sum() != current_nodes.len() {
        println!("hi");
        count += 1;
    }

    count
}