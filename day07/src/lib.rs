#![warn(clippy::pedantic)]

use std::collections::HashSet;
use std::iter::empty;
use std::iter::from_fn;

type Task = char;
type Dependency = (Task, Task); // Second depends on first.

type Result<T> = std::result::Result<T, &'static str>;

fn parse(line: &str) -> Dependency {
    let mut split = line.split(' ');

    (
        split.nth(1).unwrap().chars().next().unwrap(),
        split.nth(5).unwrap().chars().next().unwrap(),
    )
}

fn sequence_tasks(deps: &[Dependency]) -> impl Iterator<Item = Result<Task>> + '_ {
    let mut tasks: HashSet<Task> = empty()
        .chain(deps.iter().map(|(from, _)| *from))
        .chain(deps.iter().map(|(_, to)| *to))
        .collect();

    from_fn(move || {
        let result = tasks
            .iter()
            // Keep tasks that nothing depnds on or that only completed tasks depend on.
            .filter(|t| deps.iter().all(|(x, y)| *t != y || !tasks.contains(x)))
            .copied()
            .min();

        if let Some(t) = result {
            tasks.remove(&t);
        } else if !tasks.is_empty() {
            return Some(Err("Could not make progress"));
        }

        result.map(Ok)
    })
}

/// # Errors
///
/// Fails if there is no solution.
///
/// ```
/// assert_eq!(Ok("JMQZELVYXTIGPHFNSOADKWBRUC".to_string()), day07::part1());
/// ```
pub fn part1() -> Result<String> {
    let deps: Vec<Dependency> = include_str!("input.txt")
        .lines()
        .map(|line| parse(line))
        .collect();

    sequence_tasks(&deps).collect()
}

type Duration = i32;

/// ```
/// assert_eq!(Ok(1133), day07::part2());
/// ```
pub fn part2() -> Result<i32> {
    let deps: Vec<Dependency> = include_str!("input.txt")
        .lines()
        .map(|line| parse(line))
        .collect();

    let mut duration = 0;
    let mut ongoing = std::collections::HashMap::<Task, Duration>::new();
    let mut remaining_tasks: HashSet<Task> = empty()
        .chain(deps.iter().map(|(from, _)| *from))
        .chain(deps.iter().map(|(_, to)| *to))
        .collect();
    let mut completed_tasks: HashSet<Task> = HashSet::new();

    loop {
        ongoing = ongoing
            .iter()
            .map(|(task, duration)| (*task, duration - 1))
            .filter(|(task, duration)| {
                if *duration == 0 {
                    completed_tasks.insert(*task);
                    false
                } else {
                    true
                }
            })
            .collect();

        let capacity = 5 - ongoing.len();
        for _ in 0..capacity {
            let next_task = remaining_tasks
                .iter()
                // Keep tasks that nothing depnds on or that only completed tasks depend on.
                .filter(|t| {
                    deps.iter()
                        .all(|(x, y)| *t != y || completed_tasks.contains(x))
                })
                .copied()
                .min();

            if let Some(t) = next_task {
                remaining_tasks.remove(&t);
                ongoing.insert(t, 61 + (t as i32) - ('A' as i32));
            }
        }

        if ongoing.is_empty() {
            return Ok(duration);
        }

        duration += 1;
    }
}
