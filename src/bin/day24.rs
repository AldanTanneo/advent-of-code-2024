use hashbrown::HashMap;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, one_of},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult, Parser,
};

fn init(input: &str) -> IResult<&str, (&str, bool)> {
    separated_pair(
        alphanumeric1,
        tag(": "),
        one_of("01").map(|c| match c {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        }),
    )
    .parse(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GateKind {
    And,
    Or,
    Xor,
}

fn gate_kind(input: &str) -> IResult<&str, GateKind> {
    alt((
        tag("AND").map(|_| GateKind::And),
        tag("OR").map(|_| GateKind::Or),
        tag("XOR").map(|_| GateKind::Xor),
    ))
    .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate<'a> {
    kind: GateKind,
    a: &'a str,
    b: &'a str,
}

fn gate(input: &str) -> IResult<&str, (&str, Gate<'_>)> {
    tuple((
        alphanumeric1,
        delimited(tag(" "), gate_kind, tag(" ")),
        alphanumeric1,
        preceded(tag(" -> "), alphanumeric1),
    ))
    .map(|(a, gate, b, out)| {
        (
            out,
            Gate {
                kind: gate,
                a: a.min(b),
                b: a.max(b),
            },
        )
    })
    .parse(input)
}

type Values<'a> = HashMap<&'a str, bool>;
type Gates<'a> = HashMap<&'a str, Gate<'a>>;

fn parse_input(input: &str) -> IResult<&str, (Values<'_>, Gates<'_>)> {
    separated_pair(
        separated_list1(newline, init).map(|v| v.into_iter().collect()),
        pair(newline, newline),
        separated_list1(newline, gate).map(|v| v.into_iter().collect()),
    )
    .parse(input)
}

fn output(register: &str, values: &mut Values<'_>, gates: &Gates<'_>) -> bool {
    if let Some(&val) = values.get(register) {
        val
    } else if let Some(gate) = gates.get(register) {
        let a = output(gate.a, values, gates);
        let b = output(gate.b, values, gates);
        match gate.kind {
            GateKind::And => a && b,
            GateKind::Or => a || b,
            GateKind::Xor => a ^ b,
        }
    } else {
        panic!("could not compute register value")
    }
}

fn main() {
    let input = aoc::input_str(24);
    let (_, (mut values, gates)) = parse_input(&input).unwrap();

    let mut p1 = 0;
    let mut i = 0;
    loop {
        let reg = format!("z{i:02}");
        if gates.contains_key(reg.as_str()) || values.contains_key(reg.as_str()) {
            let bit = output(&reg, &mut values, &gates);
            p1 |= (bit as u64) << i;
            i += 1;
        } else {
            break;
        }
    }
    let n = i;

    let inv_map: HashMap<_, _> = gates
        .iter()
        .map(|(out, gate)| (gate.clone(), *out))
        .collect();

    // get the first carry
    let mut carry = inv_map[&Gate {
        a: "x00",
        b: "y00",
        kind: GateKind::And,
    }];

    let mut to_swap = Vec::new();

    // hypothesis: the first and last output are correct, and
    // swaps are internal to the full adders
    for i in 1..n - 1 {
        let xi = format!("x{i:02}");
        let xi = xi.as_str();
        let yi = format!("y{i:02}");
        let yi = yi.as_str();
        let zi = format!("z{i:02}");
        let zi = zi.as_str();

        let xor = inv_map[&Gate {
            a: xi,
            b: yi,
            kind: GateKind::Xor,
        }];
        let and = inv_map[&Gate {
            a: xi,
            b: yi,
            kind: GateKind::And,
        }];

        // induction: we get the correct carry in the previous iteration
        // so the second XOR gate has `carry` as operand
        let sum = if let Some((gate, out)) = inv_map.get_key_value(&Gate {
            a: xor.min(carry),
            b: xor.max(carry),
            kind: GateKind::Xor,
        }) {
            (*out, gate)
        } else {
            gates
                .iter()
                .find(|(_, gate)| {
                    gate.kind == GateKind::Xor && (gate.a == carry || gate.b == carry)
                })
                .map(|(out, gate)| (*out, gate))
                .unwrap()
        };

        let xor_out = if xor != sum.1.a && xor != sum.1.b {
            // if the first XOR output is not an operand of the second XOR gate,
            // it must be swapped (with the one that is not the carry)
            to_swap.push(xor);
            let out = if sum.1.a == carry { sum.1.b } else { sum.1.a };
            to_swap.push(out);
            // xor_out is the actual first XOR output
            out
        } else {
            xor
        };

        if sum.0 != zi {
            // if the output of the second XOR is not zi,
            // it must be swapped (with zi)
            let zi_real = gates.get_key_value(zi).unwrap().0;
            to_swap.push(zi_real);
            to_swap.push(sum.0);
        }

        // the second AND gate has the first XOR output and the carry
        // as operands, we can get it directly
        let and_carry = inv_map[&Gate {
            a: xor_out.min(carry),
            b: xor_out.max(carry),
            kind: GateKind::And,
        }];

        // if the candidate operands for the OR are already used in fixed places,
        // we can swap them with their counterpart and use it to find the OR gate
        let or_op = if and == xor_out || and_carry == xor_out {
            xor
        } else if and == zi || and_carry == zi {
            sum.0
        } else {
            and
        };

        // the OR gate is the only one with `or_op` as operand
        // (or `and` and `and_carry`, as a shortcircuit for correct adders)
        let or = if let Some((gate, out)) = inv_map.get_key_value(&Gate {
            a: and.min(and_carry),
            b: and.max(and_carry),
            kind: GateKind::Or,
        }) {
            (out, gate)
        } else {
            gates
                .iter()
                .find(|(_, gate)| gate.kind == GateKind::Or && (gate.a == or_op || gate.b == or_op))
                .unwrap()
        };

        // the new carry is the last candidate which is not an operand of some other gate
        carry = [or.0, and, and_carry, xor, sum.0]
            .into_iter()
            .find(|&c| c != zi && c != sum.1.a && c != sum.1.b && c != or.1.a && c != or.1.b)
            .unwrap();
    }

    to_swap.sort_unstable();
    let p2 = to_swap.into_iter().join(",");

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
