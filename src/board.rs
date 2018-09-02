#[derive(Debug, PartialEq)]
enum SquareContent {
    Queen,
    Attacked,
    Empty,
}

struct Square {
    pub x: usize,
    pub y: usize,
}

struct Board {
    pub cols: usize,
    cells: Vec<i32>,
}

impl Board {
    pub fn new(cols: usize) -> Board {
        Board{
            cols,
            cells: vec![0; cols * cols],
        }
    }

    fn square_to_index(&self, square: &Square) -> usize {
        square.x - 1 + self.cols * (square.y - 1)
    }

    pub fn get_cell(&self, square: &Square) -> i32 {
        self.cells[self.square_to_index(square)]
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
        let index = self.square_to_index(square);
        match self.get_square_content(square) {
            SquareContent::Empty => {
                self.execute_movement(square, -1);
                self.cells[index] = 1;
                Some(self.get_square_content(square))
            },
            SquareContent::Queen => {
                self.execute_movement(square, 1);
                self.cells[index] = 0;
                Some(self.get_square_content(square))
            },
            SquareContent::Attacked => None,
        }
    }

    fn execute_movement(&mut self, square: &Square, s: i32) {
        let index = self.square_to_index(square);
        let (x, y, c): (i32, i32, i32) =
                        (square.x as i32, square.y as i32, self.cols as i32);
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
}
