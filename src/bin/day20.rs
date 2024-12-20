use aoc::VecGrid;

fn main() {
    let input = aoc::input_bytes(20);
    let mut grid = VecGrid::new(input);

    let (mut i, mut j) = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|&c| c == b'S').map(|j| (i, j)))
        .unwrap();

    let mut dist = 0;
    let mut times = Vec::<((usize, usize), u32)>::with_capacity(8000);

    let mut p1 = 0;
    let mut p2 = 0;

    loop {
        for (prev, t) in times.iter().take_while(|(_, t)| dist - t > 100) {
            let d = prev.0.abs_diff(i) + prev.1.abs_diff(j);

            if d <= 20 && dist - t - d as u32 >= 100 {
                p2 += 1;
            }
            if d <= 2 && dist - t - d as u32 >= 100 {
                p1 += 1;
            }
        }

        if grid[i][j] == b'E' {
            break;
        }

        times.push(((i, j), dist));
        dist += 1;
        grid[i][j] = b'O';

        for (di, dj) in [(0, 1), (-1, 0), (0, -1), (1, 0)] {
            let ni = i.wrapping_add_signed(di);
            let nj = j.wrapping_add_signed(dj);

            if matches!(grid.get(ni, nj), Some(b'.' | b'E')) {
                (i, j) = (ni, nj);
                break;
            }
        }
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
