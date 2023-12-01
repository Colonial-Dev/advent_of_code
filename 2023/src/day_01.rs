//! # Day 1 - Trebuchet?! 
//! 
//! Puzzle opened late (got distracted playing Minecraft, LOL) 
//! - P1 completed @ 12:48:32 (101197)
//! - P2 completed @ 13:49:01 (70741)


use super::*;

impl Solution<DAY_01> for Solutions {
    type Input<'i> = &'i str;
    type Output = u32;

    fn parse(puzzle: &str) -> Self::Input<'_> {    
        puzzle
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {        
        let get_digits = |line: &str| {
            let l = line
                .chars()
                .find(|c| c.is_ascii_digit() )
                .map(digit_char_lut)
                .unwrap();
            
            let r = line
                .chars()
                .rfind(|c| c.is_ascii_digit() )
                .map(digit_char_lut)
                .unwrap_or(l);

            (l, r)
        };

        input
            .lines()
            .map(get_digits)
            .map(digit_pair_lut)
            .sum()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        input
            .lines()
            .map(scan)
            .map(digit_pair_lut)
            .sum()
    }
}

impl Test<DAY_01> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 142,
            PART_TWO => 281,
        }
    }
}

fn digit_char_lut(c: char) -> u32 {
    match c {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => unreachable!()
    }
}

fn digit_pair_lut(pair: (u32, u32)) -> u32 {
    match pair {
        (1, 0) => 10,
        (1, 1) => 11,
        (1, 2) => 12,
        (1, 3) => 13,
        (1, 4) => 14,
        (1, 5) => 15,
        (1, 6) => 16,
        (1, 7) => 17,
        (1, 8) => 18,
        (1, 9) => 19,
        (2, 0) => 20,
        (2, 1) => 21,
        (2, 2) => 22,
        (2, 3) => 23,
        (2, 4) => 24,
        (2, 5) => 25,
        (2, 6) => 26,
        (2, 7) => 27,
        (2, 8) => 28,
        (2, 9) => 29,
        (3, 0) => 30,
        (3, 1) => 31,
        (3, 2) => 32,
        (3, 3) => 33,
        (3, 4) => 34,
        (3, 5) => 35,
        (3, 6) => 36,
        (3, 7) => 37,
        (3, 8) => 38,
        (3, 9) => 39,
        (4, 0) => 40,
        (4, 1) => 41,
        (4, 2) => 42,
        (4, 3) => 43,
        (4, 4) => 44,
        (4, 5) => 45,
        (4, 6) => 46,
        (4, 7) => 47,
        (4, 8) => 48,
        (4, 9) => 49,
        (5, 0) => 50,
        (5, 1) => 51,
        (5, 2) => 52,
        (5, 3) => 53,
        (5, 4) => 54,
        (5, 5) => 55,
        (5, 6) => 56,
        (5, 7) => 57,
        (5, 8) => 58,
        (5, 9) => 59,
        (6, 0) => 60,
        (6, 1) => 61,
        (6, 2) => 62,
        (6, 3) => 63,
        (6, 4) => 64,
        (6, 5) => 65,
        (6, 6) => 66,
        (6, 7) => 67,
        (6, 8) => 68,
        (6, 9) => 69,
        (7, 0) => 70,
        (7, 1) => 71,
        (7, 2) => 72,
        (7, 3) => 73,
        (7, 4) => 74,
        (7, 5) => 75,
        (7, 6) => 76,
        (7, 7) => 77,
        (7, 8) => 78,
        (7, 9) => 79,
        (8, 0) => 80,
        (8, 1) => 81,
        (8, 2) => 82,
        (8, 3) => 83,
        (8, 4) => 84,
        (8, 5) => 85,
        (8, 6) => 86,
        (8, 7) => 87,
        (8, 8) => 88,
        (8, 9) => 89,
        (9, 0) => 90,
        (9, 1) => 91,
        (9, 2) => 92,
        (9, 3) => 93,
        (9, 4) => 94,
        (9, 5) => 95,
        (9, 6) => 96,
        (9, 7) => 97,
        (9, 8) => 98,
        (9, 9) => 99,
        _ => unreachable!()
    }
}

fn scan(input: &str) -> (u32, u32) {
    let mut out = Vec::new();

    for idx in 0..input.len() {
        let peek = |len: usize| {
            &input[idx..(idx + len).min(input.len())]
        };

        let mut spot = |s: &str, v| {
            let l = s.len();
            let p = peek(l);

            if s == p {
                out.push(v)
            }
        };

        let mut word = |char| match char {
            'o' => spot("one", 1),
            't' => { spot("two", 2); spot("three", 3); },
            'f' => { spot("four", 4); spot("five", 5); },
            's' => { spot("six", 6); spot("seven", 7); },
            'e' => spot("eight", 8),
            'n' => spot("nine", 9),
            _ => ()
        };

        let char = input
            .chars()
            .nth(idx)
            .unwrap();

        if let Some(digit) = char.to_digit(10) {
            out.push(digit);
            continue;
        }

        word(char)
    }

    (out[0], out[out.len() - 1])
}

derive_tests!(Solutions, DAY_01);