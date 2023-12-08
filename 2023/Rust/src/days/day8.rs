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
pub struct Day8Runner {
    /// The instructions for where to walk.
    ///
    /// The parsed right/left instructions from the input.  
    /// `true` means right and `false` means left.
    instructions: Vec<bool>,
    /// The nodes from the input file.
    ///
    /// The nodes from the input file.  
    /// The key is the current node, and the value are the possible next nodes.
    nodes: HashMap<String, (String, String)>,
}

impl DayRunner for Day8Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(8)?)?;
        let mut lines = input_data.lines().filter(|line| !line.is_empty());

        self.instructions = lines.next().unwrap().chars().map(|c| c == 'R').collect();

        self.nodes = lines
            .map(|line| line.split_once('=').unwrap())
            .map(|(from, to)| {
                (
                    from,
                    to.trim_matches(&[' ', '\t', '(', ')'][..])
                        .split_once(',')
                        .unwrap(),
                )
            })
            .map(|(from, (tol, tor))| {
                (
                    from.trim().to_owned(),
                    (tol.trim().to_owned(), tor.trim().to_owned()),
                )
            })
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let mut insts = self.instructions.iter().cycle();
        let mut current = &String::from("AAA");
        let mut steps: usize = 0;
        while current != "ZZZ" {
            current = self
                .nodes
                .get(current)
                .map(|node| {
                    if *insts.next().unwrap() {
                        &node.1
                    } else {
                        &node.0
                    }
                })
                .unwrap();
            steps += 1;
        }

        Ok(Some(steps.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let mut insts = self.instructions.iter().cycle();
        let mut current: Vec<&String> =
            self.nodes.keys().filter(|key| key.ends_with('A')).collect();
        let mut steps: usize = 0;
        while !current.iter().all(|pos| pos.ends_with('Z')) {
            let right = *insts.next().unwrap();
            for i in 0..current.len() {
                current[i] = self
                    .nodes
                    .get(current[i])
                    .map(|node| if right { &node.1 } else { &node.0 })
                    .unwrap();
            }
            steps += 1;
        }

        Ok(Some(steps.to_string()))
    }
}
