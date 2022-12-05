use super::*;

impl Solution<DAY_06> for Solutions {
    type Input<'i> = ();
    type Output = ();

    fn parse(puzzle: &str) -> Self::Input<'_> {
        
    }
}

impl Test<DAY_06> for Solutions {
    fn expected() -> (Option<Self::Output>, Option<Self::Output>) {
        (None, None)
    }
}

derive_tests!(Solutions, DAY_06);