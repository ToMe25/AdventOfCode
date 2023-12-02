use std::collections::HashMap;
use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 2 runner.
///
/// The [DayRunner] implementation for the aoc day 2.
#[derive(Debug, Clone)]
pub struct Day2Runner {
    /// The cubes that were taken out the bag at once.
    ///
    /// The outer vector contains one vector per game.  
    /// Each inner vector contains a color and the number of cubes of that color that were taken.
    cubes: Option<Vec<Vec<(String, u16)>>>,
}

impl Day2Runner {
    pub fn new() -> Day2Runner {
        Day2Runner { cubes: None }
    }
}

impl DayRunner for Day2Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input = get_input_file(2)?;
        self.cubes = Some(
            fs::read_to_string(input)?
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.split_once(':').unwrap().1.trim())
                .map(|line| line.split([';', ',']))
                .map(|game| {
                    game.map(|cubes| cubes.trim().split_once(' ').unwrap())
                        .map(|(nr, color)| (color.to_owned(), nr.parse::<u16>().unwrap()))
                        .collect()
                })
                .collect(),
        );

        Ok(())
    }

    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let available: HashMap<&str, u16> =
            HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        let sum: u16 = self
            .cubes
            .as_ref()
            .unwrap()
            .iter()
            .map(|game| game.iter())
            .map(|game| {
                game.map(|(color, nr)| available.get(&color[..]).unwrap() >= nr)
                    .reduce(|first, second| first && second)
                    .unwrap()
            })
            .enumerate()
            .map(|(i, valid)| if valid { (i + 1) as u16 } else { 0 })
            .sum();
        Ok(Some(sum.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let sum: u32 = self.cubes.as_ref().unwrap().iter().map(|game| {
            game.iter()
                .map(|(color, nr)| match color.as_str() {
                    "red" => (*nr, 0 as u16, 0 as u16),
                    "blue" => (0 as u16, *nr, 0 as u16),
                    "green" => (0 as u16, 0 as u16, *nr),
                    _ => (0, 0, 0),
                })
                .reduce(|first, second| {
                    (
                        first.0.max(second.0),
                        first.1.max(second.1),
                        first.2.max(second.2),
                    )
                }).unwrap()
        }).map(|(red, green, blue)| (red * green * blue) as u32).sum();
        Ok(Some(sum.to_string()))
    }
}
