use std::string::String;
use std::collections::HashMap;


fn indices_of_symbols(lines: &Vec<String>) -> Vec<(isize, isize)> {
    let mut res = vec![];

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                res.push((i as isize, j as isize));
            }
        }
    }

    res
}

fn surrounding_numbers(lines: &mut Vec<String>, indices: &Vec<(isize, isize)>) -> Vec<u32> {
    let mut res = vec![];

    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)
    ];

    for (y, x) in indices.iter() {
        for (dy, dx) in directions.iter() {
            let new_y = y + dy;
            let mut new_x = x + dx;
            if new_y >= 0 && new_y < lines.len() as isize && new_x >= 0 && new_x < lines[0].len() as isize {
                let c = lines[new_y as usize].chars().nth(new_x as usize).unwrap();
                if c.is_numeric() {
                    while new_x >= 1
                        && lines[new_y as usize].chars().nth((new_x - 1) as usize).unwrap().is_numeric() {
                        new_x -= 1;
                    }
                    let start_x = new_x;
                    while new_x < lines[0].len() as isize
                        && lines[new_y as usize].chars().nth(new_x as usize).unwrap().is_numeric() {
                        new_x += 1;
                    }
                    let end_x = new_x;
                    let sub_str = &lines[new_y as usize][start_x as usize..end_x as usize];
                    let num = sub_str.parse::<u32>().unwrap();
                    res.push(num);
                    for i in start_x..end_x {
                        lines[new_y as usize].replace_range(i as usize..(i + 1) as usize, ".");
                    }
                }
            }
        }
    }

    res
}


fn surrounding_numbers_2(lines: &mut Vec<String>, indices: &Vec<(isize, isize)>) -> Vec<u32> {
    let mut indices_to_gear_ratio: HashMap<(isize, isize), Vec<u32>> = HashMap::new();

    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)
    ];

    for (y, x) in indices.iter() {
        for (dy, dx) in directions.iter() {
            let new_y = y + dy;
            let mut new_x = x + dx;
            if new_y >= 0 && new_y < lines.len() as isize && new_x >= 0 && new_x < lines[0].len() as isize {
                let c = lines[new_y as usize].chars().nth(new_x as usize).unwrap();
                if c.is_numeric() {
                    while new_x >= 1
                        && lines[new_y as usize].chars().nth((new_x - 1) as usize).unwrap().is_numeric() {
                        new_x -= 1;
                    }
                    let start_x = new_x;
                    while new_x < lines[0].len() as isize
                        && lines[new_y as usize].chars().nth(new_x as usize).unwrap().is_numeric() {
                        new_x += 1;
                    }
                    let end_x = new_x;
                    let sub_str = &lines[new_y as usize][start_x as usize..end_x as usize];
                    let num = sub_str.parse::<u32>().unwrap();
                    for i in start_x..end_x {
                        lines[new_y as usize].replace_range(i as usize..(i + 1) as usize, ".");
                    }
                    let tup = (*y, *x);
                    match indices_to_gear_ratio.get_mut(&tup) {
                        Some(vec) => vec.push(num),
                        None => {
                            indices_to_gear_ratio.insert(tup, vec![num]);
                        }
                    }
                }
            }
        }
    }

    let mut res = vec![];

    for (_, v) in &indices_to_gear_ratio {
        if v.len() >= 2 {
            let mut total = v[0];
            for i in 1..v.len() {
                total *= v[i];
            }
            res.push(total);
        }
    }
    
    res
}


pub fn part1(lines: &str) -> u32 {
    let mut lines: Vec<String> = lines
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let indices = indices_of_symbols(&lines);
    let res = surrounding_numbers(&mut lines, &indices);
    let sum: u32 = res.iter().sum();
    sum
}


pub fn part2(lines: &str) -> u32 {
    let mut lines: Vec<String> = lines
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let indices = indices_of_symbols(&lines);
    let res = surrounding_numbers_2(&mut lines, &indices);
    let sum: u32 = res.iter().sum();
    sum    
}