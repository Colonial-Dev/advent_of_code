//! # Day 3 - Rucksack Reorganization
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 00:32:11 (10616)
//! - P2 completed @ 00:53:36 (11174)
//! 
//! A slight jump in difficulty, requiring us to find the unique intersection of two string slices.
//! 
//! ## Parsing
//! Like day 2, there's not much we can share between parts, so we just collect the lines of the puzzle.
//! 
//! ## Solutions
//! - Part one maps each line of the input in half, then inserts those string slices into an array.
//! Once we have an array, [`find_array_intersection`] finds the character(s) shared by both slices,
//! and then [`priority_codes`] maps them to [`u64`]s for summing.
//! - Part two is in some ways simpler; we chunk the input by 3, map each set of lines into an `[&str; 3]`, then proceed
//! as in part one.

use super::*;

impl Solution<DAY_03> for Solutions {
    type Input<'a> = Vec<&'a str>;
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle.lines().collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        input
            .iter()
            .map(|line| {
                let end = line.len();
                let midpoint = end / 2;
                [&line[0..midpoint], &line[midpoint..end]]
            })
            .map(find_array_intersection)
            .map(priority_codes)
            .sum::<u64>()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        input
            .chunks(3)
            .map(<[&str; 3]>::try_from)
            .map(Result::unwrap)
            .map(find_array_intersection)
            .map(priority_codes)
            .sum::<u64>()
    }
}

impl Test<DAY_03> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 157,
            PART_TWO => 70
        }
    }
}

derive_tests!(Solutions, DAY_03);

fn find_array_intersection<const N: usize>(set: [&str; N]) -> Vec<char> {
    let mut intersection = set[0]
        .chars()
        .filter(|char| {
            for item in set.iter().skip(1) {
                if !item.contains(*char) { return false; }
            }
            true
        })
        .collect::<Vec<char>>();

    intersection.sort_unstable();
    intersection.dedup();
    intersection
}

fn priority_codes(characters: Vec<char>) -> u64 {
    characters
        .into_iter()
        .map(|character| match character.is_uppercase() {
            true => (character as u8 - 65) + 27,
            false => character as u8 - 96
        } as u64)
        .sum::<u64>()
}