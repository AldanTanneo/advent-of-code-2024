use aoc::VecGrid;

fn check_move(grid: &VecGrid, (i, j): (usize, usize), (di, dj): (isize, isize)) -> bool {
    let new_i = i.wrapping_add_signed(di);
    let new_j = j.wrapping_add_signed(dj);
    let new_c = grid.get(new_i, new_j);
    match new_c {
        Some(b'.') => true,
        Some(b'[' | b']') if di == 0 => {
            check_move(grid, (new_i, new_j.wrapping_add_signed(dj)), (di, dj))
        }
        Some(b'[') => {
            check_move(grid, (new_i, new_j), (di, dj))
                && check_move(grid, (new_i, new_j + 1), (di, dj))
        }
        Some(b']') => {
            check_move(grid, (new_i, new_j), (di, dj))
                && check_move(grid, (new_i, new_j - 1), (di, dj))
        }
        _ => false,
    }
}

fn do_move(grid: &mut VecGrid, (i, j): (usize, usize), (di, dj): (isize, isize)) {
    let new_i = i.wrapping_add_signed(di);
    let new_j = j.wrapping_add_signed(dj);
    let new_c = grid.get(new_i, new_j);
    match new_c {
        Some(b'[' | b']') if di == 0 => {
            do_move(grid, (new_i, new_j.wrapping_add_signed(dj)), (di, dj));
            do_move(grid, (new_i, new_j), (di, dj));
        }
        Some(b'[') => {
            do_move(grid, (new_i, new_j), (di, dj));
            do_move(grid, (new_i, new_j + 1), (di, dj));
        }
        Some(b']') => {
            do_move(grid, (new_i, new_j), (di, dj));
            do_move(grid, (new_i, new_j - 1), (di, dj));
        }
        _ => (),
    }
    grid[new_i][new_j] = grid[i][j];
    grid[i][j] = b'.';
}

fn main() {
    let input = aoc::input_str(15);
    let grid_size = input.find("\n\n").unwrap();
    let mut grid = VecGrid::new(input.as_bytes()[..=grid_size].to_vec());
    let moves = input[grid_size..].as_bytes();

    let mut bot = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, c)| (*c == b'@').then_some((i, j)))
        })
        .unwrap();

    let mut grid2 = Vec::new();
    for line in grid.iter() {
        for c in line {
            match c {
                b'#' => grid2.extend([b'#'; 2]),
                b'O' => grid2.extend([b'[', b']']),
                b'.' => grid2.extend([b'.'; 2]),
                b'@' => grid2.extend([b'@', b'.']),
                _ => continue,
            }
        }
        grid2.push(b'\n')
    }

    for m in moves {
        let (di, dj) = match m {
            b'>' => (0, 1),
            b'<' => (0, -1),
            b'v' => (1, 0),
            b'^' => (-1, 0),
            _ => continue,
        };

        let mut k = 1;
        loop {
            let new_i = bot.0.wrapping_add_signed(k * di);
            let new_j = bot.1.wrapping_add_signed(k * dj);
            let moved = grid.get(new_i, new_j);
            if moved.is_none() || moved == Some(b'#') {
                break;
            } else if moved == Some(b'.') {
                if k > 1 {
                    grid[new_i][new_j] = b'O';
                }
                grid[bot.0][bot.1] = b'.';
                bot.0 = bot.0.wrapping_add_signed(di);
                bot.1 = bot.1.wrapping_add_signed(dj);
                grid[bot.0][bot.1] = b'@';
                break;
            } else {
                k += 1;
            }
        }
    }

    let mut p1 = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == b'O' {
                p1 += 100 * i + j;
            }
        }
    }

    let mut grid = VecGrid::new(grid2);

    let mut bot = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, c)| (*c == b'@').then_some((i, j)))
        })
        .unwrap();

    for m in moves {
        let (di, dj) = match m {
            b'>' => (0, 1),
            b'<' => (0, -1),
            b'v' => (1, 0),
            b'^' => (-1, 0),
            _ => continue,
        };

        if check_move(&grid, bot, (di, dj)) {
            do_move(&mut grid, bot, (di, dj));
            bot.0 = bot.0.wrapping_add_signed(di);
            bot.1 = bot.1.wrapping_add_signed(dj);
        }
    }

    let mut p2 = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == b'[' {
                p2 += 100 * i + j;
            }
        }
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
