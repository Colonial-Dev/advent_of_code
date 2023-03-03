#![warn(clippy::perf, clippy::style, warnings)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;

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
    solve_through!(Solutions, 18);
}
