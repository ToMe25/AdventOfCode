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
pub struct Day9Runner {
    /// The differences of the historic values of the input data.
    ///
    /// The differences of the historic values of the input data.  
    /// The outer most vector contains one vector for each input line.  
    /// Each of those vectors contains one vector for the initial history,  
    /// plus the recursive differences between the numbers.
    diffs: Vec<Vec<Vec<i32>>>,
}

impl DayRunner for Day9Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(9)?)?;

        self.diffs = input_data
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
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
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let sum: i32 = self
            .diffs
            .iter()
            .map(|diffs| {
                diffs
                    .iter()
                    .rev()
                    .map(|diff| diff.last().unwrap())
                    .sum::<i32>()
            })
            .sum();

        Ok(Some(sum.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let sum: i32 = self
            .diffs
            .iter()
            .map(|diffs| {
                diffs
                    .iter()
                    .rev()
                    .map(|diff| *diff.first().unwrap())
                    .reduce(|acc, nr| nr - acc)
                    .unwrap()
            })
            .sum();

        Ok(Some(sum.to_string()))
    }
}
