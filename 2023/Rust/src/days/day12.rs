//! My rust AoC 2023 day 12 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 12](https://adventofcode.com/2023/day/12).

use std::borrow::Borrow;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::ops::{Bound, RangeBounds};

use super::super::get_input_file;
use super::DayRunner;

/// The day 12 runner.
///
/// The [DayRunner] implementation for the aoc day 12.
#[derive(Debug, Clone, Default)]
pub struct Day12Runner {}

impl DayRunner for Day12Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(12)?)?;

        let sum: usize = input_data
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split_once(' ').unwrap())
            .map(|(map_line, lengths)| {
                (
                    map_line,
                    lengths
                        .split(',')
                        .map(|nr| nr.parse::<u8>().unwrap())
                        .collect::<Vec<u8>>(),
                )
            })
            .map(|(map_line, lengths)| count_possibilities(map_line, &lengths))
            .sum();

        Ok(Some(sum.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        // FIXME find a solution that finishes in a provably finite amount of time
        let input_data = fs::read_to_string(get_input_file(12)?)?;

        let sum: usize = input_data
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split_once(' ').unwrap())
            .map(|(map_line, lengths)| {
                (
                    (0..5).map(|_| map_line).collect::<Vec<&str>>().join("?"),
                    (0..5).map(|_| lengths).collect::<Vec<&str>>().join(","),
                )
            })
            .map(|(map_line, lengths)| {
                (
                    map_line,
                    lengths
                        .split(',')
                        .map(|nr| nr.parse::<u8>().unwrap())
                        .collect::<Vec<u8>>(),
                )
            })
            .map(|(map_line, lengths)| count_possibilities(&map_line, &lengths))
            .sum();

        Ok(Some(sum.to_string()))
    }
}

/// Counts the number of possible solutions to the given input data.
///
/// Counts the number of possible arrangements of the given broken hot springs in the given data.
///
/// # Panics
///
/// This function panics if `map_line` includes a character that isn't `.`, `?`, or `#`.
/// This function also panics if there is no valid solution for the given data.
///
/// # Examples
/// ```
/// use rust_aoc_2023::days::day12;
///
/// assert_eq!(day12::count_possibilities("#.???.###.#??.#????", &vec!(1, 1, 1, 3, 2, 1, 1)), 3);
/// ```
///
/// ```
/// use rust_aoc_2023::days::day12;
///
/// assert_eq!(day12::count_possibilities("?###????????", &vec!(3, 2, 1)), 10);
/// ```
///
/// ```
/// use rust_aoc_2023::days::day12;
///
/// assert_eq!(day12::count_possibilities("????.######..#####.", &vec!(1, 6, 5)), 4);
/// ```
///
/// ```
/// use rust_aoc_2023::days::day12;
///
/// assert_eq!(day12::count_possibilities("???.###????.###????.###????.###????.###",
///                                       &vec!(1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3)), 1);
/// ```
///
/// ```
/// use rust_aoc_2023::days::day12;
///
/// assert_eq!(day12::count_possibilities(".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.",
///                                       &vec!(1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3)), 16384);
/// ```
pub fn count_possibilities<'a, Str, Iter>(map_line: Str, lengths: &'a Iter) -> usize
where
    Str: AsRef<str>,
    &'a Iter: IntoIterator,
    <&'a Iter as IntoIterator>::Item: Borrow<u8>,
    <&'a Iter as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    let states: Vec<SpringState> = map_line
        .as_ref()
        .chars()
        .map(|c| match c {
            '.' => SpringState::Operational,
            '?' => SpringState::Unknown,
            '#' => SpringState::Damaged,
            x => panic!("Received unknown spring state '{}'!", x),
        })
        .collect();

    let mut last_start = states.len() + 1;
    let mut positions: Vec<Position> = lengths
        .into_iter()
        .rev()
        .map(|len| {
            let pos =
                find_last_position_before(&states, len.borrow().to_owned().into(), last_start - 1);
            if let Some(p) = pos {
                last_start = p.start as usize;
            }
            pos.unwrap_or_else(|| {
                panic!(
                    "failed to find location for area with length {}",
                    len.borrow()
                )
            })
        })
        .collect();

    positions.reverse();
    let pos_len = positions.len();

    if pos_len != lengths.into_iter().count() {
        panic!(
            "A solution for line \"{} {}\" could not be found!",
            map_line.as_ref(),
            lengths
                .into_iter()
                .map(|len| len.borrow().to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }

    let mut count = 0;
    let mut modified = true;
    while modified {
        {
            let mut valid = true;
            let mut cur_pos = 0;
            for (i, state) in states.iter().enumerate() {
                if state == &SpringState::Damaged {
                    while cur_pos < pos_len && !positions[cur_pos].contains(i) {
                        cur_pos += 1;
                        if cur_pos >= pos_len
                            || positions[cur_pos].start <= positions[cur_pos - 1].end + 1
                            || positions[cur_pos].start > (i as u32)
                        {
                            valid = false;
                            break;
                        }
                    }
                }

                if !valid {
                    break;
                }
            }

            if valid {
                count += 1;
            }
        }

        modified = false;
        for i in 0..pos_len {
            let pos = find_last_position_before(
                &states,
                (positions[i].end - positions[i].start + 1) as usize,
                positions[i].end as usize,
            );

            if let Some(p) = pos {
                let collides = (i > 0 && positions[i - 1].end + 1 >= p.start)
                    || (i < pos_len - 1 && p.end + 1 >= positions[i + 1].start);

                if !collides {
                    positions[i] = p;
                    modified = true;
                    for j in (0..i).rev() {
                        let new_pos = find_last_position_before(
                            &states,
                            (positions[j].end - positions[j].start + 1) as usize,
                            (positions[j + 1].start - 1) as usize,
                        );
                        if let Some(np) = new_pos {
                            // Even if one position didn't change, the ones before it might.
                            positions[j] = np;
                        }
                    }
                    break;
                }
            }
        }
    }
    count
}

/// Finds the last possible position for a damaged area.
///
/// The size of the damaged area to find, as well as the point before which it should be can be specified.  
/// The given end point **cannot** be part of the returned position.  
/// If the given end point is a damaged hot spring, the returned position **cannot** end directly before it.
///
/// # Examples
/// Finding a position in the full input:
/// ```
/// use rust_aoc_2023::days::day12;
/// use rust_aoc_2023::days::day12::Position;
/// use rust_aoc_2023::days::day12::SpringState;
///
/// let states = vec!(SpringState::Operational, SpringState::Unknown, SpringState::Damaged,
///                   SpringState::Unknown, SpringState::Damaged, SpringState::Unknown, SpringState::Unknown);
/// // Use a limit of one more than the last index to include the last index.
/// let position = day12::find_last_position_before(&states, 3, states.len());
/// assert_eq!(position.unwrap(), Position::build(4u32, 6u32).unwrap());
/// ```
///
/// Finding a position in a limited area:
/// ```
/// use std::iter;
/// use rust_aoc_2023::days::day12;
/// use rust_aoc_2023::days::day12::Position;
/// use rust_aoc_2023::days::day12::SpringState;
///
/// let position = day12::find_last_position_before(iter::repeat(&SpringState::Unknown).take(16).collect::<Vec<&SpringState>>(), 4usize, 12usize);
/// assert_eq!(position.unwrap(), Position::build(8u8, 11u8).unwrap());
/// ```
///
/// Finding a position that isn't allowed to end at the end of the limited area:
/// ```
/// use rust_aoc_2023::days::day12;
/// use rust_aoc_2023::days::day12::Position;
/// use rust_aoc_2023::days::day12::SpringState;
///
/// let states = vec!(SpringState::Unknown, SpringState::Unknown, SpringState::Unknown, SpringState::Unknown,
///                   SpringState::Operational, SpringState::Unknown, SpringState::Unknown, SpringState::Damaged,
///                   SpringState::Damaged, SpringState::Unknown, SpringState::Operational);
/// let position = day12::find_last_position_before(&states, 3usize, 8usize);
/// assert_eq!(position.unwrap(), Position::build(1u8, 3u8).unwrap());
/// ```
///
/// Failing to find a valid position:
/// ```
/// use rust_aoc_2023::days::day12;
/// use rust_aoc_2023::days::day12::Position;
/// use rust_aoc_2023::days::day12::SpringState;
///
/// let states = vec!(SpringState::Unknown, SpringState::Operational, SpringState::Unknown, SpringState::Unknown,
///                   SpringState::Operational, SpringState::Damaged, SpringState::Unknown, SpringState::Damaged);
/// let position = day12::find_last_position_before(&states, 3usize, 4usize);
/// assert_eq!(position, None);
/// ```
///
/// Finding a position at the very beginning:
/// ```
/// use rust_aoc_2023::days::day12;
/// use rust_aoc_2023::days::day12::Position;
/// use rust_aoc_2023::days::day12::SpringState;
///
/// let states = vec!(SpringState::Unknown, SpringState::Damaged, SpringState::Operational, SpringState::Unknown);
/// let position = day12::find_last_position_before(&states, 2, states.len());
/// assert_eq!(position.unwrap(), Position::build(0u8, 1u8).unwrap());
/// ```
pub fn find_last_position_before<'a, I>(states: I, len: usize, before: usize) -> Option<Position>
where
    I: IntoIterator<Item = &'a SpringState>,
    I::IntoIter: DoubleEndedIterator + ExactSizeIterator,
{
    let mut start_min: Option<usize> = None;
    let mut start_max: Option<usize> = None;
    let mut states_since_start: Vec<&SpringState> = Vec::new();
    let mut states_it = states.into_iter().enumerate().rev();
    let silen = states_it.len();
    states_since_start.reserve(silen / 8);
    if states_it.len() > before {
        if states_it
            .nth(silen - before - 1)
            .is_some_and(|(_, state)| state == &SpringState::Damaged)
        {
            start_min = Some(usize::MAX);
            start_max = Some(usize::MAX);
        }
    }

    for (i, state) in states_it {
        if state != &SpringState::Operational && start_min.is_none() {
            start_min = Some(i);
        }
        if state == &SpringState::Damaged && start_max.is_none() {
            start_max = Some(i);
        }

        if state != &SpringState::Damaged && start_min.is_some_and(|start| start >= len + i) {
            while start_max.is_some_and(|start| start > len + i) {
                let mut damaged_start = false;
                let mut damaged_end = false;
                let start_min_old = start_min.unwrap();
                for (j, s) in states_since_start.iter().enumerate() {
                    if !damaged_start && s == &&SpringState::Damaged {
                        damaged_start = true;
                    }

                    if damaged_start && s != &&SpringState::Damaged {
                        damaged_end = true;
                    }

                    if damaged_end {
                        start_min = Some(start_min_old - j);
                        if s == &&SpringState::Damaged {
                            start_max = Some(start_min_old - j);
                        } else {
                            start_max = None;
                        }
                        break;
                    }
                }

                if !damaged_end {
                    start_min = None;
                    start_max = None;
                    states_since_start.clear();
                }
            }

            if start_min.is_some_and(|start| start >= len + i) {
                return Some(Position::build((i + 1) as u32, (i + len) as u32).unwrap());
            }
        }

        if state == &SpringState::Operational {
            start_min = None;
            start_max = None;
            states_since_start.clear();
        }

        if start_min.is_some() {
            states_since_start.push(state);
        }
    }

    // Check for a match in the first position
    if start_min.is_some_and(|start| start >= len - 1)
        && !start_max.is_some_and(|start| start > len - 1)
    {
        return Some(Position::build(0u32, (len - 1) as u32).unwrap());
    }

    None
}

/// A struct containing the position of a single block of damaged hot springs.
///
/// Can be used to check for overlap with another block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    start: u32,
    end: u32,
}

impl Position {
    /// Creates a new position object with the given positions.
    ///
    /// The position range is start inclusive and end enclusive.
    pub fn build(
        start: impl Into<u32>,
        end: impl Into<u32>,
    ) -> Result<Position, PositionEndBeforeStartError> {
        let e = end.into();
        let s = start.into();
        if e < s {
            return Err(PositionEndBeforeStartError {});
        }
        Ok(Position { start: s, end: e })
    }

    /// Checks whether this block contains the given position.
    ///
    /// # Panics
    /// If the other value cannot be converted into a u32, and start or end cannot be converted to the type of other.
    ///
    /// # Examples
    /// ```
    /// use rust_aoc_2023::days::day12::Position;
    /// # use rust_aoc_2023::days::day12::PositionEndBeforeStartError;
    ///
    /// assert_eq!(Position::build(5u8, 7u8)?.contains(4u32), false);
    /// # Ok::<(), PositionEndBeforeStartError>(())
    /// ```
    ///
    /// ```
    /// use rust_aoc_2023::days::day12::Position;
    /// # use rust_aoc_2023::days::day12::PositionEndBeforeStartError;
    ///
    /// assert_eq!(Position::build(3u16, 180u16)?.contains(3u8), true);
    /// # Ok::<(), PositionEndBeforeStartError>(())
    /// ```
    ///
    /// ```
    /// use rust_aoc_2023::days::day12::Position;
    /// # use rust_aoc_2023::days::day12::PositionEndBeforeStartError;
    ///
    /// assert_eq!(Position::build(125u32, 166u32)?.contains(130u16), true);
    /// # Ok::<(), PositionEndBeforeStartError>(())
    /// ```
    ///
    /// ```
    /// use rust_aoc_2023::days::day12::Position;
    /// # use rust_aoc_2023::days::day12::PositionEndBeforeStartError;
    ///
    /// assert_eq!(Position::build(15u32, 30u32)?.contains(30), true);
    /// # Ok::<(), PositionEndBeforeStartError>(())
    /// ```
    pub fn contains<T>(&self, pos: T) -> bool
    where
        T: TryFrom<u32> + TryInto<u32> + Ord + Copy,
    {
        let p = pos.try_into();
        match p {
            Ok(pos) => pos >= self.start && pos <= self.end,
            Err(_) => {
                pos >= self
                    .start
                    .try_into()
                    .unwrap_or_else(|_| panic!("cannot convert self.start to other type"))
                    && pos
                        <= self
                            .end
                            .try_into()
                            .unwrap_or_else(|_| panic!("cannot convert self.end to other type"))
            }
        }
    }

    /// Checks wether the given range overlaps with this range.
    ///
    /// This method will fail if and only if converting other to a position fails.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// # use std::error::Error;
    /// use rust_aoc_2023::days::day12::Position;
    /// # use rust_aoc_2023::days::day12::PositionEndBeforeStartError;
    ///
    /// assert_eq!(Position::build(15u8, 19u8)?.overlaps(&Position::build(7u8, 16u8)?)?, true);
    /// assert_eq!(Position::build(3u8, 5u8)?.overlaps(&Position::build(18u8, 18u8)?)?, false);
    /// # Ok::<(), Box<dyn Error>>(())
    /// ```
    ///
    /// Using with ranges:
    /// ```
    /// use rust_aoc_2023::days::day12::Position;
    /// use rust_aoc_2023::days::day12::RangeBoundsWrapper;
    /// # use rust_aoc_2023::days::day12::PositionEndBeforeStartError;
    ///
    /// // The RangeBoundsWrapper is required because of https://github.com/rust-lang/rust/issues/50133
    /// assert_eq!(Position::try_from(RangeBoundsWrapper(5..19))?.overlaps(&RangeBoundsWrapper(16..25))?, true);
    /// Ok::<(), PositionEndBeforeStartError>(())
    /// ```
    pub fn overlaps<T: TryInto<Position> + Clone>(&self, other: &T) -> Result<bool, T::Error> {
        let o: Self = other.clone().try_into()?;
        Ok(self.contains(o.start)
            || self.contains(o.end)
            || o.contains(self.start)
            || o.contains(self.end))
    }
}

impl<T: RangeBounds<u32>> TryFrom<RangeBoundsWrapper<T>> for Position {
    type Error = PositionEndBeforeStartError;

    fn try_from(value: RangeBoundsWrapper<T>) -> Result<Self, Self::Error> {
        let start = match value.0.start_bound() {
            Bound::Included(s) => s.to_owned(),
            Bound::Excluded(s) => s.to_owned() + 1,
            Bound::Unbounded => 0,
        };
        let end = match value.0.end_bound() {
            Bound::Included(e) => e.to_owned(),
            Bound::Excluded(e) => e.to_owned() - 1,
            Bound::Unbounded => u32::MAX,
        };
        Position::build(start, end)
    }
}

/// The possible states each state can have.
///
/// An enum representing the state of a single hot spring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpringState {
    Damaged,
    Unknown,
    Operational,
}

/// An error meaning that the end of a [`Position`] was before its start.
///
/// [`Position`]: self::Position
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct PositionEndBeforeStartError;

impl Display for PositionEndBeforeStartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "position end was before start")
    }
}

impl Error for PositionEndBeforeStartError {}

/// A [`RangeBounds`] wrapper to work around [`rust#50133`].
///
/// [`RangeBounds`]: RangeBounds
/// [`rust#50133`]: https://github.com/rust-lang/rust/issues/50133
#[derive(Debug, Clone, Copy)]
pub struct RangeBoundsWrapper<T: RangeBounds<u32>>(pub T);

impl<T: RangeBounds<u32>> From<T> for RangeBoundsWrapper<T> {
    fn from(value: T) -> Self {
        RangeBoundsWrapper(value)
    }
}
