use super::*;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Multiply(usize, usize),
    Toggle(bool),
}

impl Solution<DAY_03> for Solutions {
    type Input<'i> = Vec<Instruction>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let filter_candidate = |s: &str| {
            let count = s
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == ',')
                .count();

            s.chars().nth(count).unwrap() == ')'
        };
        
        let parse_numbers = |s: &str| {
            let s: String = s
                .chars()
                .take_while(|c| *c != ')')
                .collect();

            let (l, r) = s.split_once(',').unwrap();

            Instruction::Multiply(
                l.parse::<usize>().unwrap(),
                r.parse::<usize>().unwrap()
            )
        };

        let mut instructions = vec![];

        for i in 0..puzzle.len() {
            let do_slice   = puzzle.get(i..=i + 3);
            let mul_slice  = puzzle.get(i..=i + 3);
            let dont_slice = puzzle.get(i..=i + 6);

            if [&do_slice, &mul_slice, &dont_slice].iter().copied().all(Option::is_none) {
                continue;
            }

            if let Some(s) = do_slice {
                if s == "do()" {
                    instructions.push(Instruction::Toggle(true));
                    continue;
                }
            }

            if let Some(s) = mul_slice {
                if s == "mul(" {
                    let rest = &puzzle[i + 4..];
                    if filter_candidate(rest) {
                        instructions.push(parse_numbers(rest));
                        continue;
                    }
                }
            }

            if let Some(s) = dont_slice {
                if s == "don't()" {
                    instructions.push(Instruction::Toggle(false));
                    continue;
                }
            }
        }

        instructions
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        input
            .iter() 
            .filter_map(|i| match i {
                Instruction::Multiply(a, b) => Some(a * b),
                _ => None,
            })
            .fold(0, |mut acc, v| {
                acc += v;
                acc
            })
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        let mut product = 0;
        let mut enabled = true;

        for instruction in input {
            match instruction {
                Instruction::Multiply(a, b) => if enabled { product += a * b},
                Instruction::Toggle(value) => enabled = *value,
            }
        }

        product
    }
}

impl Test<DAY_03> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 161,
            PART_TWO => 48,
        }
    }
}

derive_tests!(Solutions, DAY_03);