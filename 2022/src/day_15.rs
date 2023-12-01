//! # Day 15 - Beacon Exclusion Zone
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 02:00:28 (6772)
//! - P2 completed @ 20:15:24 🥴 (20963)
//! 
//! No joke, I think I had to rewrite my exclusion zone generator and range merger functions like, ten times.
//! I'm honestly amazed I got part one right with how many errors there were, and all those hidden bugs came out in full force
//! to bite me in the ass for part two.
//! 
//! My final solution for part two takes about 1.6 seconds to execute. No, I am not optimizing it, I'm fucking tired of grid/coordinate math.
//! 
//! ## Parsing
//! Parsing was pretty easy; I *did* manage to blow my own foot off by not preserving any negative signs present in the input
//! when filtering, but I managed to catch that bug fairly early and resolved it.
//! 
//! ## Solutions
//! My initial part one solution took the naive path - generating every point covered by the beacons, filtering out all those not
//! in the target row, deduping them and tallying them up. This took several hundred milliseconds, but hey, it worked!
//! 
//! Then along comes part two, and any hope of code reuse goes up in smoke. I did the math, and even running at full tilt, it would take
//! over a month (!) to scan the entire beacon space using the brute force method. Even if I maximally parallelized it (using all 12 threads in my machine)
//! it would still take three days or so, which clearly wasn't going to work.
//! 
//! So, I needed an alternate solution! ...Unfortunately, after a few hours of trying various geometrical tricks that all turned out to be either inapplicable or still
//! hilariously inefficient, I ended up putting it on hold and going to sleep.
//! 
//! In the afternoon, I ended up taking the less-efficient but still workable approach of ranges - instead of generating *every single point* covered by the sensors
//! (which, by the way, was enough to get my program OOM killed on a 32 gigabyte machine - the only reason P1 worked was because I had the foresight to generate points lazily),
//! we could just compute the X-coordinate ranges they cover at each Y-level, merging them all together to determine if there's any open spaces within the bounding box. 
//! 
//! This took *far* more work than it should have to get right, and even once I had a working merger function, my initial attempts at scanning each row were hilariously slow (on the order
//! of a day-ish to completion) because I was doing a linear scan-and-filter of the *entire* range set for *every* Y-level, like a moron. The "proper" solution turned out to be sorting the entire
//! set based on Y-level, then draining sections from the bottom-up to merge and check them for holes.
//! 
//! ... I mean, it's good enough for government work?

use std::ops::Range;

use super::*;

#[cfg(test)]
const TARGET_ROW: i64 = 10;
#[cfg(test)]
const MAX_COORD: i64 = 20;
#[cfg(not(test))]
const TARGET_ROW: i64 = 2_000_000;
#[cfg(not(test))]
const MAX_COORD: i64 = 4_000_000;

impl Solution<DAY_15> for Solutions {
    type Input<'i> = Vec<(Point, Point)>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let parse_side = |side: &str| {
            let filtered = side
                .chars()
                .filter(|c| c.is_numeric() || c == &',' || c == &'-')
                .collect::<String>();
            
            let (left, right) = filtered.split_once(',').unwrap();

            (
                left.parse::<i64>().unwrap(),
                right.parse::<i64>().unwrap()
            )
        };
        
        puzzle
            .lines()
            .map(|line| line.split_once(':').unwrap())
            .map(|(left, right)| (
                parse_side(left),
                parse_side(right)
            ))
            .map(|(left, right)| (
                Point::from(left),
                Point::from(right)
            ))
            .collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        let mut set = Vec::new();

        for (sensor, beacon) in input {
            let distance = sensor.manhattan_dist(beacon);
            let excluded = sensor.exclusion_ranges(distance as u64)
                .filter(|(i, _)| *i == TARGET_ROW)
                .map(|(_, r)| r);

                set.extend(excluded);
        }

        merge_intervals(set)
            .into_iter()
            .flatten()
            .count() - 1
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        let mut map: Vec<_> = input
            .iter()
            .flat_map(|(sensor, beacon)| {
                let distance = sensor.manhattan_dist(beacon);
                sensor.exclusion_ranges(distance as u64)
                    .filter(|(i, _)| *i >= 0 && *i <= MAX_COORD)
                    .map(|(i, mut range)| {
                        range.start = std::cmp::max(0, range.start);
                        range.end = std::cmp::min(MAX_COORD + 1, range.end);
                        (i, range)
                    })
            })
            .collect();

        map.sort_unstable_by(|a, b| {
            a.0.cmp(&b.0)
        });

        for y in (0..=MAX_COORD).rev() {
            let count = map
                .iter()
                .rev()
                .take_while(|(i, _)| i == &y)
                .count();
        
            let drained: Vec<_> = map
                .drain(map.len() - count..)
                .map(|(_, r)| r)
                .collect();

            let row = merge_intervals(drained);

            if row.len() > 1 {
                let x = row[0].end;
                return ((x * 4_000_000) + y) as usize
            }
        }

        panic!("Solution not found!")
    }
}

impl Test<DAY_15> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 26,
            PART_TWO => 56000011
        }
    }
}

type Exclusion = (i64, Range<i64>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhattan_dist(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn exclusion_ranges(&self, size: u64) -> impl Iterator<Item=Exclusion> {
        let start = Point {
            x: self.x,
            y: self.y - (size as i64),
        };

        self.widths(size)
            .enumerate()
            .map(move |(i, width)| {
                let y = start.y + (i as i64);
                let x_start = start.x - (width / 2);
                let x_end = x_start + width;

                (y, x_start..x_end)
            })
    }

    fn widths(&self, size: u64) -> impl Iterator<Item=i64> {
        let max_width = (size * 2 + 1) as i64; 

        let top_widths = (1..=max_width - 2).step_by(2);
        let peak_width = std::iter::once(max_width);
        let bottom_widths = (1..=max_width - 2)
            .rev()
            .step_by(2);

        top_widths
            .chain(peak_width)
            .chain(bottom_widths)
    }
}

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Self::new(x, y)
    }
}

fn merge_intervals(mut data: Vec<Range<i64>>) -> Vec<Range<i64>> {
    if data.is_empty() {
        return vec![]
    }
    
    let mut stack = Vec::new();

    data.sort_unstable_by(|a, b| {
        a.start.partial_cmp(&b.start).unwrap()
    });

    stack.push(data[0].clone());

    for range in data {
        let top = stack.last_mut().unwrap();

        if top.end < range.start {
            stack.push(range.clone())
        }
        else if top.end < range.end {
            top.end = range.end;
        }
    }

    stack
}

derive_tests!(Solutions, DAY_15);