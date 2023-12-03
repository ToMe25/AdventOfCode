use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 3 runner.
///
/// The [DayRunner] implementation for the aoc day 3.
#[derive(Debug, Clone)]
pub struct Day3Runner {}

impl Day3Runner {
    pub fn new() -> Day3Runner {
        Day3Runner {}
    }
}

impl DayRunner for Day3Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input = get_input_file(3)?;
        let input_data = fs::read_to_string(input)?;
        let symbols: Vec<Vec<bool>> = input_data
            .lines()
            .map(|line| line.chars().map(|c| !c.is_digit(10) && c != '.').collect())
            .collect();
        let mut numbers: Vec<Number> = Vec::new();
        let mut number: Option<Number> = None;
        for (i, line) in input_data.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    if let Some(nr) = number.as_mut() {
                        nr.value = nr.value * 10 + c.to_digit(10).unwrap() as u16;
                        nr.to = j;
                    } else {
                        number = Some(Number {
                            value: c.to_digit(10).unwrap() as u16,
                            line: i,
                            from: j,
                            to: j,
                        });
                    }
                } else if let Some(nr) = number {
                    numbers.push(nr);
                    number = None;
                }
            }
        }

        let sum: u32 = numbers
            .iter()
            .filter(|nr| {
                ((nr.from.max(1) - 1)..=(nr.to + 1))
                    .map(|i| {
                        let mut result = false;
                        if nr.line > 0 {
                            result |= symbols[nr.line - 1][i.min(symbols[nr.line - 1].len() - 1)]
                        }
                        if nr.line < symbols.len() - 1 {
                            result |= symbols[nr.line + 1][i.min(symbols[nr.line + 1].len() - 1)]
                        }
                        result
                    })
                    .reduce(|first, second| first || second)
                    .unwrap()
                    || symbols[nr.line][nr.from.max(1) - 1]
                    || symbols[nr.line][nr.to.min(symbols[nr.line].len() - 2) + 1]
            }).inspect(|nr| println!("{:?}", nr))
            .map(|nr| nr.value as u32)
            .sum();

        Ok(Some(sum.to_string()))
    }
}

/// A struct containing information about a number in the input.
///
/// This struct contains information about a single number from the input data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Number {
	/// The integer value of the number.
    value: u16,
    /// The line in which the number was found.
    line: usize,
    /// The index of the first character that is part of this number.
    from: usize,
    /// The index of the last character that is part of this number.
    to: usize,
}
