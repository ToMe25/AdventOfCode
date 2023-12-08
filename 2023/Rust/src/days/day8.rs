//! My rust AoC 2023 day 8 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 8](https://adventofcode.com/2023/day/8).

use std::collections::HashMap;
use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 8 runner.
///
/// The [DayRunner] implementation for the aoc day 8.
#[derive(Debug, Clone, Default)]
pub struct Day8Runner {}

impl DayRunner for Day8Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(8)?)?;
        let mut lines = input_data.lines().filter(|line| !line.is_empty());

        let mut instructions = lines.next().unwrap().chars().map(|c| c == 'R').cycle();

        let nodes: HashMap<&str, (&str, &str)> = lines
            .map(|line| line.split_once('=').unwrap())
            .map(|(from, to)| {
                (
                    from,
                    to.trim_matches(&[' ', '\t', '(', ')'][..])
                        .split_once(',')
                        .unwrap(),
                )
            })
            .map(|(from, (tol, tor))| (from.trim(), (tol.trim(), tor.trim())))
            .collect();

        let mut current = "AAA";
        let mut steps: usize = 0;
        while current != "ZZZ" {
            current = nodes
                .get(current)
                .map(|node| {
                    if instructions.next().unwrap() {
                        node.1
                    } else {
                        node.0
                    }
                })
                .unwrap();
            steps += 1;
        }

        Ok(Some(steps.to_string()))
    }
}
