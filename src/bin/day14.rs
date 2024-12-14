const WIDTH: u64 = 101;
const HEIGHT: u64 = 103;

const EGCD_WIDTH: i64 = aoc::extended_gcd(WIDTH, HEIGHT).1;
const EGCD_HEIGHT: i64 = aoc::extended_gcd(WIDTH, HEIGHT).2;

fn addm(p: u64, v: i64, m: u64) -> u64 {
    (p + m).wrapping_add_signed(v).rem_euclid(m)
}

#[derive(Clone, Copy)]
struct Robot {
    pos: (u64, u64),
    vel: (i64, i64),
}

impl Robot {
    fn step(&mut self) {
        self.pos.0 = addm(self.pos.0, self.vel.0, WIDTH);
        self.pos.1 = addm(self.pos.1, self.vel.1, HEIGHT);
    }
}

fn main() {
    let mut robots = aoc::input_lines(14)
        .filter_map(|line| {
            let (p, v) = line.split_once(' ')?;
            let (px, py) = p.trim_start_matches("p=").split_once(',')?;
            let (vx, vy) = v.trim_start_matches("v=").split_once(',')?;

            Some(Robot {
                pos: (px.parse().ok()?, py.parse().ok()?),
                vel: (vx.parse().ok()?, vy.parse().ok()?),
            })
        })
        .collect::<Vec<_>>();

    let len = robots.len() as u64;

    let mut p1 = 0;

    let mut min_varx = (0, u64::MAX);
    let mut min_vary = (0, u64::MAX);

    for i in 1..=103 {
        for r in &mut robots {
            r.step();
        }

        if i == 100 {
            let mut quads = [0; 4];
            for r in &robots {
                if r.pos.0 < WIDTH / 2 {
                    if r.pos.1 < HEIGHT / 2 {
                        quads[0] += 1;
                    } else if r.pos.1 > HEIGHT / 2 {
                        quads[1] += 1;
                    }
                } else if r.pos.0 > WIDTH / 2 {
                    if r.pos.1 < HEIGHT / 2 {
                        quads[2] += 1;
                    } else if r.pos.1 > HEIGHT / 2 {
                        quads[3] += 1;
                    }
                }
            }
            p1 = quads.iter().product();
        }

        let (varx, vary) = robots
            .iter()
            .map(|r| r.pos)
            .fold(((0, 0), (0, 0)), |(mx, my), (x, y)| {
                ((mx.0 + x, mx.1 + x.pow(2)), (my.0 + y, my.1 + y.pow(2)))
            });

        let (varx, vary) = (varx.1 * len - varx.0.pow(2), vary.1 * len - vary.0.pow(2));

        if varx < min_varx.1 {
            min_varx = (i, varx);
        }
        if vary < min_vary.1 {
            min_vary = (i, vary);
        }
    }

    // chinese remainder theorem with min variance on each axis
    // (repeats every WIDTH seconds for x, every HEIGHT seconds for y)
    let p2 = (min_vary.0 * EGCD_WIDTH * WIDTH as i64 + min_varx.0 * EGCD_HEIGHT * HEIGHT as i64)
        .rem_euclid((WIDTH * HEIGHT) as i64);

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
