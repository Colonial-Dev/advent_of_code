#![warn(clippy::perf, clippy::style, warnings)]
#![allow(dead_code, unused_variables)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

use lib_aoc::prelude::*;

struct Solutions {}

impl Solver for Solutions {
    fn load(day: u8, testing: bool) -> String {
        use std::path::Path;

        let path = match testing {
            false => Path::new("src/inputs").join(format!("{day:02}.txt")),
            true => Path::new("src/inputs").join(format!("test_{day:02}.txt"))
        };

        std::fs::read_to_string(path).expect("Puzzle input could not be read.")
    }
}

fn main() {
    solve_through!(Solutions, 7);
}