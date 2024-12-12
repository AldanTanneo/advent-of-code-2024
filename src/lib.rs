use std::fmt::Write;
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

pub fn input_bytes(day: u8) -> Vec<u8> {
    std::fs::read(day_file(day))
        .unwrap_or_else(|err| panic!("Could not read file for day {day}: {err}\n{err:?}"))
}

pub fn parse_dec<T: From<u8> + std::ops::Add<Output = T> + std::ops::Mul<Output = T>>(
    s: &str,
) -> T {
    s.bytes()
        .fold(T::from(0), |acc, c| T::from(10) * acc + T::from(c & 0b1111))
}

pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();

    loop {
        v >>= v.trailing_zeros();

        if u > v {
            core::mem::swap(&mut u, &mut v);
        }

        v -= u; // here v >= u

        if v == 0 {
            break;
        }
    }

    u << shift
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Grid<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Grid<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let width = data.iter().position(|&b| b == b'\n').unwrap() + 1;
        let height = data.len() / width;

        Self {
            data,
            width,
            height,
        }
    }

    pub fn len(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width - 1
    }

    pub fn iter(&'a self) -> impl DoubleEndedIterator<Item = &'a [u8]> {
        (0..self.len()).map(move |i| &self[i])
    }
}

impl<'a> std::ops::Index<usize> for Grid<'a> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.width * index..self.width * (index + 1) - 1]
    }
}

impl std::fmt::Debug for Grid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.iter() {
            for c in l {
                f.write_char(*c as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct VecGrid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl VecGrid {
    pub fn new(data: Vec<u8>) -> Self {
        let width = data.iter().position(|&b| b == b'\n').unwrap() + 1;
        let height = data.len() / (width);

        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, y: usize, x: usize) -> Option<u8> {
        if y < self.len() && x < self.width() {
            Some(self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width - 1
    }

    pub fn grid(&self) -> Grid<'_> {
        Grid {
            data: &self.data,
            width: self.width,
            height: self.height,
        }
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &[u8]> {
        (0..self.len()).map(move |i| &self[i])
    }

    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut [u8]> {
        let w = self.width();
        self.data
            .chunks_exact_mut(self.width)
            .map(move |s| &mut s[..w])
    }
}

impl std::ops::Index<usize> for VecGrid {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.width * index..self.width * (index + 1) - 1]
    }
}

impl std::ops::IndexMut<usize> for VecGrid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[self.width * index..self.width * (index + 1) - 1]
    }
}

impl std::fmt::Debug for VecGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid().fmt(f)
    }
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
