use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 1 runner.
///
/// The [DayRunner] implementation for the aoc day 1.
#[derive(Debug, Clone, Default)]
pub struct Day1Runner {}

impl DayRunner for Day1Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input = get_input_file(1)?;
        let mut sum: u32 = 0;
        for (i, line) in fs::read_to_string(input)?.lines().enumerate() {
            let mut chars = line.matches(|c: char| c.is_digit(10));
            let first = chars.next().map(|s| s.parse::<u8>().unwrap());
            let last = chars
                .last()
                .map(|s| s.parse::<u8>().unwrap())
                .unwrap_or(first.unwrap_or(0));

            if let Some(j) = first {
                sum += (j * 10 + last) as u32;
            } else {
                eprintln!("Line {}({}) didn't contain a single digit.", i, line);
            }
        }

        Ok(Some(sum.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let numbers = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let input = get_input_file(1)?;
        let mut sum: u32 = 0;
        for (i, line) in fs::read_to_string(input)?.lines().enumerate() {
            let mut first = line
                .find(|c: char| c.is_digit(10))
                .map(|i| (i, line[i..i + 1].parse::<u8>().unwrap_or(0)))
                .unwrap_or((line.len(), 0));
            let mut last = line
                .rfind(|c: char| c.is_digit(10))
                .map(|i| (i, line[i..i + 1].parse::<u8>().unwrap_or(0)))
                .unwrap_or((0, 0));

            for (i, nr) in numbers.iter().enumerate() {
                let mut found = line.match_indices(nr);
                if let Some((j, _)) = found.next() {
                    if j < first.0 {
                        first = (j, (i + 1) as u8);
                    } else if j >= last.0 {
                        last = (j, (i + 1) as u8);
                    }
                }

                if let Some((j, _)) = found.last() {
                    if j >= last.0 {
                        last = (j, (i + 1) as u8);
                    }
                }
            }

            if first.0 < line.len() {
                sum += (first.1 * 10 + last.1) as u32;
            } else {
                eprintln!("Line {}({}) didn't contain a single digit.", i, line);
            }
        }

        Ok(Some(sum.to_string()))
    }
}
