use std::usize;

use itertools::Itertools;

const REG_A: usize = 4;
const REG_B: usize = 5;
const REG_C: usize = 6;

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

#[inline(always)]
fn stepi(instr: u8, op: usize, pc: &mut usize, puter: &mut [usize; 8]) -> Option<u8> {
    let mut out = None;

    match instr {
        ADV => puter[REG_A] >>= puter[op],
        BXL => puter[REG_B] ^= op,
        BST => puter[REG_B] = puter[op] & 0b111,
        JNZ => {
            if puter[REG_A] != 0 {
                *pc = op;
                return None;
            }
        }
        BXC => puter[REG_B] ^= puter[REG_C],
        OUT => out = Some((puter[op] & 0b111) as u8),
        BDV => puter[REG_B] = puter[REG_A] >> puter[op],
        CDV => puter[REG_C] = puter[REG_A] >> puter[op],
        _ => panic!("Invalid instruction"),
    }

    *pc += 2;
    out
}

fn run(program: &[u8], [a, b, c]: [usize; 3]) -> Vec<u8> {
    let mut pc = 0;
    let mut puter = [0, 1, 2, 3, a, b, c, 0];
    let mut stdout = Vec::new();

    loop {
        let Some(&[instr, op]) = program.get(pc..pc + 2) else {
            break;
        };
        let out = stepi(instr, op as usize, &mut pc, &mut puter);
        if let Some(out) = out {
            stdout.push(out);
        }
    }

    stdout
}

fn find_quine(prog: &[u8], n: usize, [a, b, c]: [usize; 3]) -> bool {
    assert!(n <= prog.len());

    let mut i = 0;
    let mut pc = 0;
    let mut puter = [0, 1, 2, 3, a, b, c, 0];
    while i < n {
        let Some(&[instr, op]) = prog.get(pc..pc + 2) else {
            break;
        };
        let out = stepi(instr & 0b111, op as usize & 0b111, &mut pc, &mut puter);
        if let Some(out) = out {
            if prog[i] == out {
                i += 1;
            } else {
                break;
            }
        }
    }
    i == n
}

fn main() {
    let input = aoc::input_str(17);

    let (a, rest) = input.split_once('\n').unwrap();
    let (b, rest) = rest.split_once('\n').unwrap();
    let (c, rest) = rest.split_once('\n').unwrap();

    let a: usize = aoc::parse_dec(a.trim_start_matches("Register A: "));
    let b = aoc::parse_dec(b.trim_start_matches("Register B: "));
    let c = aoc::parse_dec(c.trim_start_matches("Register C: "));

    let program = rest
        .trim_ascii()
        .trim_start_matches("Program: ")
        .bytes()
        .step_by(2)
        .map(|i| i & 0b111)
        .collect::<Vec<_>>();
    let prog = program.as_slice();

    let out = run(prog, [a, b, c]);
    let p1 = out.iter().join(",");

    let mut it: Box<dyn Iterator<Item = usize>> = Box::new(0..(1 << 11));

    for (i, k) in (11..program.len() * 3).step_by(3).enumerate() {
        it = Box::new(
            it.filter(move |a| find_quine(prog, i + 1, [*a, b, c]))
                .flat_map(move |a| (0..8).map(move |b| a | (b << k))),
        );
    }

    let mut it = it.filter(|a| find_quine(prog, prog.len(), [*a, b, c]));

    let p2 = it.next().unwrap();

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
