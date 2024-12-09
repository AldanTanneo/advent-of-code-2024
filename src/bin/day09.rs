fn main() {
    let input = aoc::input_str(9);
    let input = input.trim();

    let forward = input
        .bytes()
        .step_by(2)
        .enumerate()
        .flat_map(|(id, size)| std::iter::repeat_n(id, (size - b'0') as usize));
    let mut backward = forward.rev();

    let total: usize = input.bytes().step_by(2).map(|s| (s - b'0') as usize).sum();

    let mut p1 = 0;
    let mut pos = 0;
    for (id, [file, free]) in input
        .as_bytes()
        .chunks_exact(2)
        .map(|s| <[u8; 2]>::try_from(s).unwrap().map(|s| (s - b'0') as usize))
        .enumerate()
    {
        for p in pos..(pos + file).min(total) {
            p1 += p * id;
        }
        pos += file;
        for (p, id) in (pos..(pos + free).min(total)).zip(backward.by_ref()) {
            p1 += p * id;
        }
        pos += free;

        if pos > total {
            break;
        }
    }

    let forward = input
        .bytes()
        .step_by(2)
        .map(|s| (s - b'0') as usize)
        .enumerate();
    let backward = forward.rev();

    let mut free = input
        .bytes()
        .map(|s| (s - b'0') as usize)
        .scan(0, |pos, size| {
            let tmp = *pos;
            *pos += size;
            Some((tmp, size))
        })
        .collect::<Vec<_>>();

    let mut p2 = 0;

    for (id, size) in backward {
        if let Some((fid, (pos, _))) = free[..2 * id]
            .iter()
            .skip(1)
            .step_by(2)
            .cloned()
            .enumerate()
            .find(|(_, (_, s))| *s >= size)
        {
            free[2 * fid + 1].0 += size;
            free[2 * fid + 1].1 -= size;
            for p in pos..pos + size {
                p2 += p * id;
            }
        } else {
            let pos = free[2 * id].0;
            for p in pos..pos + size {
                p2 += p * id;
            }
        }
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
