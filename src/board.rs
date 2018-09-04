use std::cmp::Ordering;
use solver::Solver;

#[derive(Debug, PartialEq)]
pub enum SquareContent {
    Queen,
    Attacked,
    Empty,
}

pub struct Square {
    pub x: usize,
    pub y: usize,
}

pub struct Move {
    pub s: Square,
    pub h: f64,
}

impl PartialEq for Move {
    fn eq(&self, other: &Move) -> bool {
        self.s.x == other.s.x && self.s.y == other.s.y
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Move) -> Option<Ordering> {
        self.h.partial_cmp(&other.h)
    }
}

impl Eq for Move {}

impl Ord for Move {
    fn cmp(&self, other: &Move) -> Ordering {
        match self.partial_cmp(other) {
            Some(m) => m,
            None => Ordering::Equal,
        }
    }
}

pub struct Board {
    pub cols: usize,
    pub signature: String,
    queen_count: usize,
    cells: Vec<i32>,
}

impl Board {
    pub fn new(cols: usize) -> Board {
        Board{
            cols,
            queen_count: 0,
            cells: vec![0; cols * cols],
            signature: "0".repeat(cols * cols),
        }
    }

    pub fn get_square_content(&self, square: &Square) -> SquareContent {
        match self.get_cell(square) {
            0 => SquareContent::Empty,
            1 => SquareContent::Queen,
            _ => SquareContent::Attacked,
        }
    }

    pub fn get_square_attacks(&self, square: &Square) -> u32 {
        let cell = self.get_cell(square);
        match cell {
            0 => 0,
            1 => 0,
            _ => (cell * (-1)) as u32,
        }
    }

    pub fn put_queen(&mut self, square: &Square) -> Option<SquareContent> {
        let square_content = self.get_square_content(square);
        match square_content {
            SquareContent::Empty | SquareContent::Queen => {
                self.execute_movement(square, square_content);
                Some(self.get_square_content(square))
            },
            _ => None,
        }
    }

    pub fn solved(&self) -> bool {
        self.queen_count == self.cols
    }

    pub fn get_available_moves(&mut self, solver: &Solver) -> Vec<Move> {
        let mut v = Vec::with_capacity(self.cols * self.cols);
        for x in 0..self.cols {
            for y in 0..self.cols {
                let s = Square {
                    x: x + 1,
                    y: y + 1,
                };
                match self.get_square_content(&s) {
                    SquareContent::Empty => {
                        let h = solver.calc_heuristic(self, &s);
                        v.push(Move {
                            s,
                            h,
                        })
                    },
                    _ => {}
                };
            }
        }
        v.sort_by(|a, b| b.cmp(a));
        v
    }

    fn square_to_index(&self, square: &Square) -> usize {
        square.x - 1 + self.cols * (square.y - 1)
    }

    fn get_cell(&self, square: &Square) -> i32 {
        self.cells[self.square_to_index(square)]
    }

    fn execute_movement(&mut self, square: &Square, square_content: SquareContent) {
        let index = self.square_to_index(square);
        let (x, y, c): (i32, i32, i32) =
                        (square.x as i32, square.y as i32, self.cols as i32);
        let s: i32 = match square_content {
            SquareContent::Queen => {
                self.queen_count -= 1;
                1
            },
            SquareContent::Empty => {
                self.queen_count += 1;
                -1
            },
            _ => 0,
        };
        self.cells[index] -= s;
        // Horizontal
        {
            let min = ((y - 1) * c) as usize;
            let max = (y * c) as usize;
            for p in min..max {
                if p != index {
                    self.cells[p] += s;
                }
            }
        }
        // Vertical
        {
            let min = (x - 1) as usize;
            let max = (c * (c - 1) + x) as usize;
            for p in (min..max).step_by(self.cols) {
                if p != index {
                    self.cells[p] += s;
                }
            }
        }
        // Diagonal lower left -> top right
        {
            let (min, max) = 
                if x + y <= c + 1 {
                    ((x + y - 2) as usize,
                     (c * (x + y - 2) + 1) as usize)
                } else {
                    ((c * (y + x - c) - 1) as usize,
                    (x + y + c * (c - 2)) as usize)
                };
            for p in (min..max).step_by(self.cols - 1) {
                if p != index {
                    self.cells[p] += s;
                }
            }
        }
        // Diagonal top left -> lower right
        {
            let (min, max) = 
                if x >= y {
                    ((x - y) as usize,
                     (c * (c + y - x)) as usize)
                } else {
                    ((c * (y - x)) as usize,
                    (x - y + c.pow(2)) as usize)
                };
            for p in (min..max).step_by(self.cols + 1) {
                if p != index {
                    self.cells[p] += s;
                }
            }
        }

        self.signature = String::with_capacity(self.cells.len());
        for cell in &self.cells {
            self.signature.push(
                match cell {
                    1 => '1',
                    _ => '0',
                }
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_new() {
        let board: Board = Board::new(8);
        assert_eq!(board.get_square_content(
                &Square {x: 1, y: 1}), SquareContent::Empty);
    }

    #[test]
    fn single_move() {
        let mut board: Board = Board::new(8);
        board.put_queen(&Square {x: 1, y: 2});
        assert_eq!(board.get_square_content(
                &Square {x: 1, y: 2}), SquareContent::Queen);
        assert_eq!(board.get_square_content(
                &Square {x: 4, y: 5}), SquareContent::Attacked);
        assert_eq!(board.get_square_attacks(
                &Square {x: 4, y: 5}), 1);
        assert_eq!(board.get_square_content(
                &Square {x: 3, y: 1}), SquareContent::Empty);
    }

    #[test]
    fn double_move() {
        let mut board: Board = Board::new(8);
        board.put_queen(&Square {x: 1, y: 2});
        board.put_queen(&Square {x: 3, y: 1});
        assert_eq!(board.get_square_content(
                &Square {x: 1, y: 2}), SquareContent::Queen);
        assert_eq!(board.get_square_content(
                &Square {x: 2, y: 2}), SquareContent::Attacked);
        assert_eq!(board.get_square_attacks(
                &Square {x: 2, y: 2}), 2);
        assert_eq!(board.get_square_content(
                &Square {x: 4, y: 3}), SquareContent::Empty);
    }

    #[test]
    fn null_move() {
        let mut board: Board = Board::new(8);
        board.put_queen(&Square {x: 1, y: 2});
        board.put_queen(&Square {x: 1, y: 2});
        for x in 1..9 {
            for y in 1..9 {
                assert_eq!(board.get_square_content(
                        &Square {x, y}), SquareContent::Empty);
            }
        }
    }

    #[test]
    fn not_solved() {
        let mut board: Board = Board::new(8);
        board.put_queen(&Square {x: 1, y: 2});
        assert_eq!(board.solved(), false);
    }

    #[test]
    fn solved() {
        let mut board: Board = Board::new(8);
        board.put_queen(&Square {x: 1, y: 5});
        board.put_queen(&Square {x: 2, y: 3});
        board.put_queen(&Square {x: 3, y: 1});
        board.put_queen(&Square {x: 4, y: 7});
        board.put_queen(&Square {x: 5, y: 2});
        board.put_queen(&Square {x: 6, y: 8});
        board.put_queen(&Square {x: 7, y: 6});
        board.put_queen(&Square {x: 8, y: 4});
        assert!(board.solved());
    }

    #[test]
    fn signature() {
        let mut board: Board = Board::new(8);
        board.put_queen(&Square {x: 1, y: 5});
        board.put_queen(&Square {x: 2, y: 3});
        board.put_queen(&Square {x: 3, y: 1});
        board.put_queen(&Square {x: 4, y: 7});
        board.put_queen(&Square {x: 5, y: 2});
        board.put_queen(&Square {x: 6, y: 8});
        board.put_queen(&Square {x: 7, y: 6});
        board.put_queen(&Square {x: 8, y: 4});
        assert_eq!(
            "0010000000001000010000000000000110000000000000100001000000000100",
            board.signature);
    }
}
