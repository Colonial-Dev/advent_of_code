use super::*;

impl Solution<DAY_06> for Solutions {
    type Input<'i> = Vec<char>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .chars()
            .collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Option<Self::Output> {
        input
            .windows(4)
            .position(uniqueness_check)
            .map(|i| i + 4)
    }

    fn part_two(input: &Self::Input<'_>) -> Option<Self::Output> {
        input
            .windows(14)
            .position(uniqueness_check)
            .map(|i| i + 14)
    }
}

fn uniqueness_check(slice: &[char]) -> bool {
    let mut seen = [false; 26];

    for character in slice {
        let i = (*character as u8 - b'a') as usize;
        match seen[i] {
            false => seen[i] = true,
            true => return false
        }
    }
    
    true
}

impl Test<DAY_06> for Solutions {
    fn expected() -> (Option<Self::Output>, Option<Self::Output>) {
        (7.into(), 19.into())
    }
}

derive_tests!(Solutions, DAY_06);