fn solve(input: &str, backwards: bool) -> i32 {
    input.lines().map(|line| {
        let mut nums = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut edge = Vec::new();

        loop {
            let differences = nums
                .windows(2)
                .map(|i| if backwards { i[0] - i[1] } else { i[1] - i[0] })
                .collect::<Vec<_>>();
            edge.push(nums[if backwards { 0 } else { nums.len() - 1 }]);
            if differences.iter().all(|&x| x == 0) {
                break edge.iter().copied().sum::<i32>();
            }
            nums = differences;
        }
        
    }).sum()
}

pub fn part1(input: &str) -> i32 {
    solve(input, false)
}

pub fn part2(input: &str) -> i32 {
    solve(input, true)
}