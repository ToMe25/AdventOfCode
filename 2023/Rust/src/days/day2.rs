use std::collections::HashMap;
use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 2 runner.
///
/// The [DayRunner] implementation for the aoc day 2.
#[derive(Debug, Clone)]
pub struct Day2Runner {}

impl Day2Runner {
    pub fn new() -> Day2Runner {
        Day2Runner {}
    }
}

impl DayRunner for Day2Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let available: HashMap<&str, u16> =
            HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        let input = get_input_file(2)?;
        let sum: u16 = fs::read_to_string(input)?
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split_once(':').unwrap().1.trim())
            .map(|line| line.split(';').map(|cubes| cubes.split(',')))
            .map(|game| game.flatten())
            .map(|game| {
                game.map(|cubes| cubes.trim().split_once(' ').unwrap())
                    .map(|(nr, color)| available.get(color).unwrap() >= &nr.parse::<u16>().unwrap())
                    .reduce(|first, second| first && second)
                    .unwrap()
            })
            .enumerate()
            .map(|(i, valid)| if valid { (i + 1) as u16 } else { 0 })
            .sum();
        Ok(Some(sum.to_string()))
    }
}
