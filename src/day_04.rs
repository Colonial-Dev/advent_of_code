use std::ops::Range;

pub fn solution() {
    let input: Vec<_> = include_str!("inputs/04.txt")
        .lines()
        .map(|line| {
            line
                .split_once(',')
                .unwrap()
        })
        .map(|(left, right)| (
            pair_to_range(left),
            pair_to_range(right)
        ))
        .collect();

    let fully_overlapping_ct = input
        .iter()
        .filter(|(left, right)| {
            (left.start >= right.start) && (left.end <= right.end)
            ||
            (right.start >= left.start) && (right.end <= left.end)
        })
        .count();

    let partially_overlapping_ct = input
        .iter()
        .filter(|(left, right)| {
            (left.start <= right.end) && (right.start <= left.end)           
        })
        .count();

    println!("{fully_overlapping_ct} assignment pairs have fully overlapping ranges.");
    println!("{partially_overlapping_ct} assignment pairs have some level of overlap.");
}

fn pair_to_range(pair: &str) -> Range<u64> {
    pair
        .split_once('-')
        .map(|(l, r)| {
            (
                l.parse::<u64>().unwrap(),
                r.parse::<u64>().unwrap()
            )
        })
        .map(|(l, r)| l..r)
        .unwrap()
}