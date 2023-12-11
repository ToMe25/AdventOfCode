//! My rust AoC 2023 day 5 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 5](https://adventofcode.com/2023/day/5).

use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;
use std::ops::{Bound, RangeBounds};
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
    map: RangeMap,
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
                self.map = self.map.merge(map.as_ref().unwrap());
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
            .map(|seed| self.map.get_or_key(*seed as u64))
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

        let min = ranges
            .iter()
            .map(|range| {
                self.map
                    .get_min_in_range(range.0 as u64..(range.1 + range.0) as u64)
            })
            .min()
            .unwrap();

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

    /// Get the value for the given key, if any.
    ///
    /// Gets the value for the given key, if the input matches a range.  
    /// Returns `None` otherwise.
    ///
    /// See also: [get_or_key](Self::get_or_key), [get_min_in_range](Self::get_min_in_range)
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
    pub fn get(&self, key: u64) -> Option<u64> {
        if self.first.is_none() {
            return None;
        }

        let current_node = self.get_range_for(key);
        if current_node.is_none() {
            return None;
        }

        let current_node_ref = current_node.as_ref().unwrap().borrow();
        if current_node_ref.start <= key && current_node_ref.end >= key {
            Some((key as i64 + current_node_ref.offset) as u64)
        } else {
            None
        }
    }

    /// Get the value for the key, or the key if it doesn't exist.
    ///
    /// Get the value for the given key, or the given key if the key doesn't match any range.  
    /// This is a convinience method matching the behaviour of `get(key).unwrap_or(key)`.
    ///
    /// See also: [get](Self::get), [get_min_in_range](Self::get_min_in_range)
    ///
    /// # Examples
    ///
    /// Getting a value that is mapped:
    /// ```
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert_raw(14, 19, -5);
    /// assert_eq!(map.get_or_key(15), 10);
    /// ```
    ///
    /// Getting an unmapped key:
    /// ```
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert_raw(3, 99, 51);
    /// assert_eq!(map.get_or_key(2), 2);
    /// ```
    pub fn get_or_key(&self, key: u64) -> u64 {
        self.get(key).unwrap_or(key)
    }

    /// Gets the smallest value for the given key range.
    ///
    /// Gets the smallest value for the given key range.  
    /// Should behave identical to `range.map(|key| map.get_or_key(key)).min()`.
    ///
    /// This function can not handle ranges without an upper bound!  
    /// Ranges without a lower bound are interpreted to be ranges that start at 0.
    ///
    /// See also: [get](Self::get), [get_or_key](Self::get_or_key)
    ///
    /// # Panics
    ///
    /// This function panics when it is given a range without an upper bound.
    ///
    /// # Examples
    ///
    /// Getting the min value from a range on an empty map:
    /// ```
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let map = RangeMap::new();
    /// assert_eq!(map.get_min_in_range(10..15), 10);
    /// ```
    ///
    /// Using a range where the min valud is between two ranges:
    /// ```
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert_raw(21, 35, 41);
    /// map.insert_raw(44, 46, -7);
    /// assert_eq!(map.get_min_in_range(26..=48), 36);
    /// ```
    ///
    /// Using a range without a lower bound:
    /// ```
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let mut map = RangeMap::new();
    /// map.insert_raw(0, 10, 7);
    /// assert_eq!(map.get_min_in_range(..15), 7);
    /// ```
    ///
    /// Using a range without an upper bound:
    /// ```should_panic
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let map = RangeMap::new();
    /// map.get_min_in_range(5..);
    /// ```
    pub fn get_min_in_range(&self, range: impl RangeBounds<u64>) -> u64 {
        let start = match range.start_bound() {
            Bound::Included(x) => *x,
            Bound::Excluded(x) => x + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(x) => *x,
            Bound::Excluded(x) => x - 1,
            Bound::Unbounded => panic!("range has to have an end bound!"),
        };

        let mut current = self.get_range_for(start);
        let mut min =
            (start as i64 + current.as_ref().map(|rc| rc.borrow().offset).unwrap_or(0)) as u64;
        if current.is_none() {
            current = self.get_range_after(start);
        }
        while current.is_some() && current.as_ref().unwrap().borrow().start <= end {
            if min
                > (current.as_ref().unwrap().borrow().start as i64
                    + current.as_ref().unwrap().borrow().offset) as u64
            {
                min = (current.as_ref().unwrap().borrow().start as i64
                    + current.as_ref().unwrap().borrow().offset) as u64;
            }

            let next = current.as_ref().unwrap().borrow().end + 1;
            current = self.get_range_for(next);
            if current.is_none() {
                if next <= end && next < min {
                    min = next;
                }
                current = self.get_range_after(next);
            }
        }

        min
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

    /// Merges this map and the given map into a new map.
    ///
    /// Creates new map such that for any given key `B.get_or_key(A.get_or_key(KEY)) == A.merge(&B).get_or_key(KEY)`.  
    /// If one of the maps is empty, the resulting map will be equal to the non-empty map.  
    ///
    /// If neither of the input maps are empty, the output map will not contain any ranges with an offset of 0.  
    /// If either of the input maps is empty, the output map will contain ranges with an offset of 0, if the non-empty input map did.
    ///
    /// # Examples
    ///
    /// Merging two not overlapping maps:
    /// ```
    /// # use rust_aoc_2023::days::day5::RangeMap;
    /// #
    /// let mut map_a = RangeMap::new();
    /// map_a.insert_raw(17, 21, -5);
    /// let mut map_b = RangeMap::new();
    /// map_b.insert_raw(23, 55, 17);
    /// let mut map_c = RangeMap::new();
    /// map_c.insert_raw(17, 21, -5);
    /// map_c.insert_raw(23, 55, 17);
    /// assert_eq!(map_a.merge(&map_b), map_c);
    /// ```
    ///
    /// Merging of two overlapping maps:
    /// ```
    /// # use rust_aoc_2023::days::day5::RangeMap;
    /// #
    /// let mut map_a = RangeMap::new();
    /// map_a.insert_raw(51, 81, 16);
    /// map_a.insert_raw(86, 99, -10);
    /// map_a.insert_raw(101, 101, 10);
    /// let mut map_b = RangeMap::new();
    /// map_b.insert_raw(35, 47, -19);
    /// map_b.insert_raw(48, 62, 12);
    /// map_b.insert_raw(75, 102, -5);
    ///
    /// let mut map_c = RangeMap::new();
    /// map_c.insert_raw(35, 47, -19);
    /// map_c.insert_raw(48, 50, 12);
    /// map_c.insert_raw(51, 58, 16);
    /// map_c.insert_raw(59, 81, 11);
    /// map_c.insert_raw(82, 85, -5);
    /// map_c.insert_raw(86, 99, -15);
    /// map_c.insert_raw(100, 100, -5);
    /// map_c.insert_raw(101, 101, 10);
    /// map_c.insert_raw(102, 102, -5);
    /// assert_eq!(map_a.merge(&map_b), map_c);
    /// ```
    ///
    /// Another example of merging two maps:
    /// ```
    /// use rust_aoc_2023::days::day5::RangeMap;
    ///
    /// let mut map_a = RangeMap::new();
    /// map_a.insert_raw(0, 16, 7);
    /// map_a.insert_raw(20, 24, -5);
    /// map_a.insert_raw(30, 60, 10);
    /// let mut map_b = RangeMap::new();
    /// map_b.insert_raw(2, 6, 5);
    /// map_b.insert_raw(9, 20, -5);
    /// map_b.insert_raw(25, 50, 15);
    ///
    /// let mut map_c = RangeMap::new();
    /// map_c.insert_raw(0, 1, 7);
    /// map_c.insert_raw(2, 13, 2);
    /// map_c.insert_raw(14, 16, 7);
    /// map_c.insert_raw(17, 19, -5);
    /// map_c.insert_raw(20, 24, -10);
    /// map_c.insert_raw(25, 29, 15);
    /// map_c.insert_raw(30, 40, 25);
    /// map_c.insert_raw(41, 60, 10);
    /// assert_eq!(map_a.merge(&map_b), map_c);
    /// ```
    pub fn merge(&self, other: &RangeMap) -> RangeMap {
        if self.is_empty() {
            return other.clone();
        } else if other.is_empty() {
            return self.clone();
        }

        let mut result = RangeMap::new();
        let mut start: u64 = 0;
        let mut scn = self.first.as_ref().unwrap().clone();
        let mut ocn = other.get_range_for(self.get_or_key(start));
        while scn.borrow().next.is_some()
            || scn.borrow().end >= start
            || (ocn.is_some() && ocn.as_ref().unwrap().borrow().end >= start)
        {
            let mut o_start = start;
            let mut offset: i64 = 0;
            if start >= scn.borrow().start && start <= scn.borrow().end {
                offset = scn.borrow().offset;
                o_start = (start as i64 + scn.borrow().offset) as u64;
            }

            ocn = other.get_range_for(o_start);
            if ocn.is_none() {
                ocn = other.get_range_after(o_start);
            } else {
                offset += ocn.as_ref().unwrap().borrow().offset;
            }

            let mut end = scn.borrow().end;
            if start < scn.borrow().start {
                end = scn.borrow().start - 1;
            } else if end < start && ocn.is_some() {
                end = ocn.as_ref().unwrap().borrow().end;
            }
            let mut o_len = u64::MAX;
            if ocn.is_some() && ocn.as_ref().unwrap().borrow().start <= o_start {
                o_len = ocn.as_ref().unwrap().borrow().end - o_start;
            } else if ocn.is_some() {
                o_len = ocn.as_ref().unwrap().borrow().start - o_start - 1;
            }

            if ocn.is_some() && o_len < end - start {
                end = start + o_len;
            }

            if offset != 0 {
                result.insert_raw(start, end, offset);
            }
            start = end + 1;

            if end == scn.borrow().end && scn.borrow().next.is_some() {
                scn = scn.clone().borrow().next.as_ref().unwrap().clone();
            }

            ocn = other.get_range_for(start);
            if ocn.is_none() {
                ocn = other.get_range_after(start);
            }
        }

        result
    }

    /// Gets the range for the given key.
    ///
    /// Gets the range that would be used to map the given key.  
    /// `None` if there is no range matching the given key.
    fn get_range_for(&self, key: u64) -> Option<Rc<RefCell<RangeNode>>> {
        if self.first.is_none() {
            return None;
        }

        let mut current_node = self.first.as_ref().unwrap().clone();
        while current_node.borrow().next.is_some() && current_node.borrow().end < key {
            current_node = current_node.clone().borrow().next.as_ref().unwrap().clone();
        }

        if current_node.borrow().start <= key && current_node.borrow().end >= key {
            Some(current_node)
        } else {
            None
        }
    }

    /// Gets the first range starting after the given key.
    ///
    /// Gets the first range starting after the given key.  
    /// Returns `None` if there isn't any.
    fn get_range_after(&self, key: u64) -> Option<Rc<RefCell<RangeNode>>> {
        if self.first.is_none() {
            return None;
        }

        let mut current_node = self.first.as_ref().map(|rc| rc.clone());
        while current_node.is_some() && current_node.as_ref().unwrap().borrow().start <= key {
            current_node = current_node
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .map(|rc| rc.clone());
        }
        current_node.map(|rc| rc.clone())
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
