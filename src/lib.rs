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
