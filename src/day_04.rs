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