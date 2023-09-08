//! This is the common library for the Advent of Code events.
//!
//! This library aims to reduce duplicated code and ease the development of the puzzles.

mod checker;
mod day;
mod event;
mod input;
mod part;
mod puzzle;
mod solution;
mod solver;
mod year;

// Re-export every items
pub use checker::Checker;
pub use day::*;
pub use event::*;
pub use input::*;
pub use part::*;
pub use puzzle::*;
pub use solution::*;
pub use solver::Solver;
pub use year::*;
