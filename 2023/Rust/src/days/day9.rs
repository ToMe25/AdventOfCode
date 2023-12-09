//! My rust AoC 2023 day 9 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 9](https://adventofcode.com/2023/day/9).

use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 9 runner.
///
/// The [DayRunner] implementation for the aoc day 9.
#[derive(Debug, Clone, Default)]
pub struct Day9Runner {}

impl DayRunner for Day9Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(9)?)?;

        let histories: Vec<Vec<i32>> = input_data
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();

        let extrapolated: Vec<i32> = histories
            .iter()
            .map(|history| {
                let mut diffs: Vec<Vec<i32>> = Vec::new();
                let mut current = history.clone();
                while !current.iter().all(|i| *i == 0) {
                    diffs.push(current.clone());
                    current = current
                        .iter()
                        .scan(None, |state: &mut Option<i32>, &nr| {
                            if state.is_some() {
                                Some(Some(nr - state.replace(nr).unwrap()))
                            } else {
                                *state = Some(nr);
                                Some(None)
                            }
                        })
                        .skip_while(|nr| nr.is_none())
                        .map(|nr| nr.unwrap())
                        .collect();
                }
                diffs
            })
            .map(|diffs| diffs.iter().rev().map(|diff| diff.last().unwrap()).sum())
            .collect();

        let sum: i32 = extrapolated.iter().sum();

        Ok(Some(sum.to_string()))
    }
}
