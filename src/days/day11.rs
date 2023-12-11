use itertools::Itertools;

pub fn calc(lines: &str, expansion_rate: u64) -> u64 {
    let expansion_rate = expansion_rate - 1;
    let galaxy: Vec<Vec<char>> = lines.lines().map(|l| l.chars().collect()).collect();
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();

    for (idx, row) in galaxy.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.push(idx);
        }
    }

    for idx in 0..galaxy[0].len() {
        let col = galaxy.iter().map(|row| row[idx]).collect::<Vec<_>>();
        if col.iter().all(|&c| c == '.') {
            empty_cols.push(idx);
        }
    }

    let galaxy_positions = galaxy
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c != '.')
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();
    
    let sum_shortest_path = galaxy_positions
        .iter()
        .combinations(2)
        .fold(0, |mut sum, pair| {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            let mut distance = (*x1 as i32 - *x2 as i32).abs() as u64 + (*y1 as i32 - *y2 as i32).abs() as u64;

            let mut empty_rows_between = 0;
            let mut empty_cols_between = 0;

            for row in empty_rows.iter() {
                if (*row > *y1 && *row < *y2) || (*row > *y2 && *row < *y1) {
                    empty_rows_between += 1;
                }
            }

            for col in empty_cols.iter() {
                if (*col > *x1 && *col < *x2) || (*col > *x2 && *col < *x1) {
                    empty_cols_between += 1;
                }
            }

            distance += (empty_rows_between + empty_cols_between) * expansion_rate;
            sum += distance;
            sum
        });
    
    sum_shortest_path
}


pub fn part1(lines: &str) -> u64 {
    calc(lines, 2)
}


pub fn part2(lines: &str) -> u64 {
    calc(lines,1000000)
}