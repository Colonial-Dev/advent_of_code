//! # Day 9 - Rope Bridge
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 00:51:10 (7331)
//! - P2 completed @ 02:42:44 (10526)
//! 
//! Fun fact - `2577` and `2557` are apparently considered equivalent by my brain!
//! 
//! ... I misread my program giving me a correct answer (the latter) as the former.
//! This took me an hour to figure out.
//! ***AAAAAAAAAAAAAAAAA-***
//! 
//! ## Parsing
//! My initial approach parsed each line into a `Movement` struct, containing a
//! `count` and `direction` field. 
//! 
//! However, once I understood the entire problem, I realized it would be better to simply 
//! eliminate the `Movement` struct altogether, flattening each instruction from `DIRECTION COUNT` into
//! `DIRECTION` repeated `COUNT` times. 
//! 
//! This means I don't have to worry about breaking up each movement command into sub-steps
//! when solving the problem; that logic has been hoisted to parse time.
//! 
//! ## Solutions
//! My initial solution was (like yesterday) a hot mess of `for` loops that (predictably) blew apart
//! at the seams when exposed to part two. I scraped together what logic I could salvage and wrote
//! a [`Rope`] type that manages an internal buffer of knot coordinates and a [`HashSet`] tracking all the
//! coordinates visited by the rope's tail knot.
//! 
//! Both solutions are identical, apart from the number of knots specified when constructing their [`Rope`].
//! They loop over each direction from the input, first calling [`Rope::move_head`] (which is a simple `match`
//! statement that increments/decrements the appropriate head coordinate), then [`Rope::update_positions`] to
//! realign all the trailing knots. The position update algorithm goes roughly like this:
//! 1. Iterate over all the indices in the range `1..knot_buffer.len()`.
//! 2. Obtain the knots at `index - 1` (the current "head") and `index` (the current "tail".)
//! 3. Compute the x-axis and y-axis difference between the head and tail.
//! 4. If the absolute value of the x/y differences is both `<= 1`, we don't need to update anything and
//! continue iteration.
//! 5. Otherwise: 
//!     - If the absolute value of the x/y differences is both `!= 0`, we increment the tail's
//! x and y coordinates by the sign number of the respective difference.
//!     - Else if the absolute value of the x difference is `>= 2`, increment the tail's x
//! coordinate by the sign number of the x difference.
//!     - Ditto for the y difference being `>= 2`.
//! 6. Once iteration is complete, update the tracking [`HashSet`] with the current coordinates of the
//! rope tail.
//! 
//! Once all the movement instructions are exhausted, the answer for both parts can be obtained using
//! `rope.hash_set.len()`.

use std::collections::HashSet;

use super::*;
use Direction::*;

impl Solution<DAY_09> for Solutions {
    type Input<'i> = Vec<Direction>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .lines()
            .flat_map(|line| {
                let (dir, ct) = line.split_once(' ').unwrap();
                let ct = ct.parse::<i32>().unwrap();
                (0..ct).map(|_| dir.into())
            })
            .collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Option<Self::Output> {
        let mut rope = Rope::new(2);

        for direction in input {
            rope.move_head(direction);
            rope.update_positions();
        }

        rope.tail_tracker.len().into()
    }

    fn part_two(input: &Self::Input<'_>) -> Option<Self::Output> {
        let mut rope = Rope::new(10);
        
        for direction in input {
            rope.move_head(direction);
            rope.update_positions();
        }

        rope.tail_tracker.len().into()
    }
}

impl Test<DAY_09> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 13,
            PART_TWO => 1
        }
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<[i32; 2]>,
    tail_tracker: HashSet<[i32; 2]>
}

impl Rope {
    pub fn new(len: usize) -> Self {
        let knots = vec![[0; 2]; len];
        let mut tail_tracker = HashSet::new();
        tail_tracker.insert([0, 0]);
        Self { knots, tail_tracker }
    }

    pub fn move_head(&mut self, direction: &Direction) {
        match direction {
            Left => self.knots[0][0] -= 1,
            Right => self.knots[0][0] += 1,
            Up => self.knots[0][1] += 1,
            Down => self.knots[0][1] -= 1
        }
    }

    pub fn update_positions(&mut self) {
        let pre_step_tail = *self.knots.last().unwrap();

        for i in 1..self.knots.len() {
            let head = self.knots[i - 1];
            let tail = &mut self.knots[i];

            let x_diff = head[0] - tail[0];
            let y_diff = head[1] - tail[1];

            // We don't need to change anything if we're still
            // adjacent to our head knot.
            if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
                // Optimization: if a knot doesn't move, we know
                // none of the later ones will either. This can save
                // 300+ μs (!) per simulation.
                break;
            }

            // If we're on a different row AND column from our
            // head knot, then we need to change both the X and Y
            // to catch up.
            if x_diff.abs() != 0 && y_diff.abs() != 0 {
                tail[0] += x_diff.signum();
                tail[1] += y_diff.signum();
            }
            // Otherwise, we only need to move on one axis to
            // catch up.
            else if x_diff.abs() >= 2 {
                tail[0] += x_diff.signum();
            }
            else if y_diff.abs() >= 2 {
                tail[1] += y_diff.signum();
            }
        }
        
        // Optimization: skip the overhead of inserting into the hash set
        // if the tail hasn't actually changed position over the course of the step.
        let post_step_tail = *self.knots.last().unwrap();
        if pre_step_tail != post_step_tail {
            self.tail_tracker.insert(post_step_tail);
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl From<&str> for Direction {
    fn from(str: &str) -> Self {
        match str {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => unreachable!()
        }
    }
}

derive_tests!(Solutions, DAY_09);