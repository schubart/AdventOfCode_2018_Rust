#![cfg(test)]
#![warn(clippy::pedantic)]

use std::collections::HashSet;

fn part1() -> i64 {
    include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .sum()
}

fn part2() -> i64 {
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

#[test]
fn test_part1() {
    assert_eq!(486, part1());
}

#[test]
fn test_part2() {
    assert_eq!(69_285, part2());
}
