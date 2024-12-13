use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};

#[derive(Debug, Clone)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn coords(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(
        preceded(tag("X+"), i64),
        tag(", "),
        preceded(tag("Y+"), i64),
    )
    .parse(input)
}

fn prize(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(
        preceded(tag("X="), i64),
        tag(", "),
        preceded(tag("Y="), i64),
    )
    .parse(input)
}

fn machine(input: &str) -> IResult<&str, Machine> {
    tuple((
        separated_pair(
            preceded(tag("Button A: "), coords),
            newline,
            preceded(tag("Button B: "), coords),
        ),
        preceded(tag("\nPrize: "), prize),
    ))
    .map(|((a, b), prize)| Machine { a, b, prize })
    .parse(input)
}

fn separated_iter0<'a, I, O1, O2: 'a, E>(
    input: &'a I,
    mut sep: impl Parser<&'a I, O1, E> + 'a,
    mut f: impl Parser<&'a I, O2, E> + 'a,
) -> impl Iterator<Item = O2> + 'a
where
    I: ?Sized,
{
    let fst = f.parse(input);
    std::iter::successors(fst.ok(), move |(i, _)| {
        sep.parse(i).ok().and_then(|(i, _)| f.parse(i).ok())
    })
    .map(|(_, o)| o)
}

fn main() {
    let input = aoc::input_str(13);

    let p1: i64 = separated_iter0(input.as_str(), tag("\n\n"), machine)
        .map(|Machine { a, b, prize: p }| {
            let delta = b.0 * a.1 - a.0 * b.1;
            let alpha = (p.1 * b.0 - p.0 * b.1) / delta;
            let beta = (p.0 * a.1 - p.1 * a.0) / delta;
            if (0..=100).contains(&alpha)
                && (0..=100).contains(&beta)
                && alpha * a.0 + beta * b.0 == p.0
                && alpha * a.1 + beta * b.1 == p.1
            {
                3 * alpha + beta
            } else {
                0
            }
        })
        .sum();

    let p2: i64 = separated_iter0(input.as_str(), tag("\n\n"), machine)
        .map(|Machine { a, b, prize: p }| {
            let p = (p.0 + 10000000000000, p.1 + 10000000000000);
            let delta = b.0 * a.1 - a.0 * b.1;
            let alpha = (p.1 * b.0 - p.0 * b.1) / delta;
            let beta = (p.0 * a.1 - p.1 * a.0) / delta;
            if alpha * a.0 + beta * b.0 == p.0 && alpha * a.1 + beta * b.1 == p.1 {
                3 * alpha + beta
            } else {
                0
            }
        })
        .sum();

    println!("p1 = {p1}");
    println!("p1 = {p2}");
}
