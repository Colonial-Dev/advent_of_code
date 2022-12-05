use super::*;
use std::ops::Range;

impl Solution<'_, DAY_04> for Solutions {
    type Input = Vec<(Range<u64>, Range<u64>)>;
    type Output = usize;

    fn parse(puzzle: &'_ str) -> Self::Input {
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

    fn part_one(input: &Self::Input) -> Option<Self::Output> {
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

    fn part_two(input: &Self::Input) -> Option<Self::Output> {
        input
            .iter()
            .filter(|(left, right)| {
                (left.start <= right.end) && (right.start <= left.end)           
            })
            .count()
            .into()
    }
}

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