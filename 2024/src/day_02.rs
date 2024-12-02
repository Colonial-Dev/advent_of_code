//! # Day 2 - Red-Nosed Reports
//! 
//! Puzzle opened late.
//! - P1 completed @ 13:16:05
//! - P2 completed @ 13:29:02

use super::*;

#[derive(Debug)]
pub struct Report {
    values: Vec<isize>
}

impl Solution<DAY_02> for Solutions {
    type Input<'i> = Vec<Report>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let parse_line = |l: &str| {
            l
                .split(' ')
                .map(str::parse::<isize>)
                .map(Result::unwrap)
                .collect()
        };

        puzzle
            .lines()
            .map(parse_line)
            .map(|v| Report { values: v })
            .collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        let mut count = 0;

        for report in input {
            let values = &report.values;

            let mut differences = vec![];

            for (i, value) in values
                .iter()
                .copied()
                .enumerate()
                .take(values.len() - 1) 
            {
                differences.push(value - values[i + 1])
            }

            let smooth_enough = differences
                .iter()
                .copied()
                .map(isize::abs)
                .all(|v| v > 0 && v < 4);

            let descending = differences
                .iter()
                .all(|v| *v > 0);

            let ascending = differences
                .iter()
                .all(|v| *v < 0);


            if smooth_enough && (ascending || descending) {
                count += 1;
            }
        }

        count
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        let mut count = 0;

        let permute = |v: &Vec<_>| {
            let mut permutations = vec![v.clone()];

            for i in 0..v.len() {
                let mut v = v.clone();
                v.remove(i);
                permutations.push(v);
            }

            permutations
        };

        for report in input
            .iter()
            .map(|r| &r.values)
            .map(permute)
        {
            for permutation in report {
                let mut differences = vec![];

                for (i, value) in permutation
                    .iter()
                    .copied()
                    .enumerate()
                    .take(permutation.len() - 1) 
                {
                    differences.push(value - permutation[i + 1])
                }
    
                let smooth_enough = differences
                    .iter()
                    .copied()
                    .map(isize::abs)
                    .all(|v| v > 0 && v < 4);
    
                let descending = differences
                    .iter()
                    .all(|v| *v > 0);
    
                let ascending = differences
                    .iter()
                    .all(|v| *v < 0);
    
    
                if smooth_enough && (ascending || descending) {
                    count += 1;
                    break;
                }
            }
        }

        count
    }
}

impl Test<DAY_02> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 2,
            PART_TWO => 4,
        }
    }
}

derive_tests!(Solutions, DAY_02);