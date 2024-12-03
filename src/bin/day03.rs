use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, u32},
    combinator::map,
    multi::fold_many0,
    sequence::{delimited, separated_pair},
    IResult,
};

enum Statement {
    Mul(u32),
    Ignore,
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, u32> {
    map(
        delimited(tag("mul("), separated_pair(u32, char(','), u32), tag(")")),
        |(a, b)| a * b,
    )(input)
}

fn do_dont(input: &str) -> IResult<&str, Statement> {
    alt((
        map(tag("do()"), |_| Statement::Do),
        map(tag("don't()"), |_| Statement::Dont),
    ))(input)
}

fn main() {
    let input = aoc::input_str(3);
    let input = input.as_str();

    let p1 = fold_many0(alt((mul, map(anychar, |_| 0))), || 0, |acc, x| acc + x)(input)
        .unwrap()
        .1;

    let (_, p2) = fold_many0(
        alt((
            map(mul, Statement::Mul),
            do_dont,
            map(anychar, |_| Statement::Ignore),
        )),
        || (true, 0),
        |(enabled, acc), stmt| match stmt {
            Statement::Mul(n) if enabled => (enabled, acc + n),
            Statement::Do => (true, acc),
            Statement::Dont => (false, acc),
            _ => (enabled, acc),
        },
    )(input)
    .unwrap()
    .1;

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
