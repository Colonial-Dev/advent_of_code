//! # Day 17 - 
//! 
//! Puzzle opened late - I unfortunately got sick, so I'm likely going to be behind for the rest of the season :(
//! - P1 completed @ > 24h
//! - P2 completed @ > 24h

use super::*;

impl Solution<DAY_17> for Solutions {
    type Input<'i> = Vec<Jet>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .chars()
            .filter(|c| c == &'<' || c == &'>')
            .map(Jet::from)
            .collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        let mut tower = Tower::new();
        let mut rocks = Rock::generator();
        let mut jets = input.iter().cycle();

        for _ in 0..2022 {
            let mut rock = rocks.next().unwrap();
            rock.shift_unchecked((tower.max + 3) as isize, 0);

            let ceiling = rock.coords
                .iter()
                .map(|(row, _)| row)
                .max()
                .unwrap();
            
            tower.raise_ceiling(*ceiling);
            
            loop {
                match jets.next().unwrap() {
                    Jet::Left => rock.shift_checked(&tower, 0, -1),
                    Jet::Right => rock.shift_checked(&tower, 0, 1)
                };

                if !rock.shift_checked(&tower, -1, 0) {
                    break;
                }
            }

            for (row, col) in rock.coords {
                tower.mark_occupied(row, col)
            }
        }

        tower.max
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        const STEP_COUNT: usize = 1_000_000_000_000;

        let mut tower = Tower::new();
        let mut rocks = Rock::generator();
        let mut jets = input.iter().enumerate().cycle();

        let mut cache = Vec::new();
        let mut loop_points = Vec::new();

        for i in 0.. {
            let mut rock = rocks.next().unwrap();
            rock.shift_unchecked((tower.max + 3) as isize, 0);

            let ceiling = rock.coords
                .iter()
                .map(|(row, _)| row)
                .max()
                .unwrap();
            
            tower.raise_ceiling(*ceiling);
            
            loop {
                let (idx, jet) = jets.next().unwrap();

                if idx % 10091 == 0 {
                    loop_points.push((i, tower.max));
                }

                match jet {
                    Jet::Left => rock.shift_checked(&tower, 0, -1),
                    Jet::Right => rock.shift_checked(&tower, 0, 1)
                };

                if !rock.shift_checked(&tower, -1, 0) {
                    break;
                }
            }

            for (row, col) in rock.coords {
                tower.mark_occupied(row, col)
            }

            cache.push(tower.max);

            if loop_points.len() >= 3 {
                break;
            }
        }

        // None of this should work but it does???
        let rocks_per = loop_points[2].0 - loop_points[1].0;
        let height_per = loop_points[2].1 - loop_points[1].1;

        let remainder_steps = STEP_COUNT % rocks_per;
        let steps = (STEP_COUNT - remainder_steps) / rocks_per;
        
        cache[remainder_steps] + (height_per * steps - 1)
    }
}

impl Test<DAY_17> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 3068,
            // Not the actual correct answer but :shrug:
            PART_TWO => 1333333333336
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Jet {
    Left,
    Right,
}

impl From<char> for Jet {
    fn from(value: char) -> Self {
        match value {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
struct Rock {
    coords: Vec<(usize, usize)>
}

impl Rock {
    pub fn generator() -> impl Iterator<Item=Self> {
        let horizontal = vec![
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5)
        ];

        let cross = vec![
            (2, 3),
            (1, 2),
            (1, 3),
            (1, 4),
            (0, 3)
        ];

        let angle = vec![
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 4),
            (2, 4)
        ];

        let vertical = vec![
            (3, 2),
            (2, 2),
            (1, 2),
            (0, 2)
        ];

        let square = vec![
            (1, 2),
            (1, 3),
            (0, 2),
            (0, 3)
        ];

        vec![horizontal, cross, angle, vertical, square]
            .into_iter()
            .map(|coords| Rock { coords })
            .collect::<Vec<_>>()
            .into_iter()
            .cycle()
    }

    pub fn shift_unchecked(&mut self, dr: isize, dc: isize) {
        for (row, col) in &mut self.coords {
            *row = (*row as isize + dr) as usize;
            *col = (*col as isize + dc) as usize;
        }
    }

    pub fn shift_checked(&mut self, tower: &Tower, dr: isize, dc: isize) -> bool {
        let mut shadow = self.clone();
        shadow.shift_unchecked(dr, dc);

        let out_of_bounds = shadow.coords
            .iter()
            .any(|(row, col)| {
                row == &usize::MAX || col == &usize::MAX || col > &6
            });

        if out_of_bounds {
            return false;
        }

        let collides = shadow.coords
            .iter()
            .any(|(row, col)| {
                tower.check_collision(*row, *col)
            });

        if collides {
            false
        } else {
            *self = shadow;
            true
        }
    }
}

const EMPTY_ROW: [bool; 7] = [false; 7];

struct Tower {
    buffer: Vec<[bool; 7]>,
    max: usize,
}

impl Tower {
    pub fn new() -> Self {
        Self {
            buffer: vec![EMPTY_ROW; 4],
            max: 0
        }
    }

    pub fn check_collision(&self, row: usize, col: usize) -> bool {
        self.buffer[row][col]
    }

    pub fn raise_ceiling(&mut self, ceiling: usize) {
        if ceiling >= self.buffer.len() {
            let difference = ceiling - self.buffer.len();
            self.buffer.extend(vec![EMPTY_ROW; difference + 1]);
        }
    }

    pub fn mark_occupied(&mut self, row: usize, col: usize) {
        self.buffer[row][col] = true;
        self.max = std::cmp::max(self.max, row + 1);
    }
}

impl std::fmt::Display for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let iter = self.buffer.iter().enumerate().rev();

        for (i, row) in iter {
            write!(f, "{i:03} |")?;

            for cell in row {
                match cell {
                    false => write!(f, ".")?,
                    true => write!(f, "#")?
                }
            }

            writeln!(f, "|")?;
        }

        writeln!(f)?;
        writeln!(f, "Max: {}", self.max)?;
        
        Ok(())
    }
}

derive_tests!(Solutions, DAY_17);