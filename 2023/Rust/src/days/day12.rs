//! My rust AoC 2023 day 12 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 12](https://adventofcode.com/2023/day/12).

use std::error::Error;
use std::fs;

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

        let mut sum: usize = 0;
        for line in input_data.lines() {
            let (map_line, lengths) = line.split_once(' ').unwrap();
            let states: Vec<SpringState> = map_line
                .chars()
                .map(|c| match c {
                    '.' => SpringState::Operational,
                    '?' => SpringState::Unknown,
                    '#' => SpringState::Damaged,
                    x => panic!("Received unknown spring state '{}'!", x),
                })
                .collect();

            let mut possibilities: Vec<Possibilities> = lengths
                .split(',')
                .map(|nr| nr.parse::<u8>().unwrap())
                .map(|nr| Possibilities {
                    length: nr,
                    positions: Vec::new(),
                })
                .collect();

            for i in 0..possibilities.len() {
                let length = possibilities[i].length;
                let start;
                if i == 0 {
                    start = 0;
                } else {
                    start = possibilities[i - 1].positions[0]
                        + possibilities[i - 1].length as usize
                        + 1;
                }

                let mut positions: Vec<usize> = Vec::new();
                for pos in start..=(states.len() - length as usize) {
                    if pos > 0 && states[pos - 1] == SpringState::Damaged {
                        continue;
                    }

                    if pos + length as usize > states.len() {
                        break;
                    }

                    if (pos..pos + length as usize)
                        .map(|i| &states[i])
                        .any(|state| state == &SpringState::Operational)
                    {
                        continue;
                    }

                    if pos + length as usize == states.len()
                        || states[pos + length as usize] != SpringState::Damaged
                    {
                        positions.push(pos);
                    }
                }

                possibilities[i].positions = positions;
            }

            for i in (0..possibilities.len() - 1).rev() {
                let end = *possibilities[i + 1].positions.last().unwrap();
                let length = possibilities[i].length as usize;
                possibilities[i].positions = possibilities[i]
                    .positions
                    .iter()
                    .filter(|pos| *pos + length < end)
                    .copied()
                    .collect();
            }

            /*possibilities = possibilities
                .into_iter()
                .inspect(|poss| {
                    assert!(
                        !poss.positions.is_empty(),
                        "Found no possible result for line \"{}\"",
                        line
                    )
                })
                .filter(|poss| poss.positions.len() > 1)
                .collect();
            if possibilities.is_empty() {
                sum += 1;
                continue;
            }*/

            let mut combis: Vec<Vec<(u8, usize)>> = Vec::new();
            combis.push(Vec::new());
            for poss in &possibilities {
                combis = combis
                    .iter()
                    .flat_map(|combi| {
                        poss.positions
                            .iter()
                            .map(|pos| (poss.length, *pos))
                            .map(|pair| {
                                let mut cmb = combi.clone();
                                cmb.push(pair);
                                cmb
                            })
                            .collect::<Vec<Vec<(u8, usize)>>>()
                    })
                    .collect();
            }

            combis = combis
                .into_iter()
                .filter(|combi| {
                    if combi.is_empty() {
                        return false;
                    }

                    let mut last: (u8, usize) = combi[0];
                    for (len, pos) in combi.iter().skip(1) {
                        if last.1 + last.0 as usize >= *pos {
                            return false;
                        }
                        last = (*len, *pos);
                    }

                    true
                })
                .collect();

            combis = combis
                .into_iter()
                .filter(|combi| {
                    states
                        .iter()
                        .enumerate()
                        .filter(|(_, &state)| state == SpringState::Damaged)
                        .map(|(i, _)| i)
                        .all(|i| {
                            combi
                                .iter()
                                .any(|(len, pos)| &i >= pos && i < pos + *len as usize)
                        })
                })
                .collect();

            sum += combis.len();
        }

        Ok(Some(sum.to_string()))
    }
}

/// A struct containing the possible positions and length of a block of damaged springs.
///
/// A struct containing the possible locations of a block of damages hot springs.  
/// From each position there have to be `length` damaged or unknown states, and then an unknown or operational one.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Possibilities {
    length: u8,
    positions: Vec<usize>,
}

/// The possible states each state can have.
///
/// An enum representing the state of a single hot spring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SpringState {
    Damaged,
    Unknown,
    Operational,
}
