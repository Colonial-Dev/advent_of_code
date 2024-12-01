//! # Day 1 - 
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 
//! - P2 completed @

use std::collections::HashMap;

use super::*;

impl Solution<DAY_01> for Solutions {
    type Input<'i> = (Vec<usize>, Vec<usize>);
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let mut l = vec![];
        let mut r = vec![];

        for line in puzzle.lines() {
            let (left, right) = line
                .split_once("   ")
                .unwrap();
            
            l.push(left);
            r.push(right);
        }

        let usize_parse = |v: Vec<_>| -> Vec<_> {
            v
                .into_iter()
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect()
        };

        let mut l = usize_parse(l);
        let mut r = usize_parse(r);

        l.sort_unstable();
        r.sort_unstable();
        
        (l, r)
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        let (l, r) = input;
        
        l
            .iter()
            .zip(r)
            .map(|(l, r)| l.abs_diff(*r) )
            .sum()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        let (l, r) = input;

        let counts = r.iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

        l
            .iter()
            .map(|n| {
                counts.get(n).unwrap_or(&0) * n
            })
            .sum::<usize>()
    }
}

impl Test<DAY_01> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 11,
            PART_TWO => 31,
        }
    }
}

derive_tests!(Solutions, DAY_01);