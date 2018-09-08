use square::Square;
use board::Board;
use heuristic::Heuristic;

#[derive(Clone)]
pub struct BruteForce {
}

impl Heuristic for BruteForce {
    fn calculate(&self, _board: &Board, _square: &Square) -> f64 {
	1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
	let board = Board::new(4);
	let square = Square {x: 1, y: 1};
	let heuristic = BruteForce {};
	assert_eq!(heuristic.calculate(&board, &square), 1.0);
    }
}
