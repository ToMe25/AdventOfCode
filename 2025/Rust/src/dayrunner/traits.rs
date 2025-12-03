//! The traits used for the Day Runners to specify which part(s) they implement.

use std::error::Error;

use super::RunContext;

/// A runner that executes the solution for a single day of the [Advent of Code](https://adventofcode.com/2025).
pub trait DayRunner {
    /// The constructor for a runner for this day.
    ///
    /// This is supposed to handle the part of the solution that is shared between both parts.
    ///
    /// # Errors
    ///
    /// The implementation is allowed to return arbitrary errors when something fails.
    ///
    /// # Panics
    ///
    /// While non-panicking implementations are preferred, to allow running the remaining days, an implementation **MAY** panic for any reason.
    fn init(ctx: &RunContext) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

/// A runner that is aware of its own date.
///
/// Moved to a separate trait to allow dyn compatibility for [`DayRunner`].
pub trait DayRunnerDate {
    /// The day this runner implements the solution for.
    const DAY: u8;
}

/// A runner that implements a solution for part one of a day of the [Advent of Code](https://adventofcode.com/2025).
pub trait Part1Runner: DayRunner {
    /// The function running the solution for part one of the current day.
    ///
    /// # Errors
    ///
    /// The implementation is allowed to return arbitrary errors when something fails.
    ///
    /// # Panics
    ///
	/// While non-panicking implementations are preferred, to allow running the remaining days, an implementation **MAY** panic for any reason.
    fn part1(&self, ctx: &RunContext) -> Result<String, Box<dyn Error>>;
}

/// A runner that implements a solution for part two of a day of the [Advent of Code](https://adventofcode.com/2025).
pub trait Part2Runner: DayRunner {
    /// The function running the solution for part two of the current day.
    ///
    /// # Errors
    ///
    /// The implementation is allowed to return arbitrary errors when something fails.
    ///
    /// # Panics
    ///
	/// While non-panicking implementations are preferred, to allow running the remaining days, an implementation **MAY** panic for any reason.
    fn part2(&self, ctx: &RunContext) -> Result<String, Box<dyn Error>>;
}

/// A marker trait to help with runners that implement both parts.
pub trait BothPartsRunner: Part1Runner + Part2Runner {}

impl<T: Part1Runner + Part2Runner> BothPartsRunner for T {}
