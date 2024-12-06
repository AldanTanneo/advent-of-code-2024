use std::collections::BTreeSet;

#[derive(Clone, Copy)]
#[repr(u8)]
enum Direction {
    Left = 1,
    Right = 2,
    Up = 4,
    Down = 8,
}

impl Direction {
    fn from_u8(c: u8) -> Self {
        match c {
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            b'^' => Direction::Up,
            b'v' => Direction::Down,
            _ => panic!("invalid direction"),
        }
    }

    fn step(&mut self, grid: &[Vec<u8>], i: &mut usize, j: &mut usize) -> bool {
        match self {
            Direction::Left => {
                if *j == 0 {
                    return true;
                }
                if grid[*i][*j - 1] == b'#' {
                    *self = Direction::Up;
                } else {
                    *j -= 1;
                }
            }
            Direction::Right => {
                if *j == grid[*i].len() - 1 {
                    return true;
                }
                if grid[*i][*j + 1] == b'#' {
                    *self = Direction::Down;
                } else {
                    *j += 1;
                }
            }
            Direction::Up => {
                if *i == 0 {
                    return true;
                }
                if grid[*i - 1][*j] == b'#' {
                    *self = Direction::Right;
                } else {
                    *i -= 1;
                }
            }
            Direction::Down => {
                if *i == grid.len() - 1 {
                    return true;
                }
                if grid[*i + 1][*j] == b'#' {
                    *self = Direction::Left;
                } else {
                    *i += 1;
                }
            }
        }
        false
    }
}

fn detect_loop(
    grid: &[Vec<u8>],
    (mut i, mut j, mut dir): (usize, usize, Direction),
    (oi, oj): (usize, usize),
) -> bool {
    let mut grid = grid.to_vec();
    grid[oi][oj] = b'#';

    loop {
        if grid[i][j] >= 16 {
            grid[i][j] = 0;
        }
        if grid[i][j] & dir as u8 != 0 {
            break true;
        }
        grid[i][j] |= dir as u8;
        if dir.step(&grid, &mut i, &mut j) {
            break false;
        }
    }
}

fn main() {
    let input = aoc::input_str(6);
    let mut grid = input
        .split_ascii_whitespace()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(i, l)| {
            l.iter().enumerate().find_map(|(j, c)| {
                if *c != b'.' && *c != b'#' {
                    Some((i, j, Direction::from_u8(*c)))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let (mut i, mut j, mut dir) = start;
    let (mut oi, mut oj);

    let mut p2 = 0;
    let mut p2_set = BTreeSet::new();

    loop {
        grid[i][j] = b'X';
        (oi, oj) = (i, j);
        if dir.step(&grid, &mut i, &mut j) {
            break;
        }
        if (oi, oj) != (i, j) && detect_loop(&grid, start, (i, j)) {
            if p2_set.insert((i, j)) {
                p2 += 1;
            }
        }
    }

    let p1 = grid.iter().flatten().filter(|&&x| x == b'X').count();

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
