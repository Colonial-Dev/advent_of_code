//! # Day 10 - Cathode-Ray Tube
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 00:18:00 (3418)
//! - P2 completed @ 00:51:21 (5346)
//! 
//! I swear, some of these problems are specially crafted to be off-by-one bait.
//! Also, how are we building a whole ass electron gun *in the middle of the jungle*?
//! 
//! ## Parsing
//! We're dealing with what amounts to "baby's first ASM," so for parsing we just convert
//! each instruction to an equivalent [`Instruction`] enum variant.
//! 
//! ## Solutions
//! My initial solution for part one took the obvious approach - define some mutable
//! variables for the cycle count and register value, and loop through the parsed instructions,
//! updating them and the "signal strengths" as appropriate. Add on an `.iter().sum()` and I had the answer, GG EZ.
//! 
//! For part two, after spending five minutes trying to comprehend it using my strung-out clump of brain cells
//! I decided to extract out the "execution" functionality into a [`Processor`] struct, and additionally
//! create a [`Screen`] struct to manage the simulated pixel buffer and electron beam. 
//! 
//! Initially, [`Processor`] contained all the functionality needed for both solutions, such as a [`Vec`] used to record
//! signal peaks and a [`Screen`]. However, this was unwieldy, so I decided to instead have [`Processor`] accept a *callback* (a function pointer or closure) 
//! during construction, which would be invoked at each clock cycle with the current state of the register (`rax`) and the cycle count.
//! 
//! Because closures in particular are allowed to capture mutable references to items from the scope they were originally defined in, 
//! this means that the peaks [`Vec`] and [`Screen`] instance can be extracted out into their respective solutions, 
//! and I can simply create a closure in each that mutates them appropriately. 
//! (Being honest, I'm very happy that I finally got an excuse to use closure callbacks.)
//! 
//! With the setup done, and ignoring the several off-by-one errors I ran into, writing the actual solution functions was pretty simple;
//! just create a callback closure with associated state variables, pass it into a new [`Processor`], run the "program" on it and finally return the
//! state as the answer.

use super::*;
use lib_aoc::Split;

impl Solution<DAY_10> for Solutions {
    type Input<'i> = Vec<Instruction>;
    type Output = Split<isize, String>;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .lines()
            .map(Instruction::from)
            .collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        let breakpoints = [20, 60, 100, 140, 180, 220];
        let mut peaks = Vec::with_capacity(6);

        Processor::new(|rax, cycle| {
            if breakpoints.binary_search(&cycle).is_ok() {
                peaks.push(rax * cycle as isize);
            }
        }).execute(input);

        let ans = peaks
            .iter()
            .sum::<isize>();
        
        Split::P1(ans)
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        let mut screen = Screen::new();

        Processor::new(|rax, _| screen.update(rax))
            .execute(input);

        Split::P2(format!("{screen}"))
    }
}

impl Test<DAY_10> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => Split::P1(13360),
            PART_TWO => Split::P2(
                "\n\
                ██  ██  ██  ██  ██  ██  ██  ██  ██  ██  \n\
                ███   ███   ███   ███   ███   ███   ███ \n\
                ████    ████    ████    ████    ████    \n\
                █████     █████     █████     █████     \n\
                ██████      ██████      ██████      ████\n\
                ███████       ███████       ███████     \n"
            .to_string())
        }
    }
}

pub enum Instruction {
    Noop,
    Add(isize)
}

impl From<&str> for Instruction {
    fn from(str: &str) -> Self {
        if str == "noop" {
            Self::Noop
        } else {
            let (_, count) = str.split_once(' ').unwrap();
            let count = count.parse::<isize>().unwrap();
            Self::Add(count)
        }
    }
}

struct Processor<'a> {
    rax: isize,
    cycle: usize,
    callback: Box<dyn FnMut(isize, usize) + 'a>,
}

impl<'a> Processor<'a> {
    pub fn new(callback: impl FnMut(isize, usize) + 'a) -> Self {
        Self {
            rax: 1,
            cycle: 0,
            callback: Box::new(callback)
        }
    }

    pub fn execute(mut self, program: &[Instruction]) {
        (self.callback)(self.rax, self.cycle);
        for instruction in program {
            self.step(instruction)
        }
    }

    fn step(&mut self, op: &Instruction) {
        match op {
            Instruction::Noop => self.tick(),
            Instruction::Add(value) => {
                self.tick();
                self.rax += value;
                self.tick();
            }
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;
        (self.callback)(self.rax, self.cycle);
    }
}

struct Screen {
    pixels: [[bool; 40]; 6],
    beam: (usize, usize)
}

impl Screen {
    pub fn new() -> Self {
        Self {
            pixels: [[false; 40]; 6],
            beam: (0, 0)
        }
    }

    pub fn update(&mut self, rax: isize) {
        let (x, y) = self.beam;
        
        if (rax as i32 - x as i32).abs() <= 1 {
            self.pixels[y][x] = true; 
        }
        
        self.beam = match x + 1 {
            0..=39 => (x + 1, y),
            _ => (0, y + 1)
        };
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for i in 0..6 {
            for j in 0..40 {
                match self.pixels[i][j] {
                    false => write!(f, " ")?,
                    true => write!(f, "█")?
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

derive_tests!(Solutions, DAY_10);