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
    fn expected() -> (Option<Self::Output>, Option<Self::Output>) {
        (95437.into(), 24933642.into())
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