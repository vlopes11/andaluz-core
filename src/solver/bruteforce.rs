use square::Square;
use board::Board;
use solver::Solver;

pub struct BruteForce {
    jumps: u64,
}

impl Solver for BruteForce {
    fn solve(&mut self, board: &mut Board) -> bool {
        if board.solved() {
            return true;
        }

        self.jumps += 1;

        let moves = board.get_available_moves(self);
        for m in moves {
            board.put_queen(&m.s);
            self.solve(board);
            if ! board.solved() {
                board.put_queen(&m.s);
            }
        }

        board.solved()
    }

    fn calc_heuristic(&self, _board: &mut Board, _square: &Square) -> f64 {
        1.0
    }

    fn get_jumps(&self) -> u64 {
        self.jumps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let mut board: Board = Board::new(4);
        let mut solver: BruteForce = BruteForce {jumps: 0};
        solver.solve(&mut board);
        assert!(board.solved());
    }
}
