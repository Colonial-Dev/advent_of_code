//! # Day 16 - Proboscidea Volcanium
//! 
//! Puzzle opened late (had an exam in the morning + drove home for Christmas break.)
//! - P1 completed @ >24h (14280)
//! - P2 completed @ >24h (11613)

use std::collections::{HashMap, VecDeque, HashSet};

use super::*;

const START_VALVE: &str = "AA";

impl Solution<DAY_16> for Solutions {
    type Input<'i> = HashMap<String, Valve>;
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let parse_line = |line: &str| {
            let (left, right) = line.split_once(';').unwrap();
            let (valve, flow) = left.split_once('=').unwrap();

            let id = valve
                .trim_start_matches("Valve ")
                .trim_end_matches(" has flow rate")
                .to_owned();
            
            let flow = flow.parse::<u64>().unwrap();

            let tunnels: Vec<_> = right
                .chars()
                .filter(|c| c.is_uppercase() || c == &',')
                .collect::<String>()
                .split(',')
                .map(str::trim)
                .map(str::to_owned)
                .map(|id| (1, id))
                .collect();

            (id, Valve { flow, tunnels })
        };

        let graph_base: HashMap<_, _> = puzzle
            .lines()
            .map(parse_line)
            .collect();

        let graph_view: Vec<_> = graph_base
            .iter()
            .filter(|(id, v)| v.flow != 0 || id == &START_VALVE)
            .collect();
        
        graph_view
            .iter()
            .map(|(outer, valve)| {
                let tunnels = graph_view
                    .iter()
                    .filter(|(inner, _)| inner != outer && inner != &START_VALVE)
                    .map(|(inner, _)| {
                        (shortest_path(&graph_base, outer, inner), inner.to_string())
                    })
                    .collect();

                (outer.to_string(), Valve {
                    flow: valve.flow,
                    tunnels
                })
            })
            .collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        depth_first(input, State {
            id: START_VALVE,
            flow_total: 0,
            time_left: 30,
            explored: vec![START_VALVE]
        })
        .iter()
        .map(|state| state.flow_total)
        .max()
        .unwrap()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        let paths: Vec<_> = depth_first(input, State {
            id: START_VALVE,
            flow_total: 0,
            time_left: 26,
            explored: vec![START_VALVE]
        })
        .iter()
        .cloned()
        .map(|mut state| {
            state.explored.remove(0);
            state
        })
        .collect();

        let mut max = 0;

        for outer in &paths {
            for inner in &paths {
                if outer.explored.iter().any(|valve| inner.explored.contains(valve)) {
                    continue;
                } else {
                    max = max.max(outer.flow_total + inner.flow_total);
                }
            }
        }

        max
    }
}

impl Test<DAY_16> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 1651,
            PART_TWO => 1707
        }
    }
}

#[derive(Debug)]
pub struct Valve {
    flow: u64,
    tunnels: Vec<(usize, String)>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct State<'a> {
    id: &'a str,
    flow_total: u64,
    time_left: u64,
    explored: Vec<&'a str>,
}

fn shortest_path(map: &HashMap<String, Valve>, start: &str, end: &str) -> usize {
    let mut stack = VecDeque::new();
    let mut explored = HashSet::new();

    explored.insert(start);
    stack.push_back((0, start));


    while let Some((steps, id)) = stack.pop_front() {            
        if id == end {
            return steps;
        }

        for edge in &map.get(id).unwrap().tunnels {
            if explored.insert(edge.1.as_str()) {
                stack.push_back((steps + 1, edge.1.as_str()))
            }
        }
        
    }

    panic!("No path found between {start} and {end}!")
}

fn depth_first<'a>(map: &'a HashMap<String, Valve>, init: State<'a>) -> HashSet<State<'a>> {
    let mut stack = vec![init];
    let mut paths = HashSet::new();

    while let Some(state) = stack.pop() {
        for (dist, id) in &map[state.id].tunnels {
            let time_left = match state.time_left < (dist + 1) as u64 {
                false => state.time_left - (dist + 1) as u64,
                true => continue
            };

            if time_left > 0 && !state.explored.contains(&id.as_str()) {
                let mut explored = state.explored.clone();
                explored.push(id);

                stack.push(State {
                    id,
                    flow_total: state.flow_total + (map[id].flow * time_left),
                    time_left,
                    explored
                })
            }
        }

        paths.insert(state);
    }

    paths
}

derive_tests!(Solutions, DAY_16);