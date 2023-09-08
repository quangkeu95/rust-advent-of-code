pub mod day_1;
pub mod day_2;

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub enum Puzzles {
    Day1,
    Day2,
}
