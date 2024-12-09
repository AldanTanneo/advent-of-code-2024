fn main() {
    let mut tmp = Vec::with_capacity(256);

    let p1: u64 = aoc::input_lines(7)
        .filter(|x| !x.is_empty())
        .filter_map(|l| {
            let (res, nums) = l.split_once(": ")?;
            let res = aoc::parse_dec::<u64>(res);

            for (i, n) in nums
                .split_ascii_whitespace()
                .map(aoc::parse_dec::<u64>)
                .enumerate()
            {
                if i == 0 {
                    tmp.push(n);
                } else {
                    let len = tmp.len();
                    for i in 0..len {
                        let mul = tmp[i].checked_mul(n).filter(|&x| x <= res);
                        if let Some(mul) = mul {
                            tmp.push(mul);
                        }
                    }
                    for elt in &mut tmp[..len] {
                        *elt += n;
                    }
                }
            }

            let found = tmp.iter().copied().find(|x| *x == res);
            tmp.clear();
            found
        })
        .sum();

    let p2: u64 = aoc::input_lines(7)
        .filter(|x| !x.is_empty())
        .filter_map(|l| {
            let (res, nums) = l.split_once(": ")?;
            let res = aoc::parse_dec::<u64>(res);

            for (i, n) in nums
                .split_ascii_whitespace()
                .map(aoc::parse_dec::<u64>)
                .enumerate()
            {
                if i == 0 {
                    tmp.push(n);
                } else {
                    let len = tmp.len();
                    for i in 0..len {
                        let mul = tmp[i].checked_mul(n).filter(|&x| x <= res);
                        if let Some(mul) = mul {
                            tmp.push(mul);
                        }
                    }
                    for i in 0..len {
                        let concat = 10u64
                            .checked_pow(n.max(1).ilog10() + 1)
                            .and_then(|x| x.checked_mul(tmp[i]))
                            .and_then(|x| x.checked_add(n))
                            .filter(|&x| x <= res);
                        if let Some(concat) = concat {
                            tmp.push(concat);
                        }
                    }
                    for elt in &mut tmp[..len] {
                        *elt += n;
                    }
                }
            }

            let found = tmp.iter().copied().find(|x| *x == res);
            tmp.clear();
            found
        })
        .sum();

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
