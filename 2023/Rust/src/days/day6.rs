//! My rust AoC 2023 day 6 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 6](https://adventofcode.com/2023/day/6).

use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 6 runner.
///
/// The [DayRunner] implementation for the aoc day 6.
#[derive(Debug, Clone, Default)]
pub struct Day6Runner {}

impl DayRunner for Day6Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(6)?)?;
        let input_lines = &mut input_data.lines();
        let times: Vec<u16> = input_lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|time| time.parse().unwrap())
            .collect();

        let distances: Vec<u16> = input_lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|dist| dist.parse().unwrap())
            .collect();

        let result: usize = times
            .iter()
            .zip(distances.iter())
            .map(|(time, dist)| {
                (0..*time)
                    .scan(false, |found, t| {
                        let distance = (time - t) * t;
                        if *found && distance <= *dist {
                            None
                        } else {
                            if distance > *dist {
                                *found = true;
                            }
                            Some(distance)
                        }
                    })
                    .skip_while(|d| d <= dist)
                    .count()
            })
            .product();

        Ok(Some(result.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(6)?)?;
        let input_lines = &mut input_data.lines();
        let time: u32 = input_lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .replace(&[' ', '\t'], "")
            .parse()?;

        let distance: u64 = input_lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .replace(&[' ', '\t'], "")
            .parse()?;

        let result = (0..time)
            .scan(false, |found, t| {
                let dist = (time - t) as u64 * t as u64;
                if *found && dist <= distance {
                    None
                } else {
                    if dist > distance {
                        *found = true;
                    }
                    Some(dist)
                }
            })
            .skip_while(|d| d <= &distance)
            .count();

        Ok(Some(result.to_string()))
    }
}
