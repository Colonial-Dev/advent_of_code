#![warn(clippy::perf, clippy::style, warnings)]
#![allow(dead_code, unused_variables)]

mod framework;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

use framework::prelude::*;

struct Solutions {}

fn main() {
    solve!(Solutions, DAY_01, include_str!("inputs/01.txt"));
    solve!(Solutions, DAY_02, include_str!("inputs/02.txt"));
    solve!(Solutions, DAY_03, include_str!("inputs/03.txt"));
    solve!(Solutions, DAY_04, include_str!("inputs/04.txt"));
    solve!(Solutions, DAY_05, include_str!("inputs/05.txt"));
}