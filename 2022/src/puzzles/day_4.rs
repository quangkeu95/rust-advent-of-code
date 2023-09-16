use anyhow::anyhow;
use std::str::FromStr;

use crate::utils::FileUtils;

pub fn run() {
    let contents: Vec<String> = FileUtils::parse_text_file("src/puzzles/day_4/input.txt").unwrap();

    // for each line (pair), we got a list of assignments in the format `min-max`
    // where `min` is the smallest assignment id and max is the largest assignment id for each elf

    // the goal is to find how many pairs that one elf's assignment can contain the other elf assignment

    let pairs = contents
        .iter()
        .filter_map(|line| pair_coverage(&line))
        .collect::<Vec<(usize, AssignedSections)>>();

    let overlap_pairs_count = contents
        .iter()
        .filter_map(|line| is_pair_overlap(&line).then(|| ()))
        .count();

    dbg!(overlap_pairs_count);
}

#[derive(Debug, Clone)]
pub struct AssignedSections {
    pub from: u64,
    pub to: u64,
}

impl FromStr for AssignedSections {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, "-");
        let from = parts
            .next()
            .ok_or(anyhow!("cannot parse the assignment from index"))?;
        let to = parts
            .next()
            .ok_or(anyhow!("cannot parse the assignment to index"))?;

        let from = from.parse::<u64>()?;
        let to = to.parse::<u64>()?;

        Ok(Self { from, to })
    }
}

impl AssignedSections {
    pub fn could_cover(&self, another: &AssignedSections) -> bool {
        return self.from <= another.from && self.to >= another.to;
    }

    pub fn could_overlap(&self, another: &AssignedSections) -> bool {
        return self.from >= another.from && self.from <= another.to
            || self.to >= another.from && self.to <= another.to
            || another.from >= self.from && another.from <= self.to
            || another.to >= self.from && another.to <= self.to;
    }
}

fn pair_coverage<S>(line: S) -> Option<(usize, AssignedSections)>
where
    S: AsRef<str>,
{
    let line = line.as_ref();

    // iterator through each elf's assignment
    // assume the first elf assignment can cover others elf's assignments, we call it the covered_assignment
    // when there is another elf's assignment range can cover the current covered_assignment, save the new covered_assignment
    // when there is a elf's assignment that the current covered_assignment cannot cover and that elf's assignment cannot cover the current covered_assignment, then we return None

    let mut assignments = line.split(",");
    let mut covered_assignment = AssignedSections::from_str(assignments.next().unwrap()).unwrap();
    let mut covered_index = 0;

    for (index, assignment) in assignments.enumerate() {
        let assignment = AssignedSections::from_str(assignment).unwrap();

        if !covered_assignment.could_cover(&assignment)
            && !assignment.could_cover(&covered_assignment)
        {
            return None;
        } else if assignment.could_cover(&covered_assignment) {
            covered_assignment = assignment.clone();
            covered_index = index + 1;
        };
    }

    Some((covered_index, covered_assignment))
}

fn is_pair_overlap<S>(line: S) -> bool
where
    S: AsRef<str>,
{
    let line = line.as_ref();

    let mut assignments = line
        .split(",")
        .map(AssignedSections::from_str)
        .collect::<Result<Vec<AssignedSections>, _>>()
        .unwrap();
    let mut i = 0;

    while i < assignments.len() - 1 {
        if !assignments[i].could_overlap(&assignments[i + 1]) {
            return false;
        }
        i += 1;
    }
    true
}
