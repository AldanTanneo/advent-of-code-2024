fn main() {
    let (mut a, mut b): (Vec<_>, Vec<_>) = aoc::input_lines(1)
        .flat_map(|line| {
            line.split_once("   ").map(|(a, b)| {
                let a = aoc::parse_dec::<u32>(a);
                let b = aoc::parse_dec::<u32>(b);
                (a, b)
            })
        })
        .unzip();

    a.sort_unstable();
    b.sort_unstable();

    let p1: u32 = a.iter().zip(&b).map(|(a, b)| a.abs_diff(*b)).sum();

    let mut p2 = 0;
    let mut b_iter = b.iter().copied();
    let mut b_prev = b_iter.next().unwrap();
    let mut x_prev = None;
    let mut sum_prev = 0;
    for x in a.iter().copied() {
        p2 += sum_prev;
        if Some(x) == x_prev {
            continue;
        }
        sum_prev = 0;
        while x >= b_prev {
            if b_prev == x {
                sum_prev += x;
            }
            let Some(prev) = b_iter.next() else { break };
            b_prev = prev;
        }
        x_prev = Some(x);
    }
    p2 += sum_prev;

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
