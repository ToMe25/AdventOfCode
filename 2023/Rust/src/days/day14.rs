//! My rust AoC 2023 day 14 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 14](https://adventofcode.com/2023/day/14).

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::num::NonZero;
use std::ops::{Index, IndexMut};

use super::super::get_input_file;
use super::DayRunner;

/// The day 14 runner.
///
/// The [DayRunner] implementation for the aoc day 14.
#[derive(Debug, Clone, Default)]
pub struct Day14Runner {
    /// The slots in which round rocks can be, for each direction.
    slots: [Vec<Vec<RockSlot>>; 4],
    /// The initial positions of the round rocks.
    initial_positions: Vec<Rock>,
    /// The last existing x position in the input data.
    max_x: u32,
    /// The last existing y position in the input data.
    max_y: u32,
}

impl Day14Runner {
    /// Calculates the distance from a position to the map edge in a given direction.
    pub fn calculate_dist_to_edge(&self, pos: &Position, direction: Direction) -> u32 {
        match direction {
            Direction::North => pos.get_y(),
            Direction::South => self.max_y - pos.get_y(),
            Direction::West => pos.get_x(),
            Direction::East => self.max_x - pos.get_x(),
        }
    }

    /// Tilts the platform in one of the cardinal directions.
    ///
    /// Generates a new map of the rocks on the platform based on a previous map and a tilt direction.
    pub fn tilt_platform(&self, round_rocks: &Vec<Rock>, direction: Direction) -> Vec<Rock> {
        let mut slots = self.slots[direction].clone();
        round_rocks.iter().for_each(|rock| {
            let dist = self.calculate_dist_to_edge(rock.get_pos(), direction);
            match direction {
                Direction::North | Direction::South => {
                    &mut slots[TryInto::<usize>::try_into(rock.get_pos().get_x())
                        .expect("Rock X pos outside usize range")]
                }
                Direction::West | Direction::East => {
                    &mut slots[TryInto::<usize>::try_into(rock.get_pos().get_y())
                        .expect("Rock Y pos outside usize range")]
                }
            }
            .iter_mut()
            .filter(|slot| slot.get_start_pos() <= dist)
            .last()
            .expect("No valid slot found")
            .insert_rock(dist)
            .expect("Inserting rock failed");
        });

        let result: Vec<Rock> = slots
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().flat_map(move |slot| {
                    (0..slot.get_rocks()).map(move |j| match direction {
                        Direction::North => Rock::new(
                            RockShape::Round,
                            i.try_into().expect("X pos outside u32 range"),
                            slot.get_start_pos() + j,
                        ),
                        Direction::South => Rock::new(
                            RockShape::Round,
                            i.try_into().expect("X pos outside u32 range"),
                            self.max_y - slot.get_start_pos() - j,
                        ),
                        Direction::West => Rock::new(
                            RockShape::Round,
                            slot.get_start_pos() + j,
                            i.try_into().expect("Y pos outside u32 range"),
                        ),
                        Direction::East => Rock::new(
                            RockShape::Round,
							self.max_x - slot.get_start_pos() - j,
                            i.try_into().expect("Y pos outside u32 range"),
                        ),
                    })
                })
            })
            .collect();

        result
    }

    /// Calculates the load on the support beams on one side of the platform.
    ///
    /// Sums the distances of [round](RockShape::Round) rocks from the side.
    /// This is considered to be the load on the support beams on the given side.
    /// Cube-shaped rocks are considered to cause no load, perhaps because they only cause static load?
    pub fn calculate_load(&self, rocks: &Vec<Rock>, side: Direction) -> u64 {
        rocks
            .iter()
            .filter(|rock| rock.can_roll())
            .map(|rock| self.calculate_dist_to_edge(rock.get_pos(), side.oposite()) + 1)
            .map(|load| Into::<u64>::into(load))
            .sum()
    }
}

impl DayRunner for Day14Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(14)?)?;
        let rocks: Vec<Rock> = input_data
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        let x: u32 = x.try_into().expect("X position out of range");
                        let y: u32 = y.try_into().expect("Y position out of range");
                        self.max_x = self.max_x.max(x);
                        self.max_y = self.max_y.max(y);
                        match c {
                            'O' => Some(Rock::new(RockShape::Round, x, y)),
                            '#' => Some(Rock::new(RockShape::Cube, x, y)),
                            _ => None,
                        }
                    })
                    .collect::<Vec<Rock>>()
            })
            .collect();

        for dir in [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            self.slots[dir] = rocks
                .iter()
                .fold(
                    match dir {
                        Direction::North | Direction::South => 0..=self.max_x,
                        Direction::West | Direction::East => 0..=self.max_y,
                    }
                    .map(|_| Vec::new())
                    .collect::<Vec<Vec<Rock>>>(),
                    |mut acc, &rock| {
                        Vec::<Rock>::push(
                            match dir {
                                Direction::North | Direction::South => {
                                    &mut acc[TryInto::<usize>::try_into(rock.get_pos().get_x())
                                        .expect("X pos too large for usize")]
                                }
                                Direction::West | Direction::East => {
                                    &mut acc[TryInto::<usize>::try_into(rock.get_pos().get_y())
                                        .expect("Y pos too large for usize")]
                                }
                            },
                            rock,
                        );
                        acc
                    },
                )
                .iter()
                .map(|row| {
                    let mut slots = row
                        .iter()
                        .filter(|&rock| !rock.can_roll())
                        .filter_map(|rock| {
                            let start_dist = self.calculate_dist_to_edge(rock.get_pos(), dir) + 1;
                            let end_dist = row
                                .iter()
                                .filter(|&rock| !rock.can_roll())
                                .map(|rock| self.calculate_dist_to_edge(rock.get_pos(), dir) + 1)
                                .filter(|&dist| dist > start_dist)
                                .min()
                                .unwrap_or(match dir {
                                    Direction::North | Direction::South => self.max_y + 2,
                                    Direction::West | Direction::East => self.max_x + 2,
                                });

                            if end_dist - start_dist < 2 {
                                None
                            } else {
                                Some(RockSlot::new(
                                    start_dist,
                                    NonZero::new(end_dist - start_dist - 1)?,
                                    dir,
                                ))
                            }
                        })
                        .collect::<Vec<RockSlot>>();

                    let end_dist = row
                        .iter()
                        .filter(|&rock| !rock.can_roll())
                        .map(|rock| self.calculate_dist_to_edge(rock.get_pos(), dir) + 1)
                        .min()
                        .unwrap_or(match dir {
                            Direction::North | Direction::South => self.max_y + 2,
                            Direction::West | Direction::East => self.max_x + 2,
                        });
                    let len = NonZero::new(end_dist.saturating_sub(1));
                    match len {
                        Some(len) => {
                            slots.push(RockSlot::new(0, len, dir));
                        }
                        None => (),
                    }
                    slots.sort();
                    slots
                })
                .collect();
        }

        self.initial_positions = rocks
            .iter()
            .filter(|rock| rock.can_roll())
            .cloned()
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let rocks = self.tilt_platform(&self.initial_positions, Direction::North);
        let load = self.calculate_load(&rocks, Direction::North);
        Ok(Some(load.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let mut map = self.initial_positions.clone();
        for _ in 0..1000000000 {
            for dir in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                map = self.tilt_platform(&map, dir);
            }
        }

        let load = self.calculate_load(&map, Direction::North);
        Ok(Some(load.to_string()))
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
    /// Returns a reference to the shape of this rock.
    pub fn get_shape(&self) -> &RockShape {
        &self.shape
    }

    /// Gets the [position](Position) of this rock.
    ///
    /// Returns a reference to the position of this rock.
    pub fn get_pos(&self) -> &Position {
        &self.pos
    }

    /// Checks whether this rock can roll around.
    ///
    /// Rocks can only roll around when their [shape](RockShape) is [round](RockShape::Round).
    pub fn can_roll(&self) -> bool {
        self.shape == RockShape::Round
    }
}

/// The four cardinal directions.
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Direction {
    North = 0,
    South,
    West,
    East,
}

impl Direction {
    /// Gets the direction oposite from this one.
    pub fn oposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

impl<T, const N: usize> Index<Direction> for [T; N] {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T, const N: usize> IndexMut<Direction> for [T; N] {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

/// The errors that can occur during [RockSlot] operations.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RockSlotError {
    /// Trying to create a [RockSlot] with a length of 0.
    ZeroSlotLength,
    /// Trying to add a [Rock] after the end of this slot.
    IndexOutOfBounds { index: u32, min: u32, max: u32 },
    /// Trying to increase the [Rock] count to more than length.
    CountOutOfBounds { count: u32, len: u32 },
}

impl Display for RockSlotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::ZeroSlotLength => write!(f, "Received a slot length of 0!"),
            Self::IndexOutOfBounds { index, min, max } => {
                write!(
                    f,
                    "Trying to add a rock at index {index} in range {min} - {max}!"
                )
            }
            Self::CountOutOfBounds { count, len } => {
                write!(f, "Trying to set rock count to {count} with length {len}!")
            }
        }
    }
}

/// A space that can contain multiple round [rocks](Rock) between two [cube-shaped](RockShape::Cube) [rocks](Rock) or map borders.
///
/// Each RockSlot can contain a limited number of [round](RockShape::Round) rocks.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct RockSlot {
    /// The distance from the map border in the direction of the slot of the first empty space.
    start_pos: u32,
    /// The number of free spots until the next border or [cube-shaped](RockShape::Cube) [rock](Rock).
    length: NonZero<u32>,
    /// The number of [round](RockShape::Round) [rocks](Rock) in this slot.
    rocks: u32,
    /// The direction in which this slot is sligned.
    direction: Direction,
}

impl RockSlot {
    /// Creates a new RockSlot at the given position.
    ///
    /// This new RockSlot will be initialized with zero [round](RockShape::Round) [rocks](Rock) contained in it.
    /// Rocks can be added with [insert_rock](Self::insert_rock], [add_rock](Self::add_rock), or [set_rocks](Self::set_rocks).
    pub fn new(start_pos: u32, length: NonZero<u32>, direction: Direction) -> RockSlot {
        RockSlot {
            start_pos,
            length,
            rocks: 0,
            direction,
        }
    }

    /// Creates a new RockSlot with a given number of rocks.
    ///
    /// This RockSlot will be initialized with a custom number of [round](RockShape::Round) [rocks](Rock) contained in it.
    pub fn with_rocks(
        start_pos: u32,
        length: NonZero<u32>,
        rocks: u32,
        direction: Direction,
    ) -> RockSlot {
        RockSlot {
            start_pos,
            length,
            rocks,
            direction,
        }
    }

    /// Gets the first position in the direction of this slot at which a rock could be placed.
    pub fn get_start_pos(&self) -> u32 {
        self.start_pos
    }

    /// Gets the first position after this slot.
    ///
    /// Returns the first position at which a rock in this slot may not be placed.
    /// This could either be the position after the map border, or the position of a [cube-shaped](RockShape::Cube) [rock](Rock).
    pub fn get_end_pos(&self) -> u32 {
        self.start_pos + self.length.get()
    }

    /// Gets the number of [rocks](Rock) this slot can contain.
    pub fn get_length(&self) -> NonZero<u32> {
        self.length
    }

    /// Gets the direction in which this slot is aligned.
    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    /// Gets the number of [rocks](Rock) that are currently known to be be contained in this slot.
    ///
    /// This number can be changed using [insert_rock](Self::insert_rock), [add_rock](Self::add_rock), or [set_rocks](Self::set_rocks).
    pub fn get_rocks(&self) -> u32 {
        self.rocks
    }

    /// Sets the number of rocks contained in this slot.
    ///
    /// # Errors
    ///
    /// If the number of rocks is greater than the length of the slot.
    pub fn set_rocks(&mut self, new_rocks: u32) -> Result<(), RockSlotError> {
        if new_rocks > self.length.get() {
            Err(RockSlotError::CountOutOfBounds {
                count: new_rocks,
                len: self.length.get(),
            })
        } else {
            self.rocks = new_rocks;
            Ok(())
        }
    }

    /// Adds a single new rock with an unknown position to this slot.
    ///
    /// To insert a rock with a known position use [insert_rock](Self::insert_rock).
    /// To get the number of contained rocks use [get_rocks](Self::get_rocks).
    ///
    /// # Errors
    ///
    /// If the number of rocks is greater than the length of the slot.
    pub fn add_rock(&mut self) -> Result<(), RockSlotError> {
        self.set_rocks(self.rocks + 1)
    }

    /// Inerts a new rock with a known position into this slot.
    ///
    /// The position is the distance to the map border in the direction of this slot.
    ///
    /// # Errors
    ///
    /// If the number of rocks is greater than the length of the slot.
    /// Or if the position is outside of this slot.
    pub fn insert_rock(&mut self, position: u32) -> Result<(), RockSlotError> {
        if position < self.start_pos || position >= self.get_end_pos() {
            Err(RockSlotError::IndexOutOfBounds {
                index: position,
                min: self.start_pos,
                max: self.get_end_pos(),
            })
        } else {
            self.add_rock()
        }
    }
}
