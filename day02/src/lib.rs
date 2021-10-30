#![warn(clippy::pedantic)]

use counter::Counter;
use itertools::Itertools;

/// ```
/// assert_eq!(6200, day02::part1());
/// ```
#[must_use]
pub fn part1() -> usize {
    let mut count_2 = 0;
    let mut count_3 = 0;

    include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Counter<char>>())
        .for_each(|counter| {
            if counter.values().any(|v| *v == 2) {
                count_2 += 1;
            }
            if counter.values().any(|v| *v == 3) {
                count_3 += 1;
            }
        });

    count_2 * count_3
}

/// ```
/// assert_eq!(Some("xpysnnkqrbuhefmcajodplyzw".to_string()), day02::part2());
/// ```
#[must_use]
pub fn part2() -> Option<String> {
    for (line1, line2) in include_str!("input.txt")
        .lines()
        .tuple_combinations()
        // Words have same length? (Not really needed for this input.)
        .filter(|(line1, line2)| line1.len() == line2.len())
    {
        let common = line1
            .chars()
            .zip(line2.chars())
            .filter(|(char1, char2)| char1 == char2)
            .map(|(char1, _)| char1);

        if common.clone().count() == line1.len() - 1 {
            return Some(common.collect());
        }
    }

    None
}
