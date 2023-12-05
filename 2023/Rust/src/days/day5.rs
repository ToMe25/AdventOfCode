//! My rust AoC 2023 day 5 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 5](https://adventofcode.com/2023/day/5).

use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use std::{fmt, fs};

use super::super::get_input_file;
use super::DayRunner;

/// The day 5 runner.
///
/// The [DayRunner] implementation for the aoc day 5.
#[derive(Debug, Clone, Default)]
pub struct Day5Runner {
    seeds: Vec<u32>,
    maps: Vec<RangeMap>,
}

impl DayRunner for Day5Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input = get_input_file(5)?;
        let input_data = fs::read_to_string(input)?;
        let input_lines = &mut input_data.lines();
        self.seeds = input_lines
            .skip_while(|line| line.is_empty())
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|seed| seed.parse().unwrap())
            .collect();

        let mut map = Some(RangeMap::new());
        while map.is_some() {
            if !map.as_ref().unwrap().is_empty() {
                // self.map = self.map.merge(map.as_ref().unwrap());
                self.maps.push(map.unwrap());
            }

            map = Some(RangeMap::parse_lines(
                input_lines
                    .skip_while(|line| line.is_empty())
                    .skip(1)
                    .take_while(|line| !line.is_empty()),
            ));

            if map.as_ref().unwrap().is_empty() {
                map = None;
            }
        }

        Ok(())
    }

    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let result = self
            .seeds
            .iter()
            .map(|seed| {
                let mut seed = *seed;
                self.maps
                    .iter()
                    .for_each(|map| seed = map.get_or_value(seed as u64) as u32);
                seed
            })
            .min()
            .unwrap();
        Ok(Some(result.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let parts: (Vec<(usize, u32)>, Vec<(usize, u32)>) = self
            .seeds
            .iter()
            .enumerate()
            .map(|(i, v)| (i, *v))
            .partition(|(i, _)| i % 2 == 0);
        let ranges: Vec<(u32, u32)> = parts
            .0
            .iter()
            .map(|(_, v)| *v)
            .zip(parts.1.iter().map(|(_, v)| *v))
            .collect();

        let mut min: u64 = u64::MAX;
        for range in ranges {
            for i in range.0..=range.1 + range.0 {
                let mut val = i as u64;
                for map in &self.maps {
                    val = map.get_or_value(val);
                }
                if val < min {
                    min = val;
                }
            }
        }

        Ok(Some(min.to_string()))
    }
}

/// A map containing ranges instead of individual keys.
///
/// A map that maps values based on key ranges, rather than individual keys.  
/// A range is defined as a strart and end value, as well as an offset.  
/// A value is mapped by finding the range it is contained in, and adding the offset of that range to the value.
///
/// # Examples
///
/// Mapping a value using a map with a single range:
/// ```
/// use rust_aoc_2023::days::day5::RangeMap;
///
/// let mut map = RangeMap::new();
/// map.insert(16, 19, 6);
/// assert_eq!(map.get(17), Some(20));
/// assert_eq!(map.get(13), None);
/// assert_eq!(map.get(21), Some(24));
/// ```
#[derive(Debug, PartialEq, Default)]
pub struct RangeMap {
    first: Option<Rc<RefCell<RangeNode>>>,
}

impl RangeMap {
    /// Creates a new range map.
    ///
    /// This function creates a new empty range map.
    pub fn new() -> RangeMap {
        RangeMap { first: None }
    }

    /// Parses the strings from the iterator as a range map.
    ///
    /// This function parses the strings from the iterator as a range map.
    ///
    /// The expected format of the lines is `START_OUT START_END LENGTH`.
    ///
    /// # Panics
    ///
    /// This function panics if the iterator contains a line that doesn't match the required format.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let lines = ["15 3 19", "19 22 7"];
    /// let map = RangeMap::parse_lines(lines.iter().map(|line| *line));
    /// #
    /// # let mut ref_map = RangeMap::new();
    /// # ref_map.insert_raw(3, 21, 12);
    /// # ref_map.insert_raw(22, 28, -3);
    /// # assert_eq!(map, ref_map);
    /// ```
    pub fn parse_lines<'a, I: Iterator<Item = &'a str>>(lines: I) -> RangeMap {
        let mut map = RangeMap::new();
        lines
            .filter(|line| !line.is_empty())
            .map(|line| line.split_whitespace().map(|nr| nr.parse::<u64>().unwrap()))
            .map(|mut line| {
                (
                    line.next().unwrap(),
                    line.next().unwrap(),
                    line.next().unwrap(),
                )
            })
            .for_each(|(start_out, start_in, len)| map.insert(start_in, start_out, len));
        map
    }

    /// Inserts a raw range into the map.
    ///
    /// This function inserts a raw range in the format `input range start -> input range end`, both inclusive, into the map.  
    /// Inputs in the range will be mapped to `input + offset` with the given offset.
    ///
    /// Overlapping ranges are not allowed.
    ///
    /// See also: [insert](Self::insert).
    ///
    /// # Panics
    ///
    /// This function panics if a given range overlaps with a previously inserted range.  
    /// And also if end_in is smaller than start_in.
    ///
    /// # Examples
    ///
    /// Inserting a range that will cause values from 10 to 15(exclusive) to themselves plus 2:
    /// ```
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert_raw(10, 14, 2);
    /// # assert_eq!(map.get(11), Some(13));
    /// # assert_eq!(map.get(5), None);
    /// # assert_eq!(map.get(13), Some(15));
    /// # assert_eq!(map.get(15), None);
    /// ```
    ///
    /// Insert multiple ranges:
    /// ```
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert_raw(9, 17, 3);
    /// map.insert_raw(3, 8, 5);
    /// map.insert_raw(18, 22, 1);
    ///
    /// assert_eq!(map.get(4), Some(9));
    /// assert_eq!(map.get(2), None);
    /// assert_eq!(map.get(16), Some(19));
    /// assert_eq!(map.get(20), Some(21));
    /// ```
    pub fn insert_raw(&mut self, start_in: u64, end_in: u64, offset: i64) {
        if end_in < start_in {
            panic!(
                "Input start({}) cannot be smaller than input end({})!",
                start_in, end_in
            );
        }

        let mut new_node = RangeNode {
            start: start_in,
            end: end_in,
            offset,
            next: None,
        };

        if self.first.is_none() {
            self.first = Some(Rc::new(RefCell::new(new_node)));
        } else {
            if end_in < self.first.as_ref().unwrap().borrow().end {
                new_node.next = self.first.clone();
                self.first = Some(Rc::new(RefCell::new(new_node)));
            } else {
                let mut current_node = self.first.as_ref().unwrap().clone();
                while current_node.borrow().next.is_some()
                    && start_in > current_node.borrow().next.as_ref().unwrap().borrow().end
                {
                    current_node = current_node.clone().borrow().next.as_ref().unwrap().clone();
                }

                let next_node = current_node.borrow().next.clone();
                if start_in > current_node.borrow().end
                    && next_node.is_some()
                    && end_in > next_node.as_ref().unwrap().borrow().start
                {
                    panic!(
                        "Given range ({} -> {}) overlaps with existing range ({} -> {})!",
                        start_in,
                        end_in,
                        next_node.as_ref().unwrap().borrow().start,
                        next_node.as_ref().unwrap().borrow().end
                    );
                }

                new_node.next = current_node.borrow().next.clone();
                let _ = current_node.borrow_mut().next = Some(Rc::new(RefCell::new(new_node)));
            }
        }
    }

    /// Insert a range from the input.
    ///
    /// Inserts a range in the format `start_in -> start_in + length`, start inclusive end exclusive, into the map.  
    /// Numbers in this range will be mapped to `input + start_out - start_in`.
    ///
    /// Overlapping ranges are not allowed.
    ///
    /// See also: [insert](Self::insert_raw).
    ///
    /// # Panics
    ///
    /// This function panics if a given range overlaps with a previously inserted range.
    ///
    /// # Examples
    ///
    /// Inserting a range that will cause values from 10 to 15(exclusive) to themselves plus 2:
    /// ```
    /// # use std::cell::RefCell;
    /// # use std::rc::Rc;
    /// #
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert(10, 12, 5);
    /// # let mut ref_map = RangeMap::new();
    /// # ref_map.insert_raw(10, 14, 2);
    /// # assert_eq!(map, ref_map);
    /// ```
    pub fn insert(&mut self, start_in: u64, start_out: u64, length: u64) {
        self.insert_raw(
            start_in,
            start_in + length - 1,
            start_out as i64 - start_in as i64,
        );
    }

    /// Get the value for the given input, if any.
    ///
    /// Gets the value for the given input, of the input matches a range.  
    /// Returns `None` otherwise.
    ///
    /// # Examples
    ///
    /// Getting a value from a single range map:
    /// ```
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert(16, 21, 6);
    /// assert_eq!(map.get(16), Some(21));
    /// assert_eq!(map.get(13), None);
    /// ```
    pub fn get(&self, input: u64) -> Option<u64> {
        if self.first.is_none() {
            return None;
        }

        let mut current_node = self.first.as_ref().unwrap().clone();
        while current_node.borrow().next.is_some() && current_node.borrow().end < input {
            current_node = current_node.clone().borrow().next.as_ref().unwrap().clone();
        }

        if current_node.borrow().start <= input && current_node.borrow().end >= input {
            Some((input as i64 + current_node.borrow().offset) as u64)
        } else {
            None
        }
    }

    /// Get the value for the key, or the key if it doesn't exist.
    ///
    /// Get the value for the given input, or the given input if the value doesn't match any range.  
    /// This is a convinience method matching the behaviour of `get(input).unwrap_or(input)`.
    ///
    /// See also: [get](Self::get)
    pub fn get_or_value(&self, input: u64) -> u64 {
        self.get(input).unwrap_or(input)
    }

    /// Checks if this rangemap is empty.
    ///
    /// Checks whether this rangemap is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_aoc_2023::days::day5::RangeMap;
    /// #
    /// let mut map = RangeMap::new();
    /// assert!(map.is_empty());
    /// map.insert(3, 6, 1);
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    pub fn merge(&self, other: &RangeMap) -> RangeMap {
        if self.first.is_none() {
            return other.clone();
        } else if other.first.is_none() {
            return self.clone();
        }

        let mut result = RangeMap::new();

        let mut scn = self.first.as_ref().unwrap().clone();
        let mut ocn = other.first.as_ref().unwrap().clone();
        let mut start = scn.borrow().start.min(ocn.borrow().start);
        while scn.borrow().next.is_some()
            || ocn.borrow().next.is_some()
            || scn.borrow().end >= start
            || ocn.borrow().end >= start
        {
            let mut end = scn.borrow().end.max(ocn.borrow().end);
            if start < scn.borrow().start && scn.borrow().start <= end {
                end = scn.borrow().start - 1;
            } else if start >= scn.borrow().start
                && start <= scn.borrow().end
                && end > scn.borrow().end
            {
                end = scn.borrow().end;
            }

            if start < ocn.borrow().start && ocn.borrow().start <= end {
                end = ocn.borrow().start - 1;
            } else if start >= ocn.borrow().start
                && start <= ocn.borrow().end
                && end > ocn.borrow().end
            {
                end = ocn.borrow().end;
            }

            let mut offset: i64 = 0;
            if (start >= scn.borrow().start && start <= scn.borrow().end)
                || (end >= scn.borrow().start && end <= scn.borrow().end)
            {
                offset += scn.borrow().offset;
            }
            if (start >= ocn.borrow().start && start <= ocn.borrow().end)
                || (end >= ocn.borrow().start && end <= ocn.borrow().end)
            {
                offset += ocn.borrow().offset;
            }
            result.insert_raw(start, end, offset);
            start = end + 1;

            if end == scn.borrow().end && scn.borrow().next.is_some() {
                scn = scn.clone().borrow().next.as_ref().unwrap().clone();
            }
            if end == ocn.borrow().end && ocn.borrow().next.is_some() {
                ocn = ocn.clone().borrow().next.as_ref().unwrap().clone();
            }
        }

        result
    }
}

impl Clone for RangeMap {
    fn clone(&self) -> Self {
        RangeMap {
            first: self
                .first
                .as_ref()
                .map(|rc| Rc::new(RefCell::new(rc.borrow().clone()))),
        }
    }
}

impl Display for RangeMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RangeMap [")?;
        if self.first.is_some() {
            let mut node = self.first.as_ref().unwrap().clone();
            write!(f, "{}", node.borrow())?;
            while node.borrow().next.is_some() {
                node = node.clone().borrow().next.as_ref().unwrap().clone();
                write!(f, ", {}", node.borrow())?;
            }
        }
        write!(f, "]")
    }
}

/// A single range representation for [RangeMap].
///
/// A node of a [RangeMap], representing a single range.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RangeNode {
    pub start: u64,
    pub end: u64,
    pub offset: i64,
    pub next: Option<Rc<RefCell<RangeNode>>>,
}

impl Clone for RangeNode {
    fn clone(&self) -> Self {
        RangeNode {
            start: self.start,
            end: self.end,
            offset: self.offset,
            next: self
                .next
                .as_ref()
                .map(|rc| Rc::new(RefCell::new(rc.borrow().clone()))),
        }
    }
}

impl Display for RangeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node {{Range: {} -> {}, Offset: {}}}",
            self.start, self.end, self.offset
        )
    }
}