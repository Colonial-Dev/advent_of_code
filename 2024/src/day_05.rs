use std::collections::{HashMap, HashSet};

use super::*;

#[derive(Clone)]
pub struct Input {
    rules: HashMap<usize, (HashSet<usize>, HashSet<usize>)>,
    pages: Vec<Vec<usize>>,
}

impl Solution<DAY_05> for Solutions {
    type Input<'i> = Input;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let (rules, pages) = puzzle
            .split_once("\n\n")
            .unwrap();

        let mut r = vec![];
        let mut p = vec![];

        for rule in rules.lines() {
            let (a, b) = rule
                .split_once("|")
                .unwrap();

            r.push(
                (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
            )
        }

        for numbers in pages.lines() {
            let v: Vec<_> = numbers
                .split(",")
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect();

            p.push(v);
        }

        let mut map = HashMap::new();

        let numbers: HashSet<_> = r
            .iter()
            .copied()
            .flat_map(|(a, b)| [a, b])
            .collect();

        for n in numbers {
            let before: HashSet<_> = r
                .iter()
                .filter(|(_, r)| *r == n)
                .map(|(l, _)| l)
                .copied()
                .collect();

            let after: HashSet<_> = r
                .iter()
                .filter(|(l, _)| *l == n)
                .map(|(_, r)| r)
                .copied()
                .collect();

            map.insert(
                n,
                (before, after)
            );
        }

        Input {
            rules: map,
            pages: p,
        }
    }

    fn part_one(input: &Input) -> Self::Output {
        input
            .pages
            .iter()
            .filter(|v| {
                for (i, n) in v.iter().enumerate() {
                    if let Some(rules) = input.rules.get(n) {
                        if v[..i].iter().any(|b| rules.1.contains(b) ) {
                            return false;
                        }
                    }
                }

                true
            })
            .map(|v| v[v.len() / 2])
            .sum()
    }

    fn part_two(input: &Input) -> Self::Output {
        #[allow(clippy::manual_inspect)]
        input
            .clone()
            .pages
            .iter_mut()
            .filter(|v| {
                for (i, n) in v.iter().enumerate() {
                    if let Some(rules) = input.rules.get(n) {
                        if (0..i).map(|i| v[i]).any(|x| rules.1.contains(&x)) {
                            return true;
                        }
                    }
                }

                false
            })
            .map(|v| {
                v.sort_by(|a, b| {
                    let (r_a, _) = input.rules.get(a).unwrap();
                    let (_, r_b) = input.rules.get(b).unwrap();
    
                    if r_a.contains(b) { std::cmp::Ordering::Less }
                    else if r_b.contains(a) { std::cmp::Ordering::Greater }
                    else { std::cmp::Ordering::Equal }
                });

                v
            })
            .map(|v| v[v.len() / 2])
            .sum()
    }
}

impl Test<DAY_05> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 143,
            PART_TWO => 123,
        }
    }
}

derive_tests!(Solutions, DAY_05);