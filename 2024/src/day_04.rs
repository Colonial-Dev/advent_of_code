use super::*;

#[derive(Debug, Clone)]
pub struct Grid<T: Clone> {
    data: Box<[T]>,
    rows: usize,
    cols: usize,
}

impl<T: Clone> Grid<T> {
    pub fn from_square(data: &[T], dim: usize) -> Self {
        Self {
            data: data.to_vec().into_boxed_slice(),
            rows: dim,
            cols: dim,
        }
    }

    pub fn row(&self, index: usize) -> impl Iterator<Item = &T> {
        self
            .data
            .iter()
            .skip(index * self.cols)
            .take(self.cols)
    }

    pub fn col(&self, index: usize) -> impl Iterator<Item = &T> {
        self
            .data
            .iter()
            .skip(index)
            .step_by(self.cols)
            .take(self.rows)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        
        self.data.get(self.rows * row + col)
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

impl Solution<DAY_04> for Solutions {
    type Input<'i> = Grid<char>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let data: Vec<_> = puzzle
            .lines()
            .flat_map(str::chars)
            .collect();

        Grid::from_square(
            &data,
            puzzle.lines().count()
        )
    }

    fn part_one(input: &Grid<char>) -> Self::Output {
        let (rows, _) = input.dimensions();
        let mut count = 0;

        for i in 0..rows {
            let r: String = input.row(i).collect();
            let c: String = input.col(i).collect();

            count += r.matches("XMAS").count();
            count += r.matches("SAMX").count();
            count += c.matches("XMAS").count();
            count += c.matches("SAMX").count();
        }

        for k in 0..rows * 2 {
            let mut s = String::new();

            for j in 0..=k {
                let i = k.saturating_sub(j);

                if let Some(c) = input.get(i, j) {
                    s.push(*c);
                }
            }

            count += s.matches("XMAS").count();
            count += s.matches("SAMX").count();
        }

        let mut rev = input.clone();

        for i in 0..rev.rows {
            rev.data[rev.rows * i..(rev.rows * i) + rev.cols].reverse();
        }

        for k in 0..rows * 2 {
            let mut s = String::new();

            for j in 0..=k {
                let i = k.saturating_sub(j);

                if let Some(c) = rev.get(i, j) {
                    s.push(*c);
                }
            }

            count += s.matches("XMAS").count();
            count += s.matches("SAMX").count();
        }
        
        count
    }

    fn part_two(input: &Grid<char>) -> Self::Output {
        let mut count = 0;

        for r in 0..input.rows - 2 {
            for c in 0..input.cols - 2 {
                let a = [
                    (r, c),
                    (r + 1, c + 1),
                    (r + 2, c + 2),
                ];

                let b = [
                    (r, c + 2),
                    (r + 1, c + 1),
                    (r + 2, c),
                ];
                
                let stringify = |coords: [(usize, usize); 3]| -> String {
                    coords
                        .into_iter()
                        .map(|(r, c)| input.get(r, c) )
                        .map(Option::unwrap)
                        .copied()
                        .collect()             
                };

                let a = stringify(a);
                let b = stringify(b);

                if (a == "MAS" || a == "SAM") && (b == "MAS"  || b == "SAM") {
                    count += 1;
                }
            }
        }

        count
    }
}

impl Test<DAY_04> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 18,
            PART_TWO => 9,
        }
    }
}

derive_tests!(Solutions, DAY_04);