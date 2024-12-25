fn main() {
    let input = aoc::input_str(25);

    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut cur = [0; 5];
    let mut chr = 0;

    for line in input.split('\n') {
        if chr == 0 {
            if line == "#####" {
                chr = b'#';
            } else {
                chr = b'.';
            }
        }
        if line.is_empty() {
            if chr == b'#' {
                keys.push(cur);
            } else {
                locks.push(cur);
            }
            cur = [0; 5];
            chr = 0;
        } else {
            for (i, b) in line.bytes().enumerate() {
                if b == chr {
                    cur[i] += 1;
                }
            }
        }
    }

    let mut p1 = 0;

    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock).all(|(k, l)| k <= l) {
                p1 += 1;
            }
        }
    }

    println!("p1 = {p1}");
}
