//! # Day 13 - Distress Signal
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 05:32:38 (13633)
//! - P2 completed @ 05:54:01 (13016)
//! 
//! When I completed this puzzle at almost 6 in the morning, my first instinct was to post that fucking GIF of
//! Frodo saying "it's done" in Mount Doom.
//! 
//! ## Parsing
//! I actually lied about some of the previous puzzles being parsing hell - *this* was real parsing hell, especially
//! without access to anything except `std`. After wasting an hour trying to be clever by flattening out each packet
//! into a single, non-nested list of numbers, I ended up having to hand-roll my own recursive descent parser before I could
//! even get started on the rest of the problem!
//! 
//! ## Solutions
//! To compute the actual solutions, I primarily needed to implement [`PartialEq`] and [`PartialOrd`] (and, for part two, their more
//! rigorous cousins [`Eq`] and [`Ord`] by just deferring to the partial implementations) for my custom [`Value`] type. A [`Value`]
//! can be one of two things:
//! - A single number ([`u64`]).
//! - A list of other [`Value`]s, i.e. a [`Vec<Value>`].
//! 
//! Fortunately, thanks to the magic of tuple pattern matching and blanket implementations, this was easy to implement in a concise manner.
//! For cases where the [`Value`]s being compared are both numbers or lists, we can just defer to the existing implementations on those types;
//! as for cases where one value is a list and the other is a number, we just repack the number into a slice (avoiding a temporary heap alloc) 
//! and compare it with the list.
//! 
//! With implementations of the equality and ordering traits, we can now use comparison operators like `<` or `==` with [`Value`]s, which makes
//! finding the solutions very simple.
//! - For part one:
//!   - Chunk the input by twos.
//!   - Enumerate.
//!   - Filter out any pairs where the first is not less than the second.
//!   - Map out the actual [`Value`] pairs, only keeping the index (and incrementing it in the process to be compliant with Elf indexing.)
//!   - Sum the remaining indices.
//! - For part two:
//!   - Clone the input (as we need to mutate it.)
//!   - Create two new [`Value`]s corresponding to our divider packets, and insert clones of them into our input.
//!   - Sort the input and iterate over it.
//!   - Enumerate.
//!   - Filter out any values not equal to the divider packets.
//!   - Map out the [`Value`]s, once again only keeping + incrementing the index.
//!   - Get the product of the remaining indices.

use super::*;

impl Solution<DAY_13> for Solutions {
    type Input<'i> = Vec<Value>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::trim)
            .map(Value::from)
            .collect()
    }

    fn part_one(input: &Vec<Value>) -> Self::Output {  
        input
            .chunks(2)
            .enumerate()
            .filter(|(_, values)| values[0] < values[1])
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn part_two(input: &Vec<Value>) -> Self::Output {
        let mut input = input.clone();
        
        let divider_a = Value::from("[[2]]");
        let divider_b = Value::from("[[6]]");

        input.push(divider_a.clone());
        input.push(divider_b.clone());
        input.sort_unstable();

        input
            .iter()
            .enumerate()
            .filter(|(_, value)| *value == &divider_a || *value == &divider_b)
            .map(|(i, _)| i + 1)
            .product()
    }
}

impl Test<DAY_13> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 13,
            PART_TWO => 140
        }
    }
}

#[derive(Eq, Debug, Clone)]
pub enum Value {
    Data(u64),
    List(Vec<Value>)
}

impl From<&str> for Value {
    fn from(data: &str) -> Self {
        if data == "[]" {
            return Self::List(vec![]);
        }

        if let Ok(data) = data.trim().parse::<u64>() {
            return Self::Data(data)
        }

        let range = 1..data.len() - 1;
        let chars = data[range].chars();

        let mut list = Vec::new();
        let mut buffer = String::new();
        let mut depth = 0;

        for character in chars {
            match character {
                '[' => {
                    depth += 1;
                    buffer.push(character);
                    continue;
                }
                ']' => {
                    depth -= 1;
                    buffer.push(character);
                    continue;
                }
                _ => ()
            }

            if depth == 0 {
                match character {
                    ',' => {
                        list.push(Self::from(buffer.as_str()));
                        buffer.clear();
                    }
                    _ => buffer.push(character)
                }
            } else {
                buffer.push(character);
            }
        }

        if !buffer.is_empty() {
            list.push(Self::from(buffer.as_str()))
        }

        Self::List(list)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Data(lhs), Data(rhs)) => lhs.eq(rhs),
            (List(lhs), List(rhs)) => lhs.eq(rhs),
            (Data(_), List(rhs)) => rhs.first().eq(&Some(self)),
            (List(lhs), Data(_)) => lhs.first().eq(&Some(other)),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Value::*;
        match (self, other) {
            (Data(lhs), Data(rhs)) => lhs.partial_cmp(rhs),
            (List(lhs), List(rhs)) => lhs.partial_cmp(rhs),
            (Data(lhs), List(rhs)) => ([Data(*lhs)].as_slice()).partial_cmp(rhs.as_slice()),
            (List(lhs), Data(rhs)) => lhs.as_slice().partial_cmp([Data(*rhs)].as_slice())
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

derive_tests!(Solutions, DAY_13);