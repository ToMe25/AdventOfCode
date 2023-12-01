use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

#[derive(Debug, Clone)]
pub struct Day1Runner {}

impl Day1Runner {
    pub fn new() -> Day1Runner {
        Day1Runner {}
    }
}

impl DayRunner for Day1Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input = get_input_file(1)?;
        let mut sum: u32 = 0;
        for (i, line) in fs::read_to_string(input)?.lines().enumerate() {
            let mut first: Option<u8> = None;
            let mut last: u8 = 0;
            for c in line.chars() {
                if c >= '0' && c <= '9' {
                    let _ = first.get_or_insert(c as u8 - b'0');
                    last = c as u8 - b'0';
                }
            }

            if let Some(j) = first {
                sum += (j * 10 + last) as u32;
            } else {
                eprintln!("Line {}({}) didn't contain a single digit.", i, line);
            }
        }

        Ok(Some(sum.to_string()))
    }
}
