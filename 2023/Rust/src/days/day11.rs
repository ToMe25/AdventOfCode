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
pub struct Day11Runner {
    /// The galaxies from the input data.
    ///
    /// The galaxies as parsed from the input data, without any space expansion.
    galaxies: Vec<Galaxy>,
    /// The empty rows from the input data.
    ///
    /// The rows from the input data that are filled with only dots.  
    /// Zero indexed.
    empty_rows: Vec<usize>,
    /// The empty columns from the input data.
    ///
    /// The colums from the input data that are filled with only dots.  
    /// Zero indexed.
    empty_cols: Vec<usize>,
}

impl DayRunner for Day11Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(11)?)?;
        let input_lines: Vec<&str> = input_data
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();

        self.galaxies = input_lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'#')
                    .map(|(x, _)| Galaxy { x_pos: x, y_pos: y })
                    .collect::<Vec<Galaxy>>()
            })
            .collect();

        self.empty_rows = input_lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.trim().chars().all(|c| c == '.'))
            .map(|(y, _)| y)
            .collect();

        self.empty_cols = (0..input_lines[0].len())
            .filter(|col| {
                input_lines
                    .iter()
                    .map(|line| line.chars().nth(*col).unwrap())
                    .all(|c| c == '.')
            })
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let galaxies: Vec<Galaxy> = self
            .galaxies
            .iter()
            .map(|galaxy| {
                let offset_x = self
                    .empty_cols
                    .iter()
                    .filter(|col| **col < galaxy.x_pos)
                    .count();
                let offset_y = self
                    .empty_rows
                    .iter()
                    .filter(|row| **row < galaxy.y_pos)
                    .count();
                Galaxy {
                    x_pos: galaxy.x_pos + offset_x,
                    y_pos: galaxy.y_pos + offset_y,
                }
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

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let galaxies: Vec<Galaxy> = self
            .galaxies
            .iter()
            .map(|galaxy| {
                let offset_x = self
                    .empty_cols
                    .iter()
                    .filter(|col| **col < galaxy.x_pos)
                    .count();
                let offset_y = self
                    .empty_rows
                    .iter()
                    .filter(|row| **row < galaxy.y_pos)
                    .count();
                Galaxy {
                    x_pos: galaxy.x_pos + offset_x * 999999,
                    y_pos: galaxy.y_pos + offset_y * 999999,
                }
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
