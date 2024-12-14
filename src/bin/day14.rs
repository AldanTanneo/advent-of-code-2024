const WIDTH: usize = 101;
const HEIGHT: usize = 103;

fn addm(p: usize, v: isize, m: usize) -> usize {
    (p + m).wrapping_add_signed(v) % m
}

#[derive(Clone, Copy)]
struct Robot {
    pos: (usize, usize),
    vel: (isize, isize),
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

    let mut p1 = 0;
    let mut p2 = 0;
    let mut danger = u32::MAX;

    for i in 1.. {
        for r in &mut robots {
            r.step();
        }

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

        let new_danger = quads.into_iter().product();

        if i == 100 {
            p1 = new_danger;
        }

        if new_danger < danger {
            danger = new_danger;

            // if danger reached a new minimum: compute the variance
            let (varx, vary) = robots
                .iter()
                .map(|r| r.pos)
                .fold(((0, 0), (0, 0)), |(mx, my), (x, y)| {
                    ((mx.0 + x, mx.1 + x.pow(2)), (my.0 + y, my.1 + y.pow(2)))
                });

            let (varx, vary) = (
                (varx.1 - varx.0.pow(2) / robots.len()) / robots.len(),
                (vary.1 - vary.0.pow(2) / robots.len()) / robots.len(),
            );

            // check for acceptable variance values for image
            if varx < 500 && vary < 500 {
                p2 = i;
                break;
            }
        }
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
