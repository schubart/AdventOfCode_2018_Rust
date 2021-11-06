#![warn(clippy::pedantic)]

use itertools::Itertools; // for unique()

// Two hars reatc if they both exist, are different but equal ignoring case.
fn pair_reacts(c1: Option<char>, c2: char) -> bool {
    match c1 {
        Some(c1) => c1 != c2 && c1.eq_ignore_ascii_case(&c2),
        _ => false,
    }
}

fn shrink(input: &str) -> String {
    let mut output = String::new();

    for c in input.chars() {
        if pair_reacts(output.chars().last(), c) {
            output.pop();
        } else {
            output.push(c);
        }
    }

    output
}

/// ```
/// assert_eq!(11540, day05::part1());
/// ```
#[must_use]
pub fn part1() -> usize {
    let input = include_str!("input.txt").trim();
    let shrunk = shrink(input);

    shrunk.len()
}

/// ```
/// assert_eq!(Some(6918), day05::part2());
/// ```
#[must_use]
pub fn part2() -> Option<usize> {
    let input = include_str!("input.txt").trim();
    // Efficency: Shrink once before trying to eliminate different chars.
    let pre_shrunk = shrink(input);
    let unique_chars = pre_shrunk.chars().map(|c| c.to_ascii_lowercase()).unique();

    unique_chars
        .map(|c| {
            // Remove both cases of c from pre-shrunk string.
            let stripped: String = pre_shrunk
                .chars()
                .filter(move |x| !c.eq_ignore_ascii_case(x))
                .collect();
            // Shrink it further.
            let shrunk = shrink(&stripped);

            // Get length.
            shrunk.len()
        })
        // Find shortest.
        .min()
}
