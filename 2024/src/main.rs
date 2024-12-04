#![warn(clippy::perf, clippy::style, warnings)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;

use lib_aoc::prelude::*;

struct Solutions {}

impl Solver for Solutions {
    fn load(day: u8) -> String {
        std::fs::read_to_string(format!("src/inputs/{day:02}.txt"))
            .expect("Puzzle input could not be read.")
    }

    fn load_test(day: u8, part: bool) -> String {
        let puzzle = std::fs::read_to_string(format!("src/inputs/test_{day:02}.txt"))
            .expect("Puzzle input could not be read.");
        
        if let Some(split) = puzzle.split_once("--- PART END ---") {
            match part {
                PART_ONE => split.0.trim().to_owned(),
                PART_TWO => split.1.trim().to_owned()
            }
        }
        else {
            puzzle
        }
    }
}

fn main() {
    solve_through!(Solutions, 4);
}
