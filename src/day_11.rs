//! # Day 11 - Monkey in the Middle
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 01:22:42 (7727)
//! - P2 completed @ 02:13:16 (6474)
//! 
//! I'm just gonna say it - this was a lame puzzle. It was 90% parsing boilerplate and 10% skill checking your
//! knowledge of modular arithmetic; it would have taken me *much* longer to figure out part 2 if I hadn't gotten some hints
//! from other Rustaceans.
//! 
//! ## Parsing
//! I was seriously tempted to just hard-code my input data, but that's honestly for tryhard sweats who are chasing the
//! global leaderboard. So I spent the first 30-45 minutes parsing the data into well-formed [`Monkey`] structs. What fun!
//! 
//! ## Solutions
//! After that, part one was relatively simple - I briefly butted heads with the borrow checker when trying 
//! to iterate over one monkey and mutate another simultaneously, but I circumvented it by just indexing directly 
//! into the [`Vec`] to constrain the borrow lifetimes.
//! 
//! Part two was... harder. I knew that there had to be some mathematical trick to cut down on the worry levels while
//! still preserving their "meaning," so to speak, but I had no idea what that trick would be - so eventually I caved
//! and asked for a hint on Discord, figuring it out pretty quickly after that. I think I could have derived the necessary
//! property on my own, but it would have taken a day or maybe two of chewing on the problem.
//! 
//! (If this explanation seems short compared to the others - it is, because again, this puzzle was kinda lame.)

use std::collections::VecDeque;

use super::*;
use Operation::*;

impl Solution<DAY_11> for Solutions {
    type Input<'i> = Vec<Monkey>;
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .split("\n\n")
            .map(Monkey::from)
            .collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        let mut monkeys = input.clone();

        for _ in 0..20 {
            pass(&mut monkeys, |item| *item /= 3);
        }

        compute_monkey_business(&monkeys)
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        let mut monkeys = input.clone();

        // Because all the divisors are primes, we can just find
        // their product to get their LCM.
        let lcm = monkeys
            .iter()
            .map(|monkey| monkey.divisor)
            .product::<u64>();

        for _ in 0..10_000 {
            pass(&mut monkeys, |item| *item %= lcm);
        }

        compute_monkey_business(&monkeys)
    }
}

impl Test<DAY_11> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 10_605,
            PART_TWO => 2_713_310_158
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add(u64),
    Mul(u64),
    AddSelf,
    MulSelf
}

impl Operation {
    pub fn apply(&self, lhs: &mut u64) {
        match self {
            Add(ct) => *lhs += ct,
            AddSelf => *lhs += *lhs,
            Mul(ct) => *lhs *= ct,
            MulSelf => *lhs *= *lhs
        }
    }
}

impl From<&str> for Operation {
    fn from(expr: &str) -> Self {
        let expr = expr.trim_start_matches("Operation: new = ");
        let operand = match expr.contains('+') {
            false => "*",
            true => "+"
        };
    
        let rhs = expr
            .split_once(operand)
            .map(|(_, rhs)| rhs.trim().parse::<u64>())
            .unwrap();

        match operand {
            "*" => match rhs {
                Ok(constant) => Mul(constant),
                Err(_) => MulSelf
            }
            "+" => match rhs {
                Ok(constant) => Add(constant),
                Err(_) => AddSelf
            }
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    targets: (u8, u8),
    inspections: u64
}

impl From<&str> for Monkey {
    fn from(str: &str) -> Self {
        let lines = str
            .lines()
            .map(str::trim)
            .skip(1)
            .collect::<Vec<_>>();

        let items = lines[0]
            .chars()
            .filter(|char| char.is_numeric() || char == &',')
            .collect::<String>()
            .split(',')
            .map(str::parse::<u64>)
            .map(Result::unwrap)
            .collect::<VecDeque<_>>();

        let operation = Operation::from(lines[1]);
        
        let divisor = lines[2]
            .trim_start_matches("Test: divisible by ")
            .parse::<u64>()
            .unwrap();

        let true_case = lines[3]
            .trim_start_matches("If true: throw to monkey ")
            .parse::<u8>()
            .unwrap();
        
        let false_case = lines[4]
            .trim_start_matches("If false: throw to monkey ")
            .parse::<u8>()
            .unwrap();

        Self {
            items,
            operation,
            divisor,
            targets: (true_case, false_case),
            inspections: 0
        }
    }
}

fn pass(monkeys: &mut [Monkey], callback: impl Fn(&mut u64)) {
    // We have to index into the slice instead of iterating to
    // avoid incurring the borrow checker's wrath.
    for i in 0..monkeys.len() {
        while let Some(mut item) = monkeys[i].items.pop_front() {
            // We use an braced expression to artificially limit the scope
            // of our monkey mutable borrow.
            let targets = {
                let mut monkey = &mut monkeys[i];

                monkey.operation.apply(&mut item);
                callback(&mut item);
                monkey.inspections += 1;

                (
                    monkey.targets.0 as usize, 
                    monkey.targets.1 as usize
                )
            };

            // Pass the item.
            if item % monkeys[i].divisor == 0 {
                monkeys[targets.0].items.push_back(item);
            } else {
                monkeys[targets.1].items.push_back(item);
            }
        }
    }
}

fn compute_monkey_business(monkeys: &[Monkey]) -> u64 {
    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<_>>();
    
    inspections.sort_unstable();
    inspections[monkeys.len() - 2 ..]
        .iter()
        .product()
}

derive_tests!(Solutions, DAY_11);