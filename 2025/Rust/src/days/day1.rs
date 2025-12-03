//! The [Advent of Code](https://adventofcode.com/) [2025](https://adventofcode.com/2025) [Day 1](https://https://adventofcode.com/2025/day/1) runner.
//!
//! This module contains my personal solution of said [Advent of Code](https://adventofcode.com/) challenge.

use std::error::Error;

use crate::dayrunner::{DayRunner, DayRunnerDate, Part1Runner, Part2Runner, RunContext};

/// The day [Advent of Code 2025](https://adventofcode.com/2025) runner.
pub struct Day1Runner {
    /// The individual steps from the input.
    steps: Vec<String>,
}

impl DayRunner for Day1Runner {
    fn init(ctx: &RunContext) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(Day1Runner {
            steps: ctx
                .read_day_input_file()?
                .lines()
                .map(String::from)
                .collect(),
        })
    }
}

impl DayRunnerDate for Day1Runner {
    const DAY: u8 = 1;
}

impl Part1Runner for Day1Runner {
    fn part1(&self, _: &RunContext) -> Result<String, Box<dyn Error>> {
        let result = self
            .steps
            .iter()
            .map(|line| line.split_at(1))
            .map(|(dir, dist)| {
                (
                    dir,
                    dist.parse::<i16>().expect("Failed to parse input distance"),
                )
            })
            .scan(50i16, |pos, (dir, dist)| {
                *pos = (*pos
                    + match dir {
                        "R" => dist,
                        "L" => 100 - dist % 100,
                        dir => panic!("Received invalid input direction '{}'", dir),
                    })
                    % 100;
                Some(*pos)
            })
            .filter(|&pos| pos == 0)
            .count();

        Ok(result.to_string())
    }
}

impl Part2Runner for Day1Runner {
    fn part2(&self, _: &RunContext) -> Result<String, Box<dyn Error>> {
        let result = self
            .steps
            .iter()
            .map(|line| line.split_at(1))
            .map(|(dir, dist)| {
                (
                    dir,
                    dist.parse::<i16>().expect("Failed to parse input distance"),
                )
            })
            .scan((50i16, 0u16), |(pos, count), (dir, dist)| {
                let mut new_pos = match dir {
                    "R" => *pos + dist,
                    "L" => *pos - dist,
                    dir => panic!("Received invalid input direction '{}'", dir),
                };
                let mut new_count = *count;
                new_count += (new_pos / 100).unsigned_abs();
                if new_pos == 0 && pos != &0 {
                    new_count += 1;
                } else if new_pos < 0 && pos != &0 {
                    new_count += 1;
                }
                new_pos = (new_pos % 100 + 100) % 100;
                *pos = new_pos;
                *count = new_count;
                Some(new_count)
            })
            .last()
            .expect("Didn't receive any input");

        Ok(result.to_string())
    }
}
