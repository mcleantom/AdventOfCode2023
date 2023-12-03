use std::fs::File;
use std::io::{self, prelude::*, BufReader};
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


fn main() -> io::Result<()> {
    /*
        The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

        Here is an example engine schematic:

        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..

        In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
    */
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let indices = indices_of_symbols(&lines);
    let res = surrounding_numbers(&mut lines, &indices);
    let sum: u32 = res.iter().sum();
    println!("{}", sum);
    Ok(())
}