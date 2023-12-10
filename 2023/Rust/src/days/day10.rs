//! My rust AoC 2023 day 10 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 10](https://adventofcode.com/2023/day/10).

use std::collections::HashMap;
use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 10 runner.
///
/// The [DayRunner] implementation for the aoc day 10.
#[derive(Debug, Clone, Default)]
pub struct Day10Runner {}

impl DayRunner for Day10Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(10)?)?;

        let mut pipes: Vec<Vec<Pipe>> = input_data
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '|' => Pipe::Vertical,
                        '-' => Pipe::Horizontal,
                        'L' => Pipe::NorthEast,
                        'J' => Pipe::NorthWest,
                        '7' => Pipe::SouthWest,
                        'F' => Pipe::SouthEast,
                        'S' => Pipe::Start(None),
                        _ => Pipe::None,
                    })
                    .collect()
            })
            .collect();

        let mut start_pos: Option<(usize, usize)> = None;
        for y in 0..pipes.len() {
            for x in 0..pipes[y].len() {
                if matches!(pipes[y][x], Pipe::Start(..)) {
                    start_pos = Some((x, y));
                    let north = if y > 0 {
                        pipes[y - 1][x].south_open().unwrap()
                    } else {
                        false
                    };
                    let south = if y < pipes.len() - 1 {
                        pipes[y + 1][x].north_open().unwrap()
                    } else {
                        false
                    };
                    let west = if x > 0 {
                        pipes[y][x - 1].east_open().unwrap()
                    } else {
                        false
                    };
                    let east = if x < pipes[y].len() - 1 {
                        pipes[y][x + 1].west_open().unwrap()
                    } else {
                        false
                    };

                    if north && south {
                        pipes[y][x] = Pipe::Start(Some(Box::new(Pipe::Vertical)));
                    } else if west && east {
                        pipes[y][x] = Pipe::Start(Some(Box::new(Pipe::Horizontal)));
                    } else if north && west {
                        pipes[y][x] = Pipe::Start(Some(Box::new(Pipe::NorthWest)));
                    } else if north && east {
                        pipes[y][x] = Pipe::Start(Some(Box::new(Pipe::NorthEast)));
                    } else if south && west {
                        pipes[y][x] = Pipe::Start(Some(Box::new(Pipe::SouthWest)));
                    } else if south && east {
                        pipes[y][x] = Pipe::Start(Some(Box::new(Pipe::SouthEast)));
                    } else {
                        pipes[y][x] = Pipe::Start(Some(Box::new(Pipe::None)));
                    }
                }
            }
        }

        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push(start_pos.unwrap());
        let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
        distances.insert(start_pos.unwrap(), 0);
        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            let dist = *distances.get(&current).unwrap();
            if current.1 > 0 && pipes[current.1][current.0].north_open().unwrap() {
                if dist + 1
                    < *distances
                        .get(&(current.0, current.1 - 1))
                        .unwrap_or(&usize::MAX)
                {
                    distances.insert((current.0, current.1 - 1), dist + 1);
                    stack.push((current.0, current.1 - 1));
                }
            }
            if current.1 < pipes.len() && pipes[current.1][current.0].south_open().unwrap() {
                if dist + 1
                    < *distances
                        .get(&(current.0, current.1 + 1))
                        .unwrap_or(&usize::MAX)
                {
                    distances.insert((current.0, current.1 + 1), dist + 1);
                    stack.push((current.0, current.1 + 1));
                }
            }
            if current.0 > 0 && pipes[current.1][current.0].west_open().unwrap() {
                if dist + 1
                    < *distances
                        .get(&(current.0 - 1, current.1))
                        .unwrap_or(&usize::MAX)
                {
                    distances.insert((current.0 - 1, current.1), dist + 1);
                    stack.push((current.0 - 1, current.1));
                }
            }
            if current.0 < pipes[current.1].len()
                && pipes[current.1][current.0].east_open().unwrap()
            {
                if dist + 1
                    < *distances
                        .get(&(current.0 + 1, current.1))
                        .unwrap_or(&usize::MAX)
                {
                    distances.insert((current.0 + 1, current.1), dist + 1);
                    stack.push((current.0 + 1, current.1));
                }
            }
        }

        let result = *distances.values().max().unwrap();

        Ok(Some(result.to_string()))
    }
}

/// A single pipe node from the input data.
///
/// An enum representing a single character on the input map.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Pipe {
    None,
    Vertical,
    Horizontal,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Start(Option<Box<Pipe>>),
}

impl Pipe {
    /// Checks whether this pipe is connected with the pipe to its north.
    ///
    /// Checks whether this pipe is connected with the pipe to its north.  
    /// This can be either because this pipe is of a type that is open to the north,
    /// or because this is a Self::Start pipe that wraps one that is open to the north.
    fn north_open(&self) -> Option<bool> {
        if let Pipe::Start(pipe) = self {
            if let Some(p) = pipe {
                return p.north_open();
            } else {
                return None;
            }
        }

        Some(self == &Pipe::Vertical || self == &Pipe::NorthWest || self == &Pipe::NorthEast)
    }

    /// Checks whether this pipe is connected with the pipe to its south.
    ///
    /// Checks whether this pipe is connected with the pipe to its south.  
    /// This can be either because this pipe is of a type that is open to the south,
    /// or because this is a Self::Start pipe that wraps one that is open to the south.
    fn south_open(&self) -> Option<bool> {
        if let Pipe::Start(pipe) = self {
            if let Some(p) = pipe {
                return p.south_open();
            } else {
                return None;
            }
        }

        Some(self == &Pipe::Vertical || self == &Pipe::SouthWest || self == &Pipe::SouthEast)
    }

    /// Checks whether this pipe is connected with the pipe to its west.
    ///
    /// Checks whether this pipe is connected with the pipe to its west.  
    /// This can be either because this pipe is of a type that is open to the west,
    /// or because this is a Self::Start pipe that wraps one that is open to the west.
    fn west_open(&self) -> Option<bool> {
        if let Pipe::Start(pipe) = self {
            if let Some(p) = pipe {
                return p.west_open();
            } else {
                return None;
            }
        }

        Some(self == &Pipe::Horizontal || self == &Pipe::NorthWest || self == &Pipe::SouthWest)
    }

    /// Checks whether this pipe is connected with the pipe to its east.
    ///
    /// Checks whether this pipe is connected with the pipe to its east.  
    /// This can be either because this pipe is of a type that is open to the east,
    /// or because this is a Self::Start pipe that wraps one that is open to the east.
    fn east_open(&self) -> Option<bool> {
        if let Pipe::Start(pipe) = self {
            if let Some(p) = pipe {
                return p.east_open();
            } else {
                return None;
            }
        }

        Some(self == &Pipe::Horizontal || self == &Pipe::NorthEast || self == &Pipe::SouthEast)
    }
}
