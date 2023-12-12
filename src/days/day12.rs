fn count_arangements(line: &str, counts: &[usize]) -> usize {
    let line = line.as_bytes();
    let n = line.len();
    let m = counts.len();
    let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];

    dp[n][m][0] = 1;
    dp[n][m - 1][counts[m - 1]] = 1;

    for pos in (0..n).rev() {
        for (group, &max_count) in counts.iter().enumerate() {
            for count in 0..=max_count {
                for &c in &[b'.', b'#'] {
                    if line[pos] == c || line[pos] == b'?' {
                        if c == b'.' && count == 0 {
                            dp[pos][group][count] += dp[pos + 1][group][0];
                        } else if c == b'.' && group < m && counts[group] == count {
                            dp[pos][group][count] += dp[pos + 1][group + 1][0];
                        } else if c == b'#' {
                            dp[pos][group][count] += dp[pos + 1][group][count + 1];
                        }
                    }
                }
            }
        }
        if matches!(line[pos], b'.' | b'?') {
            dp[pos][m][0] += dp[pos + 1][m][0];
        }
    }

    dp[0][0][0]
}


pub fn part1(lines: &str) -> usize {
    lines
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let pattern = parts.next().unwrap();
            let counts: Vec<usize> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|count| count.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            count_arangements(pattern, &counts)
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum()
}


pub fn part2(lines: &str) -> usize {
    lines
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let pattern = parts.next().unwrap();
            let counts: Vec<usize> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|count| count.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let pattern = std::iter::repeat(pattern).take(5).collect::<Vec<_>>().join("?");
            let counts = counts.repeat(5);
            count_arangements(&pattern, &counts)
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum()
}