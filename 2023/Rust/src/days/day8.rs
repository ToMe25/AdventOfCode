//! My rust AoC 2023 day 8 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 8](https://adventofcode.com/2023/day/8).

use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::rc::Rc;

use super::super::get_input_file;
use super::DayRunner;

/// The day 8 runner.
///
/// The [DayRunner] implementation for the aoc day 8.
#[derive(Debug, Clone, Default)]
pub struct Day8Runner {
    /// The instructions for where to walk.
    ///
    /// The parsed right/left instructions from the input.  
    /// `true` means right and `false` means left.
    instructions: Vec<bool>,
    /// The nodes from the input file.
    ///
    /// The nodes from the input file.  
    /// The key is the current node, and the value are the possible next nodes.
    nodes: HashMap<String, Rc<RefCell<Node>>>,
}

impl DayRunner for Day8Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(8)?)?;
        let mut lines = input_data.lines().filter(|line| !line.is_empty());

        self.instructions = lines.next().unwrap().chars().map(|c| c == 'R').collect();

        let nodes: HashMap<&str, (&str, &str)> = lines
            .map(|line| line.split_once('=').unwrap())
            .map(|(from, to)| {
                (
                    from,
                    to.trim_matches(&[' ', '\t', '(', ')'][..])
                        .split_once(',')
                        .unwrap(),
                )
            })
            .map(|(from, (tol, tor))| (from.trim(), (tol.trim(), tor.trim())))
            .collect();

        self.nodes = nodes
            .keys()
            .map(|name| {
                (
                    (*name).to_owned(),
                    Rc::new(RefCell::new(Node {
                        name: (*name).to_owned(),
                        left: None,
                        right: None,
                    })),
                )
            })
            .collect();

        nodes.iter().for_each(|(key, (left, right))| {
            let node = self.nodes.get(*key).unwrap().clone();
            node.borrow_mut().left = Some(self.nodes.get(*left).unwrap().clone());
            node.borrow_mut().right = Some(self.nodes.get(*right).unwrap().clone());
        });

        Ok(())
    }

    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let mut insts = self.instructions.iter().cycle();
        let mut current = self.nodes.get("AAA").unwrap().clone();
        let mut steps: usize = 0;
        while current.borrow().name != "ZZZ" {
            if *insts.next().unwrap() {
                current = current.clone().borrow().right.clone().unwrap();
            } else {
                current = current.clone().borrow().left.clone().unwrap();
            }
            steps += 1;
        }

        Ok(Some(steps.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
    	// FIXME find a solution that is fast enough to finish before the end of time.
        let mut insts = self.instructions.iter().cycle();
        let mut current: Vec<Rc<RefCell<Node>>> = self
            .nodes
            .iter()
            .filter(|(key, _)| key.ends_with('A'))
            .map(|(_, node)| node.clone())
            .collect();

        let mut steps: usize = 0;
        while !current.iter().all(|pos| pos.borrow().name.ends_with('Z')) {
            let right = *insts.next().unwrap();
            current = current
                .iter()
                .map(|node| {
                    if right {
                        node.borrow().right.clone().unwrap()
                    } else {
                        node.borrow().left.clone().unwrap()
                    }
                })
                .collect();
            steps += 1;
        }

        Ok(Some(steps.to_string()))
    }
}

/// A node from the input data.
///
/// A single node from the input data.  
/// Contains its name, its left next node, and its right next node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    name: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}
