use std::collections::{HashMap, HashSet};

use crate::utils::FileUtils;

pub fn run() {
    let input_file = "src/puzzles/day_3/input.txt";
    let contents: Vec<String> = FileUtils::parse_text_file(input_file).unwrap();

    let mut sum_priority = 0;
    for line in contents.iter() {
        let common_char = common_item(line).unwrap();
        let priority = priority(&common_char);
        sum_priority += priority;
    }
    dbg!(sum_priority);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Compartment {
    First,
    Second,
}

fn common_item(line: &str) -> Option<char> {
    let line = line.trim();
    let num_items = line.len();

    if num_items < 2 {
        panic!("invalid number of characters in line, require at least 2 characters");
    }

    let mid_index = num_items / 2 - 1;

    let mut characters = HashMap::<char, Compartment>::new();

    for (i, c) in line.char_indices() {
        if !characters.contains_key(&c) {
            let compartment = compartment(i, mid_index);

            characters.insert(c, compartment);
        } else {
            let existed_value = characters.get(&c).unwrap();
            let compartment = compartment(i, mid_index);

            if *existed_value != compartment {
                return Some(c);
            }
        }
    }

    None
}

fn compartment(index: usize, mid_index: usize) -> Compartment {
    if index <= mid_index {
        Compartment::First
    } else {
        Compartment::Second
    }
}

fn priority(c: &char) -> u32 {
    let digit = c.to_digit(36).unwrap();
    // a or A is digit 10
    let priority = if c.is_lowercase() {
        digit - 9
    } else {
        digit + 17
    };
    priority
}

pub fn run_part_2() {
    let input_file = "src/puzzles/day_3/part2_input.txt";
    let contents: Vec<String> = FileUtils::parse_text_file(input_file).unwrap();

    let chunks = contents.chunks(3);
    let mut total_priority = 0;

    for chunk in chunks {
        let mut i = 0;
        let mut base_set: Option<HashSet<char>> = None;
        while i <= chunk.len() - 2 {
            let new_set = common_item_between_chunks(&chunk[i], &chunk[i + 1], base_set);
            base_set = Some(new_set);
            i += 1;
        }
        let mut base_set = base_set.unwrap();

        let mut drain_set = base_set.drain();

        let common_item = drain_set.next().unwrap();

        let priority = priority(&common_item);

        total_priority += priority;
    }

    dbg!(total_priority);
}

fn common_item_between_chunks(
    chunk_a: &str,
    chunk_b: &str,
    base_set: Option<HashSet<char>>,
) -> HashSet<char> {
    let mut common_set = HashSet::<char>::new();

    let mut previous_set = HashSet::<char>::new();

    if let Some(base_set) = base_set {
        chunk_a.chars().for_each(|c| {
            if base_set.contains(&c) && !previous_set.contains(&c) {
                previous_set.insert(c);
            }
        });
    } else {
        chunk_a.chars().for_each(|c| {
            if !previous_set.contains(&c) {
                previous_set.insert(c);
            }
        });
    }

    chunk_b.chars().for_each(|c| {
        if previous_set.contains(&c) && !common_set.contains(&c) {
            common_set.insert(c);
        }
    });

    common_set
}
