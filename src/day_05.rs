use super::*;

impl Solution<DAY_05> for Solutions {
    type Input<'a> = (Vec<Stack>, Vec<Command>);
    type Output = String;

    fn parse(puzzle: &'_ str) -> Self::Input<'_> {
        let (drawing, commands) = puzzle
            .split_once("\n\n")
            .unwrap();

        let mut stacks = vec![Stack::new(); 9];
        
        drawing
            .lines()
            .map(|line| line
                .chars()
                .skip(1)
                .step_by(4)
                .collect::<Vec<_>>()
            )
            .for_each(|row| {
                for i in 0..row.len() {
                    stacks[i].push(row[i])
                }
            });
        
        for stack in &mut stacks {
            stack.retain(|char| *char != ' ')
        }

        let commands: Vec<_> = commands
            .lines()
            .map(Command::from_str)
            .collect();
        
        (stacks, commands)
    }

    fn part_one(input: &Self::Input<'_>) -> Option<Self::Output> {
        let (stacks, commands) = input;
        let mut stacks = stacks.to_owned();
        simulate_craning(commands, &mut stacks, false).into()
    }

    fn part_two(input: &Self::Input<'_>) -> Option<Self::Output> {
        let (stacks, commands) = input;
        let mut stacks = stacks.to_owned();
        simulate_craning(commands, &mut stacks, true).into()
    }
}

fn simulate_craning(commands: &[Command], stacks: &mut [Stack], reversed: bool) -> String {
    for cmd in commands {
        let source = &mut stacks[cmd.source];
        let removed = source
            .drain(0..std::cmp::min(cmd.count, source.len()))
            .collect::<Vec<_>>()
            .into_iter();
        
        match reversed {
            true => removed.rev().for_each(|char| stacks[cmd.dest].insert(0, char)),
            false => removed.for_each(|char| stacks[cmd.dest].insert(0, char))
        }
    }

    stacks
        .iter()
        .map(|stack| stack[0])
        .collect::<String>()
}

type Stack = Vec<char>;

#[derive(Debug, Clone)]
pub struct Command {
    count: usize,
    source: usize,
    dest: usize
}

impl Command {
    pub fn from_str(s: &str) -> Self {
        let values: Vec<_> = s
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect();
        
        Self {
            count: values[0],
            source: values[1] - 1,
            dest: values[2] - 1
        }
    }
}