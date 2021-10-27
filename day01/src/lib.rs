#![warn(clippy::pedantic)]

use std::collections::HashSet;

/// # Panics
///
/// Will panic if the input contains non-integers.
///
/// ```
/// assert_eq!(486, day01::part1());
/// ```
#[must_use]
pub fn part1() -> i64 {
    include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .sum()
}

/// # Panics
///
/// Will panic if the input contains non-integers.
///
/// ```
/// assert_eq!(69285, day01::part2());
/// ```
#[must_use]
pub fn part2() -> i64 {
    // Yes, this might iterate infinitely, but for the given input it does not.
    #![allow(clippy::maybe_infinite_iter)]

    let mut seen = HashSet::new();
    include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .cycle()
        // Map each element to a running sum.
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        // Find first sum seen before.
        .find(|sum| !seen.insert(*sum))
        // Safe: The above loops forever or returns Some, never None.
        .unwrap()
}
