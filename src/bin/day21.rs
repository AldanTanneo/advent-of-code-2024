use std::iter::{repeat_n, Chain, RepeatN};

use hashbrown::HashMap;

type Seq = Chain<RepeatN<DirPad>, RepeatN<DirPad>>;

fn opt_seq(i: isize, j: isize) -> Seq {
    use DirPad::*;
    if j < 0 {
        repeat_n(Left, j.unsigned_abs())
            .chain(repeat_n(if i > 0 { Down } else { Up }, i.unsigned_abs()))
    } else {
        repeat_n(if i > 0 { Down } else { Up }, i.unsigned_abs())
            .chain(repeat_n(Right, j.unsigned_abs()))
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum KeyPad {
    Activate = b'A',
    Button0 = b'0',
    Button1 = b'1',
    Button2 = b'2',
    Button3 = b'3',
    Button4 = b'4',
    Button5 = b'5',
    Button6 = b'6',
    Button7 = b'7',
    Button8 = b'8',
    Button9 = b'9',
}

impl KeyPad {
    const fn parse(val: u8) -> Self {
        use KeyPad::*;
        match val {
            b'A' => Activate,
            b'0' => Button0,
            b'1' => Button1,
            b'2' => Button2,
            b'3' => Button3,
            b'4' => Button4,
            b'5' => Button5,
            b'6' => Button6,
            b'7' => Button7,
            b'8' => Button8,
            b'9' => Button9,
            _ => panic!("Invalid value"),
        }
    }

    const fn coords(self) -> (isize, isize) {
        use KeyPad::*;
        match self {
            Button7 => (0, 0),
            Button8 => (0, 1),
            Button9 => (0, 2),
            Button4 => (1, 0),
            Button5 => (1, 1),
            Button6 => (1, 2),
            Button1 => (2, 0),
            Button2 => (2, 1),
            Button3 => (2, 2),
            Button0 => (3, 1),
            Activate => (3, 2),
        }
    }

    fn go(&mut self, other: KeyPad) -> impl Iterator<Item = DirPad> {
        use DirPad::*;
        let (i1, j1) = self.coords();
        let (i2, j2) = other.coords();
        let (i, j) = (i2 - i1, j2 - j1);
        *self = other;

        if i1 == 3 && j2 == 0 {
            repeat_n(Up, i.unsigned_abs()).chain(repeat_n(Left, j.unsigned_abs()))
        } else if j1 == 0 && i2 == 3 {
            repeat_n(Right, j.unsigned_abs()).chain(repeat_n(Down, i.unsigned_abs()))
        } else {
            opt_seq(i, j)
        }
        .chain([Activate])
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
enum DirPad {
    Activate = b'A',
    Left = b'<',
    Up = b'^',
    Right = b'>',
    Down = b'v',
}

impl DirPad {
    const fn coords(self) -> (isize, isize) {
        use DirPad::*;
        match self {
            Up => (0, 1),
            Activate => (0, 2),
            Left => (1, 0),
            Down => (1, 1),
            Right => (1, 2),
        }
    }

    fn go(&mut self, other: Self) -> impl Iterator<Item = Self> {
        use DirPad::*;
        let (i1, j1) = self.coords();
        let (i2, j2) = other.coords();
        let (i, j) = (i2 - i1, j2 - j1);
        *self = other;

        if i1 == 0 && j2 == 0 {
            repeat_n(Down, i.unsigned_abs()).chain(repeat_n(Left, j.unsigned_abs()))
        } else if j1 == 0 && i2 == 0 {
            repeat_n(Right, j.unsigned_abs()).chain(repeat_n(Up, i.unsigned_abs()))
        } else {
            opt_seq(i, j)
        }
        .chain([Activate])
    }

    fn sequence_length(
        &mut self,
        to: DirPad,
        level: usize,
        cache: &mut HashMap<(usize, DirPad, DirPad), usize>,
    ) -> usize {
        let from = *self;
        *self = to;
        if level == 0 {
            return 1;
        } else if let Some(&res) = cache.get(&(level, from, to)) {
            return res;
        }

        let mut res = 0;
        let mut bot = DirPad::Activate;
        for next in from.clone().go(to) {
            res += bot.sequence_length(next, level - 1, cache);
        }

        cache.insert((level, from, to), res);
        res
    }
}

fn main() {
    let mut p1: usize = 0;
    let mut p2: usize = 0;

    let mut cache = HashMap::new();

    for code in aoc::input_lines(21) {
        let num: usize = aoc::parse_dec(&code[..3]);

        let mut keypad = KeyPad::Activate;
        let mut bot = DirPad::Activate;
        let mut len = 0;

        for next in code.bytes().flat_map(|v| keypad.go(KeyPad::parse(v))) {
            len += bot.sequence_length(next, 2, &mut cache);
        }
        p1 += num * len;

        len = 0;
        for next in code.bytes().flat_map(|v| keypad.go(KeyPad::parse(v))) {
            len += bot.sequence_length(next, 25, &mut cache);
        }
        p2 += num * len;
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
