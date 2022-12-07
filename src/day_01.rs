//! # Day 1 - Calorie Counting
//! 
//! Puzzle opened late. 
//! - P1 completed @ 02:13:26 (19716)
//! - P2 completed @ 02:23:17 (19249)
//! 
//! Easy problem (obviously) with all actual computation being trivially handled during parsing.
//! 
//! ## Parsing
//! 1. Split the puzzle on blank lines ("\n\n")
//! 2. Map each line set, parsing their lines to [`u64`]s and summing them.
//! 3. Fold the [`u64`] sums, using a `[u64; 3]` as the accumulator and a simple for loop to update
//! the three largest elements. (This approach doesn't work if the sums are in ascending order, but whatever.)
//! 
//! ## Solutions
//! - Part one requested the largest sum, which can be grabbed with no work by indexing into the top three array at 0.
//! - Part two requested the sum of the three largest sums, which can be computed by simply using `iter().sum()` on the top three array.

use super::*;

impl Solution<DAY_01> for Solutions {
    type Input<'a> = [u64; 3];
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .split("\n\n")
            .map(|set| {
                set.lines()
                    .map(str::parse::<u64>)
                    .map(Result::unwrap)
                    .sum::<u64>()
            })
            .fold([0, 0, 0], |mut acc, n| {
                for value in &mut acc {
                    if n > *value {
                        *value = n;
                        break;
                    }
                }
                acc
            })
    }

    fn part_one(input: &Self::Input<'_>) -> Option<Self::Output> {
        Some(input[0])
    }

    fn part_two(input: &Self::Input<'_>) -> Option<Self::Output> {
        Some(input.iter().sum::<u64>())
    }
}

impl Test<DAY_01> for Solutions {
    fn expected() -> (Option<Self::Output>, Option<Self::Output>) {
        (24_000.into(), 34_000.into())
    }
}

derive_tests!(Solutions, DAY_01);