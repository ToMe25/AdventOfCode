use std::collections::HashSet;
use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 4 runner.
///
/// The [DayRunner] implementation for the aoc day 4.
#[derive(Debug, Clone, Default)]
pub struct Day4Runner {}

impl DayRunner for Day4Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input = get_input_file(4)?;
        let sum: usize = fs::read_to_string(input)?
            .lines()
            .map(|line| line.split_once(':').unwrap().1)
            .map(|game| game.split_once('|').unwrap())
            .map(|(win, have)| {
                (
                    win.split_whitespace()
                        .map(|nr| nr.parse::<u8>().unwrap())
                        .collect::<HashSet<u8>>(),
                    have,
                )
            })
            .map(|(win, have)| {
                have.split_whitespace()
                    .map(|nr| nr.parse::<u8>().unwrap())
                    .map(|nr| win.contains(&nr))
                    .filter(|win| *win)
                    .count()
            })
            .map(|wins| 1.min(wins) << (wins.max(1) - 1))
            .sum();

        Ok(Some(sum.to_string()))
    }
}
