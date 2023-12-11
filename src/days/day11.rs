use itertools::Itertools;

pub fn part1(lines: &str) -> u64 {
    let mut galaxy: Vec<Vec<char>> = lines.lines().map(|l| l.chars().collect()).collect();

    let mut idx = 0;
    while idx < galaxy.len() {
        let row = &galaxy[idx];
        if row.iter().all(|&c| c == '.') {
            galaxy.insert(idx, vec!['.'; row.len()]);
            idx += 1;
        }
        idx += 1;
    }

    let mut idx = 0;
    while idx < galaxy[0].len() {
        let col = galaxy.iter().map(|row| row[idx]).collect::<Vec<_>>();
        if col.iter().all(|&c| c == '.') {
            for row in &mut galaxy {
                row.insert(idx, '.');
            }
            idx += 1;
        }
        idx += 1;
    }

    let galaxy_positions = galaxy
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();
    
    let sum_shortest_path = galaxy_positions
        .iter()
        .combinations(2)
        .fold(0, |mut sum, pair| {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            sum += (*x1 as i32 - *x2 as i32).abs() as u64 + (*y1 as i32 - *y2 as i32).abs() as u64;
            sum
        });
    
    sum_shortest_path
}