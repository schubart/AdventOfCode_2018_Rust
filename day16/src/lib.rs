#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::Lines;

type Value = usize;

struct Sample {
    before: [Value; 4],
    opcode: Value,
    a: Value,
    b: Value,
    c: Value,
    after: [Value; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn apply(opcode: Opcode, a: Value, b: Value, c: Value, registers: &mut [Value; 4]) {
    registers[c] = match opcode {
        Opcode::Addr => registers[a] + registers[b],
        Opcode::Addi => registers[a] + b,
        Opcode::Mulr => registers[a] * registers[b],
        Opcode::Muli => registers[a] * b,
        Opcode::Banr => registers[a] & registers[b],
        Opcode::Bani => registers[a] & b,
        Opcode::Borr => registers[a] | registers[b],
        Opcode::Bori => registers[a] | b,
        Opcode::Setr => registers[a],
        Opcode::Seti => a,
        Opcode::Gtir => (a > registers[b]) as Value,
        Opcode::Gtri => (registers[a] > b) as Value,
        Opcode::Gtrr => (registers[a] > registers[b]) as Value,
        Opcode::Eqir => (a == registers[b]) as Value,
        Opcode::Eqri => (registers[a] == b) as Value,
        Opcode::Eqrr => (registers[a] == registers[b]) as Value,
    }
}

#[allow(dead_code)]
static OPCODES: [Opcode; 16] = [
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

#[test]
fn addi() {
    for opcode in OPCODES {
        let mut registers = [3, 2, 1, 1];
        apply(opcode, 2, 1, 2, &mut registers);
        assert!(
            (registers == [3, 2, 2, 1])
                == (opcode == Opcode::Mulr || opcode == Opcode::Addi || opcode == Opcode::Seti)
        );
    }
}

impl Sample {
    fn is_valid(&self, opcode: Opcode) -> bool {
        let mut registers = self.before;
        apply(opcode, self.a, self.b, self.c, &mut registers);
        registers == self.after
    }
}

fn parse(lines: &mut Lines) -> Option<Sample> {
    let before = lines.next().unwrap();
    if before.is_empty() {
        return None;
    }
    assert!(before.starts_with("Before: "));
    let mut split1 = before
        .split(&['[', ',', ']'][..])
        .skip(1)
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap);

    let args = lines.next().unwrap();
    let mut split2 = args
        .split(' ')
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap);

    let after = lines.next().unwrap();
    let mut split3 = after
        .split(&['[', ',', ']'][..])
        .skip(1)
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap);

    let empty = lines.next().unwrap();
    assert!(empty.is_empty());

    Some(Sample {
        before: [
            split1.next().unwrap(),
            split1.next().unwrap(),
            split1.next().unwrap(),
            split1.next().unwrap(),
        ],
        opcode: split2.next().unwrap(),
        a: split2.next().unwrap(),
        b: split2.next().unwrap(),
        c: split2.next().unwrap(),
        after: [
            split3.next().unwrap(),
            split3.next().unwrap(),
            split3.next().unwrap(),
            split3.next().unwrap(),
        ],
    })
}

/// # Panics
///
/// Will panic if the input is malformed.
///
/// ```
/// assert_eq!(493, day16::part1());
/// ```
#[must_use]
pub fn part1() -> usize {
    let mut lines = include_str!("input.txt").lines();

    std::iter::from_fn(|| parse(&mut lines))
        .map(|sample| {
            OPCODES
                .iter()
                .filter(|opcode| sample.is_valid(**opcode))
                .count()
        })
        .filter(|&c| c > 3)
        .count()
}

/// # Panics
///
/// Will panic if the input is malformed.
///
/// ```
/// assert_eq!(445, day16::part2());
/// ```
#[must_use]
pub fn part2() -> usize {
    let mut lines = include_str!("input.txt").lines();

    let mut mapping: HashMap<Value, HashSet<Opcode>> = HashMap::new();
    (0..OPCODES.len()).for_each(|i| {
        mapping.insert(i, OPCODES.iter().copied().collect());
    });

    std::iter::from_fn(|| parse(&mut lines)).for_each(|sample| {
        let candidates = mapping.get_mut(&sample.opcode).unwrap();

        *candidates = candidates
            .iter()
            .filter(|opcode| sample.is_valid(**opcode))
            .copied()
            .collect();
    });

    loop {
        let unique = mapping
            .values()
            .filter(|x| x.len() == 1)
            .flat_map(HashSet::iter)
            .copied()
            .collect::<HashSet<_>>();
        if unique.len() == OPCODES.len() {
            break;
        }

        mapping
            .values_mut()
            .filter(|x| x.len() > 1)
            .for_each(|x| x.retain(|y| !unique.contains(y)));
    }
    println!("{:?}", mapping);

    let mut registers = [0; 4];
    for line in lines.skip(1) {
        let mut split = line.split(' ').map(str::parse).map(Result::unwrap);

        let opcode: Value = split.next().unwrap();
        let opcode = mapping[&opcode].iter().next().unwrap();

        apply(
            *opcode,
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            &mut registers,
        );
    }

    registers[0]
}
