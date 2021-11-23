#![warn(clippy::pedantic)]

type Number = usize;

fn node_value1(numbers: &mut impl Iterator<Item = Number>) -> Number {
    let child_count = numbers.next().unwrap();
    let data_count = numbers.next().unwrap();

    let child_sum: Number = (0..child_count).map(|_| node_value1(numbers)).sum();
    let data_sum: Number = (0..data_count).map(|_| numbers.next().unwrap()).sum();

    child_sum + data_sum
}

/// # Panics
///
/// Will panic if there is too much or not enough input or it is malformed.
///
/// ```
/// assert_eq!(45_618, day08::part1());
/// ```
#[must_use]
pub fn part1() -> Number {
    let mut numbers = include_str!("input.txt")
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap());

    let result = node_value1(&mut numbers);
    assert_eq!(None, numbers.next()); // Iterator should be fully consumed.
    result
}

fn node_value2(numbers: &mut impl Iterator<Item = Number>) -> Number {
    let child_count = numbers.next().unwrap();
    let data_count = numbers.next().unwrap();

    if child_count == 0 {
        // No children? Sum up metadata like in part 1.
        (0..data_count).map(|_| numbers.next().unwrap()).sum()
    } else {
        // Some children? Recusrively read them.
        let children = (0..child_count)
            .map(|_| node_value2(numbers))
            .collect::<Vec<_>>();

        // Treat metadata as 1-based indices into children.
        (0..data_count)
            .map(|_| numbers.next().unwrap())
            .filter(|index| *index > 0)
            .filter_map(|index| children.get(index - 1))
            .sum()
    }
}

/// # Panics
///
/// Will panic if there is too much or not enough input or it is malformed.
///
/// ```
/// assert_eq!(22_306, day08::part2());
/// ```
#[must_use]
pub fn part2() -> Number {
    let mut numbers = include_str!("input.txt")
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap());

    let result = node_value2(&mut numbers);
    assert_eq!(None, numbers.next()); // Iterator should be fully consumed.
    result
}
