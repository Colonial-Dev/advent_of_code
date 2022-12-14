//! # Day 14 - Regolith Reservoir
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 01:13:55 (5449)
//! - P2 completed @ 01:18:53 (4655)
//! 
//! This was an eminently brute-forceable puzzle, but also a very optimizable one! It's possible to do much better than
//! simulating each grain of sand, one step at a time, especially when it comes to part two.
//! 
//! ## Parsing
//! Today's parsing was pretty trivial; just a few splits to break the input down into parseable number pairs, followed
//! by some Cartesian interpolation. I *did* manage to somehow botch the first and second iterations of my interpolation
//! function, mainly because I was worrying too much about preserving direction when it didn't matter, but I ironed out
//! those kinks pretty quickly and stuffed the resulting interpolated "rock points" into a [`HashSet`] (which itself was placed in a [`Scan`].)
//! 
//! Later, I also added an optimization when it came to light that the input contained lots of duplicate lines; folding through a [`HashSet`] to remove
//! duplicates before the actual parsing shaved parsing runtime from about 265 micros to 65 micros.
//! 
//! ## Solutions
//! My initial solutions took the naive approach of simulating each grain of sand from release until settling, which was (obviously)
//! *super* slow - taking ~160 millis for part two, even in release mode.
//! 
//! However, after seeing a visualization of someone else's P2, I realized that it looked awfully similar to a *breadth-first search* -
//! and this was indeed a valid and powerful optimization over step-by-step simulation! Although BFS is typically used to find a single "node",
//! the rules for edge traversal and termination can be tweaked so that it just explores until all possible avenues (in this case, sand positions)
//! are exhausted; once that's done, the number of nodes explored is precisely equal to the number of sand blocks that can possibly fall.
//! 
//! Unfortunately, I don't think a BFS is applicable to part one. Someone did tell me that they got a DFS to work instead, and I *think* I was able
//! to replicate that approach - it works by "simulating" the falling as normal, pushing each open cell it encounters onto a stack until it can't move anymore;
//! then it registers the final cell as a settled sand block, pops off the stack, and repeats from there. I'm not 100% sure if this qualifies as a DFS, but it certainly
//! looks kinda like one. Also, it yielded a ~2000% speed boost over the naive approach, so...

use std::{
    collections::{HashSet, VecDeque},
    ops::Range
};

use super::*;

impl Solution<DAY_14> for Solutions {
    type Input<'i> = Scan;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let parse_line = |line: &&str| {
            line
                .split("->")
                .map(|pair| {
                    pair
                        .trim()
                        .split_once(',')
                        .unwrap()
                })
                .map(|(l, r)| {
                    (
                        l.parse::<u64>().unwrap(),
                        r.parse::<u64>().unwrap()
                    )
                })
                .collect::<Vec<_>>()
        };

        let interpolate_line = |line: Vec<(u64, u64)>| {
            line
                .windows(2)
                .flat_map(|w| cartesian_interpolate(&w[0], &w[1]))
                .collect::<Vec<_>>()
        };
        
        puzzle
            .lines()
            .fold(HashSet::new(), |mut acc, line| {
                acc.insert(line);
                acc
            })
            .iter()
            .map(parse_line)
            .map(interpolate_line)
            .fold(HashSet::new(), |mut acc, coord| {
                acc.extend(coord);
                acc
            })
            .into()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        let mut stack = Vec::new();
        let mut filled = input.filled.clone();
        let mut position = START_POSITION;
        let mut settled = 0;

        loop {
            if position.1 == input.y_max {
                break;
            }

            let new_cells = possible_positions(position);

            if !filled.contains(&new_cells[0]) {
                stack.push(position);
                position = new_cells[0];
            }
            else if position.0 == 0 {
                break;
            }
            else if !filled.contains(&new_cells[1]) {
                stack.push(position);
                position = new_cells[1];
            }
            else if position.0 + 1 == input.x_bounds.end {
                break;
            }
            else if !filled.contains(&new_cells[2]) {
                stack.push(position);
                position = new_cells[2];
            }
            else {
                settled += 1;
                filled.insert(position);

                match stack.pop() {
                    Some(pos) => position = pos,
                    None => break
                };
            }
        }

        settled
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {        
        let mut queue = VecDeque::new();
        let mut explored = HashSet::new();

        explored.insert(START_POSITION);
        queue.push_back(START_POSITION);

        while let Some(position) = queue.pop_front() {            
            if position.1 > input.y_max {
                continue;
            }
            
            possible_positions(position)
                .into_iter()
                .filter(|pos| !input.filled.contains(pos))
                .for_each(|pos| {
                    if explored.insert(pos) {
                        queue.push_back(pos)
                    }
                })
        }

        explored.len()
    }
}

impl Test<DAY_14> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 24,
            PART_TWO => 93
        }
    }
}

fn cartesian_interpolate(a: &(u64, u64), b: &(u64, u64)) -> impl Iterator<Item=(u64, u64)> {
    let delta_is_y = a.0 == b.0;

    let (delta_a, delta_b) = match delta_is_y {
        false => (a.0, b.0),
        true => (a.1, b.1)
    };

    let stable = match delta_is_y {
        false => a.1,
        true => a.0
    };

    let min = std::cmp::min(delta_a, delta_b);
    let max = std::cmp::max(delta_a, delta_b);
    let range = min..=max;

    range
        .into_iter()
        .map(move |point| match delta_is_y {
            false => (point, stable),
            true => (stable, point)
        })
}

const START_POSITION: (u64, u64) = (500, 0);

fn possible_positions(start: (u64, u64)) -> [(u64, u64); 3] {
    [
        (start.0, start.1 + 1),
        (start.0 - 1, start.1 + 1),
        (start.0 + 1, start.1 + 1)
    ]
}

#[derive(Debug)]
pub struct Scan {
    filled: HashSet<(u64, u64)>,
    x_bounds: Range<u64>,
    y_max: u64,
}

impl Scan {
    pub fn new(filled: HashSet<(u64, u64)>) -> Self {
        let (x_min, x_max) = filled
            .iter()
            .map(|(x, _)| x)
            .fold((500, 500), |mut acc, x| {
                acc.0 = u64::min(*x, acc.0);
                acc.1 = u64::max(*x, acc.1);
                acc
            });

        let y_max = *filled
            .iter()
            .map(|(_, y)| y)
            .max()
            .unwrap();

        Self {
            filled,
            x_bounds: x_min..x_max,
            y_max
        }
    }
}

impl From<HashSet<(u64, u64)>> for Scan {
    fn from(filled: HashSet<(u64, u64)>) -> Self {
        Self::new(filled)
    }
}

derive_tests!(Solutions, DAY_14);