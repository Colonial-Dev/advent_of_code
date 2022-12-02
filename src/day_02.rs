use Shape::*;
use DesiredOutcome::*;

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

pub fn solution() {
    let matchups = include_str!("inputs/02.txt").lines();
    
    let initial_score = matchups
        .clone()
        .map(|line| {
            line
                .split_once(' ')
                .map(|(c, r)| (Shape::from(c), Shape::from(r)))
                .unwrap()
        })
        .map(Shape::compute_outcome)
        .sum::<u64>();

    let correct_score = matchups
        .clone()
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
        .sum::<u64>();
    

    println!("The predicted total of all scores using the initial method is {initial_score}.");
    println!("The predicted total of all scores using the correct method is {correct_score}.");
}