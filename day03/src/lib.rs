#![warn(clippy::pedantic)]

use counter::Counter;
use itertools::iproduct;
use lazy_static::lazy_static;
use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Claim {
    id: String,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Claim {
    fn parse(line: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\S+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }

        let captures = RE.captures(line).ok_or(line)?;

        Ok(Claim {
            id: captures[1].to_string(),
            x: captures[2].parse()?,
            y: captures[3].parse()?,
            w: captures[4].parse()?,
            h: captures[5].parse()?,
        })
    }

    fn coords(&self) -> impl Iterator<Item = (i32, i32)> {
        let xs = (self.x)..(self.x + self.w);
        let ys = (self.y)..(self.y + self.h);

        iproduct!(xs, ys)
    }
}

/// # Panics
///
/// Will panic if the input is malformed.
///
/// ```
/// assert_eq!(111630, day03::part1());
/// ```
#[must_use]
pub fn part1() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|line| Claim::parse(line).unwrap())
        .flat_map(|claim| claim.coords())
        .collect::<Counter<_>>()
        .values()
        .filter(|frequency| **frequency > 1)
        .count()
}

/// # Panics
///
/// Will panic if the input is malformed.
///
/// ```
/// assert_eq!(Some("724".to_string()), day03::part2());
/// ```
#[must_use]
pub fn part2() -> Option<String> {
    let claims = include_str!("input.txt")
        .lines()
        .map(|line| Claim::parse(line).unwrap());

    let frequency = claims
        .clone()
        .flat_map(|claim| claim.coords())
        .collect::<Counter<_>>();

    claims
        .filter(|claim| claim.coords().all(|coord| frequency[&coord] == 1))
        .map(|claim| claim.id)
        .next()
}
