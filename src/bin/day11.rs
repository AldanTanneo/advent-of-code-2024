use hashbrown::HashMap;

fn blink(map: &mut HashMap<u64, u64>, buf: &mut HashMap<u64, u64>) {
    for (k, v) in map.drain() {
        if k == 0 {
            *buf.entry(1).or_default() += v;
        } else {
            let digits = k.ilog10() + 1;
            if digits % 2 == 0 {
                let half_digits = 10u64.pow(digits / 2);
                let left = k / half_digits;
                let right = k % half_digits;
                *buf.entry(left).or_default() += v;
                *buf.entry(right).or_default() += v;
            } else {
                *buf.entry(k * 2024).or_default() += v;
            }
        }
    }
    core::mem::swap(map, buf);
}

fn main() {
    let input = aoc::input_str(11);

    let mut map = input
        .split_ascii_whitespace()
        .map(|s| (aoc::parse_dec::<u64>(s), 1))
        .collect::<HashMap<_, _>>();
    let mut buf = HashMap::with_capacity(map.len());

    for _ in 0..25 {
        blink(&mut map, &mut buf);
    }

    let p1 = map.values().sum::<u64>();

    for _ in 0..50 {
        blink(&mut map, &mut buf);
    }

    let p2 = map.values().sum::<u64>();

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
