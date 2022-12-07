//! # Day 4 - Camp Cleanup
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 00:11:08 (5286)
//! - P2 completed @ 00:23:34 (7879)
//! 
//! Easy day; literally just some boolean logic.
//! 
//! ## Parsing
//! We map each line in the input to a pair of [`Range<u64>`], using [`str::split_once`] and
//! [`pair_to_range`].
//! 
//! ## Solutions
//! Since we already have the inputs as well-typed ranges, we simply have to filter
//! out any range pairs where one does not fully contain the other and count how many remain.
//! Simple chained boolean comparisons work well for both parts.

use super::*;
use std::ops::Range;

impl Solution<DAY_04> for Solutions {
    type Input<'a> = Vec<(Range<u64>, Range<u64>)>;
    type Output = usize;

    fn parse(puzzle: &'_ str) -> Self::Input<'_> {
        puzzle
            .lines()
            .map(|line| {
                line
                    .split_once(',')
                    .unwrap()
            })
            .map(|(left, right)| (
                pair_to_range(left),
                pair_to_range(right)
            ))
            .collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Option<Self::Output> {
        input
            .iter()
            .filter(|(left, right)| {
                (left.start >= right.start) && (left.end <= right.end)
                ||
                (right.start >= left.start) && (right.end <= left.end)
            })
            .count()
            .into()
    }

    fn part_two(input: &Self::Input<'_>) -> Option<Self::Output> {
        input
            .iter()
            .filter(|(left, right)| {
                (left.start <= right.end) && (right.start <= left.end)           
            })
            .count()
            .into()
    }
}

impl Test<DAY_04> for Solutions {
    fn expected() -> (Option<Self::Output>, Option<Self::Output>) {
        (2.into(), 4.into())
    }
}

derive_tests!(Solutions, DAY_04);

fn pair_to_range(pair: &str) -> Range<u64> {
    pair
        .split_once('-')
        .map(|(l, r)| {
            (
                l.parse::<u64>().unwrap(),
                r.parse::<u64>().unwrap()
            )
        })
        .map(|(l, r)| l..r)
        .unwrap()
}