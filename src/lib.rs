use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn day_file(day: u8) -> PathBuf {
    if day == 0 {
        return "test.txt".into();
    }
    format!("input/day{day:02}.txt").into()
}

pub fn input_lines(day: u8) -> impl Iterator<Item = String> {
    BufReader::new(std::fs::File::open(day_file(day)).unwrap())
        .lines()
        .map_while(Result::ok)
}

pub fn input_str(day: u8) -> String {
    std::fs::read_to_string(day_file(day))
        .unwrap_or_else(|err| panic!("Could not read file for day {day}: {err}\n{err:?}"))
}

pub fn parse_dec<T: From<u8> + std::ops::Add<Output = T> + std::ops::Mul<Output = T>>(
    s: &str,
) -> T {
    s.bytes()
        .fold(T::from(0), |acc, c| T::from(10) * acc + T::from(c & 0b1111))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dec_u32() {
        assert_eq!(parse_dec::<u32>("0"), 0);
        assert_eq!(parse_dec::<u32>("123"), 123);
        assert_eq!(parse_dec::<u32>("123456789"), 123456789);
        assert_eq!(parse_dec::<u32>("4294967295"), u32::MAX);
    }
}
