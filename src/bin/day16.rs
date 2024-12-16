use aoc::VecGrid;
use hashbrown::{HashMap, HashSet};
use std::collections::{BTreeSet, BinaryHeap, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
enum Direction {
    East = 1,
    North = 2,
    West = 4,
    South = 8,
}

impl Direction {
    const fn delta(self) -> (isize, isize) {
        match self {
            Direction::East => (0, 1),
            Direction::North => (-1, 0),
            Direction::West => (0, -1),
            Direction::South => (1, 0),
        }
    }

    fn cost(self, other: Self) -> u32 {
        if self == other {
            1
        } else if self == !other {
            2001
        } else {
            1001
        }
    }
}

impl std::ops::Not for Direction {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Direction::East => Direction::West,
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
        }
    }
}

#[derive(PartialEq, Eq)]
struct Reindeer {
    score: u32,
    dir: Direction,
    pos: (usize, usize),
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut grid = VecGrid::new(aoc::input_bytes(16));

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|&c| c == b'S').map(|j| (i, j)))
        .unwrap();

    let mut queue = BinaryHeap::new();
    queue.push(Reindeer {
        pos: start,
        dir: Direction::East,
        score: 0,
    });
    let mut prev = HashMap::new();
    prev.insert((start, Direction::East), (0, BTreeSet::new()));

    let mut p1 = u32::MAX;
    let mut end = ((0, 0), Direction::East);

    while let Some(Reindeer {
        score,
        dir,
        pos: (i, j),
    }) = queue.pop()
    {
        if grid.get(i, j) == Some(b'E') {
            p1 = score;
            end = ((i, j), dir);
            break;
        }

        if grid[i][j] == b'.' {
            grid[i][j] = 0;
        }
        grid[i][j] |= dir as u8;

        for d in [
            Direction::East,
            Direction::North,
            Direction::West,
            Direction::South,
        ] {
            if d == !dir {
                continue;
            }
            let (di, dj) = d.delta();
            let new_i = i.wrapping_add_signed(di);
            let new_j = j.wrapping_add_signed(dj);
            let Some(node) = grid.get(new_i, new_j) else {
                continue;
            };
            if node == b'.' || node == b'E' || (node < 16 && node & d as u8 == 0) {
                let new_score = score + dir.cost(d);
                queue.push(Reindeer {
                    score: new_score,
                    dir: d,
                    pos: (new_i, new_j),
                });

                let entry = prev
                    .entry(((new_i, new_j), d))
                    .or_insert_with(|| (new_score, BTreeSet::new()));

                if entry.0 == new_score {
                    entry.1.insert(((i, j), dir));
                } else if entry.0 > new_score {
                    entry.0 = new_score;
                    entry.1 = BTreeSet::from_iter([((i, j), dir)]);
                }
            }
        }
    }

    let mut seats = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(end);
    while let Some(point) = queue.pop_front() {
        seats.insert(point.0);
        queue.extend(prev[&point].1.iter());
    }
    let p2 = seats.len();

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
