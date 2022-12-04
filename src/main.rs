#![allow(dead_code)]
#![warn(clippy::perf, clippy::style, warnings)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;

macro_rules! time_exec {
    ($fn:expr) => {
        let start = std::time::Instant::now();
        $fn();
        println!("Execution took {} μs", start.elapsed().as_micros());
    };
}

fn main() {
    time_exec!(day_04::solution);
}