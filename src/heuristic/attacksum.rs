use square::Square;
use board::Board;
use heuristic::Heuristic;

#[derive(Clone)]
pub struct AttackSum {
}

impl Heuristic for AttackSum {
    fn calculate(&self, board: &Board, _square: &Square) -> f64 {
        let cols = board.cols;
        let maximum_attacks: u32 = (cols.pow(2) * 4) as u32;
        let mut attacks: u32 = 0;
	for x in 0..cols {
	    for y in 0..cols {
                attacks += board.get_square_attacks(&Square {
		    x: x + 1,
		    y: y + 1,
                });
	    }
	}
        attacks as f64 / maximum_attacks as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
	let mut board: Board = Board::new(4);
        board.reset_heuristic();
        board.inject_heuristic(AttackSum {}, 1.0);
        board.solve();
	assert!(board.solved());
    }
}
