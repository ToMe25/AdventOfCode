//! My rust AoC 2023 day 14 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 14](https://adventofcode.com/2023/day/14).

use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 14 runner.
///
/// The [DayRunner] implementation for the aoc day 14.
#[derive(Debug, Clone, Default)]
pub struct Day14Runner {
    /// The positions of all the rocks in the input data.
    initial_map: Vec<Vec<Option<Rock>>>,
}

impl DayRunner for Day14Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(14)?)?;
        self.initial_map = input_data
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let x: u32 = x.try_into().expect("X position out of range");
                        let y: u32 = y.try_into().expect("Y position out of range");
                        match c {
                            'O' => Some(Rock::new(RockShape::Round, x, y)),
                            '#' => Some(Rock::new(RockShape::Cube, x, y)),
                            _ => None,
                        }
                    })
                    .collect::<Vec<Option<Rock>>>()
            })
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let map = tilt_platform(&self.initial_map, Direction::North);
        let load = calculate_load(&map, Direction::North);
        Ok(Some(load.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let mut map = self.initial_map.clone();
        for _ in 0..1000000000 {
            for dir in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                map = tilt_platform(&map, dir);
            }
        }

        let load = calculate_load(&map, Direction::North);
        Ok(Some(load.to_string()))
    }
}

/// Tilts the platform in one of the cardinal directions.
///
/// Generates a new map of the rocks on the platform based on a previous map and a tilt direction.
pub fn tilt_platform(
    platform: &Vec<Vec<Option<Rock>>>,
    direction: Direction,
) -> Vec<Vec<Option<Rock>>> {
    let mut new_map: Vec<Vec<Option<Rock>>> = platform
        .iter()
        .map(|row| (0..row.len()).map(|_| None).collect())
        .collect();
    match direction {
        Direction::North => platform.iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, rock)| rock.map(|r| (x, r)))
                .for_each(|(x, rock)| {
                    if !rock.can_roll() {
                        new_map.get_mut(y).expect("Invalid new_map")[x] = Some(rock.clone());
                    } else {
                        let new_y = (0..y)
                            .rev()
                            .take_while(|pos| new_map[*pos][x].is_none())
                            .last()
                            .unwrap_or(y);
                        assert!(new_map[new_y][x].is_none(), "Overriding another rock");
                        new_map[new_y][x] =
                            Some(Rock::new(rock.get_shape(), x as u32, new_y as u32));
                    }
                })
        }),
        Direction::South => platform.iter().enumerate().rev().for_each(|(y, row)| {
            let max_y = platform.len();
            row.iter()
                .enumerate()
                .filter_map(|(x, rock)| rock.map(|r| (x, r)))
                .for_each(|(x, rock)| {
                    if !rock.can_roll() {
                        new_map.get_mut(y).expect("Invalid new_map")[x] = Some(rock.clone());
                    } else {
                        let new_y = (y + 1..max_y)
                            .take_while(|pos| new_map[*pos][x].is_none())
                            .last()
                            .unwrap_or(y);
                        assert!(new_map[new_y][x].is_none(), "Overriding another rock");
                        new_map[new_y][x] =
                            Some(Rock::new(rock.get_shape(), x as u32, new_y as u32));
                    }
                })
        }),
        Direction::West => platform.iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, rock)| rock.map(|r| (x, r)))
                .for_each(|(x, rock)| {
                    if !rock.can_roll() {
                        new_map.get_mut(y).expect("Invalid new_map")[x] = Some(rock.clone());
                    } else {
                        let new_x = (0..x)
                            .rev()
                            .take_while(|pos| new_map[y][*pos].is_none())
                            .last()
                            .unwrap_or(x);
                        assert!(new_map[y][new_x].is_none(), "Overriding another rock");
                        new_map[y][new_x] =
                            Some(Rock::new(rock.get_shape(), new_x as u32, y as u32));
                    }
                })
        }),
        Direction::East => platform.iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .enumerate()
                .rev()
                .filter_map(|(x, rock)| rock.map(|r| (x, r)))
                .for_each(|(x, rock)| {
                    if !rock.can_roll() {
                        new_map.get_mut(y).expect("Invalid new_map")[x] = Some(rock.clone());
                    } else {
                        let new_x = (x + 1..row.len())
                            .take_while(|pos| new_map[y][*pos].is_none())
                            .last()
                            .unwrap_or(x);
                        assert!(new_map[y][new_x].is_none(), "Overriding another rock");
                        new_map[y][new_x] =
                            Some(Rock::new(rock.get_shape(), new_x as u32, y as u32));
                    }
                })
        }),
    }

    new_map
}

/// Calculates the load on the support beams on one side of the platform.
///
/// Sums the distances of [round](RockShape::Round) rocks from the side.
/// This is considered to be the load on the support beams on the given side.
/// Cube-shaped rocks are considered to cause no load, perhaps because they only cause static load?
pub fn calculate_load(platform: &Vec<Vec<Option<Rock>>>, side: Direction) -> u64 {
    match side {
        Direction::North => {
            let max_dist = platform.len();
            platform
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .map(|rock| {
                            if rock.is_some_and(|rock| rock.can_roll()) {
                                (max_dist - y).try_into().expect("Distance out of range")
                            } else {
                                0u64
                            }
                        })
                        .sum::<u64>()
                })
                .sum()
        }
        Direction::South => platform
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .map(|rock| {
                        if rock.is_some_and(|rock| rock.can_roll()) {
                            (y + 1).try_into().expect("Distance out of range")
                        } else {
                            0u64
                        }
                    })
                    .sum::<u64>()
            })
            .sum(),
        Direction::West => platform
            .iter()
            .map(|row| {
                let max_dist = row.len();
                row.iter()
                    .enumerate()
                    .map(|(x, rock)| {
                        if rock.is_some_and(|rock| rock.can_roll()) {
                            (max_dist - x).try_into().expect("Distance out of range")
                        } else {
                            0u64
                        }
                    })
                    .sum::<u64>()
            })
            .sum(),
        Direction::East => platform
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(x, rock)| {
                        if rock.is_some_and(|rock| rock.can_roll()) {
                            (x + 1).try_into().expect("Distance out of range")
                        } else {
                            0u64
                        }
                    })
                    .sum::<u64>()
            })
            .sum(),
    }
}

/// Writes the text representation of the given map to the standard output.
pub fn print_map(map: &Vec<Vec<Option<Rock>>>) {
    map.iter().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|rock| match rock {
                    &Some(r) => {
                        match r.get_shape() {
                            RockShape::Round => 'O',
                            RockShape::Cube => '#',
                        }
                    }
                    &None => '.',
                })
                .collect::<String>()
        );
    });
}

/// A position on a two dimensional map.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    /// Creates a new position object, representing a position on a 2d map.
    pub fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }

    /// Gets the x component of this position.
    pub fn get_x(&self) -> u32 {
        self.x.clone()
    }

    /// Gets the y component of this position.
    pub fn get_y(&self) -> u32 {
        self.y.clone()
    }

    /// Sets the x component of this position.
    pub fn set_x(&mut self, new_x: u32) {
        self.x = new_x;
    }

    /// Sets the y component of this position.
    pub fn set_y(&mut self, new_y: u32) {
        self.y = new_y;
    }
}

/// The possible shapes a [Rock] can have.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RockShape {
    /// A rounded rock that will roll around when the platform is tilted.
    Round,
    // A cube-shaped rock that wont move around.
    Cube,
}

/// A single rock on the platform adjusting the orientation of the parabolic mirrors.
///
/// Each rock has a [shape](RockShape) and a 2d position.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Rock {
    /// The shape of this rock.
    shape: RockShape,
    /// The 2d position of this rock on the map.
    pos: Position,
}

impl Rock {
    /// Creates a new rock.
    ///
    /// Creates a new rock from its [shape](RockShape) and 2d position on the map.
    pub fn new(shape: RockShape, pos_x: u32, pos_y: u32) -> Rock {
        Rock {
            shape,
            pos: Position::new(pos_x, pos_y),
        }
    }

    /// Gets the [shape](RockShape) of this rock.
    ///
    /// Returns the shape of this rock.
    pub fn get_shape(&self) -> RockShape {
        self.shape.clone()
    }

    /// Gets the [position](Position) of this rock.
    ///
    /// Returns a copy of the position of this rock.
    pub fn get_pos(&self) -> Position {
        self.pos.clone()
    }

    /// Checks whether this rock can roll around.
    ///
    /// Rocks can only roll around when their [shape](RockShape) is [round](RockShape::Round).
    pub fn can_roll(&self) -> bool {
        self.shape == RockShape::Round
    }
}

/// The four cardinal directions.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    West,
    East,
}
