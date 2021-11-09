#![warn(clippy::pedantic)]

use counter::Counter;
use itertools::iproduct;
use std::collections::HashSet;
use std::iter::empty;
use std::ops::RangeInclusive;

type Scalar = i32;
type Point = (Scalar, Scalar);

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse(string: &str) -> Result<Point> {
    let mut split = string.split(", ");

    Ok((
        split.next().ok_or("Too few columns")?.parse()?,
        split.next().ok_or("Too few columns")?.parse()?,
    ))
}

fn bounds(points: &[Point]) -> Option<(RangeInclusive<Scalar>, RangeInclusive<Scalar>)> {
    if points.is_empty() {
        return None;
    }

    let x_min = points.iter().map(|p| p.0).min().unwrap();
    let x_max = points.iter().map(|p| p.0).max().unwrap();
    let y_min = points.iter().map(|p| p.1).min().unwrap();
    let y_max = points.iter().map(|p| p.1).max().unwrap();

    Some((x_min..=x_max, y_min..=y_max))
}

fn distance(p1: Point, p2: Point) -> Scalar {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

/// Returns the point among `candidates` that is closest to `target`, if there
/// is exactly one such point.
fn nearest(target: Point, candidates: &[Point]) -> Option<Point> {
    // State machine: Find the nearest point in one go, watch out for ties.
    #[derive(Clone, Copy)]
    enum State {
        Start,
        Min(Scalar, Point),
        Tie(Scalar),
    }
    use State::{Min, Start, Tie};

    let end_state = candidates.iter().fold(Start, |old_state, &new_point| {
        let new_dist = distance(target, new_point);

        match old_state {
            // First point automatically becomes new minimum.
            Start => Min(new_dist, new_point),

            // Have minimum, next point has same distance. It is now a tie.
            Min(old_dist, _) if new_dist == old_dist => Tie(new_dist),

            // Have minimum or tie, next point is nearer. Found new minimum.
            Min(old_dist, _) | Tie(old_dist) if new_dist < old_dist => Min(new_dist, new_point),

            // No change otherwise.
            _ => old_state,
        }
    });

    // Unwrap the end state returned by the state machine.
    match end_state {
        Min(_, nearest_point) => Some(nearest_point),
        _ => None,
    }
}

/// # Panics
///
/// Will panic if the input is malformed or empty.
///
/// ```
/// assert_eq!(Some(5975), day06::part1());
/// ```
#[must_use]
pub fn part1() -> Option<usize> {
    let points: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| parse(line).unwrap())
        .collect();

    let (xs, ys) = bounds(&points).unwrap();

    // Find points whose nearest areas stretch to the edges. These areas are infinite.
    let infinites: HashSet<Point> = empty()
        .chain(xs.clone().map(|x| (x, *ys.start()))) // top
        .chain(xs.clone().map(|x| (x, *ys.end()))) // bottom
        .chain(ys.clone().map(|y| (*xs.start(), y))) // left
        .chain(ys.clone().map(|y| (*xs.end(), y))) // right
        .filter_map(|p| nearest(p, &points))
        .collect();

    // For each coordinate pair in the bounds...
    iproduct!(xs, ys)
        // Find the nearest point, if any.
        .filter_map(|p| nearest(p, &points))
        // Ignore it if the point's area is infinite.
        .filter(|p| !infinites.contains(p))
        // Tallly the points, i.e. calculate the size of their nearest area.
        .collect::<Counter<Point>>()
        // Iterate areas by size.
        .most_common()
        // Find the point with the biggest area and the size of that area.
        .first()
        // Only care about the area's size, not which point has that area.
        .map(|(_point, size)| *size)
}

/// # Panics
///
/// Will panic if the input is malformed or empty.
///
/// ```
/// assert_eq!(38670, day06::part2());
/// ```
#[must_use]
pub fn part2() -> usize {
    let points: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| parse(line).unwrap())
        .collect();

    let (xs, ys) = bounds(&points).unwrap();

    iproduct!(xs, ys)
        .filter(|p1| points.iter().map(|p2| distance(*p1, *p2)).sum::<Scalar>() < 10_000)
        .count()
}
