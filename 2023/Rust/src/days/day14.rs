//! My rust AoC 2023 day 14 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 14](https://adventofcode.com/2023/day/14).

use std::borrow::Borrow;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::FusedIterator;
use std::num::NonZero;
use std::ops::{Index, IndexMut};
use std::slice::ChunksExact;
use std::thread::panicking;
use std::{fs, mem};

use super::super::get_input_file;
use super::DayRunner;

/// The day 14 runner.
///
/// The [DayRunner] implementation for the aoc day 14.
#[derive(Debug, Clone, Default)]
pub struct Day14Runner {
    /// The map with the rocks on it.
    map: RockMap,
}

impl Day14Runner {
    /// Calculates the distance from a position to the map edge in a given direction.
    pub fn calculate_dist_to_edge(&self, pos: &Position, direction: Direction) -> u32 {
        match direction {
            Direction::North => pos.get_y(),
            Direction::South => self.map.get_height() - pos.get_y() - 1,
            Direction::West => pos.get_x(),
            Direction::East => self.map.get_width() - pos.get_x() - 1,
        }
    }

    /// Tilts the platform in one of the cardinal directions.
    ///
    /// Generates a new map of the rocks on the platform based on a previous map and a tilt direction.
    pub fn tilt_platform(&self, map: &mut RockMap, direction: Direction) {
        match direction {
            Direction::North => {
                (0..map.get_width()).for_each(|column| {
                    map.col_iter_mut(column)
                        .expect("Invalid column")
                        .fold((None, 0), |(mut last_rock, count), rock| {
                            if let Some(rock) = rock {
                                if rock.can_roll() {
                                    rock.get_pos_mut().set_y(last_rock.map_or(0, |y| y + 1));
                                }
                                last_rock.replace(rock.get_pos().get_y());
                            }
                            (last_rock, count + 1)
                        })
                        .1;
                });
            }
            Direction::South => {
                let map_height = map.get_height();
                (0..map.get_width()).for_each(|column| {
                    map.col_iter_mut(column)
                        .expect("Invalid column")
                        .rev()
                        .fold((None, 0), |(mut last_rock, count), rock| {
                            if let Some(rock) = rock {
                                if rock.can_roll() {
                                    rock.get_pos_mut()
                                        .set_y(last_rock.map_or(map_height - 1, |y| y - 1));
                                }
                                last_rock.replace(rock.get_pos().get_y());
                            }
                            (last_rock, count + 1)
                        })
                        .1;
                });
            }
            Direction::West => {
                (0..map.get_height()).for_each(|row| {
                    map.row_iter_mut(row)
                        .expect("Invalid row")
                        .fold((None, 0), |(mut last_rock, count), rock| {
                            if let Some(rock) = rock {
                                if rock.can_roll() {
                                    rock.get_pos_mut().set_x(last_rock.map_or(0, |x| x + 1));
                                }
                                last_rock.replace(rock.get_pos().get_x());
                            }
                            (last_rock, count + 1)
                        })
                        .1;
                });
            }
            Direction::East => {
                let map_width = map.get_width();
                (0..map.get_height()).for_each(|row| {
                    map.row_iter_mut(row)
                        .expect("Invalid row")
                        .rev()
                        .fold((None, 0), |(mut last_rock, count), rock| {
                            if let Some(rock) = rock {
                                if rock.can_roll() {
                                    rock.get_pos_mut()
                                        .set_x(last_rock.map_or(map_width - 1, |x| x - 1));
                                }
                                last_rock.replace(rock.get_pos().get_x());
                            }
                            (last_rock, count + 1)
                        })
                        .1;
                });
            }
        }
    }

    /// Calculates the load on the support beams on one side of the platform.
    ///
    /// Sums the distances of [round](RockShape::Round) rocks from the side.
    /// This is considered to be the load on the support beams on the given side.
    /// Cube-shaped rocks are considered to cause no load, perhaps because they only cause static load?
    pub fn calculate_load(&self, map: &RockMap, side: Direction) -> u64 {
        map.iter()
            .flatten()
            .filter_map(|rock| rock.as_ref())
            .filter(|rock| rock.can_roll())
            .map(|rock| {
                Into::<u64>::into(self.calculate_dist_to_edge(rock.get_pos(), side.oposite()) + 1)
            })
            .sum()
    }
}

impl DayRunner for Day14Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(14)?)?;
        self.map = input_data
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        let x: u32 = x.try_into().expect("X position out of range");
                        let y: u32 = y.try_into().expect("Y position out of range");
                        match c {
                            'O' => Some(Rock::new(RockShape::Round, x, y)),
                            '#' => Some(Rock::new(RockShape::Cube, x, y)),
                            _ => None,
                        }
                    })
                    .collect::<Vec<Rock>>()
            })
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let mut map = self.map.clone();
        self.tilt_platform(&mut map, Direction::North);
        let load = self.calculate_load(&map, Direction::North);
        Ok(Some(load.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let mut map = self.map.clone();
        for _ in 0..1000000000 {
            for dir in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                self.tilt_platform(&mut map, dir);
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
                    &Some(r) if r.can_roll() => 'O',
                    &Some(_) => '#',
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

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

impl From<&Position> for Position {
    fn from(value: &Position) -> Self {
        value.clone()
    }
}

impl From<(u32, u32)> for Position {
    fn from(value: (u32, u32)) -> Self {
        Position::new(value.0, value.1)
    }
}

/// The possible shapes a [Rock] can have.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RockShape {
    /// A rounded rock that will roll around when the platform is tilted.
    Round,
    /// A cube-shaped rock that wont move around.
    Cube,
}

impl Display for RockShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
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
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let rock = Rock::new(RockShape::Cube, 107, 21);
    /// ```
    pub fn new(shape: RockShape, pos_x: u32, pos_y: u32) -> Rock {
        Self::with_position(shape, Position::new(pos_x, pos_y))
    }

    /// Creates a new rock from an exsting [Position].
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let rock = Rock::with_position(RockShape::Cube, Position::new(5, 3));
    /// ```
    ///
    /// A tuple of two u32s can also be used instead of a position:
    /// ```
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let rock = Rock::with_position(RockShape::Cube, (1, 315));
    /// ```
    pub fn with_position<Pos: Into<Position>>(shape: RockShape, position: Pos) -> Rock {
        Rock {
            shape,
            pos: position.into(),
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

    /// Gets the [position](Position) of this rock.
    ///
    /// Returns a mutable reference to the position of this rock.
    pub fn get_pos_mut(&mut self) -> &mut Position {
        &mut self.pos
    }

    /// Sets the [position](Position) of this rock.
    pub fn set_pos<Pos: Into<Position>>(&mut self, pos: Pos) {
        self.pos = pos.into();
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

    /// Returns the numerical index representing this direction.
    pub fn index(self) -> usize {
        self.into()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        value as u8
    }
}

impl From<Direction> for u16 {
    fn from(value: Direction) -> Self {
        value as u16
    }
}

impl From<Direction> for u32 {
    fn from(value: Direction) -> Self {
        value as u32
    }
}

impl From<Direction> for u64 {
    fn from(value: Direction) -> Self {
        value as u64
    }
}

impl From<Direction> for u128 {
    fn from(value: Direction) -> Self {
        value as u128
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        value as usize
    }
}

impl<T, const N: usize> Index<Direction> for [T; N] {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        &self[index.index()]
    }
}

impl<T, const N: usize> IndexMut<Direction> for [T; N] {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self[index.index()]
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
    /// Rocks can be added with [insert_rock](Self::insert_rock), [add_rock](Self::add_rock), or [set_rocks](Self::set_rocks).
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

/// The errors that can occur during [RockMap] operations.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RockMapError {
    /// The required vector would be larger than the memory limit for Vec allows.
    /// The requested size may be more than usize::MAX, so use u128 instead.
    VecSizeLimit(u128),
    /// The requested position is outside the area the map covers.
    PositionOutOfBounds {
        pos_x: Option<u32>,
        pos_y: Option<u32>,
        width: u32,
        height: u32,
    },
    /// There is already a rock at the given position.
    PositionOccupied(Position),
    /// Attempting to shrink the map.
    MapShrinking {
        new_width: u32,
        new_height: u32,
        old_width: u32,
        old_height: u32,
    },
}

impl Display for RockMapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::VecSizeLimit(req) => {
                write!(
                    f,
                    "Requested map size of {req} bytes is more than the Vec limit of {} bytes",
                    isize::MAX
                )
            }
            Self::PositionOutOfBounds {
                pos_x,
                pos_y,
                width,
                height,
            } => {
                write!(
                    f,
                    "Position {}:{} outside map with size {width}x{height}",
                    pos_x.map_or("?".to_string(), |x| x.to_string()),
                    pos_y.map_or("?".to_string(), |y| y.to_string())
                )
            }
            Self::PositionOccupied(pos) => {
                write!(f, "Position {pos} is already occupied")
            }
            Self::MapShrinking {
                new_width,
                new_height,
                old_width,
                old_height,
            } => {
                write!(f, "Trying to shring map from {old_width}x{old_height} to {new_width}x{new_height}")
            }
        }
    }
}

/// A two dimensional map of [rocks](Rock).
///
/// A RockMap stores the type of [rock](Rock) placed at each position.  
/// It is guaranteed to always be rectangular.
///
/// It also automatically grows when a rock is inserted at a position outside the current map.
///
/// In addition a RockMap allows you to iterate over both rows and columns of [rocks](Rock).
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rust_aoc_2023::days::day14::Position;
/// use rust_aoc_2023::days::day14::Rock;
/// use rust_aoc_2023::days::day14::RockMap;
/// use rust_aoc_2023::days::day14::RockMapError;
/// use rust_aoc_2023::days::day14::RockShape;
///
/// let mut map = RockMap::with_size(17, 14)?;
/// map.add_rock(Position::new(5, 13), RockShape::Round)?;
/// map.add_rock(Position::new(11, 4), RockShape::Cube)?;
/// map.add_rock(Position::new(9, 9), RockShape::Round)?;
///
/// assert_eq!(map.get(Position::new(5, 5))?, &None);
/// assert_eq!(map.get(Position::new(11, 4))?, &Some(Rock::new(RockShape::Cube, 11, 4)));
/// assert_eq!(map.get(Position::new(12, 13))?, &None);
/// assert_eq!(map.get(Position::new(15, 15)), Err(RockMapError::PositionOutOfBounds{
///     pos_x: Some(15), pos_y: Some(15), width: 17, height: 14
/// }));
/// #
/// # Result::<(), RockMapError>::Ok(())
/// ```
///
/// Getting iterators for rows and columns of the map:
/// ```
/// use rust_aoc_2023::days::day14::Position;
/// use rust_aoc_2023::days::day14::Rock;
/// use rust_aoc_2023::days::day14::RockMap;
/// # use rust_aoc_2023::days::day14::RockMapError;
/// use rust_aoc_2023::days::day14::RockShape;
///
/// let mut map = RockMap::with_size(15, 15)?;
/// map.add_rock(Position::new(7, 5), RockShape::Round)?;
/// map.add_rock(Position::new(5, 7), RockShape::Round)?;
/// map.add_rock(Position::new(7, 7), RockShape::Cube)?;
/// map.add_rock(Position::new(9, 7), RockShape::Round)?;
/// map.add_rock(Position::new(7, 9), RockShape::Round)?;
///
/// assert_eq!(map.row_iter(7)?.copied().collect::<Vec<Option<Rock>>>(), vec![
///     None, None, None, None, None, Some(Rock::new(RockShape::Round, 5, 7)),
///     None, Some(Rock::new(RockShape::Cube, 7, 7)), None,
///     Some(Rock::new(RockShape::Round, 9, 7)), None, None, None, None, None]);
/// assert_eq!(map.col_iter(7)?.copied().collect::<Vec<Option<Rock>>>(), vec![
///     None, None, None, None, None, Some(Rock::new(RockShape::Round, 7, 5)),
///     None, Some(Rock::new(RockShape::Cube, 7, 7)), None,
///     Some(Rock::new(RockShape::Round, 7, 9)), None, None, None, None, None]);
/// #
/// # Result::<(), RockMapError>::Ok(())
/// ```
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct RockMap {
    /// The vector containing the rocks that are on the map.
    rocks: Vec<Option<Rock>>,
    /// The current width of this map.
    width: u32,
    /// The current height of this map.
    height: u32,
}

impl RockMap {
    /// Creates a new map with a width and height of zero.
    ///
    /// This map will change its size when rocks are added.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_aoc_2023::days::day14::RockMap;
    ///
    /// let map = RockMap::new();
    /// assert_eq!(map.get_width(), 0);
    /// assert_eq!(map.get_height(), 0);
    /// ```
    pub fn new() -> RockMap {
        RockMap {
            rocks: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    /// Creates a new map with a given size.
    ///
    /// Each position is initialized to not contain any rock at all.
    ///
    /// Note that if either the width or height is zero, both dimensions of the new map will be zero.
    ///
    /// # Errors
    ///
    /// This function returns an error if <code>width * height * [mem::size_of]<[Rock]>()</code> is greater than [`isize::MAX`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    ///
    /// let map = RockMap::with_size(15, 12)?;
    /// assert_eq!(map.get_width(), 15);
    /// assert_eq!(map.get_height(), 12);
    /// assert!(map.iter().flatten().all(|rock| rock.is_none()));
    /// assert_eq!(map.iter().flatten().count(), 15 * 12);
    /// assert!(map.iter().all(|row| row.len() == 15));
    /// assert_eq!(map.iter().count(), 12);
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    ///
    /// Trying to create a map that uses too much memory:
    /// ```
    /// use rust_aoc_2023::days::day14::RockMap;
    /// use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::Rock;
    ///
    /// let map = RockMap::with_size(u32::MAX / 2, u32::MAX / 2);
    /// let size: u128 = (2u128.pow(31) - 1).pow(2)
    ///     * TryInto::<u128>::try_into(std::mem::size_of::<Rock>()).unwrap();
    /// assert_eq!(map, Err(RockMapError::VecSizeLimit(size)));
    /// ```
    pub fn with_size(mut width: u32, mut height: u32) -> Result<RockMap, RockMapError> {
        if width == 0 {
            height = 0;
        }
        if height == 0 {
            width = 0;
        }

        let requested: u128 = Self::calculate_memory(width, height);
        if requested
            > isize::MAX
                .try_into()
                .expect("No support for > 128 bit pointers, lol")
        {
            Err(RockMapError::VecSizeLimit(requested))
        } else {
            Ok(RockMap {
                rocks: (0..width * height).map(|_| None).collect(),
                width,
                height,
            })
        }
    }

    /// Calculates the memory used for a map of a given size.
    #[inline]
    fn calculate_memory(width: u32, height: u32) -> u128 {
        let rock_size = mem::size_of::<Rock>();
        // If one Rock uses more than u128::MAX bytes, this was never going to run anyway
        Into::<u128>::into(width)
            * Into::<u128>::into(height)
            * TryInto::<u128>::try_into(rock_size)
                .expect("size_of<Rock>() shouldn't be more than u128::MAX")
    }

    /// Gets the current width of this map.
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Gets the current height of this map.
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Gets a reference to the [Rock] at a given position.
    ///
    /// Note that there isn't a `get_mut` because updating the map when a rock is changed that way would be hard.
    ///
    /// # Errors
    ///
    /// This function returns an error when the given position is outside if its current size.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::with_size(3, 5)?;
    /// map.add_rock(Position::new(2, 2), RockShape::Round)?;
    /// assert_eq!(map.get(Position::new(2, 2)), Ok(&Some(Rock::new(RockShape::Round, 2, 2))));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    ///
    /// Trying to get a rock outside the map:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// use rust_aoc_2023::days::day14::RockMapError;
    ///
    /// let mut map = RockMap::with_size(7, 1)?;
    /// assert_eq!(map.get(Position::new(5, 2)), Err(RockMapError::PositionOutOfBounds{
    ///     pos_x: Some(5), pos_y: Some(2), width: 7, height: 1
    /// }));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    pub fn get<Pos: Borrow<Position>>(&self, pos: Pos) -> Result<&Option<Rock>, RockMapError> {
        let pb = pos.borrow();
        if pb.get_x() >= self.width || pb.get_y() >= self.height {
            Err(RockMapError::PositionOutOfBounds {
                pos_x: Some(pb.get_x()),
                pos_y: Some(pb.get_y()),
                width: self.width,
                height: self.height,
            })
        } else {
            Ok(self
                .rocks
                .get(
                    TryInto::<usize>::try_into(pb.get_x() + pb.get_y() * self.width)
                        .expect("Size checked above"),
                )
                .expect("Size checked above"))
        }
    }

    /// Increases the size of the map to the given dimensions.
    ///
    /// All the newly created spaces are filled with [`None`].
    ///
    /// # Errors
    ///
    /// This function errors if one of the new dimensions is smaller than the maps current size in that dimension.  
    /// It also returns an error if the new memory size would be more than [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::with_size(3, 3)?;
    /// map.add_rock(Position::new(2, 2), RockShape::Cube)?;
    /// map.grow(5, 7)?;
    /// # assert_eq!(map.get_width(), 5);
    /// # assert_eq!(map.get_height(), 7);
    /// assert_eq!(map.get(Position::new(4, 6))?, &None);
    /// assert_eq!(map.get(Position::new(2, 2))?, &Some(Rock::new(RockShape::Cube, 2, 2)));
    /// # assert_eq!(map.get(Position::new(2, 7)), Err(RockMapError::PositionOutOfBounds{pos_x: Some(2), pos_y: Some(7), width: 5, height: 7}));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    ///
    /// Trying to reduce map size:
    /// ```
    /// use rust_aoc_2023::days::day14::RockMap;
    /// use rust_aoc_2023::days::day14::RockMapError;
    ///
    /// let mut map = RockMap::with_size(5, 3)?;
    /// assert_eq!(map.grow(3, 5), Err(RockMapError::MapShrinking{
    ///     new_width: 3,
    ///     new_height: 5,
    ///     old_width: 5,
    ///     old_height: 3
    /// }));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    ///
    /// Set size of empty map:
    /// ```
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    ///
    /// let mut map = RockMap::new();
    /// map.grow(7, 15)?;
    /// assert_eq!(map.get_width(), 7);
    /// assert_eq!(map.get_height(), 15);
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    ///
    /// Attempt to grow map to more than [`isize::MAX`] bytes:
    /// ```
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// use rust_aoc_2023::days::day14::RockMapError;
    ///
    /// let mut map = RockMap::with_size(71, 95)?;
    /// assert_eq!(map.grow(u32::MAX, u32::MAX / 4), Err(RockMapError::VecSizeLimit(
    ///     Into::<u128>::into(u32::MAX) * Into::<u128>::into(u32::MAX / 4)
    ///         * TryInto::<u128>::try_into(std::mem::size_of::<Rock>()).unwrap()
    /// )));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    pub fn grow(&mut self, new_width: u32, new_height: u32) -> Result<(), RockMapError> {
        if new_width < self.width || new_height < self.height {
            return Err(RockMapError::MapShrinking {
                new_width,
                new_height,
                old_width: self.width,
                old_height: self.height,
            });
        }

        let mut new_width = new_width.max(self.width);
        let mut new_height = new_height.max(self.height);
        if new_width == 0 {
            new_height = 0;
        }
        if new_height == 0 {
            new_width = 0;
        }

        if new_width == self.width && new_height == self.height {
            return Ok(());
        }

        let requested: u128 = Self::calculate_memory(new_width, new_height);
        if requested
            > isize::MAX
                .try_into()
                .expect("No support for > 128 bit pointers, lol")
        {
            return Err(RockMapError::VecSizeLimit(requested));
        }

        let additional_width = new_width - self.width;
        let additional_height = new_height - self.height;
        if additional_width > 0 {
            for y in 0..self.height {
                (0..additional_width).for_each(|x| {
                    self.rocks.insert(
                        TryInto::<usize>::try_into(y * new_width + self.width + x)
                            .expect("Size checked above"),
                        None,
                    )
                });
            }
            self.width = new_width;
        }
        if additional_height > 0 {
            self.rocks
                .extend((0..additional_height * self.width).map(|_| None));
            self.height = new_height;
        }

        Ok(())
    }

    /// Adds a new rock to the map.
    ///
    /// Places a new rock with the given shape at the given positions.  
    /// This operation grows the map, if necessary.
    ///
    /// # Errors
    ///
    /// This function returns an error if there is already a rock at the given position.  
    /// It also returns an error if growing the map enough to contain the new position would cause it to use more than [`isize::MAX`] bytes of memory.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// # use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::with_size(15, 9)?;
    /// map.add_rock(Position::new(7, 5), RockShape::Cube);
    /// # assert_eq!(map.get(Position::new(7, 5)), Ok(&Some(Rock::new(RockShape::Cube, 7, 5))));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    ///
    /// Initializing a map by adding a rock:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// # use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::new();
    /// map.add_rock((7, 15), RockShape::Round)?;
    ///
    /// assert_eq!(map.get_width(), 8);
    /// assert_eq!(map.get_height(), 16);
    /// # assert_eq!(map.get(Position::new(7, 15)), Ok(&Some(Rock::new(RockShape::Round, 7, 15))));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    ///
    /// Growing a map by adding a rock:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::with_size(3, 1)?;
    /// map.add_rock(&Position::new(1, 6), RockShape::Cube)?;
    ///
    /// assert_eq!(map.get_width(), 3);
    /// assert_eq!(map.get_height(), 7);
    /// assert_eq!(map.get(Position::new(1, 6)), Ok(&Some(Rock::new(RockShape::Cube, 1, 6))));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    pub fn add_rock<Pos: Into<Position>>(
        &mut self,
        pos: Pos,
        shape: RockShape,
    ) -> Result<(), RockMapError> {
        self.insert_rock(Rock::with_position(shape, pos))
    }

    /// Adds a new rock to the map.
    ///
    /// Places a given rock at its position in the map.  
    /// This operation grows the map, if necessary.
    ///
    /// # Errors
    ///
    /// This function returns an error if there is already a rock at the given position.  
    /// It also returns an error if growing the map enough to contain the new position would cause it to use more than [`isize::MAX`] bytes of memory.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// # use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::with_size(7, 19)?;
    /// map.add_rock(Position::new(3, 15), RockShape::Cube);
    /// # assert_eq!(map.get(Position::new(3, 15)), Ok(&Some(Rock::new(RockShape::Cube, 3, 15))));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    ///
    /// Initializing a map by adding a rock:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// # use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::new();
    /// map.add_rock((9, 2), RockShape::Round)?;
    ///
    /// assert_eq!(map.get_width(), 10);
    /// assert_eq!(map.get_height(), 3);
    /// # assert_eq!(map.get(Position::new(9, 2)), Ok(&Some(Rock::new(RockShape::Round, 9, 2))));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    ///
    /// Growing a map by adding a rock:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::with_size(2, 5)?;
    /// map.add_rock(&Position::new(5, 2), RockShape::Cube)?;
    ///
    /// assert_eq!(map.get_width(), 6);
    /// assert_eq!(map.get_height(), 5);
    /// assert_eq!(map.get(Position::new(5, 2)), Ok(&Some(Rock::new(RockShape::Cube, 5, 2))));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    pub fn insert_rock(&mut self, rock: Rock) -> Result<(), RockMapError> {
        if rock.get_pos().get_x() >= self.width || rock.get_pos().get_y() >= self.height {
            self.grow(
                self.width.max(rock.get_pos().get_x() + 1),
                self.height.max(rock.get_pos().get_y() + 1),
            )?;
        }

        let idx = TryInto::<usize>::try_into(
            rock.get_pos().get_x() + rock.get_pos().get_y() * self.width,
        )
        .expect("Size checked above");
        if self.rocks[idx].is_some() {
            Err(RockMapError::PositionOccupied(rock.get_pos().clone()))
        } else {
            self.rocks[idx] = Some(rock);
            Ok(())
        }
    }

    /// Creates an iterator iterating over the rows of this map.
    ///
    /// Each row is returned as a reference to a slice of type <code>[Option]<[Rock]></code>.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::with_size(5, 3)?;
    /// map.add_rock(Position::new(1, 2), RockShape::Round)?;
    /// map.add_rock(Position::new(4, 0), RockShape::Cube)?;
    ///
    /// let mut iterator = map.iter();
    /// assert_eq!(iterator.next(), Some(&[None, None, None, None, Some(Rock::new(RockShape::Cube, 4, 0))][..]));
    /// assert_eq!(iterator.next(), Some(&[None, None, None, None, None][..]));
    /// assert_eq!(iterator.next(), Some(&[None, Some(Rock::new(RockShape::Round, 1, 2)), None, None, None][..]));
    /// assert_eq!(iterator.next(), None);
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    pub fn iter(&self) -> ChunksExact<'_, Option<Rock>> {
        self.rocks.chunks_exact(
            self.width
                .try_into()
                .expect("Width should be checked in constructor"),
        )
    }

    // TODO implement (mutable) iterators over all rows/columns

    /*pub fn iter_mut(&mut self) -> ChunksExactMut<'_, Option<Rock>> {
        // FIXME update map
        self.rocks.chunks_exact_mut(
            self.width
                .try_into()
                .expect("Width should be checked in constructor"),
        )
    }*/

    /// Creates an iterator iterating over a single row of this map.
    ///
    /// # Errors
    ///
    /// This function returns an error of the given row is outside the map.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::with_size(5, 4)?;
    /// map.add_rock(Position::new(1, 2), RockShape::Round)?;
    /// map.add_rock(Position::new(2, 3), RockShape::Cube)?;
    /// map.add_rock(Position::new(3, 3), RockShape::Round)?;
    ///
    /// let mut iter = map.row_iter(3)?;
    /// # assert_eq!(iter.size_hint(), (5, Some(5)));
    /// assert_eq!(iter.next(), Some(&None));
    /// assert_eq!(iter.next(), Some(&None));
    /// assert_eq!(iter.next_back(), Some(&None));
    /// # assert_eq!(iter.size_hint(), (2, Some(2)));
    /// assert_eq!(iter.next(), Some(&Some(Rock::new(RockShape::Cube, 2, 3))));
    /// assert_eq!(iter.next_back(), Some(&Some(Rock::new(RockShape::Round, 3, 3))));
    /// # assert_eq!(iter.size_hint(), (0, Some(0)));
    /// assert_eq!(iter.next(), None);
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    pub fn row_iter(&self, row: u32) -> Result<RowIter<'_>, RockMapError> {
        if row >= self.height {
            Err(RockMapError::PositionOutOfBounds {
                pos_x: None,
                pos_y: Some(row),
                width: self.width,
                height: self.height,
            })
        } else {
            Ok(RowIter::new_row(self, row))
        }
    }

    /// Creates an iterator iterating over a single column of this map.
    ///
    /// # Errors
    ///
    /// This function returns an error of the given column is outside the map.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::with_size(3, 5)?;
    /// map.add_rock(Position::new(1, 4), RockShape::Round)?;
    /// map.add_rock(Position::new(2, 0), RockShape::Cube)?;
    /// map.add_rock(Position::new(2, 3), RockShape::Round)?;
    ///
    /// let mut iter = map.col_iter(2)?;
    /// # assert_eq!(iter.size_hint(), (5, Some(5)));
    /// assert_eq!(iter.next(), Some(&Some(Rock::new(RockShape::Cube, 2, 0))));
    /// assert_eq!(iter.next(), Some(&None));
    /// # assert_eq!(iter.size_hint(), (3, Some(3)));
    /// assert_eq!(iter.next_back(), Some(&None));
    /// assert_eq!(iter.next_back(), Some(&Some(Rock::new(RockShape::Round, 2, 3))));
    /// assert_eq!(iter.next(), Some(&None));
    /// # assert_eq!(iter.size_hint(), (0, Some(0)));
    /// assert_eq!(iter.next(), None);
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    pub fn col_iter(&self, column: u32) -> Result<RowIter<'_>, RockMapError> {
        if column >= self.width {
            Err(RockMapError::PositionOutOfBounds {
                pos_x: Some(column),
                pos_y: None,
                width: self.width,
                height: self.height,
            })
        } else {
            Ok(RowIter::new_col(self, column))
        }
    }

    /// Creates an iterator that mutably iterates over a single row of this map.
    ///
    /// # Errors
    ///
    /// This function returns an error if the given row is outside the map.
    ///
    /// # Panics
    ///
    /// This function doesn't panic, but the returned iterator **MAY** panic.  
    /// The iterator panics if a [rock](Rock) is moved outside of the area of the map,  
    /// or to a position where a [rock](Rock) is already present.
    ///
    /// # Examples
    ///
    /// Moving all [rocks](Rock) one row down:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::new();
    /// map.add_rock((2, 3), RockShape::Round)?;
    /// map.add_rock((1, 4), RockShape::Cube)?;
    /// map.add_rock((5, 3), RockShape::Cube)?;
    /// map.add_rock((4, 3), RockShape::Cube)?;
    ///
    /// map.row_iter_mut(3)?.for_each(|rock| {rock.as_mut().map(|rock| rock.get_pos_mut().set_y(4));});
    /// assert_eq!(map.get(Position::new(4, 3))?, &None);
    /// assert_eq!(map.get(Position::new(4, 4))?, &Some(Rock::new(RockShape::Cube, 4, 4)));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    ///
    /// Moving all [rocks](Rock) one column to the left:  
    /// Note that moving [rocks](Rock) to the right without using [Iterator::rev] is not recommended and may cause undexpected panics with the current implementation.
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::new();
    /// map.add_rock((1, 2), RockShape::Round)?;
    /// map.add_rock((2, 1), RockShape::Cube)?;
    /// map.add_rock((3, 2), RockShape::Cube)?;
    /// map.add_rock((4, 2), RockShape::Round)?;
    ///
    /// map.row_iter_mut(2)?.for_each(|rock| {
    ///     rock.as_mut().map(|rock| rock.set_pos(Position::new(rock.get_pos().get_x() - 1, rock.get_pos().get_y())));
    /// });
    /// assert_eq!(map.get(Position::new(0, 2))?, &Some(Rock::new(RockShape::Round, 0, 2)));
    /// assert_eq!(map.get(Position::new(1, 2))?, &None);
    /// assert_eq!(map.get(Position::new(2, 2))?, &Some(Rock::new(RockShape::Cube, 2, 2)));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    pub fn row_iter_mut(&mut self, row: u32) -> Result<RowIterMut<'_>, RockMapError> {
        if row >= self.height {
            Err(RockMapError::PositionOutOfBounds {
                pos_x: None,
                pos_y: Some(row),
                width: self.width,
                height: self.height,
            })
        } else {
            Ok(RowIterMut::new_row(self, row))
        }
    }

    /// Creates an iterator that mutably iterates over a single column of this map.
    ///
    /// # Errors
    ///
    /// This function returns an error if the given column is outside the map.
    ///
    /// # Panics
    ///
    /// This function doesn't panic, but the returned iterator **MAY** panic.  
    /// The iterator panics if a [rock](Rock) is moved outside of the area of the map,  
    /// or to a position where a [rock](Rock) is already present.
    ///
    /// # Examples
    ///
    /// Moving all [rocks](Rock) one column to the right:
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::new();
    /// map.add_rock((2, 2), RockShape::Cube)?;
    /// map.add_rock((2, 4), RockShape::Round)?;
    /// map.add_rock((4, 3), RockShape::Round)?;
    /// map.add_rock((2, 5), RockShape::Cube)?;
    ///
    /// map.col_iter_mut(2)?.for_each(|rock| {rock.as_mut().map(|rock| rock.get_pos_mut().set_x(3));});
    /// assert_eq!(map.get(Position::new(2, 4))?, &None);
    /// assert_eq!(map.get(Position::new(3, 4))?, &Some(Rock::new(RockShape::Round, 3, 4)));
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    ///
    /// Moving all [rocks](Rock) one row up:  
    /// Note that moving [rocks](Rock) down without using [Iterator::rev] is not recommended and may cause undexpected panics with the current implementation.
    /// ```
    /// use rust_aoc_2023::days::day14::Position;
    /// use rust_aoc_2023::days::day14::Rock;
    /// use rust_aoc_2023::days::day14::RockMap;
    /// # use rust_aoc_2023::days::day14::RockMapError;
    /// use rust_aoc_2023::days::day14::RockShape;
    ///
    /// let mut map = RockMap::new();
    /// map.add_rock((3, 1), RockShape::Round)?;
    /// map.add_rock((3, 2), RockShape::Cube)?;
    /// map.add_rock((2, 2), RockShape::Cube)?;
    /// map.add_rock((3, 4), RockShape::Round)?;
    ///
    /// map.col_iter_mut(3)?.for_each(|rock| {
    ///     rock.as_mut().map(|rock| rock.set_pos(Position::new(rock.get_pos().get_x(), rock.get_pos().get_y() - 1)));
    /// });
    /// assert_eq!(map.get(Position::new(3, 0))?, &Some(Rock::new(RockShape::Round, 3, 0)));
    /// assert_eq!(map.get(Position::new(3, 1))?, &Some(Rock::new(RockShape::Cube, 3, 1)));
    /// assert_eq!(map.get(Position::new(3, 2))?, &None);
    /// #
    /// # Result::<(), RockMapError>::Ok(())
    /// ```
    pub fn col_iter_mut(&mut self, column: u32) -> Result<RowIterMut<'_>, RockMapError> {
        if column >= self.width {
            Err(RockMapError::PositionOutOfBounds {
                pos_x: Some(column),
                pos_y: None,
                width: self.width,
                height: self.height,
            })
        } else {
            Ok(RowIterMut::new_col(self, column))
        }
    }
}

impl Display for RockMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.iter().try_for_each(|row| {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|r| match r {
                        &Some(r) if r.can_roll() => 'O',
                        &Some(_) => '#',
                        &None => '.',
                    })
                    .collect::<String>()
            )
        })
    }
}

impl<'a> IntoIterator for &'a RockMap {
    type Item = &'a [Option<Rock>];
    type IntoIter = ChunksExact<'a, Option<Rock>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/*impl<'a> IntoIterator for &'a mut RockMap {
    type Item = &'a mut [Option<Rock>];
    type IntoIter = ChunksExactMut<'a, Option<Rock>>;

    fn into_iter(self) -> Self::IntoIter {
        // TODO requires iter_mut
        self.iter_mut()
    }
}*/

impl<I: Borrow<Position>> Index<I> for RockMap {
    type Output = Option<Rock>;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index).expect("position outside of map")
    }
}

impl Extend<Rock> for RockMap {
    fn extend<Iter: IntoIterator<Item = Rock>>(&mut self, iter: Iter) {
        iter.into_iter()
            .for_each(|rock| self.insert_rock(rock).expect("Inserting a rock failed"));
    }
}

impl<'a> Extend<&'a Rock> for RockMap {
    fn extend<Iter: IntoIterator<Item = &'a Rock>>(&mut self, iter: Iter) {
        iter.into_iter()
            .copied()
            .for_each(|rock| self.insert_rock(rock).expect("Inserting a rock failed"));
    }
}

impl FromIterator<Rock> for RockMap {
    fn from_iter<Iter: IntoIterator<Item = Rock>>(iter: Iter) -> Self {
        let mut map = RockMap::new();
        map.extend(iter);
        map
    }
}

impl<'a> FromIterator<&'a Rock> for RockMap {
    fn from_iter<Iter: IntoIterator<Item = &'a Rock>>(iter: Iter) -> Self {
        let mut map = RockMap::new();
        map.extend(iter);
        map
    }
}

/// An iterator iterating over a single row or column of a [RockMap].
///
/// Instances of the class can be optained using [RockMap::row_iter] or [RockMap::col_iter].
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rust_aoc_2023::days::day14::Rock;
/// use rust_aoc_2023::days::day14::RockMap;
/// # use rust_aoc_2023::days::day14::RockMapError;
/// use rust_aoc_2023::days::day14::RockShape;
///
/// let mut map = RockMap::new();
/// map.add_rock((3, 4), RockShape::Cube)?;
/// map.add_rock((2, 6), RockShape::Round)?;
/// map.add_rock((3, 2), RockShape::Round)?;
/// map.add_rock((4, 2), RockShape::Round)?;
/// map.add_rock((3, 7), RockShape::Cube)?;
///
/// let mut iter = map.col_iter(3)?;
/// assert_eq!(iter.next(), Some(&None));
/// assert_eq!(iter.next_back(), Some(&Some(Rock::new(RockShape::Cube, 3, 7))));
/// assert_eq!(iter.nth_back(2), Some(&Some(Rock::new(RockShape::Cube, 3, 4))));
/// assert_eq!(iter.nth(1), Some(&Some(Rock::new(RockShape::Round, 3, 2))));
/// assert_eq!(iter.len(), 1);
/// assert_eq!(iter.next(), Some(&None));
/// assert_eq!(iter.next_back(), None);
/// #
/// # Result::<(), RockMapError>::Ok(())
/// ```
#[must_use]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct RowIter<'a> {
    /// The map to iterate over.
    map: &'a RockMap,
    /// The offset of the first item to return from the front of the rocks Vec.
    offset_front: usize,
    /// The offset of the last item to return from the end of the rocks Vec.
    offset_back: usize,
    /// The step size for getting the next element.
    distance: usize,
    /// The index of the next element `next` should return.
    index_front: usize,
    /// The index of the next element `next_back` should return.  
    /// TODO invert to be the index of the next element to be returned, rather than the distance from the end.
    index_back: usize,
}

impl<'a> RowIter<'a> {
    fn new_row(map: &'a RockMap, row: u32) -> RowIter<'a> {
        RowIter {
            map,
            offset_front: (row * map.width).try_into().expect("Guaranteed by the map"),
            offset_back: ((map.height - row - 1) * map.width)
                .try_into()
                .expect("Guaranteed by the map"),
            distance: 1,
            index_front: 0,
            index_back: 0,
        }
    }

    fn new_col(map: &'a RockMap, column: u32) -> RowIter<'a> {
        RowIter {
            map,
            offset_front: column.try_into().expect("Guaranteed by the map"),
            offset_back: (map.width - column - 1)
                .try_into()
                .expect("Guaranteed by the map"),
            distance: map.width.try_into().expect("Guaranteed by the map"),
            index_front: 0,
            index_back: 0,
        }
    }
}

impl<'a> Iterator for RowIter<'a> {
    type Item = &'a Option<Rock>;

    fn next(&mut self) -> Option<Self::Item> {
        self.nth(0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.map.rocks.len() - self.offset_back - self.offset_front)
            .div_ceil(self.distance)
            - self.index_front
            - self.index_back;
        (len, Some(len))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if (self.index_front + self.index_back + n) * self.distance
            >= self.map.rocks.len() - self.offset_back - self.offset_front
        {
            None
        } else {
            let result =
                Some(&self.map.rocks[self.offset_front + (self.index_front + n) * self.distance]);
            self.index_front += n + 1;
            result
        }
    }
}

impl<'a> DoubleEndedIterator for RowIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.nth_back(0)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if (self.index_front + self.index_back + n) * self.distance
            >= self.map.rocks.len() - self.offset_back - self.offset_front
        {
            None
        } else {
            let result = Some(
                &self.map.rocks[self.map.rocks.len()
                    - 1
                    - self.offset_back
                    - (self.index_back + n) * self.distance],
            );
            self.index_back += n + 1;
            result
        }
    }
}

impl<'a> ExactSizeIterator for RowIter<'a> {}

impl<'a> FusedIterator for RowIter<'a> {}

/// An iterator mutably iterating over a single row or column of a [RockMap].
///
/// Modifying a [Rock] in the iterator causes it to be moved in the [map](RockMap).  
/// Note that the map is updated after, not during iteration.
///
/// Instances of this class can be optained using [RockMap::row_iter_mut] or [RockMap::col_iter_mut].
///
/// # Panics
///
/// When a [Rock] is moved to a position where a rock is already present.  
/// And when a [Rock] is moved outside the area of the [map](RockMap).
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rust_aoc_2023::days::day14::Position;
/// use rust_aoc_2023::days::day14::Rock;
/// use rust_aoc_2023::days::day14::RockMap;
/// # use rust_aoc_2023::days::day14::RockMapError;
/// use rust_aoc_2023::days::day14::RockShape;
///
/// let mut map = RockMap::new();
/// map.add_rock((2, 4), RockShape::Round)?;
/// map.add_rock((1, 2), RockShape::Cube)?;
/// map.add_rock((5, 4), RockShape::Cube)?;
/// map.add_rock((4, 4), RockShape::Cube)?;
///
/// map.row_iter_mut(4)?.for_each(|rock| {rock.as_mut().map(|rock| rock.get_pos_mut().set_y(3));});
/// assert_eq!(map.get(Position::new(4, 4))?, &None);
/// assert_eq!(map.get(Position::new(4, 3))?, &Some(Rock::new(RockShape::Cube, 4, 3)));
/// #
/// # Result::<(), RockMapError>::Ok(())
/// ```
///
/// Moving all rocks in a column down by one with a reversed iterator:
/// ```
/// use rust_aoc_2023::days::day14::Position;
/// use rust_aoc_2023::days::day14::Rock;
/// use rust_aoc_2023::days::day14::RockMap;
/// # use rust_aoc_2023::days::day14::RockMapError;
/// use rust_aoc_2023::days::day14::RockShape;
///
/// let mut map = RockMap::new();
/// map.add_rock((0, 1), RockShape::Cube)?;
/// map.add_rock((0, 3), RockShape::Round)?;
/// map.add_rock((0, 4), RockShape::Round)?;
/// map.add_rock((1, 5), RockShape::Cube)?;
///
/// map.col_iter_mut(0)?.rev().for_each(|rock| {
///     rock.as_mut().map(|rock| rock.set_pos(Position::new(rock.get_pos().get_x(), rock.get_pos().get_y() + 1)));
/// });
/// assert_eq!(map.get(Position::new(0, 0))?, &None);
/// assert_eq!(map.get(Position::new(0, 1))?, &None);
/// assert_eq!(map.get(Position::new(0, 2))?, &Some(Rock::new(RockShape::Cube, 0, 2)));
/// assert_eq!(map.get(Position::new(0, 3))?, &None);
/// assert_eq!(map.get(Position::new(0, 4))?, &Some(Rock::new(RockShape::Round, 0, 4)));
/// assert_eq!(map.get(Position::new(0, 5))?, &Some(Rock::new(RockShape::Round, 0, 5)));
/// #
/// # Result::<(), RockMapError>::Ok(())
/// ```
///
/// Trying to do the same with a forward iterator panics:
/// ```should_panic
/// use rust_aoc_2023::days::day14::Position;
/// use rust_aoc_2023::days::day14::Rock;
/// use rust_aoc_2023::days::day14::RockMap;
/// # use rust_aoc_2023::days::day14::RockMapError;
/// use rust_aoc_2023::days::day14::RockShape;
///
/// let mut map = RockMap::new();
/// map.add_rock((0, 1), RockShape::Cube)?;
/// map.add_rock((0, 3), RockShape::Round)?;
/// map.add_rock((0, 4), RockShape::Round)?;
/// map.add_rock((1, 5), RockShape::Cube)?;
///
/// map.col_iter_mut(0)?.for_each(|rock| {
///     rock.as_mut().map(|rock| rock.set_pos(Position::new(rock.get_pos().get_x(), rock.get_pos().get_y() + 1)));
/// });
/// assert_eq!(map.get(Position::new(0, 0))?, &None);
/// assert_eq!(map.get(Position::new(0, 1))?, &None);
/// assert_eq!(map.get(Position::new(0, 2))?, &Some(Rock::new(RockShape::Cube, 0, 2)));
/// assert_eq!(map.get(Position::new(0, 3))?, &None);
/// assert_eq!(map.get(Position::new(0, 4))?, &Some(Rock::new(RockShape::Round, 0, 4)));
/// assert_eq!(map.get(Position::new(0, 5))?, &Some(Rock::new(RockShape::Round, 0, 5)));
/// #
/// # Result::<(), RockMapError>::Ok(())
/// ```
#[must_use]
#[derive(PartialEq, Eq, Debug)]
pub struct RowIterMut<'a> {
    /// The map to iterate over.
    map: &'a mut RockMap,
    /// The offset of the first item to return from the front of the rocks Vec.
    offset_front: usize,
    /// The offset of the last item to return from the end of the rocks Vec.
    offset_back: usize,
    /// The step size for getting the next element.
    distance: usize,
    /// The index of the next element `next` should return.
    index_front: usize,
    /// The index of the next element `next_back` should return.  
    /// TODO invert to be the index of the next element to be returned, rather than the distance from the end.
    index_back: usize,
}

impl<'a> RowIterMut<'a> {
    fn new_row(map: &'a mut RockMap, row: u32) -> RowIterMut<'a> {
        let map_width = map.width;
        let map_height = map.height;
        RowIterMut {
            map,
            offset_front: (row * map_width).try_into().expect("Guaranteed by the map"),
            offset_back: ((map_height - row - 1) * map_width)
                .try_into()
                .expect("Guaranteed by the map"),
            distance: 1,
            index_front: 0,
            index_back: 0,
        }
    }

    fn new_col(map: &'a mut RockMap, column: u32) -> RowIterMut<'a> {
        let map_width = map.width;
        RowIterMut {
            map,
            offset_front: column.try_into().expect("Guaranteed by the map"),
            offset_back: (map_width - column - 1)
                .try_into()
                .expect("Guaranteed by the map"),
            distance: map_width.try_into().expect("Guaranteed by the map"),
            index_front: 0,
            index_back: 0,
        }
    }

    /// Updates the rock at the given position.
    fn update_idx(&mut self, idx: usize) {
        let new_pos = if let Some(last_rock) = &self.map.rocks[idx] {
            Some(last_rock.get_pos().clone())
        } else {
            None
        };
        if let Some(new_pos) = new_pos {
            let new_idx = (new_pos.y * self.map.width + new_pos.x)
                .try_into()
                .expect("New index outside usize range");
            if idx != new_idx {
                if !self
                    .map
                    .get(new_pos)
                    .expect("New rock position out of map range")
                    .is_none()
                {
                    panic!("New rock position already occupied");
                }
                self.map.rocks.swap(idx, new_idx);
            }
        }
    }

    /// Updates the last returned rocks position, if it changed.
    ///
    /// This function tries to update the last rock returned by nth and nth_back.
    fn update_last(&mut self) {
        // Update last front index
        if self.index_front > 0 {
            let idx = self.offset_front + (self.index_front - 1) * self.distance;
            self.update_idx(idx);
        }
        // Update last back index
        if self.index_back > 0 {
            let idx =
                self.map.rocks.len() - 1 - self.offset_back - (self.index_back - 1) * self.distance;
            self.update_idx(idx);
        }
    }
}

impl<'a> Iterator for RowIterMut<'a> {
    type Item = &'a mut Option<Rock>;

    fn next(&mut self) -> Option<Self::Item> {
        self.nth(0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.map.rocks.len() - self.offset_back - self.offset_front)
            .div_ceil(self.distance)
            - self.index_front
            - self.index_back;
        (len, Some(len))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if (self.index_front + self.index_back + n) * self.distance
            >= self.map.rocks.len() - self.offset_back - self.offset_front
        {
            None
        } else {
            if self.index_front > 0 {
                self.update_last();
            }
            let rock: *mut Option<Rock> =
                &mut self.map.rocks[self.offset_front + (self.index_front + n) * self.distance];
            self.index_front += n + 1;
            // SAFETY: Nothing else can point to the same Rock.
            unsafe { Some(&mut *rock) }
        }
    }
}

impl<'a> DoubleEndedIterator for RowIterMut<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.nth_back(0)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if (self.index_front + self.index_back + n) * self.distance
            >= self.map.rocks.len() - self.offset_back - self.offset_front
        {
            None
        } else {
            if self.index_back > 0 {
                self.update_last();
            }
            let idx =
                self.map.rocks.len() - 1 - self.offset_back - (self.index_back + n) * self.distance;
            let rock: *mut Option<Rock> = &mut self.map.rocks[idx];
            self.index_back += n + 1;
            // SAFETY: Nothing else can point to the same Rock.
            unsafe { Some(&mut *rock) }
        }
    }
}

impl<'a> ExactSizeIterator for RowIterMut<'a> {}

impl<'a> FusedIterator for RowIterMut<'a> {}

impl<'a> Drop for RowIterMut<'a> {
    fn drop(&mut self) {
        if panicking() {
            return;
        } else {
            self.update_last();
        }
    }
}
