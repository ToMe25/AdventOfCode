//! My rust AoC 2023 day 4 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 4](https://adventofcode.com/2023/day/4).

use std::collections::HashSet;
use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 4 runner.
///
/// The [DayRunner] implementation for the aoc day 4.
#[derive(Debug, Clone, Default)]
pub struct Day4Runner {
	/// The number of wins per scratchcard.
	///
	/// This vector contains the number of wins for each scratchcard.  
	/// Each number in the second set of numbers that is also found in the first set counts as one win.
    wins: Vec<usize>,
}

impl DayRunner for Day4Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input = get_input_file(4)?;
        self.wins = fs::read_to_string(input)?
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
            .collect();
        Ok(())
    }

    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let sum: usize = self
            .wins
            .iter()
            .map(|wins| 1.min(*wins) << (wins.max(&1) - 1))
            .sum();

        Ok(Some(sum.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let sum: usize = self
            .wins
            .iter()
            .fold((0, Vec::<usize>::new()), |(acc, wins_acc), wins| {
                let copies = 1 + wins_acc.len();
                let mut new_wins: Vec<usize> =
                    wins_acc.iter().map(|i| i - 1).filter(|i| *i > 0).collect();
                if *wins > 0 {
                    new_wins.extend((0..copies).map(|_| *wins))
                }
                (acc + copies, new_wins)
            })
            .0;
        Ok(Some(sum.to_string()))
    }
}
