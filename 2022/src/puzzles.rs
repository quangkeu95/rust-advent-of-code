pub mod day_1;
pub mod day_2;
pub mod day_3;

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum Puzzles {
    Day1,
    Day2,
    Day3,
    Day3Part2,
}

impl Puzzles {
    pub fn run(self) {
        match self {
            Puzzles::Day1 => day_1::run(),
            Puzzles::Day2 => day_2::run(),
            Puzzles::Day3 => day_3::run(),
            Puzzles::Day3Part2 => day_3::run_part_2(),
        }
    }
}
