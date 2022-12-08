//! # Day 7 - No Space Left On Device
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 03:53:12 (18848)
//! - P2 completed @ 04:06:23 (17989)
//! 
//! "Coping, seething and losing my mind" - Me, Rust Discord `#advent-of-code-2022`, after reimplementing my solution for the fifth time
//! 
//! Now *this* was a difficulty spike if I've ever seen one. Also, I'm wondering if this communication device
//! is going to become a recurring theme. (Rule of 3 occurrences, perhaps?)
//! 
//! ## Parsing
//! Although this is a complex problem, we can actually offload all of the computation to parse time -
//! the actual solutions only require us to inspect a well-structured representation of the device filesystem.
//! 
//! So, for parsing, we need to step through the `.bash_history` (essentially) that is our puzzle input and
//! reconstruct the filesystem. I tried various "clever" (stupid) ways to do this with liberal use of recursion,
//! but ultimately I boiled down my choices to either:
//! - Building a tree structure
//! - Using a HashMap
//! 
//! I settled on the latter, although I suspect both would take about the same amount of work.
//! 
//! To represent the filesystem, I created the [`Directory`] and [`Filesystem`] structs.
//! - A [`Directory`] has `local_size` (the total size of all files it directly contains) and `deep_size`
//! (its local size plus the local size of all its child directories) fields, as well as a `children` field
//! comprised of a [`Vec<PathBuf>`].
//! - A [`Filesystem`] consists of a `table` (a [`HashMap<PathBuf, Directory>`]) and a `path` (a [`PathBuf`],
//! representing the current directory.)
//! 
//! To build the filesystem, we iterate over the lines of the input (filtering out those that start with `$ ls`)
//! and decide what to do using the following rules:
//! - If the line is `$ cd ..`, call [`Filesystem::ascend`], which pops one component off the internal [`PathBuf`].
//! - If the line matches `$ cd <DIR>`, call [`Filesystem::descend`], which pushes `<DIR>` onto the internal [`PathBuf`] and creates
//! an entry for it in the `table` if it doesn't already exist.
//! - If the line matches `$ dir <DIR>`, call [`Filesystem::push_child`] to register the directory as a child of the current directory.
//! - Otherwise, split the line at the first space and parse the left-hand result to a [`usize`], calling [`Filesystem::push_size`] to
//! bump up the total size of the current directory.
//! 
//! Once the entire input has been walked, we still need to do a second pass over the [`Filesystem`] to compute the deep size
//! property of each directory. The [`Filesystem::update_sizes`] method implements this algorithm:
//! 1. Clone the `table`.
//! 2. For each value (directory) in the *cloned* table, recurse mutably over its children 
//! (looking up their keys in the *original* table, due to mutability rules) to sum their local sizes together
//! and update the parent directory's deep size in the *cloned* table.
//! 3. Replace the old table with the new, updated table.
//! 
//! Once all that is done, we have a complete and accurate representation of the filesystem!
//! 
//! ## Solutions
//! Thanks to all the pre-existing parsing work, the actual solutions are simple.
//! - The first part amounts to merely filtering the values of the filesystem table for all entries
//! with a deep size `<= 100_000` and summing their deep sizes together.
//! - The second part requires us to do some quick subtraction to determine a size threshold, then
//! once again filtering the values of the filesystem table for all values *smaller than* the threshold and using
//! `min()` to find the smallest one out of those.

use std::{collections::HashMap, path::PathBuf};

use super::*;

impl Solution<DAY_07> for Solutions {
    type Input<'i> = Filesystem;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let mut fs = puzzle
            .lines()
            .filter(|line| !line.starts_with("$ ls"))
            .fold(Filesystem::new(), |mut fs, line| {
                if line.starts_with("$ cd ") {
                    let path = line.trim_start_matches("$ cd ");
                    match path {
                        ".." => fs.ascend(),
                        _ => fs.descend(path)
                    }
                }
                else if line.starts_with("dir") {
                    let (_, name) = line.split_once(' ').unwrap();
                    fs.push_child(name);
                }
                else {
                    let (left, right) = line.split_once(' ').unwrap();
                    match left {
                        "dir" => fs.push_child(right),
                        _ => {
                            let size = left.parse::<usize>().unwrap();
                            fs.push_size(size);
                        }
                    }
                }
                fs
            });

        fs.update_sizes();
        fs
    }

    fn part_one(input: &Self::Input<'_>) -> Option<Self::Output> {
        input.table
            .values()
            .filter_map(|dir| match dir.deep_size {
                0..=100_000 => Some(dir.deep_size),
                _ => None
            })
            .sum::<usize>()
            .into()
    }

    fn part_two(input: &Self::Input<'_>) -> Option<Self::Output> {
        const FILESYSTEM_SIZE: usize = 70_000_000;
        const UPDATE_SIZE: usize = 30_000_000;

        let available_space = FILESYSTEM_SIZE - (input.load("/").deep_size);
        let needed_space = UPDATE_SIZE - available_space;

        input.table
            .values()
            .filter_map(|dir| {
                if dir.deep_size < needed_space { None }
                else { Some(dir.deep_size) }
            })            
            .min()
    }
}

impl Test<DAY_07> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 95_437,
            PART_TWO => 24_933_642
        }
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    local_size: usize,
    deep_size: usize,
    children: Vec<PathBuf>
}

impl Directory {
    pub fn new() -> Self {
        Self {
            local_size: 0,
            deep_size: 0,
            children: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Filesystem {
    pub table: HashMap<PathBuf, Directory>,
    path: PathBuf
}

impl Filesystem {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            path: PathBuf::new()
        }
    }

    pub fn load(&self, path: &str) -> &Directory {
        self.table.get(&PathBuf::from(path)).unwrap()
    }

    pub fn ascend(&mut self) {
        self.path.pop();
    }

    pub fn descend(&mut self, directory: &str) {
        self.path.push(directory);

        if !self.table.contains_key(&self.path) {
            self.table.insert(
                self.path.to_owned(),
                Directory::new()
            );
        }
    }

    pub fn push_size(&mut self, size: usize) {
        let directory = self.table.get_mut(&self.path).unwrap();
        directory.local_size += size;
        directory.deep_size += size;
    }

    pub fn push_child(&mut self, directory: &str) {
        let child = self.path.join(directory);
        self.table
            .get_mut(&self.path)
            .unwrap()
            .children.push(child);
    }

    pub fn update_sizes(&mut self) {
        let mut new_table = self.table.clone(); 

        for directory in new_table.values_mut() {
            for child in &directory.children {
                directory.deep_size += self.resolve_deep_size(child)
            }
        }

        self.table = new_table;
    }

    fn resolve_deep_size(&self, key: &PathBuf) -> usize {
        let root = self.table.get(key).unwrap();
        let mut size = root.local_size;

        for directory in &root.children {
            size += self.resolve_deep_size(directory)
        }

        size
    }
}

derive_tests!(Solutions, DAY_07);