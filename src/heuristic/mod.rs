pub mod bruteforce;

use square::Square;
use board::Board;

pub trait Heuristic {
    fn calculate(&self, &Board, &Square) -> f64;
}
