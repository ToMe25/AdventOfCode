//! My rust AoC 2023 day 8 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 8](https://adventofcode.com/2023/day/8).

use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display};
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
    instructions: Vec<MovementDirection>,
    /// The nodes from the input file.
    ///
    /// The nodes from the input file.  
    /// The key is the name of the current node, and the value is the parsed node.
    nodes: HashMap<String, Rc<RefCell<Node>>>,
}

impl DayRunner for Day8Runner {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(8)?)?;
        let mut lines = input_data.lines().filter(|line| !line.is_empty());

        self.instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| MovementDirection::parse(c))
            .collect();

        let nodes: HashMap<&str, (&str, &str)> = lines
            .map(|line| line.split_once('=').unwrap())
            .map(|(from, to)| {
                (
                    from,
                    to.trim_matches([' ', '\t', '(', ')'])
                        .split_once(',')
                        .unwrap(),
                )
            })
            .map(|(from, (tol, tor))| (from.trim(), (tol.trim(), tor.trim())))
            .collect();

        self.nodes = nodes
            .keys()
            .map(|name| ((*name).to_owned(), Rc::new(RefCell::new(Node::new(*name)))))
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
        let mut current = self.nodes.get("AAA").expect("Node AAA not found").clone();
        let mut steps: u64 = 0;
        while current.borrow().name != "ZZZ" {
            current = match insts.next().unwrap() {
                &MovementDirection::Left => current.clone().borrow().left.clone().unwrap(),
                &MovementDirection::Right => current.clone().borrow().right.clone().unwrap(),
            };
            steps += 1;
        }

        Ok(Some(steps.to_string()))
    }

    fn part2(&self) -> Result<Option<String>, Box<dyn Error>> {
        let mut insts = self.instructions.iter().cycle();
        let mut patterns: Vec<MovementPattern> = self
            .nodes
            .values()
            .filter(|val| val.borrow().node_type == NodeType::Start)
            .map(|node| MovementPattern::new(node.clone()))
            .collect();

        patterns = patterns
            .into_iter()
            .map(|mut pattern| {
                let mut current = pattern.start.clone();
                let mut steps: u64 = 0;
                while !pattern.verified {
                    current = match insts.next().unwrap() {
                        &MovementDirection::Left => current.clone().borrow().left.clone().unwrap(),
                        &MovementDirection::Right => {
                            current.clone().borrow().right.clone().unwrap()
                        }
                    };
                    steps += 1;

                    if current.borrow().node_type == NodeType::Destination {
                        pattern.arrived_at(current.clone(), steps);
                    }
                }

                pattern
            })
            .collect();

        let (mut steps, max_steps) = patterns
            .iter()
            .map(|pat| {
                (
                    pat.steps_first.expect("pattern steps_first not set"),
                    pat.steps.expect("pattern steps not set"),
                )
            })
            .max_by(|val, other| val.1.cmp(&other.1))
            .expect("max not found");

        while patterns.iter().any(|pat| !pat.is_at_destination(steps)) {
            steps += max_steps;
        }

        Ok(Some(steps.to_string()))
    }
}

/// A node from the input data.
///
/// A single node from the input data.  
/// Contains its name, its left next node, and its right next node.
#[derive(Clone, PartialEq, Eq)]
struct Node {
    pub name: String,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub node_type: NodeType,
}

impl Node {
    /// Creates a new `Node` with the given name.
    pub fn new(name: &str) -> Self {
        let node_type = match name.chars().last().expect("Received empty string") {
            'A' | 'a' => NodeType::Start,
            'Z' | 'z' => NodeType::Destination,
            _ => NodeType::Normal,
        };
        Self {
            name: name.to_owned(),
            left: None,
            right: None,
            node_type,
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left_str = match &self.left {
            Some(x) => format!("Some(Node {{ name: \"{}\", .. }})", x.borrow().name),
            None => String::from("None"),
        };
        let right_str = match &self.right {
            Some(x) => format!("Some(Node {{ name: \"{}\", .. }})", x.borrow().name),
            None => String::from("None"),
        };
        write!(
            f,
            "Node: {{ name: {:?}, left: {left_str}, right: {right_str}, node_type: {:?} }}",
            self.name, self.node_type
        )
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// The type of a node.
///
/// A value of [`Start`](Self::Start) represents a node with a name ending with an `A`,  
/// and a value of [`Destination`](Self::Destination) represents a name ending with a `Z`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum NodeType {
    #[default]
    Normal,
    Start,
    Destination,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

/// A direction to move in.
///
/// An enum representing the possible movement directions from the input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MovementDirection {
    Left,
    Right,
}

impl MovementDirection {
    /// Gets the movement direction represented by the given character.
    ///
    /// This function is not case sensitive.
    ///
    /// # Panics
    ///
    /// This function panics if the given character is neither "R" nor "L".
    pub fn parse(c: char) -> Self {
        match c {
            'L' | 'l' => Self::Left,
            'R' | 'r' => Self::Right,
            _ => panic!("received invalid character {c}"),
        }
    }
}

/// A struct representing the stable loop of steps required from one start point to one end point.
///
/// If a `MovementPattern` is verified, that means that it has checked whether the movement is a stable loop.  
/// That means that after that point it can be safely assumed that it will arrive at its destination every `steps` stesp.
#[derive(Debug, Clone, PartialEq, Eq)]
struct MovementPattern {
    pub start: Rc<RefCell<Node>>,
    pub destination: Option<Rc<RefCell<Node>>>,
    /// The number of steps that were taken from the start node to the destination node.
    pub steps_first: Option<u64>,
    pub steps: Option<u64>,
    pub verified: bool,
}

impl MovementPattern {
    /// Creates a new `MovementPattern` starting at the given node.
    pub fn new(start: Rc<RefCell<Node>>) -> Self {
        MovementPattern {
            start,
            destination: None,
            steps_first: None,
            steps: None,
            verified: false,
        }
    }

    /// Updates this movement pattern with the information that it arrived at the current destination node after X steps.
    ///
    /// If it did not have a destination node, its destination node and steps_first will be set.  
    /// If it did not yet have a stable steps count, that will be set.  
    /// If it has both, but was not yet verified its steps count will be verified.
	///
	/// If this pattern is already verified this function will not do anything.
    ///
    /// # Panics
    ///
    /// * When one of destination and steps_first was set, but not the other.
    /// * If node isn't a [`Destination`](NodeType::Destination) node.
    /// * If the number of steps from the first time on the destination to the second, and from the second to the third don't match.
    ///
    pub fn arrived_at(&mut self, node: Rc<RefCell<Node>>, steps: u64) {
        assert!(
            self.steps_first.is_some() == self.destination.is_some(),
            "steps_first and destination must be set at the same time"
        );
        assert!(
            node.borrow().node_type == NodeType::Destination,
            "node has to be a destination node"
        );

        if self.steps_first.is_none() {
            self.steps_first = Some(steps);
            self.destination = Some(node);
        } else if self.steps.is_none() {
            assert!(
                self.destination.as_ref().expect("destination not set") == &node,
                "arrived at different destination"
            );
            self.steps = Some(steps - self.steps_first.expect("steps_first not set"));
        } else if !self.verified {
            assert!(
                self.steps_first.expect("steps_first not set")
                    + self.steps.expect("steps not set") * 2
                    == steps,
                "steps count first and second time didn't match"
            );
            self.verified = true;
        }
    }

    /// Checks whether this movement pattern is at a destination after the given number of steps.
    ///
    /// # Panics
    ///
    /// This function panics if `steps` or `steps_first` isn't set yet, or this pattern isn't verified yet.
    pub fn is_at_destination(&self, steps: u64) -> bool {
        assert!(self.verified, "self not verified");
        steps % self.steps.expect("steps not set")
            == self.steps_first.expect("steps_first not set") % self.steps.expect("steps not set")
    }
}
