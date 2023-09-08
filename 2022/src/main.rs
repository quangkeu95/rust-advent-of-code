use aoc_2022::{run_puzzle, Puzzles};
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub puzzle: Puzzles,
}

fn main() {
    let cli = Cli::parse();

    run_puzzle(cli.puzzle);
}
