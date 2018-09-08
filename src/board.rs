use board_move::BoardMove;
use square::{SquareContent, Square};
use heuristic::Heuristic;
use std::collections::HashSet;

pub struct HeuristicWeight {
    h: Box<Heuristic>,
    w: f64,
}

impl Clone for HeuristicWeight {
    fn clone(&self) -> HeuristicWeight {
	HeuristicWeight {
	    h: self.h.clone_box(),
	    w: self.w.clone(),
	}
    }
}

pub struct BoardOptions {
    pub cols: usize,
    pub heuristics: Vec<HeuristicWeight>,
}

pub struct Board {
    pub cols: usize,
    pub signature: String,
    pub jumps: u64,
    pub heuristics: Vec<HeuristicWeight>,
    cells: Vec<i32>,
    hash: HashSet<String>,
    queen_count: usize,
}

impl Board {
    pub fn new(cols: usize) -> Board {
	Board{
	    cols,
	    queen_count: 0,
	    jumps: 0,
	    heuristics: vec![],
	    hash: HashSet::new(),
	    cells: vec![0; cols * cols],
	    signature: "0".repeat(cols * cols),
	}
    }

    pub fn from_options(options: BoardOptions) -> Board {
	Board{
	    cols: options.cols,
	    queen_count: 0,
	    jumps: 0,
	    heuristics: options.heuristics,
	    hash: HashSet::new(),
	    cells: vec![0; options.cols * options.cols],
	    signature: "0".repeat(options.cols * options.cols),
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

    pub fn solve(&mut self) -> bool {
	self.jumps = 0;
	self.hash.clear();

	self.solve_internal()
    }

    pub fn get_available_moves(&mut self) -> Vec<BoardMove> {
	let mut v = Vec::with_capacity(self.cols * self.cols);
	for x in 0..self.cols {
	    for y in 0..self.cols {
		let s = Square {
		    x: x + 1,
		    y: y + 1,
		};
		match self.get_square_content(&s) {
		    SquareContent::Empty => {
			let h = self.calc_heuristic(&s);
			v.push(BoardMove {
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

    pub fn reset_heuristic(&mut self) { self.heuristics.clear(); }

    pub fn inject_heuristic<T: Heuristic + 'static>(&mut self, heuristic: T, weight: f64)
    {
        self.heuristics.push(HeuristicWeight {
            h: Box::new(heuristic),
            w: weight
        });
    }

    pub fn calc_heuristic(&mut self, square: &Square) -> f64 {
	let mut h = 0.0;
	let mut total_weight = 0.0;
	let mut calculated_heuristics: Vec<(f64, f64)> =
	    Vec::with_capacity(self.heuristics.len());

	self.put_queen(square);
	for heuristic_weight in self.heuristics.iter().cloned() {
	    calculated_heuristics.push((
		    heuristic_weight.h.calculate(&self, square),
		    heuristic_weight.w));
	    total_weight += heuristic_weight.w;
	}
	self.put_queen(square);

	if total_weight >= 0.0 {
	    for (heuristic, weigth) in calculated_heuristics {
		h += heuristic * weigth / total_weight;
	    }
	}

	h
    }

    fn solve_internal(&mut self) -> bool {
	if self.solved() {
	    return true;
	} else if self.hash.contains(&self.signature) {
	    return false;
	}

	self.jumps += 1;

	let moves = self.get_available_moves();
	for m in moves {
	    self.put_queen(&m.s);
	    self.solve_internal();
	    if ! self.solved() {
		self.put_queen(&m.s);
		self.hash.insert(self.signature.clone());
	    }
	}

	self.solved()
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

    #[test]
    fn solved_without_heuristics() {
	let mut board: Board = Board::from_options(
            BoardOptions {
                cols: 4,
                heuristics: vec![],
            });
        board.solve();
	assert!(board.solved());
    }
}
