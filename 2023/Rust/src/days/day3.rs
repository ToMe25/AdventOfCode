//! My rust AoC 2023 day 3 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 3](https://adventofcode.com/2023/day/3).

use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::rc::Rc;

use super::super::get_input_file;
use super::DayRunner;

/// The day 3 runner.
///
/// The [DayRunner] implementation for the aoc day 3.
#[derive(Debug, Clone, Default)]
pub struct Day3Runner {
	/// The parsed input data.
	///
	/// The outer vector contains one vector per line of input data.  
	/// Each inner vector contains one Rc per character.
	///
	/// For numbers that are more than one character, they have one Rc per character pointing to a common [Value::Part] instance.
	///
	/// Numbers that aren't part numbers are considered [Value::None].  
	/// Gears that don't have exactly two adjacent parts are considered [symbols](Value::Symbol).
    values: Vec<Vec<Rc<RefCell<Value>>>>,
}

impl DayRunner for Day3Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input = get_input_file(3)?;
        // Read file and convert characters to values.
        self.values = fs::read_to_string(input)?
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        if c.is_digit(10) {
                            Value::Part {
                                value: c.to_digit(10).unwrap() as u16,
                                line: i,
                                from: j,
                                to: j,
                            }
                        } else if c == '*' {
                            Value::Gear {
                                line: i,
                                index: j,
                                numbers: Vec::new(),
                            }
                        } else if c == '.' {
                            Value::None
                        } else {
                            Value::Symbol
                        }
                    })
                    .map(RefCell::new)
                    .map(Rc::new)
                    .collect()
            })
            .collect();

        // Merge Number values.
        for line in 0..self.values.len() {
            for idx in 0..(self.values[line].len() - 1) {
                let mut value: u16;
                let mut to: usize;
                if let Value::Part {
                    value: val, to: t, ..
                } = *self.values[line][idx].borrow()
                {
                    value = val;
                    to = t;
                } else {
                    continue;
                }

                if to < self.values[line].len() - 1 {
                    let mut merge = false;
                    if let Value::Part {
                        value: v, to: t, ..
                    } = *self.values[line][idx + 1].borrow_mut()
                    {
                        value = value * 10 + v;
                        to = t;
                        merge = true;
                    }
                    if merge {
                        self.values[line][idx + 1] = self.values[line][idx].clone();
                    }
                }

                if let Value::Part {
                    value: val, to: t, ..
                } = &mut *self.values[line][idx].borrow_mut()
                {
                    *val = value;
                    *t = to;
                }
            }
        }

        // Remove numbers that aren't parts.
        self.values = self
            .values
            .iter()
            .map(|line| {
                line.iter()
                    .map(|val| {
                        if let Value::Part {
                            value: _,
                            line: ln,
                            from,
                            to,
                        } = *val.borrow()
                        {
                            let part = ((from.max(1) - 1)..=(to + 1))
                                .map(|i| {
                                    ((ln.max(1) - 1)..=(ln.min(self.values.len() - 2) + 1))
                                        .map(|l| {
                                            self.values[l][i.min(self.values[l].len() - 1)]
                                                .to_owned()
                                        })
                                        .map(|v| match *v.borrow() {
                                            Value::Gear { .. } | Value::Symbol => true,
                                            _ => false,
                                        })
                                        .reduce(|first, second| first || second)
                                        .unwrap()
                                })
                                .reduce(|first, second| first || second)
                                .unwrap();
                            if part {
                                val.to_owned()
                            } else {
                                Rc::new(RefCell::new(Value::None))
                            }
                        } else {
                            val.to_owned()
                        }
                    })
                    .collect()
            })
            .collect();

        // Collect gear number neighbors.
        for line in 0..self.values.len() {
            for idx in 0..self.values[line].len() {
                let mut numbers: Vec<Rc<RefCell<Value>>> = Vec::new();
                for y in (line.max(1) - 1)..=(line.min(self.values.len() - 2) + 1) {
                    for x in (idx.max(1) - 1)..=(idx.min(self.values[y].len() - 2) + 1) {
                        if let Value::Part { .. } = *self.values[y][x].borrow() {
                            if !numbers.contains(&self.values[y][x]) {
                                numbers.push(self.values[y][x].clone())
                            }
                        }
                    }
                }
                if let Value::Gear { numbers: nums, .. } = &mut *self.values[line][idx].borrow_mut()
                {
                    *nums = numbers;
                }
            }
        }

        // Remove gears that aren't actually gears.
        self.values = self
            .values
            .iter()
            .map(|line| {
                line.iter()
                    .map(|val| match &*val.borrow() {
                        Value::Gear { numbers, .. } => {
                            if numbers.len() == 2 {
                                val.to_owned()
                            } else {
                                Rc::new(RefCell::new(Value::Symbol))
                            }
                        }
                        _ => val.to_owned(),
                    })
                    .collect()
            })
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let sum: u32 = self
            .values
            .iter()
            .flat_map(|line| {
                line.iter()
                    .enumerate()
                    .filter(|(i, val)| match *val.borrow() {
                        Value::Part { from, .. } => from == *i,
                        _ => false,
                    })
                    .map(|(_, val)| val)
            })
            .map(|val| match *val.borrow() {
                Value::Part { value, .. } => value as u32,
                _ => 0,
            })
            .sum();

        Ok(Some(sum.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let sum: u32 = self
            .values
            .iter()
            .flat_map(|line| line.iter())
            .filter(|val| {
                if let Value::Gear { .. } = *val.borrow() {
                    true
                } else {
                    false
                }
            })
            .map(|gear| {
                if let Value::Gear { numbers, .. } = &*gear.borrow() {
                    if let Value::Part { value: n1, .. } = *numbers[0].borrow() {
                        if let Value::Part { value: n2, .. } = *numbers[1].borrow() {
                            return n1 as u32 * n2 as u32;
                        }
                    }
                }
                0
            })
            .sum();

        Ok(Some(sum.to_string()))
    }
}

/// A single value from the input data.
///
/// Each instance of this enum represents a single position on the 2D input data.  
/// Depending on the type this stores different data, depending on what seems useful.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Part {
        value: u16,
        line: usize,
        from: usize,
        to: usize,
    },
    Gear {
        line: usize,
        index: usize,
        numbers: Vec<Rc<RefCell<Value>>>,
    },
    Symbol,
    None,
}
