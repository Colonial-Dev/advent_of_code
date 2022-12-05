use super::*;

impl<'a> Solution<'a, DAY_03> for Solutions {
    type Input = Vec<&'a str>;
    type Output = u64;

    fn parse(puzzle: &'a str) -> Self::Input {
        puzzle.lines().collect()
    }

    fn part_one(input: &Self::Input) -> Option<Self::Output> {
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
            .into()
    }

    fn part_two(input: &Self::Input) -> Option<Self::Output> {
        input
            .chunks(3)
            .map(<[&str; 3]>::try_from)
            .map(Result::unwrap)
            .map(find_array_intersection)
            .map(priority_codes)
            .sum::<u64>()
            .into()
    }
}

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