use hashbrown::HashMap;
use itertools::Itertools;

struct Monkey(u32);

impl Monkey {
    fn next(&mut self) {
        self.0 ^= self.0 << 6;
        self.0 &= 0xffffff;
        self.0 ^= self.0 >> 5;
        self.0 ^= self.0 << 11;
        self.0 &= 0xffffff;
    }

    fn get(&self) -> u64 {
        self.0 as u64
    }

    fn price(&self) -> i8 {
        (self.0 % 10) as i8
    }
}

struct Price {
    last_update: u32,
    price: u32,
}

fn seq_to_u32(seq: (i8, i8, i8, i8)) -> u32 {
    unsafe { core::mem::transmute(seq) }
}

fn main() {
    let secrets = aoc::input_lines(22)
        .map(|s| aoc::parse_dec::<u32>(&s))
        .collect::<Vec<_>>();

    let p1: u64 = secrets
        .iter()
        .map(|seed| {
            let mut monkey = Monkey(*seed);
            for _ in 0..2000 {
                monkey.next();
            }
            monkey.get()
        })
        .sum();

    let mut sequences = HashMap::with_capacity(1 << 16);
    let mut p2 = 0;
    for (i, seed) in secrets.iter().enumerate() {
        let mut monkey = Monkey(*seed);
        (0..2001)
            .map(|_| {
                let old = monkey.price();
                monkey.next();
                old
            })
            .tuple_windows()
            .map(|(a, b, c, d, e)| ((b - a, c - b, d - c, e - d), e))
            .for_each(|(seq, price)| {
                let entry = sequences.entry(seq_to_u32(seq)).or_insert(Price {
                    last_update: u32::MAX,
                    price: 0,
                });
                if entry.last_update != i as u32 {
                    entry.last_update = i as u32;
                    entry.price += price as u32;
                }
                if entry.price > p2 {
                    p2 = entry.price;
                }
            });
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
