//! # Day 14 - 
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 
//! - P2 completed @

use super::*;

impl Solution<DAY_14> for Solutions {
    type Input<'i> = ();
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {

    }
}

impl Test<DAY_14> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => unimplemented!(),
            PART_TWO => unimplemented!()
        }
    }
}

derive_tests!(Solutions, DAY_14);