pub mod bruteforce;
pub mod attacksum;

use square::Square;
use board::Board;

pub trait HeuristicClone {
    fn clone_box(&self) -> Box<Heuristic>;
}

impl<T> HeuristicClone for T
where
T: 'static + Heuristic + Clone,
{
    fn clone_box(&self) -> Box<Heuristic> {
	Box::new(self.clone())
    }
}

impl Clone for Box<Heuristic> {
    fn clone(&self) -> Box<Heuristic> {
	self.clone_box()
    }
}

pub trait Heuristic: HeuristicClone {
    fn calculate(&self, &Board, &Square) -> f64;
}
