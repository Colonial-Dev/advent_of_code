
use std::{fmt::Debug, time::{Instant, Duration}};

use colored::{Colorize, ColoredString};

mod days {
    pub const DAY_01: u8 = 1;
    pub const DAY_02: u8 = 2;
    pub const DAY_03: u8 = 3;
    pub const DAY_04: u8 = 4;
    pub const DAY_05: u8 = 5;
    pub const DAY_06: u8 = 6;
    pub const DAY_07: u8 = 7;
    pub const DAY_08: u8 = 8;
    pub const DAY_09: u8 = 9;
    pub const DAY_10: u8 = 10;
    pub const DAY_11: u8 = 11;
    pub const DAY_12: u8 = 12;
    pub const DAY_13: u8 = 13;
    pub const DAY_14: u8 = 14;
    pub const DAY_15: u8 = 15;
    pub const DAY_16: u8 = 16;
    pub const DAY_17: u8 = 17;
    pub const DAY_18: u8 = 18;
    pub const DAY_19: u8 = 19;
    pub const DAY_20: u8 = 20;
    pub const DAY_21: u8 = 21;
    pub const DAY_22: u8 = 22;
    pub const DAY_23: u8 = 23;
    pub const DAY_24: u8 = 24;
    pub const DAY_25: u8 = 25;
}

pub mod prelude {
    pub use super::*;
    pub use days::*;
}

pub trait Solution<'a, const DAY: u8> {
    type Input;
    type Output: Debug;

    fn run(puzzle: &'a str) {
        let mut clock = Clock::new();

        let solution_input = Self::parse(puzzle);
        clock.mark("Parsing");

        let part_one_ans = Self::part_one(&solution_input);
        clock.mark("Part 1");

        let part_two_ans = Self::part_two(&solution_input);
        clock.mark("Part 2");
        clock.mark_total("Total");

        println!("\n--- DAY {} ---", DAY.to_string().bright_cyan().bold());
        println!("{}: {}", "Part 1".bold(), format_answer(part_one_ans));
        println!("{}: {}", "Part 2".bold(), format_answer(part_two_ans));

        if cfg!(debug_assertions) {
            println!("\n--- BENCH {} ---\n{clock}", "(DEBUG)".yellow().bold())
        } else {
            println!("\n--- BENCH {} ---\n{clock}", "(RELEASE)".green().bold())
        }
    }

    fn parse(puzzle: &'a str) -> Self::Input;

    fn part_one(input: &Self::Input) -> Option<Self::Output> {
        None
    }

    fn part_two(input: &Self::Input) -> Option<Self::Output> {
        None
    }
}

#[macro_export]
macro_rules! solve {
    ($sols:ty, $day:expr, $input:expr) => {
        <$sols as Solution<$day>>::run($input);
    };
}

fn format_answer(ans: Option<impl Debug>) -> ColoredString {
    match ans {
        Some(answer) => format!("{answer:?}").green(),
        None => "unimplemented".red()
    }
}

struct Clock {
    start: Instant,
    previous: Instant,
    buffer: Vec<(&'static str, Duration)>
}

impl Clock {
    pub fn new() -> Self {
        let buffer = Vec::with_capacity(4);
        let init = Instant::now();
        Self { 
            start: init,
            previous: init,
            buffer
        }
    }

    pub fn mark(&mut self, label: &'static str) {
        self.buffer.push((
            label,
            self.previous.elapsed()
        ));
        self.previous = Instant::now();
    }

    pub fn mark_total(&mut self, label: &'static str) {
        self.buffer.push((
            label,
            self.start.elapsed()
        ));
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = &self.buffer
            .iter()
            .map(|(label, time)| format!(
                "{}: {} μs / {} ns\n",
                label.bold(),
                time.as_micros().to_string().green(),
                time.as_nanos().to_string().green()
            ))
            .collect::<String>();
        
        write!(f, "{}", output.trim())?;
        Ok(())
    }
}