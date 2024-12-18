use std::{cmp::Ordering, collections::VecDeque};

const N: usize = 70;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Free,
    Visited,
    Corrupted,
}

fn bfs(obstacles: &[(usize, usize)]) -> Option<u32> {
    let mut grid = [[Cell::Free; N + 1]; N + 1];

    for &(x, y) in obstacles {
        grid[y][x] = Cell::Corrupted;
    }

    let start = (0, 0, 0);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    grid[0][0] = Cell::Visited;

    while let Some((x, y, d)) = queue.pop_front() {
        if x == N && y == N {
            return Some(d);
        }

        for (dx, dy) in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
            let x = x.wrapping_add_signed(dx);
            let y = y.wrapping_add_signed(dy);

            if (0..=N).contains(&x) && (0..=N).contains(&y) && grid[y][x] == Cell::Free {
                grid[y][x] = Cell::Visited;
                queue.push_back((x, y, d + 1));
            }
        }
    }

    None
}

fn main() {
    let obstacles = aoc::input_lines(18)
        .flat_map(|l| {
            l.split_once(',')
                .map(|(x, y)| (aoc::parse_dec(x), aoc::parse_dec(y)))
        })
        .collect::<Vec<(usize, usize)>>();

    let mut grid = [[Cell::Free; N + 1]; N + 1];

    for &(x, y) in obstacles.iter().take(1024) {
        grid[y][x] = Cell::Corrupted;
    }

    let start = (0, 0, 0);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    grid[0][0] = Cell::Visited;

    let p1 = bfs(&obstacles[..1024]).unwrap();

    let indices = (1024..obstacles.len()).collect::<Vec<_>>();

    let fst_idx = indices
        .binary_search_by(|&i| {
            if bfs(&obstacles[..=i]).is_some() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap_err();
    let fst = indices[fst_idx];
    let coords = obstacles[fst];
    let p2 = format!("{},{}", coords.0, coords.1);

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
