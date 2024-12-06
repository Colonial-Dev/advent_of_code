use std::collections::HashSet;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Filled,
    Guard
}

impl Solution<DAY_06> for Solutions {
    type Input<'i> = Grid<Cell>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let data: Vec<_> = puzzle
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| match c {
                '.' => Cell::Empty,
                '#' => Cell::Filled,
                '^' => Cell::Guard,
                _ => unreachable!()
            })
            .collect();

        Grid::from_square(
            &data,
            puzzle.lines().count()
        )
    }

    fn part_one(input: &Grid<Cell>) -> Self::Output {
        let (mut r, mut c) = input
            .coordinates()
            .find(|(r, c)| input.get(*r, *c).unwrap() == &Cell::Guard)
            .map(|(r, c)| (r as isize, c as isize))
            .unwrap();

        let (mut rowv, mut colv) = (-1_isize, 0_isize);
        
        let mut set = HashSet::new();

        while (r >= 0 && c >= 0) && (r < input.rows as isize && c < input.cols as isize) {
            set.insert((r, c));

            let check_next = {
                let n_r = r + rowv;
                let n_c = c + colv;
                
                #[allow(clippy::match_like_matches_macro)]
                if let Some(&Cell::Filled) = input.get(n_r as usize, n_c as usize) {
                    true
                }
                else {
                    false
                }
            };
    
            if check_next {
                match (rowv, colv) {
                    (1, 0) => {
                        rowv = 0;
                        colv = -1;
                    },
                    (0, 1) => {
                        rowv = 1;
                        colv = 0;
                    },
                    (-1, 0) => {
                        rowv = 0;
                        colv = 1;
                    },
                    (0, -1) => {
                        rowv = -1;
                        colv = 0;
                    },
                    _ => unreachable!()
                }
            } else {
                r += rowv;
                c += colv;
            }
        }

        set.len()
    }

    fn part_two(input: &Grid<Cell>) -> Self::Output {
        let mut cycles = 0;

        for (a, b) in input
            .coordinates()
            .filter(|(r, c)| {
                let v = input.get(*r, *c).unwrap();
                v == &Cell::Empty
            }) 
        {
            let mut input = input.clone();

            input.set(a, b, Cell::Filled);

            let (mut r, mut c) = input
                .coordinates()
                .find(|(r, c)| input.get(*r, *c).unwrap() == &Cell::Guard)
                .map(|(r, c)| (r as isize, c as isize))
                .unwrap();

            let (mut rowv, mut colv) = (-1_isize, 0_isize);
            let mut iter_count = 0;
            
            while (r >= 0 && c >= 0) && (r < input.rows as isize && c < input.cols as isize) {
                iter_count += 1;

                // deeply fucking evil solution
                if iter_count > 10_000 {
                    cycles += 1;
                    break;
                }

                let check_next = {
                    let n_r = r + rowv;
                    let n_c = c + colv;
                    
                    #[allow(clippy::match_like_matches_macro)]
                    if let Some(&Cell::Filled) = input.get(n_r as usize, n_c as usize) {
                        true
                    }
                    else {
                        false
                    }
                };
        
                if check_next {
                    match (rowv, colv) {
                        (1, 0) => {
                            rowv = 0;
                            colv = -1;
                        },
                        (0, 1) => {
                            rowv = 1;
                            colv = 0;
                        },
                        (-1, 0) => {
                            rowv = 0;
                            colv = 1;
                        },
                        (0, -1) => {
                            rowv = -1;
                            colv = 0;
                        },
                        _ => unreachable!()
                    }
                } else {
                    r += rowv;
                    c += colv;
                }
            }
        }

        cycles
    }
}

impl Test<DAY_06> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 41,
            PART_TWO => 6,
        }
    }
}

derive_tests!(Solutions, DAY_06);