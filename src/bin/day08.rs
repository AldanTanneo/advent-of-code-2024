use std::collections::{HashMap, HashSet};

fn main() {
    let input = aoc::input_str(8);
    let coordinates = input
        .split_ascii_whitespace()
        .filter(|x| !x.is_empty())
        .enumerate()
        .fold(HashMap::<u8, Vec<_>>::new(), |mut map, (i, line)| {
            for (j, c) in line.bytes().enumerate() {
                if c != b'.' {
                    map.entry(c).or_default().push((i as isize, j as isize));
                }
            }
            map
        });

    let len = input.bytes().position(|x| x == b'\n').unwrap() as isize;
    let mut set = HashSet::new();

    for (_, coords) in &coordinates {
        for (x, fst) in coords.iter().enumerate() {
            for snd in &coords[x + 1..] {
                let i1 = 2 * fst.0 - snd.0;
                let j1 = 2 * fst.1 - snd.1;
                if (0..len).contains(&i1) && (0..len).contains(&j1) {
                    set.insert((i1, j1));
                }

                let i2 = 2 * snd.0 - fst.0;
                let j2 = 2 * snd.1 - fst.1;
                if (0..len).contains(&i2) && (0..len).contains(&j2) {
                    set.insert((i2, j2));
                }
            }
        }
    }

    let p1 = set.len();

    set.clear();

    for (_, coords) in &coordinates {
        for (x, fst) in coords.iter().enumerate() {
            for snd in &coords[x + 1..] {
                let di = snd.0 - fst.0;
                let dj = snd.1 - fst.1;

                if di == 0 {
                    set.extend((0..len).map(|j| (fst.0, j)));
                } else if dj == 0 {
                    set.extend((0..len).map(|i| (i, fst.1)));
                } else {
                    let gcd = aoc::gcd(di.abs() as u64, dj.abs() as u64);
                    let di = di / gcd as isize;
                    let dj = dj / gcd as isize;

                    set.extend(
                        (1..)
                            .map(|n| (fst.0 - n * di, fst.1 - n * dj))
                            .take_while(|(i, j)| (0..len).contains(i) && (0..len).contains(j)),
                    );
                    set.extend(
                        (0..)
                            .map(|n| (fst.0 + n * di, fst.1 + n * dj))
                            .take_while(|(i, j)| (0..len).contains(i) && (0..len).contains(j)),
                    );
                }
            }
        }
    }

    let p2 = set.len();

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
