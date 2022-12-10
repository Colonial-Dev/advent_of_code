//! # Day 6 - Tuning Trouble
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 00:15:01 (9222)
//! - P2 completed @ 00:25:09 (11420)
//! 
//! "It can't be that easy" (it was)
//! 
//! ## Parsing
//! This problem is dead simple; we just expand out the puzzle into its characters for parsing.
//! 
//! ## Solutions
//! Both parts merely require us to find the end of the first substring of length `n` with all-unique
//! characters. 
//! 
//! My original naive solution just cloned/sorted/deduplicated each window in the puzzle input
//! and checked its length to determine uniqueness, but I subsequently boiled that logic down to the [`uniqueness_check`]
//! function. It uses a `[bool; 26]` in a `for` loop to track if it sees the same character more than once, returning `false`
//! if it does and `true` if the loop completes without a duplicate being found. 
//! 
//! This is very efficient (I believe `O(n)`?) since:
//! - It short circuits if any non-uniqueness is detected
//! - It only requires a single pass through the slice
//! - A [`bool`] array is *super* fast to allocate + access, since it's on the stack.
//! 
//! By using:
//! ``` ignore
//! input
//!     .windows(n)
//!     .position(uniqueness_check)
//!     .map(|i| i + n)
//!     .unwrap()
//! ```
//! ... the solution can be found (assuming it exists) for any `n`.

use super::*;

impl Solution<DAY_06> for Solutions {
    type Input<'i> = Vec<char>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .chars()
            .collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        input
            .windows(4)
            .position(uniqueness_check)
            .map(|i| i + 4)
            .unwrap()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        input
            .windows(14)
            .position(uniqueness_check)
            .map(|i| i + 14)
            .unwrap()
    }
}

fn uniqueness_check(slice: &[char]) -> bool {
    let mut seen = [false; 26];

    for character in slice {
        let i = (*character as u8 - b'a') as usize;
        match seen[i] {
            false => seen[i] = true,
            true => return false
        }
    }
    
    true
}

impl Test<DAY_06> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 7,
            PART_TWO => 19
        }
    }
}

derive_tests!(Solutions, DAY_06);