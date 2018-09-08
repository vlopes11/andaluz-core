use square::{SquareContent, Square};
use board::Board;
use heuristic::Heuristic;

#[derive(Clone)]
pub struct Horse {
}

impl Horse {
    fn check_jump(&self, board: &Board, square: &Square, x: i32, y: i32) -> bool {
        let cols = board.cols as i32;
        let sx = square.x as i32;
        let sy = square.y as i32;
        if sx + x < 1 || sx + x > cols || sy + y < 1 || sy + y > cols {
            return false;
        }
        board.get_square_content(&Square {
            x: (sx + x) as usize,
            y: (sy + y) as usize,
        }) == SquareContent::Queen
    }
}

impl Heuristic for Horse {
    fn calculate(&self, board: &Board, square: &Square) -> f64 {
        let mut horse = false;

        horse = horse || self.check_jump(board, square, -1, -2);
        horse = horse || self.check_jump(board, square, -2, -1);
        horse = horse || self.check_jump(board, square, -2, 1);
        horse = horse || self.check_jump(board, square, -1, 2);
        horse = horse || self.check_jump(board, square, 1, 2);
        horse = horse || self.check_jump(board, square, 2, 1);
        horse = horse || self.check_jump(board, square, 2, -1);
        horse = horse || self.check_jump(board, square, 1, -2);

        if horse {1.0} else {0.0}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
	let mut board: Board = Board::new(4);
        board.reset_heuristic();
        board.inject_heuristic(Horse {}, 1.0);
        board.solve();
	assert!(board.solved());
    }
}
