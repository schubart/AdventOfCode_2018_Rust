#![cfg(test)]
#![warn(clippy::pedantic)]

use counter::Counter;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type Guard = i32;
type Minute = i32;

// For each guard, for each minute: How many times was the guard asleep.
type Summary = HashMap<Guard, Counter<Minute>>;

#[derive(Debug)]
enum Event {
    BeginsShift(Minute, Guard),
    FallsAsleep(Minute),
    WakesUp(Minute),
}

#[derive(Debug)]
enum State {
    Start(Summary),
    Working(Summary, Guard, Minute),
    Sleeping(Summary, Guard, Minute),
}

impl Event {
    fn parse(line: &str) -> Result<Self> {
        #[rustfmt::skip]
        lazy_static! {
            static ref BEGINS_SHIFT: Regex = Regex::new(r":(\d{2})\] Guard #(\d+) begins shift").unwrap();
            static ref FALLS_ASLEEP: Regex = Regex::new(r":(\d{2})\] falls asleep").unwrap();
            static ref WAKES_UP:     Regex = Regex::new(r":(\d{2})\] wakes up").unwrap();
        }

        if let Some(c) = BEGINS_SHIFT.captures(line) {
            Ok(Self::BeginsShift(c[1].parse()?, c[2].parse()?))
        } else if let Some(c) = FALLS_ASLEEP.captures(line) {
            Ok(Self::FallsAsleep(c[1].parse()?))
        } else if let Some(c) = WAKES_UP.captures(line) {
            Ok(Self::WakesUp(c[1].parse()?))
        } else {
            Err(format!("Invalid line {}", line).into())
        }
    }
}

impl State {
    fn start() -> State {
        State::Start(Summary::new())
    }

    fn handle(self, event: Event) -> State {
        use Event::{BeginsShift, FallsAsleep, WakesUp};
        use State::{Sleeping, Start, Working};

        match (self, event) {
            // First or sunsequent guard begins shift.
            (Start(summary) | Working(summary, _, _), BeginsShift(time, id)) => {
                // Guard is now working.
                Working(summary, id, time)
            }

            // Guard falls asleep.
            (Working(summary, id, _), FallsAsleep(time)) => {
                // Guard is now sleeping.
                Sleeping(summary, id, time)
            }

            // Guard wakes up.
            (Sleeping(mut summary, id, begin), WakesUp(time)) => {
                // Record for how long guard was asleep.
                summary.entry(id).or_default().extend(begin..time);
                // Gurard is now working again.
                Working(summary, id, time)
            }

            // Other transitions do not make sense (e.g. FallsAsleep in
            // Sleeping) or are not used by this particular data set (e.g
            // BeginsShift while Sleeping).
            x => panic!("Unexpected: {:?}", x),
        }
    }
}

fn summarize() -> Summary {
    let end_state = include_str!("input.txt")
        .lines()
        .sorted()
        .map(|line| Event::parse(line).unwrap())
        .fold(State::start(), State::handle);

    // Extract summary and ensure final state is sensible.
    match end_state {
        State::Working(summary, _, _) => summary,
        x => panic!("Unexpected end state: {:?}", x),
    }
}

fn part1() -> (Guard, Minute) {
    let summary = summarize();

    // Find the guard who slept the most minutes.
    let (&guard, counter) = summary
        .iter()
        .max_by_key(|(_, counter)| counter.values().sum::<usize>())
        // Assume there is at least one guard.
        .unwrap();
    // Find the most frequent sleep minute of that guard.
    let (minute, _frequency) = counter.most_common()[0];

    (guard, minute)
}

fn part2() -> (Guard, Minute) {
    let summary = summarize();

    // Find the guard whose most frequent sleep minute is most frequent overall.
    let (&guard, counter) = summary
        .iter()
        .max_by_key(|(_, counter)| {
            let (_minute, frequency) = counter.most_common()[0];
            frequency
        })
        // Assume there is at least one guard.
        .unwrap();
    // Find the most frequent sleep minute of that guard.
    let (minute, _frequency) = counter.most_common()[0];

    (guard, minute)
}

#[test]
fn test_part1() {
    let (id, time) = part1();
    assert_eq!(2_593, id);
    assert_eq!(40, time);
    assert_eq!(103_720, id * time);
}

#[test]
fn test_part() {
    let (id, time) = part2();
    assert_eq!(3_361, id);
    assert_eq!(33, time);
    assert_eq!(110_913, id * time);
}
