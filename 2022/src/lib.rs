pub mod puzzles;
pub use puzzles::Puzzles;
pub mod utils;

pub fn run_puzzle(puzzle: Puzzles) {
    puzzle.run()
}
