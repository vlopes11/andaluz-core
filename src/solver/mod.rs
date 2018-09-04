pub mod bruteforce;

use board::{Board, Square};

pub trait Solver {
    fn solve(&mut self, &mut Board) -> bool;
    fn calc_heuristic(&self, &mut Board, &Square) -> f64;
    fn get_jumps(&self) -> u64;
}
