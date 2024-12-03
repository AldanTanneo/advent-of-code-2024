use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, u32},
    multi::fold_many0,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

enum Statement {
    Ignore,
    Mul(u32),
    Do,
    Dont,
}

pub trait FoldParser<I, O, E>: Parser<I, O, E> + Sized
where
    I: Clone + nom::InputLength,
    E: nom::error::ParseError<I>,
{
    fn fold<G, H, R>(self, init: H, g: G) -> impl FnMut(I) -> IResult<I, R, E>
    where
        G: FnMut(R, O) -> R,
        H: FnMut() -> R,
    {
        fold_many0(self, init, g)
    }
}

impl<I, O, E, P> FoldParser<I, O, E> for P
where
    P: Parser<I, O, E> + Sized,
    I: Clone + nom::InputLength,
    E: nom::error::ParseError<I>,
{
}

fn mul(input: &str) -> IResult<&str, u32> {
    delimited(tag("mul("), separated_pair(u32, char(','), u32), tag(")"))
        .map(|(a, b)| a * b)
        .parse(input)
}

fn do_dont(input: &str) -> IResult<&str, Statement> {
    tag("do()")
        .map(|_| Statement::Do)
        .or(tag("don't()").map(|_| Statement::Dont))
        .parse(input)
}

fn main() {
    let input = aoc::input_str(3);
    let input = input.as_str();

    let p1 = mul
        .or(anychar.map(|_| 0))
        .fold(|| 0, |acc, x| acc + x)
        .parse(input)
        .unwrap()
        .1;

    let (_, p2) = mul
        .map(Statement::Mul)
        .or(do_dont)
        .or(anychar.map(|_| Statement::Ignore))
        .fold(
            || (true, 0),
            |(enabled, acc), stmt| match stmt {
                Statement::Mul(n) if enabled => (enabled, acc + n),
                Statement::Do => (true, acc),
                Statement::Dont => (false, acc),
                _ => (enabled, acc),
            },
        )
        .parse(input)
        .unwrap()
        .1;

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
