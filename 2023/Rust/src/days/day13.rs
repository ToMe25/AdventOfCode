//! My rust AoC 2023 day 13 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 13](https://adventofcode.com/2023/day/13).

use std::error::Error;
use std::fmt::Display;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 13 runner.
///
/// The [DayRunner] implementation for the aoc day 13.
#[derive(Debug, Clone, Default)]
pub struct Day13Runner {}

impl DayRunner for Day13Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(13)?)?;
        let patterns: Vec<Pattern> = input_data
            .lines()
            .fold(Vec::new(), |mut list, line| {
                if list.is_empty() || line.is_empty() {
                    list.push(Vec::new());
                }

                if !line.is_empty() {
                    list.last_mut().unwrap().push(line);
                }

                list
            })
            .iter()
            .map(Pattern::parse)
            .collect();

        let result: usize = patterns
            .into_iter()
            .map(|mut pattern| {
                pattern.find_mirror();
                pattern
            })
            .map(|pattern| pattern.summarize())
            .enumerate()
            .filter_map(|(i, summary)| match summary {
                Some(_) => summary,
                None => {
                    eprintln!("Pattern {} failed to find a mirror axis.", i + 1);
                    None
                }
            })
            .sum();

        return Ok(Some(result.to_string()));
    }
}

/// A single one of the patterns from the input data.
///
/// Contains the materials for each position in the pattern.
/// Also calculates and stores the mirror axis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    map: Vec<Vec<Material>>,
    mirror_axis: MirrorAxis,
}

impl Pattern {
    /// Creates a new pattern with the given materials.
    ///
    /// Creates a new pattern object representing the given 2D vector of Materials.
    /// The newly created pattern will not yet have a mirror axis.
    pub fn new(materials: Vec<Vec<Material>>) -> Pattern {
        Pattern {
            map: materials,
            mirror_axis: MirrorAxis::default(),
        }
    }

    /// Creates a new pattern from a list of input lines.
    ///
    /// Parses a single pattern from its string representation.  
    /// Empty lines and unknown characters are ignored as if they simply didn't exist.  
    /// Both of these cases cause a log message to be generated.
    ///
    /// # Examples
    /// Parsing a simple pattern:
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    /// # use rust_aoc_2023::days::day13::Material;
    ///
    /// let pattern = Pattern::parse(&vec!("#.#", ".#.", "#.#"));
    /// #
    /// # let ref_pat = Pattern::new(vec!(vec!(Material::Rock, Material::Ash, Material::Rock),
    /// #                                 vec!(Material::Ash, Material::Rock, Material::Ash),
    /// #                                 vec!(Material::Rock, Material::Ash, Material::Rock)));
    /// # assert_eq!(pattern, ref_pat);
    /// ```
    ///
    /// Unknown characters are treated as if they didn't exist:
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    ///
    /// let pattern = Pattern::parse(&vec!("##?#", "#.."));
    /// assert_eq!(pattern, Pattern::parse(&vec!("###", "#..")));
    /// ```
    ///
    /// While there is no good reason to do so, lines may be different in length:
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    ///
    /// let pattern = Pattern::parse(&vec!("#...#", "##.", ".###"));
    /// ```
    ///
    /// Empty lines are also completely ignored:
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    ///
    /// let pattern = Pattern::parse(&vec!("#..#", "", ".##.", "", "...#"));
    /// assert_eq!(pattern, Pattern::parse(&vec!("#..#", ".##.", "...#")));
    /// ```
    pub fn parse(lines: &Vec<&str>) -> Pattern {
        let materials: Vec<Vec<Material>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .filter_map(|c| match c {
                        '#' => Some(Material::Rock),
                        '.' => Some(Material::Ash),
                        _ => {
                            eprintln!("Received unknown pattern component '{}'! Igoring it.", c);
                            None
                        }
                    })
                    .collect::<Vec<Material>>()
            })
            .filter(|row| {
                if row.is_empty() {
                    eprintln!("Received an empty line! Ignoring it.");
                    return false;
                }
                true
            })
            .collect();

        Pattern {
            map: materials,
            mirror_axis: MirrorAxis::default(),
        }
    }

    /// Adds the given row of materials to the bottom of this pattern.
    ///
    /// Adds a new row to the bottom of the pattern.
    ///
    /// # Examples
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    /// use rust_aoc_2023::days::day13::Material;
    ///
    /// let mut pattern = Pattern::new(vec!(vec!(Material::Rock, Material::Ash, Material::Rock),
    ///                                vec!(Material::Ash, Material::Rock, Material::Ash)));
    /// pattern.add_row(vec!(Material::Rock, Material::Ash, Material::Rock));
    /// #
    /// # let ref_pat = Pattern::new(vec!(vec!(Material::Rock, Material::Ash, Material::Rock),
    /// #                            vec!(Material::Ash, Material::Rock, Material::Ash),
    /// #                            vec!(Material::Rock, Material::Ash, Material::Rock)));
    /// # assert_eq!(pattern, ref_pat);
    /// ```
    pub fn add_row(&mut self, row: Vec<Material>) {
        self.map.push(row);
    }

    /// Finds the mirror axis in a pattern and stores it in the pattern object.
    ///
    /// Finds the mirror axis in a pattern.  
    /// Will not run again if a mirror axis was already found.  
    /// This means it cannot be used to update the axis after an [`add_row`] call.
    ///
    /// If some lines are shorter, those missing positions are considered to match any material.
    ///
    /// Note that this method only finds the first mirror axis.
    ///
    /// Use [`get_mirror`] to retrieve the mirror axis found by this function.
    ///
    /// # Examples
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    /// use rust_aoc_2023::days::day13::MirrorAxis;
    ///
    /// let mut pattern = Pattern::parse(&vec!("#..#.", ".##.#", "....#"));
    /// pattern.find_mirror();
    /// assert_eq!(pattern.get_mirror(), &MirrorAxis::Vertical(1));
    /// ```
    ///
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    /// use rust_aoc_2023::days::day13::MirrorAxis;
    ///
    /// let mut pattern = Pattern::parse(&vec!("##.#..#", "..#....", "#.##..#", "#.##..#", "..#...."));
    /// pattern.find_mirror();
    /// assert_eq!(pattern.get_mirror(), &MirrorAxis::Horizontal(2));
    /// ```
    ///
    /// If some lines are shorter, those missing positions are considered to match any material:
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    /// use rust_aoc_2023::days::day13::MirrorAxis;
    ///
    /// let mut pattern = Pattern::parse(&vec!("..##..#", "..##.", ".#..#.#"));
    /// pattern.find_mirror();
    /// assert_eq!(pattern.get_mirror(), &MirrorAxis::Vertical(2));
    /// ```
    ///
    /// If no mirror can be found, the mirror axis will stay as unknown:
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    /// use rust_aoc_2023::days::day13::MirrorAxis;
    ///
    /// let mut pattern = Pattern::parse(&vec!("..#.", ".#..", "####"));
    /// pattern.find_mirror();
    /// assert_eq!(pattern.get_mirror(), &MirrorAxis::Unknown);
    /// ```
    ///
    /// This method cannot be used to update the mirror axis by calling it a second time:
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    /// use rust_aoc_2023::days::day13::Material;
    /// use rust_aoc_2023::days::day13::MirrorAxis;
    ///
    /// // Note that horizontal mirrors are checked first.
    /// let mut pattern = Pattern::parse(&vec!("..##..", ".####.", ".####."));
    /// pattern.find_mirror();
    /// # assert_eq!(pattern.get_mirror(), &MirrorAxis::Horizontal(1));
    /// pattern.add_row(vec!(Material::Rock, Material::Ash, Material::Ash, Material::Ash, Material::Ash, Material::Rock));// #....#
    /// pattern.find_mirror();
    /// assert_eq!(pattern.get_mirror(), &MirrorAxis::Horizontal(1));
    /// ```
    ///
    /// [`add_row`]: Self#method.add_row
    /// [`get_mirror`]: Self#method.get_mirror
    pub fn find_mirror(&mut self) {
        if self.mirror_axis != MirrorAxis::Unknown {
            return;
        }

        let num_rows = self.map.len();
        for i in 0..(num_rows - 1) {
            let mut failed = false;
            for y in 0..(i + 1).min(num_rows - i - 1) {
                if self.map[i - y] != self.map[i + y + 1] {
                    failed = true;
                    break;
                }
            }
            if !failed {
                self.mirror_axis = MirrorAxis::Horizontal(i);
                return;
            }
        }

        let max_cols = self.map.iter().map(|row| row.len()).max().unwrap_or(0);
        for i in 0..(max_cols - 1) {
            let mut failed = false;
            for x in 0..(i + 1).min(max_cols - i - 1) {
                for y in 0..num_rows {
                    if self.map[y].len() > i + x + 1 && self.map[y][i - x] != self.map[y][i + x + 1]
                    {
                        failed = true;
                        break;
                    }
                }
                if failed {
                    break;
                }
            }
            if !failed {
                self.mirror_axis = MirrorAxis::Vertical(i);
                return;
            }
        }
    }

    /// Gets the mirror axis of this pattern.
    ///
    /// Will be [`Unknown`] if [`find_mirror`] was not yet called, or if no mirror axis could be found.
    ///
    /// [`Unknown`]: self::MirrorAxis#variant.Unknown
    /// [`find_mirror`]: Self#method.find_mirror
    pub fn get_mirror(&self) -> &MirrorAxis {
        &self.mirror_axis
    }

    /// Calculate the summary of this pattern.
    ///
    /// Returns the number of rows above the mirror in case of a horizontal reflection,  
    /// and 100 times the number of columns to the left of the mirror for a vertical one.
    ///
    /// Returns [`None`] if the mirror axis is not known.
    ///
    /// # Examples
    /// Result with a horizontal reflection:
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    ///
    /// let mut pattern = Pattern::parse(&vec!(".#.#", "##..", "##..", ".#.#", "..##"));
    /// pattern.find_mirror();
    /// assert_eq!(pattern.summarize(), Some(200));
    /// ```
    ///
    /// An example with a vertical mirror axis:
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    ///
    /// let mut pattern = Pattern::parse(&vec!("..##..", "#.##.#", "##..##"));
    /// pattern.find_mirror();
    /// assert_eq!(pattern.summarize(), Some(3));
    /// ```
    ///
    /// An example with an unknown mirror axis, in this case because [`find_mirror`] wasn't run yet:
    /// ```
    /// use rust_aoc_2023::days::day13::Pattern;
    ///
    /// let pattern = Pattern::parse(&vec!("..##..", "##..##", "######"));
    /// assert_eq!(pattern.summarize(), None);
    /// ```
    ///
    /// [`None`]: Option#variant.None
    /// [`find_mirror`]: Self#method.find_mirror
    pub fn summarize(&self) -> Option<usize> {
        match self.mirror_axis {
            MirrorAxis::Horizontal(y) => Some((y + 1) * 100),
            MirrorAxis::Vertical(x) => Some(x + 1),
            MirrorAxis::Unknown => None,
        }
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Pattern:")?;
        self.map.iter().enumerate().try_for_each(|(y, line)| {
            line.iter()
                .enumerate()
                .try_for_each(|(x, mat)| -> Result<(), std::fmt::Error> {
                    match mat {
                        Material::Rock => write!(f, "#")?,
                        Material::Ash => write!(f, ".")?,
                    };

                    if matches!(self.mirror_axis, MirrorAxis::Vertical(c) if c == x) {
                        write!(f, "|")?;
                    }

                    Ok(())
                })?;

            if matches!(self.mirror_axis, MirrorAxis::Horizontal(r) if r == y) {
                writeln!(f)?;
                for _ in 0..line.len() {
                    write!(f, "-")?;
                }
            }
            writeln!(f)
        })
    }
}

/// The possible materials that every location in a pattern can have.
///
/// An enum representing the visible material at a single position in a pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Material {
    Rock,
    Ash,
}

/// The mirror axis of a pattern.
///
/// An enum representing the mirror axis of a pattern.
/// The values other than Unknown have an usize representing the row/column after which the pattern is mirrored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MirrorAxis {
    /// The pattern is mirrored vertically after the given column.
    Vertical(usize),
    /// The pattern is mirrored horizontally after the given column.
    Horizontal(usize),
    /// The mirror axis of the pattern is not (yet) known.
    #[default]
    Unknown,
}