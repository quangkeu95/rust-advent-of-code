use std::{collections::VecDeque, str::FromStr};

use crate::utils::FileUtils;

pub fn run() {
    let contents: Vec<String> = FileUtils::parse_text_file("src/puzzles/day_5/input.txt").unwrap();

    let mut splitted = contents.split(|line| line.is_empty());

    let cargo_content = splitted.next().unwrap();
    let mut cargo = Cargo::new(cargo_content);

    let instructions_content = splitted.next().unwrap();
    let instructions = Instruction::from_raw_input(instructions_content);

    // Part 1
    CargoExecutor::execute(&mut cargo, &instructions);

    let top_of_stacks = cargo.top_of_stacks();
    dbg!(top_of_stacks.iter().collect::<String>());

    // Part 2

    let mut cargo = Cargo::new(cargo_content);

    CargoExecutorV2::execute(&mut cargo, &instructions);

    let top_of_stacks = cargo.top_of_stacks();
    dbg!(top_of_stacks.iter().collect::<String>());
}

#[derive(Debug)]
pub struct Cargo {
    pub stacks: Vec<Stack>,
}

impl Cargo {
    pub fn new<S>(raw_input: &[S]) -> Self
    where
        S: AsRef<str>,
    {
        let mut indexes: Vec<usize> = Vec::new();
        let mut char_cache: Vec<char> = Vec::new();

        // firstly, we gonna collect number of stacks by extracting the last line from the raw input
        for c in raw_input[raw_input.len() - 1].as_ref().chars() {
            if c == ' ' {
                if !char_cache.is_empty() {
                    let num_str: String = char_cache.iter().collect();
                    let num = num_str.parse::<usize>().unwrap();
                    indexes.push(num);
                    char_cache.clear();
                }
                continue;
            } else {
                char_cache.push(c);
            }
        }

        // make a vector of stacks with given number of stacks
        let mut stacks: Vec<Stack> = indexes.iter().map(|i| Stack::new(*i, vec![])).collect();

        // we iterator each line from the bottom up, in order to push the item into the stacks
        for i in (0..=raw_input.len() - 2).rev() {
            let line = raw_input[i].as_ref();

            let mut stack_index = 0usize;

            let mut cache: Vec<char> = Vec::new();
            let mut j = 0;
            let line_chars: Vec<char> = line.chars().collect();

            while j < line_chars.len() || cache.len() > 0 {
                if cache.len() == 3 {
                    cache.pop();
                    let crate_name = cache.pop().unwrap();

                    if crate_name != ' ' {
                        stacks[stack_index].items.push(crate_name);
                    };
                    stack_index += 1;
                    cache.clear();
                } else {
                    let c = line_chars[j];
                    cache.push(c);
                }

                j += 1;
            }
        }

        Self { stacks }
    }

    pub fn pop_from_stack(&mut self, stack_index: usize) -> Option<char> {
        let mut result: Option<char> = None;
        self.stacks.iter_mut().for_each(|item| {
            if item.index == stack_index {
                result = item.pop();
                return;
            }
        });

        result
    }

    pub fn push_to_stack(&mut self, stack_index: usize, c: char) {
        self.stacks.iter_mut().for_each(|item| {
            if item.index == stack_index {
                item.push(c);
            }
        });
    }

    pub fn pop_multi_from_stack(&mut self, stack_index: usize, num_crates: usize) -> Vec<char> {
        let mut result: Vec<char> = vec![];
        self.stacks.iter_mut().for_each(|item| {
            if item.index == stack_index {
                for _i in 0..num_crates {
                    if let Some(single_item) = item.pop() {
                        result.push(single_item);
                    }
                }
                return;
            }
        });

        return result;
    }

    pub fn push_multi_to_stack(&mut self, stack_index: usize, crates: &[char]) {
        self.stacks.iter_mut().for_each(|item| {
            if item.index == stack_index {
                let mut vec_dequeue = crates.iter().map(|item| *item).collect::<Vec<char>>();

                while let Some(cr) = vec_dequeue.pop() {
                    item.push(cr);
                }
            }
        });
    }

    pub fn top_of_stacks(&self) -> Vec<char> {
        self.stacks
            .iter()
            .filter_map(|stack| stack.top().map(|t| *t))
            .collect::<Vec<char>>()
    }
}

pub trait Executor {
    fn execute(cargo: &mut Cargo, instructions: &[Instruction]);
}

#[derive(Default)]
pub struct CargoExecutor {}

impl Executor for CargoExecutor {
    fn execute(cargo: &mut Cargo, instructions: &[Instruction]) {
        for instruction in instructions {
            for _i in 0..instruction.num_crates {
                if let Some(item) = cargo.pop_from_stack(instruction.from) {
                    cargo.push_to_stack(instruction.to, item);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct CargoExecutorV2 {}

impl Executor for CargoExecutorV2 {
    fn execute(cargo: &mut Cargo, instructions: &[Instruction]) {
        for instruction in instructions {
            let crates = cargo.pop_multi_from_stack(instruction.from, instruction.num_crates);

            cargo.push_multi_to_stack(instruction.to, &crates);
        }
    }
}

#[derive(Debug)]
pub struct Stack {
    pub index: usize,
    pub items: Vec<char>,
}

impl Stack {
    pub fn new(index: usize, items: Vec<char>) -> Self {
        Self { index, items }
    }

    pub fn pop(&mut self) -> Option<char> {
        self.items.pop()
    }

    pub fn push(&mut self, c: char) {
        self.items.push(c);
    }

    pub fn top(&self) -> Option<&char> {
        self.items.last()
    }
}

#[derive(Debug, Default)]
pub struct Instruction {
    from: usize,
    to: usize,
    num_crates: usize,
}

impl Instruction {
    pub fn new(from: usize, to: usize, num_crates: usize) -> Self {
        Self {
            from,
            to,
            num_crates,
        }
    }

    pub fn from_raw_input<S>(raw_input: &[S]) -> Vec<Self>
    where
        S: AsRef<str>,
    {
        let instructions = raw_input
            .iter()
            .map(|line| {
                let line = line.as_ref();

                let items = line.split(" ").collect::<Vec<&str>>();
                if items.len() < 6 {
                    panic!("invalid instruction length");
                }

                let num_crates = items[1].parse::<usize>().unwrap();
                let from = items[3].parse::<usize>().unwrap();
                let to = items[5].parse::<usize>().unwrap();

                Instruction::new(from, to, num_crates)
            })
            .collect::<Vec<Instruction>>();

        instructions
    }
}

#[derive(Debug)]
pub enum Command {
    Move,
    From,
    To,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "move" => Ok(Self::Move),
            "from" => Ok(Self::From),
            "to" => Ok(Self::To),
            _ => panic!("unknown command"),
        }
    }
}
