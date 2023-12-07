//! My rust AoC 2023 day 7 implementation.
//!
//! This module contains my solution to the [Advent of Code](https://adventofcode.com/) [2023](https://adventofcode.com/2023/) [Day 7](https://adventofcode.com/2023/day/7).

use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use super::super::get_input_file;
use super::DayRunner;

/// The day 7 runner.
///
/// The [DayRunner] implementation for the aoc day 7.
#[derive(Debug, Clone, Default)]
pub struct Day7Runner {}

impl DayRunner for Day7Runner {
    fn part1(&self) -> Result<Option<String>, Box<dyn Error>> {
        let input_data = fs::read_to_string(get_input_file(7)?)?;
        let cards: HashMap<char, u8> = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ]
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i as u8))
        .collect();

        let hands: Vec<&str> = input_data
            .lines()
            .map(|line| line.split_once(&[' ', '\t']).unwrap().0)
            .collect();

        let mut types: Vec<(&str, HandType)> = hands
            .iter()
            .map(|hand| {
                let mut amounts: HashMap<char, u8> = HashMap::new();
                hand.chars()
                    .for_each(|c| *amounts.entry(c).or_insert(0) += 1);
                amounts
            })
            .map(|counts| counts.into_values().collect::<Vec<u8>>())
            .map(|mut counts| {
                counts.sort();
                counts
            })
            .map(|counts| {
                if counts[0] == 5 {
                    HandType::FiveOfAKind
                } else if counts[1] == 4 {
                    HandType::FourOfAKind
                } else if counts[1] == 3 && counts[0] == 2 {
                    HandType::FullHouse
                } else if counts[2] == 3 {
                    HandType::ThreeOfAKind
                } else if counts[1] == 2 && counts[2] == 2 {
                    HandType::TwoPair
                } else if counts[3] == 2 {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            })
            .zip(hands.iter())
            .map(|(typ, hand)| (*hand, typ))
            .collect();

        types.sort_by(|(hand0, typ0), (hand1, typ1)| {
            if typ0.cmp(typ1) != Ordering::Equal {
                typ0.cmp(typ1)
            } else {
                hand0
                    .chars()
                    .map(|c| cards.get(&c).unwrap_or(&0))
                    .zip(hand1.chars().map(|c| cards.get(&c).unwrap_or(&0)))
                    .map(|(c0, c1)| c0.cmp(&c1))
                    .filter(|ord| *ord != Ordering::Equal)
                    .next()
                    .unwrap_or(Ordering::Equal)
            }
        });

        let bets: HashMap<&str, u16> = input_data
            .lines()
            .map(|line| (line.split_once(&[' ', '\t']).unwrap()))
            .map(|(hand, bet)| (hand, bet.parse().unwrap()))
            .collect();

        let result: u64 = types
            .iter()
            .map(|(hand, _)| bets.get(hand).unwrap())
            .enumerate()
            .map(|(i, bet)| *bet as u64 * (i + 1) as u64)
            .sum();

        Ok(Some(result.to_string()))
    }
}

/// The various types of hands.
///
/// Each value of this enum represents one possible type of hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
