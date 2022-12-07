//! # Day 5 - Supply Stacks
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 01:11:38 (12976)
//! - P2 completed @ 01:13:06 (11725)
//! 
//! "Oh God, it's actually just parsing hell." - Me, after reading the problem
//! 
//! ## Parsing
//! This is what took most of my time. Parsing the commands is easy (see [`Command::from_str`]), but turning the
//! little stack drawing into a well-typed data structure was... not.
//! 
//! My solution ended up using [`Iterator::skip`] and [`Iterator::step_by`] on the characters of each line to
//! pull out each row of the stack, empty spaces included, then rotating them into a [`Vec<Vec<char>>`] via [`Iterator::for_each`] 
//! and using [`Vec::retain`] to filter out the empties.
//! 
//! ## Solutions
//! The two parts have almost the exact same behavior, so I was able to hoist it into the [`simulate_craning`] function.
//! All it does is iterate over the movement commands, popping (or, more accurately, draining) the correct number of crates
//! from the source stack and pushing them onto the front of the destination stack. 
//! 
//! The `reversed` parameter is used for part two, which pushes the crates onto the destination stack in reverse order 
//! (to simulate the behavior of their order remaining the same.)
//! 
//! Once all commands have been exhausted, the stacks are iterated over and mapped to produce a string of the top crate in each stack.

use super::*;

impl Solution<DAY_05> for Solutions {
    type Input<'a> = (Vec<Stack>, Vec<Command>);
    type Output = String;

    fn parse(puzzle: &'_ str) -> Self::Input<'_> {
        let (drawing, commands) = puzzle
            .split_once("\n\n")
            .unwrap();

        let mut stacks = vec![Stack::new(); 9];
        
        drawing
            .lines()
            .map(|line| line
                .chars()
                .skip(1)
                .step_by(4)
                .collect::<Vec<_>>()
            )
            .for_each(|row| {
                for i in 0..row.len() {
                    stacks[i].push(row[i])
                }
            });
        
        for stack in &mut stacks {
            stack.retain(|char| *char != ' ')
        }

        let commands: Vec<_> = commands
            .lines()
            .map(Command::from_str)
            .collect();
        
        (stacks, commands)
    }

    fn part_one(input: &Self::Input<'_>) -> Option<Self::Output> {
        let (stacks, commands) = input;
        let mut stacks = stacks.to_owned();
        simulate_craning(commands, &mut stacks, false).into()
    }

    fn part_two(input: &Self::Input<'_>) -> Option<Self::Output> {
        let (stacks, commands) = input;
        let mut stacks = stacks.to_owned();
        simulate_craning(commands, &mut stacks, true).into()
    }
}

impl Test<DAY_05> for Solutions {
    fn expected() -> (Option<Self::Output>, Option<Self::Output>) {
        ("CMZ".to_string().into(), "MCD".to_string().into())
    }
}

derive_tests!(Solutions, DAY_05);

fn simulate_craning(commands: &[Command], stacks: &mut [Stack], reversed: bool) -> String {
    for cmd in commands {
        let source = &mut stacks[cmd.source];
        let removed = source
            .drain(0..std::cmp::min(cmd.count, source.len()))
            .collect::<Vec<_>>()
            .into_iter();
        
        match reversed {
            true => removed.rev().for_each(|char| stacks[cmd.dest].insert(0, char)),
            false => removed.for_each(|char| stacks[cmd.dest].insert(0, char))
        }
    }

    stacks
        .iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack[0])
        .collect::<String>()
}

type Stack = Vec<char>;

#[derive(Debug, Clone)]
pub struct Command {
    count: usize,
    source: usize,
    dest: usize
}

impl Command {
    pub fn from_str(s: &str) -> Self {
        let values: Vec<_> = s
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect();
        
        Self {
            count: values[0],
            source: values[1] - 1,
            dest: values[2] - 1
        }
    }
}