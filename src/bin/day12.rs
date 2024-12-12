use std::collections::VecDeque;

fn main() {
    let input: Vec<u8> = aoc::input_bytes(12);
    let mut grid = aoc::VecGrid::new(input);
    let mut queue = VecDeque::new();

    let (mut i, mut j) = (0, 0);
    let mut p1 = 0;
    let mut p2 = 0;

    while grid[i][j] != 0 {
        let plant = grid[i][j];

        grid[i][j] |= 128;
        queue.push_back((i, j));

        let mut area = 0;
        let mut perimeter = 0;
        let mut sides = 0;

        while let Some((y, x)) = queue.pop_front() {
            area += 1;

            for dy in [-1, 1] {
                for dx in [-1, 1] {
                    let ry = (dx - dy) / 2;
                    let rx = (dx + dy) / 2;

                    let yn = y.wrapping_add_signed(ry);
                    let xn = x.wrapping_add_signed(rx);

                    let n = grid.get(yn, xn).unwrap_or_default();
                    if n == plant {
                        grid[yn][xn] |= 128;
                        queue.push_back((yn, xn));
                    } else if n & 127 != plant {
                        perimeter += 1;
                    }

                    let y2 = y.wrapping_add_signed(dy);
                    let x2 = x.wrapping_add_signed(dx);

                    let a = grid.get(y2, x).unwrap_or_default();
                    let c = grid.get(y, x2).unwrap_or_default();
                    let b = grid.get(y2, x2).unwrap_or_default();

                    if (a & 127 != plant && c & 127 != plant)
                        || (a & 127 == plant && c & 127 == plant && b & 127 != plant)
                    {
                        sides += 1
                    }
                }
            }
        }

        p1 += perimeter * area;
        p2 += sides * area;

        while i < grid.len() {
            while j < grid.width() {
                if grid[i][j] < 128 {
                    break;
                }
                j += 1;
            }
            if j == grid.width() {
                j = 0;
            } else {
                break;
            }
            i += 1;
        }
        if i == grid.len() {
            break;
        }
    }

    println!("p1 = {p1}");
    println!("p1 = {p2}");
}
