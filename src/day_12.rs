//! # Day 12 - Hill Climbing Algorithm
//! 
//! Puzzle opened late (had exam in the morning, went to bed early.)
//! - P1 completed @ 16:00:44 (29204)
//! - P2 completed @ 16:08:58 (28186)
//! 
//! Now that they're trucking out the pathfinding problems, I think it's safe to say that we're now in the big leagues.
//! 
//! Although - I'm ashamed to say I didn't immediately see this as a pathfinding problem! My initial idea (which I never expounded on, but still)
//! involved a recursive algorithm that would have equated to a worse version of BFS (it would have computed every possible path to the target, using
//! a hash set to prevent loops, then used a sort or min-heap to find the smallest one.)
//! 
//! ## Parsing
//! My parsing approach to this problem was to map and collect the input into a 2D character array, then unroll that back into a 1D array of 
//! graph vertices with "edges" pointing to their neighbors in the original 2D array. (I'm not sure if collecting the 2D vec is strictly necessary,
//! but it makes flattening the 2D map to 1D a lot easier when all the information is in front of you.)
//! 
//! ## Solutions
//! The heart of both solutions is an implementation of the breadth-first search algorithm on [`Graph`] that finds the shortest path (measured in edge traversals)
//! from a given start point to an end point. This isn't a particularly *generic* or *optimized* BFS: 
//! - It uses three different collections during its runtime:
//!   - A [`VecDeque`] for storing vertices that need to be explored
//!   - A [`HashSet`] for tracking vertices that have already *been* explored
//!   - A plain old [`Vec`] for storing the parents of vertices so we can compute the traversal count later
//! - It has code baked in to handle special cases from the input (turning 'S' and 'E' into 'a' and 'z' to avoid miscomparisons 
//! when checking that edges are actually traversable) 
//! 
//! ... but it works! It even accepts a closure callback that's used to decide if an edge is traversable from the current vertex,
//! so both parts can optimize it to their needs.
//! 
//! The solution functions themselves are nearly identical - only differing in their inputs to [`Graph::shortest_path`]. 
//! - The first solution finds the starting index of `S` and queries shortest path with a target of `E` and a filter function of
//! `edge <= vertex + 1`.
//! - The second solution finds the starting index of `E` and queries shortest path with a target of `a` and a filter function of 
//! `edge >= vertex - 1`.
//! 
//! This allows both parts to find the absolute shortest path in under 500 micros.

use std::{collections::{VecDeque, HashSet}};

use super::*;

impl Solution<DAY_12> for Solutions {
    type Input<'i> = Graph;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let grid = puzzle
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        Graph::unroll_grid(grid)
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {        
        let find_shortest = |i: usize| {
            input.shortest_path(i, 'E', |e, v| {
                e <= v + 1
            })
        };
        
        input.vertices
            .iter()
            .position(|vertex| vertex.value == 'S')
            .map(find_shortest)
            .unwrap()
            .unwrap()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        let find_shortest = |i: usize| {
            input.shortest_path(i, 'a', |e, v| {
                e >= v - 1
            })
        };

        input.vertices
            .iter()
            .position(|vertex| vertex.value == 'E')
            .map(find_shortest)
            .unwrap()
            .unwrap()
    }
}

impl Test<DAY_12> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 31,
            PART_TWO => 29
        }
    }
}

#[derive(Debug)]
struct Vertex {
    edges: Vec<usize>,
    value: char,
}

#[derive(Debug)]
pub struct Graph {
    vertices: Vec<Vertex>
}

impl Graph {
    /// Unroll a grid (2D vec) of characters into a 1D array
    /// of vertices. Assumes that the grid's rows and columns
    /// are of constant (but not necessarily the same) length.
    pub fn unroll_grid(grid: Vec<Vec<char>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();

        let get_unrolled_index = |pair: (usize, usize)| {
            (pair.0 * width) + pair.1
        };

        let mut vertices = Vec::with_capacity(height * width);

        for row in 0..height {
            for col in 0..width {
                let value = grid[row][col];

                let edges = adjacent_indices(row, col, grid.as_slice())
                    .map(get_unrolled_index)
                    .collect::<Vec<_>>();
                
                vertices.push(Vertex {
                    edges,
                    value,
                });
            }
        }

        Self { vertices }
    }
    
    /// Implementation of breadth-first search that finds the length
    /// of the shortest path between the given start and target, measured in edge traversals.
    pub fn shortest_path<F>(&self, start: usize, target: char, filter: F) -> Option<usize> where
        F: Fn(u8, u8) -> bool
    {
        let mut queue = VecDeque::new();
        let mut explored = HashSet::new();
        let mut parents = vec![None; self.vertices.len()];

        explored.insert(start);
        queue.push_back(start);

        while let Some(index) = queue.pop_front() {
            let vertex = &self.vertices[index];

            if vertex.value == target {
                return parent_distance(&parents, index, 0).into()
            }

            let value = match vertex.value {
                'S' => 'a',
                'E' => 'z',
                _ => vertex.value
            };

            vertex.edges
                .iter()
                .copied()
                .filter(|i| {
                    let edge = self.vertices[*i].value;
                    let edge = match edge {
                        'S' => b'a',
                        'E' => b'z',
                        _ => edge as u8
                    };
                    filter(edge, value as u8)
                })
                .for_each(|i| {
                    if explored.insert(i) {
                        queue.push_back(i);
                        parents[i] = Some(index);
                    }
                });
        }

        None
    }
}

fn adjacent_indices(row: usize, col: usize, source: &'_[Vec<char>]) -> impl Iterator<Item=(usize, usize)> + '_ {
    let mut indices = vec![(row + 1, col), (row, col + 1)];

    if row > 0 {
        indices.push((row - 1, col));
    }
    if col > 0 {
        indices.push((row, col - 1));
    }

    indices
        .into_iter()
        .filter(|(row, col)| {
            if let Some(row) = source.get(*row) {
                return row.get(*col).is_some()
            }
            false
        })
}

fn parent_distance(parents: &[Option<usize>], index: usize, depth: usize) -> usize {
    match parents[index] {
        Some(parent) => parent_distance(parents, parent,depth + 1),
        None => depth
    }
}

derive_tests!(Solutions, DAY_12);