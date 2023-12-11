//! My rust AoC 2023 day 11 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 11](https://adventofcode.com/2023/day/11).

use std::collections::HashSet;
use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 11 runner.
///
/// The [DayRunner] implementation for the aoc day 11.
#[derive(Debug, Clone, Default)]
pub struct Day11Runner {}

impl DayRunner for Day11Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(11)?)?;
        let mut map: Vec<Vec<char>> = input_data
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let mut inserted: usize = 0;
        for y in 0..map.len() {
            let empty = map[y + inserted].iter().all(|c| c == &'.');
            if empty {
                map.insert(
                    y + inserted,
                    (0..map[y + inserted].len()).map(|_| '.').collect(),
                );
                inserted += 1;
            }
        }

        inserted = 0;
        // Assume the map is rectangular
        for x in 0..map[0].len() {
            let empty = map.iter().map(|line| line[x + inserted]).all(|c| c == '.');
            if empty {
                map.iter_mut().for_each(|line| {
                    line.insert(x + inserted, '.');
                });
                inserted += 1;
            }
        }

        let galaxies: Vec<Galaxy> = map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, c)| *c == &'#')
                    .map(|(x, _)| Galaxy { x_pos: x, y_pos: y })
                    .collect::<Vec<Galaxy>>()
            })
            .collect();

        let mut checked: HashSet<Galaxy> = HashSet::new();
        let distance_sum: usize = galaxies
            .clone()
            .iter()
            .flat_map(|galaxy| {
                checked.insert(*galaxy);
                galaxies
                    .iter()
                    .filter(|gal| !checked.contains(gal))
                    .map(|gal| galaxy.manhatten_distance_to(gal))
                    .collect::<Vec<usize>>()
            })
            .sum();

        Ok(Some(distance_sum.to_string()))
    }
}

/// A struct representing a galaxy from the input data.
///
/// A struct representing the position of a galaxy from the input.  
/// Can be used to calculate the manhatten distance to another galaxy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Galaxy {
    x_pos: usize,
    y_pos: usize,
}

impl Galaxy {
    /// Calculates the manhatten distance to another galaxy.
    ///
    /// This function calculates the manhatten distance to another galaxy.  
    /// The [Manhatten distance](https://en.wikipedia.org/wiki/Manhatten_distance) is the sum
    /// of the difference of the x coordinates and the difference of the y coordinates.
    pub fn manhatten_distance_to(&self, other: &Galaxy) -> usize {
        self.x_pos.abs_diff(other.x_pos) + self.y_pos.abs_diff(other.y_pos)
    }
}
