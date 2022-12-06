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