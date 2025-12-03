//! This module contains the implementations of the runners for the individual days of the [Advent of Code 2025](https://adventofcode.com/2025).

use crate::register_runner;

/// Registers all the runners of the individual day solutions.
pub fn register_runners() {
    register_runner!(day1::Day1Runner);
}

pub mod day1;
