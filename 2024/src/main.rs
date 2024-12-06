#![warn(clippy::perf, clippy::style, warnings)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

use lib_aoc::prelude::*;

#[derive(Debug, Clone)]
pub struct Grid<T: Clone> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Clone> Grid<T> {
    pub fn from_square(data: &[T], dim: usize) -> Self {
        Self {
            data: data.to_vec(),
            rows: dim,
            cols: dim,
        }
    }

    pub fn row(&self, index: usize) -> impl Iterator<Item = &T> {
        self
            .data
            .iter()
            .skip(index * self.cols)
            .take(self.cols)
    }

    pub fn col(&self, index: usize) -> impl Iterator<Item = &T> {
        self
            .data
            .iter()
            .skip(index)
            .step_by(self.cols)
            .take(self.rows)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        
        self.data.get(self.rows * row + col)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T){
        if row >= self.rows || col >= self.cols {
            return;
        }
        
        self.data[self.rows * row + col] = value;
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn coordinates(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.rows)
            .flat_map(|r| {
                (0..self.cols)
                    .map(move |c| (r, c))
            })
    }
}

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
    solve_through!(Solutions, 6);
}
