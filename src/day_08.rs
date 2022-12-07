//! # Day 8 - 
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 
//! - P2 completed @ 

use super::*;

impl Solution<DAY_08> for Solutions {
    type Input<'i> = ();
    type Output = ();

    fn parse(puzzle: &str) -> Self::Input<'_> {

    }

}

impl Test<DAY_08> for Solutions {
    fn expected() -> (Option<Self::Output>, Option<Self::Output>) {
        (None, None)
    }
}

derive_tests!(Solutions, DAY_08);