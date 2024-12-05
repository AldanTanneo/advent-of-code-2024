use std::cmp::Ordering;

fn main() {
    let input = aoc::input_str(5);
    let (constraints, updates) = input.split_once("\n\n").unwrap();

    let mut constraints = constraints
        .split_ascii_whitespace()
        .flat_map(|c| c.split_once('|'))
        .map(|(a, b)| (aoc::parse_dec::<u32>(a), aoc::parse_dec::<u32>(b)))
        .collect::<Vec<_>>();
    constraints.sort_unstable();
    let mut updates = updates
        .split_ascii_whitespace()
        .map(|c| c.split(',').map(aoc::parse_dec::<u32>).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut p1 = 0;
    let mut p2 = 0;

    for u in &mut updates {
        if constraints.iter().all(|(n, m)| {
            !u.contains(n) || u.iter().take_while(|&p| p != n).find(|&p| p == m).is_none()
        }) {
            p1 += u[u.len() / 2];
        } else {
            u.sort_unstable_by(|a, b| {
                if constraints.binary_search(&(*a, *b)).is_ok() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            p2 += u[u.len() / 2];
        }
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
