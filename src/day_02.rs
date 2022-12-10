//! # Day 2 - Rock Paper Scissors
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 00:48:58 (15168)
//! - P2 completed @ 01:37:25 (18691)
//! 
//! Another simple problem, but I wasted far too much time trying to be clever.
//! 
//! ## Parsing
//! We can't really share much work between the two parts besides splitting the puzzle
//! into its constituent lines and collecting.
//! 
//! ## Solutions
//! - Part one assumes that both characters in a game correspond to a shape, so we simply
//! map each line into two shapes, then map *those* over [`Shape::compute_outcome`] and sum to get the answer.
//! - Part two requires us to instead measure the correct approach, where the second character is the desired outcome
//! rather than what you should play. To solve this, we map each line into a [`Shape`] and a [`DesiredOutcome`], then do
//! two more maps (one to convert the desired outcome into an actual shape to play, one to compute_outcome) before again summing
//! to get the answer.

use super::*;
use Shape::*;
use DesiredOutcome::*;

impl Solution<DAY_02> for Solutions {
    type Input<'a> = Vec<&'a str>;
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle.lines().collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        input
            .iter()
            .map(|line| {
                line
                    .split_once(' ')
                    .map(|(c, r)| (Shape::from(c), Shape::from(r)))
                    .unwrap()
            })
            .map(Shape::compute_outcome)
            .sum::<u64>()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        input
            .iter()    
            .map(|line| {
                line
                    .split_once(' ')
                    .map(|(c, r)| (Shape::from(c), DesiredOutcome::from(r)))
                    .unwrap()
            })
            .map(|(challenge, outcome)| {
                (challenge, match outcome {
                    Loss => challenge.wins_against(),
                    Draw => challenge,
                    Win => challenge.loses_to()
                })
            })
            .map(Shape::compute_outcome)
            .sum::<u64>()
    }
}

impl Test<DAY_02> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 15,
            PART_TWO => 12
        }
    }
}

derive_tests!(Solutions, DAY_02);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl From<&str> for Shape {
    fn from(char: &str) -> Self {
        match char {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!()
        }
    }
}

impl Shape {    
    pub fn loses_to(&self) -> Self {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }
    }

    pub fn wins_against(&self) -> Self {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper
        }
    }

    pub fn compute_outcome(matchup: (Self, Self)) -> u64 {
        let (challenge, response) = matchup;
        
        let shape_score = match response {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        };

        let outcome_score = {
            if (challenge as u8 + 1) % 3 == response as u8 { 6 }
            else if challenge == response { 3 }
            else { 0 }
        };

        shape_score + outcome_score
    }
}

enum DesiredOutcome {
    Loss,
    Draw,
    Win
}

impl From<&str> for DesiredOutcome {
    fn from(char: &str) -> Self {
        match char {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!()
        }
    }
}