pub mod puzzles;
pub use puzzles::Puzzles;
pub mod utils;

pub fn run_puzzle(puzzle: Puzzles) {
    match puzzle {
        Puzzles::Day1 => puzzles::day_1::run(),
        Puzzles::Day2 => puzzles::day_2::run(),
    }
}
