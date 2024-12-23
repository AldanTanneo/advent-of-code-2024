use std::{fmt::Debug, hash::Hash};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Puter = [u8; 2];

fn hash_triangle(a: Puter, b: Puter, c: Puter) -> u64 {
    let (mut a, mut b, mut c) = (
        u16::from_le_bytes(a),
        u16::from_le_bytes(b),
        u16::from_le_bytes(c),
    );

    if a > b {
        (a, b) = (b, a);
    }
    if b > c {
        (b, c) = (c, b);
    }
    if a > b {
        (a, b) = (b, a);
    }

    unsafe { std::mem::transmute([a, b, c, 0]) }
}

struct Triangle([Puter; 3]);

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        hash_triangle(self.0[0], self.0[1], self.0[2])
            == hash_triangle(other.0[0], other.0[1], other.0[2])
    }
}

impl Eq for Triangle {}

impl Hash for Triangle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        hash_triangle(self.0[0], self.0[1], self.0[2]).hash(state);
    }
}

impl Debug for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{},{}{},{}{}",
            self.0[0][0] as char,
            self.0[0][1] as char,
            self.0[1][0] as char,
            self.0[1][1] as char,
            self.0[2][0] as char,
            self.0[2][1] as char
        )
    }
}

fn bronkerbosch2(
    map: &HashMap<Puter, HashSet<Puter>>,
    r: Vec<Puter>,
    mut p: HashSet<Puter>,
    mut x: HashSet<Puter>,
    max: &mut Vec<Puter>,
) {
    if let Some(u) = p.union(&x).next() {
        let pnu = p.difference(&map[u]).copied().collect::<Vec<_>>();
        for v in pnu {
            let nv = &map[&v];
            let p2 = &p & nv;
            if r.len() + p2.len() >= max.len() {
                let mut r2 = r.clone();
                r2.push(v);
                bronkerbosch2(map, r2, p2, &x & nv, max);
            }
            p.remove(&v);
            x.insert(v);
        }
    } else if r.len() > max.len() {
        *max = r;
    }
}

fn main() {
    let input = aoc::input_lines(23)
        .map(|l| {
            (
                Puter::try_from(&l.as_bytes()[0..2]).unwrap(),
                Puter::try_from(&l.as_bytes()[3..5]).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut map: HashMap<Puter, HashSet<_>> =
        input.iter().fold(HashMap::new(), |mut map, &(a, b)| {
            map.entry(a).or_default().insert(b);
            map.entry(b).or_default().insert(a);
            map
        });
    map.shrink_to_fit();

    let mut set = HashSet::new();
    for (a, b) in &input {
        set.extend(map[a].intersection(&map[b]).filter_map(|c| {
            (a[0] == b't' || b[0] == b't' || c[0] == b't').then_some(Triangle([*a, *b, *c]))
        }))
    }

    let p1 = set.len();

    let r = Vec::new();
    let p = map.keys().copied().collect();
    let x = HashSet::new();
    let mut max = Vec::new();
    bronkerbosch2(&map, r, p, x, &mut max);
    max.sort_unstable();
    let p2 = max
        .iter()
        .flat_map(|s| core::str::from_utf8(s).ok())
        .join(",");

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
