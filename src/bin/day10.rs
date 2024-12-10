use std::collections::BTreeSet;

fn dfs(grid: &[Vec<u8>], start: (usize, usize)) -> (usize, usize) {
    let mut queue = vec![(start, 1)];
    let mut set = BTreeSet::new();
    let (mut score, mut rating) = (0, 0);

    while let Some(((i, j), level)) = queue.pop() {
        if level == 10 {
            if set.insert((i, j)) {
                score += 1;
            }
            rating += 1;
            continue;
        }
        if i > 0 && grid[i - 1][j] == level {
            queue.push(((i - 1, j), level + 1));
        }
        if i < grid.len() - 1 && grid[i + 1][j] == level {
            queue.push(((i + 1, j), level + 1));
        }
        if j > 0 && grid[i][j - 1] == level {
            queue.push(((i, j - 1), level + 1));
        }
        if j < grid[i].len() - 1 && grid[i][j + 1] == level {
            queue.push(((i, j + 1), level + 1));
        }
    }

    (score, rating)
}

fn main() {
    let input: Vec<Vec<_>> = aoc::input_lines(10)
        .map(|line| line.bytes().map(|b| b & 0xf).collect())
        .collect();

    let (mut p1, mut p2) = (0, 0);

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 0 {
                let (score, rating) = dfs(&input, (i, j));
                p1 += score;
                p2 += rating;
            }
        }
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
