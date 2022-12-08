#![warn(clippy::perf, clippy::style, warnings)]
#![allow(dead_code, unused_variables)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

use lib_aoc::prelude::*;

struct Solutions {}

impl Solver for Solutions {
    fn load(day: u8) -> String {
        std::fs::read_to_string(format!("src/inputs/{day:02}.txt"))
            .expect("Puzzle input could not be read.")
    }

    fn load_test(day: u8) -> String {
        std::fs::read_to_string(format!("src/inputs/test_{day:02}.txt"))
            .expect("Puzzle input could not be read.")
    }
}

fn main() {
    solve_through!(Solutions, 8);
}