use crate::cell::{Cell, CellContent};
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct Board {
    cols: usize,
    cells: Vec<Cell>,
    signature: Vec<u8>,
    rotated_signatures: Option<Vec<Vec<u8>>>,
}

impl Board {
    /// Constructor
    ///
    /// # Example
    ///
    /// ```
    /// use andaluz_core::board::Board;
    /// use andaluz_core::cell::CellContent;
    ///
    /// let board = Board::new(6);
    ///
    /// board.get_cells()
    ///     .iter()
    ///     .for_each(|c| assert_eq!(c.get_content(), &CellContent::Empty));
    ///
    /// assert_eq!(board.get_signature(), &vec![0_u8; 5]);
    /// ```
    pub fn new(cols: usize) -> Self {
        let cells = Vec::with_capacity(cols * cols);
        let signature = Vec::with_capacity(1 + cols * cols / 8);
        let mut board = Board {
            cols,
            cells,
            signature,
            rotated_signatures: None,
        };

        board.reset();
        board
    }

    fn get_i_from_xy(&self, x: &usize, y: &usize) -> Result<usize, String> {
        let i = x - 1 + self.cols * (y - 1);
        if i >= self.cells.len() {
            Err(format!("Coordinates ({}, {}) out of bounds", x, y))
        } else {
            Ok(i)
        }
    }

    pub fn get_cell(&self, x: &usize, y: &usize) -> Result<&Cell, String> {
        Ok(&self.cells[self.get_i_from_xy(x, y)?])
    }

    pub fn get_mut_cell(&mut self, x: &usize, y: &usize) -> Result<&mut Cell, String> {
        let i = self.get_i_from_xy(x, y)?;
        let cell = &mut self.cells[i];
        Ok(cell)
    }

    pub fn get_cell_content(&self, x: &usize, y: &usize) -> Result<&CellContent, String> {
        Ok(self.get_cell(x, y)?.get_content())
    }

    pub fn get_cell_attacks(&self, x: &usize, y: &usize) -> Result<&u8, String> {
        let content = self.get_cell_content(x, y)?;
        match content {
            CellContent::Attack(n) => Ok(&n),
            _ => Err("Cell not attacked".to_string()),
        }
    }

    /// Put or remove a queen from a cell
    ///
    /// # Example
    ///
    /// ```
    /// use andaluz_core::board::Board;
    /// use andaluz_core::cell::CellContent;
    ///
    /// let mut board = Board::new(8);
    ///
    /// board.toggle_cell(&2, &2).unwrap();
    ///
    /// assert_eq!(board.get_cell_content(&2, &2).unwrap(), &CellContent::Queen);
    /// assert_eq!(board.get_signature(), &vec![0_u8, 64, 0, 0, 0, 0, 0, 0]);
    /// ```
    pub fn toggle_cell(&mut self, x: &usize, y: &usize) -> Result<(), String> {
        let i = self.get_i_from_xy(x, y)?;

        let content = self.cells[i].toggle()?;
        let attack = content == &CellContent::Queen;

        // TODO - Improve performance, some signature masks should do the trick and predict the
        // positions to be attacked or relieved

        let (xs, ys, cs) = (*x as i32, *y as i32, self.cols as i32);

        // Horizontal
        {
            let min = ((ys - 1) * cs) as usize;
            let max = (ys * cs) as usize;
            for p in min..max {
                if p != i {
                    self.cells[p].attack_or_relieve(&attack)?;
                }
            }
        }

        // Vertical
        {
            let min = (xs - 1) as usize;
            let max = (cs * (cs - 1) + xs) as usize;
            for p in (min..max).step_by(self.cols) {
                if p != i {
                    self.cells[p].attack_or_relieve(&attack)?;
                }
            }
        }

        // Diagonal lower left -> top right
        {
            let (min, max) = if xs + ys <= cs + 1 {
                ((xs + ys - 2) as usize, (cs * (xs + ys - 2) + 1) as usize)
            } else {
                (
                    (cs * (ys + xs - cs) - 1) as usize,
                    (xs + ys + cs * (cs - 2)) as usize,
                )
            };
            for p in (min..max).step_by(self.cols - 1) {
                if p != i {
                    self.cells[p].attack_or_relieve(&attack)?;
                }
            }
        }

        // Diagonal top left -> lower right
        {
            let (min, max) = if xs >= ys {
                ((xs - ys) as usize, (cs * (cs + ys - xs)) as usize)
            } else {
                ((cs * (ys - xs)) as usize, (xs - ys + cs.pow(2)) as usize)
            };
            for p in (min..max).step_by(self.cols + 1) {
                if p != i {
                    self.cells[p].attack_or_relieve(&attack)?;
                }
            }
        }

        // Update signature, single XOR will do the trick
        let sig_pos = i / 8;
        let sig_mask = 2_u8.pow((7 - (i - sig_pos * 8)) as u32);
        self.signature[sig_pos] ^= sig_mask;
        self.rotated_signatures = None;

        Ok(())
    }

    /// Re-initialize the board
    ///
    /// # Example
    ///
    /// ```
    /// use andaluz_core::board::Board;
    /// use andaluz_core::cell::CellContent;
    ///
    /// let mut  board = Board::new(6);
    ///
    /// board.toggle_cell(&1, &2).unwrap();
    /// board.reset();
    ///
    /// board.get_cells()
    ///     .iter()
    ///     .for_each(|c| assert_eq!(c.get_content(), &CellContent::Empty));
    ///
    /// assert_eq!(board.get_signature(), &vec![0_u8; 5]);
    /// ```
    pub fn reset(&mut self) {
        self.cells.clear();
        for y in 1..=self.cols {
            for x in 1..=self.cols {
                let i = self.cells.len();
                self.cells.push(Cell::new(x, y, i));
            }
        }

        self.signature.clear();
        let size = (((self.cols * self.cols) as f64) / 8.0_f64).ceil() as usize;
        for _ in 0..size {
            self.signature.push(0);
        }
    }

    pub fn mirror(&self) -> Result<Board, String> {
        let mut board = Board::new(self.cols.clone());

        for c in self.get_cells() {
            match c.get_content() {
                CellContent::Queen => {
                    let (x, y, _) = c.get_xyi();
                    board.toggle_cell(&y, &x)?;
                }
                _ => {}
            }
        }

        Ok(board)
    }

    pub fn get_cols(&self) -> &usize {
        &self.cols
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn get_available_cells(&self) -> Vec<Cell> {
        self.cells.iter().fold(vec![], |mut cells, c| {
            if let CellContent::Empty = c.get_content() {
                cells.push(c.clone());
            }

            cells
        })
    }

    pub fn get_signature(&self) -> &Vec<u8> {
        &self.signature
    }

    pub fn to_relative_coord(&self, x: &usize, y: &usize) -> (i32, i32) {
        let (fx, fy) = (*x as f64, *y as f64);
        let center = (self.cols as f64) / 2.0 + 0.5;

        let mut rx = ((fx - center).abs() + 0.5) as i32;
        let mut ry = ((fy - center).abs() + 0.5) as i32;

        if fx < center {
            rx = 0 - rx;
        }

        if fy < center {
            ry = 0 - ry;
        }

        (rx, ry)
    }

    fn from_relative_coord(&self, x: &i32, y: &i32) -> (usize, usize) {
        let center = (self.cols / 2) as i32;
        let even_cols = self.cols % 2 == 0;

        let mut ax = x + center;
        let mut ay = y + center;

        if !even_cols || x < &0 {
            ax += 1;
        }

        if !even_cols || y < &0 {
            ay += 1;
        }

        (ax as usize, ay as usize)
    }

    fn rotate_counterclockwise(&self, x: &usize, y: &usize) -> (usize, usize) {
        let (rx, ry) = self.to_relative_coord(x, y);
        let fx = -ry;
        let fy = rx;
        let (nx, ny) = self.from_relative_coord(&fx, &fy);
        (nx, ny)
    }

    /// Return the permutations of clockwise rotations
    ///
    /// # Example
    ///
    /// ```
    /// use andaluz_core::board::Board;
    /// use andaluz_core::cell::CellContent;
    ///
    /// let mut board = Board::new(8);
    ///
    /// board.toggle_cell(&1, &2).unwrap();
    /// board.toggle_cell(&2, &5).unwrap();
    /// board.toggle_cell(&3, &7).unwrap();
    /// board.toggle_cell(&4, &4).unwrap();
    /// board.toggle_cell(&5, &1).unwrap();
    /// board.toggle_cell(&6, &8).unwrap();
    /// board.toggle_cell(&7, &6).unwrap();
    /// board.toggle_cell(&8, &3).unwrap();
    ///
    /// let rs = board.get_equivalent_signatures().unwrap();
    ///
    /// assert_eq!(rs[0], vec![8, 128, 1, 16, 64, 2, 32, 4]);
    /// assert_eq!(rs[1], vec![2, 16, 64, 8, 1, 128, 32, 4]);
    /// assert_eq!(rs[2], vec![32, 4, 64, 2, 8, 128, 1, 16]);
    /// assert_eq!(rs[3], vec![32, 4, 1, 128, 16, 2, 8, 64]);
    /// assert_eq!(rs[4], vec![64, 8, 2, 16, 128, 1, 4, 32]);
    /// assert_eq!(rs[5], vec![16, 1, 128, 8, 2, 64, 4, 32]);
    /// assert_eq!(rs[6], vec![4, 32, 128, 1, 8, 64, 16, 2]);
    /// assert_eq!(rs[7], vec![4, 32, 2, 64, 16, 1, 128, 8]);
    /// ```
    pub fn get_equivalent_signatures(&mut self) -> Result<Vec<Vec<u8>>, String> {
        let mut signatures = self.get_rotated_signatures()?;

        let mut mirror_board = self.mirror()?;
        let mut mirror_signatures = mirror_board.get_rotated_signatures()?;

        signatures.append(&mut mirror_signatures);

        Ok(signatures)
    }

    fn get_rotated_signatures(&mut self) -> Result<Vec<Vec<u8>>, String> {
        let rotated_signatures = match &mut self.rotated_signatures {
            Some(r) => r.clone(),
            None => {
                let mut v = vec![];

                v.push(self.get_signature().clone());

                let mut fake_board = Board::new(self.cols.clone());
                let mut fake_queens = vec![];

                self.get_cells().iter().for_each(|c| match c.get_content() {
                    CellContent::Queen => {
                        let (x, y, _) = c.get_xyi();
                        fake_queens.push((*x, *y));
                    }
                    _ => {}
                });

                // Rotate 3 times
                for _ in 0..3 {
                    fake_board.reset();
                    fake_queens = fake_queens
                        .iter()
                        .map(|(x, y)| fake_board.rotate_counterclockwise(&x, &y))
                        .collect::<Vec<(usize, usize)>>();

                    for (x, y) in &fake_queens {
                        fake_board.toggle_cell(x, y)?;
                    }

                    v.push(fake_board.get_signature().clone());
                }

                v
            }
        };

        if self.rotated_signatures.is_none() {
            self.rotated_signatures = Some(rotated_signatures.clone());
        }

        Ok(rotated_signatures)
    }

    pub fn is_solved(&self) -> bool {
        // TODO - not correct
        let queens = self
            .get_cells()
            .iter()
            .map(|c| match c.get_content() {
                CellContent::Queen => 1,
                _ => 0,
            })
            .sum::<usize>();
        queens == self.cols
    }

    pub fn to_multiline_string(&self) -> String {
        let mut board = String::from("");
        self.get_signature()
            .iter()
            .rev()
            .for_each(|s| board = format!("{}{:08b}\n", board, s));
        board
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut board = String::from("");
        self.get_signature()
            .iter()
            .for_each(|s| board = format!("{}{:08b}", board, s));
        board
    }
}

impl From<Vec<u8>> for Board {
    fn from(signature: Vec<u8>) -> Self {
        let mut board = String::from("");

        signature
            .iter()
            .for_each(|s| board = format!("{}{:08b}", board, s));

        Board::from(board)
    }
}

impl From<String> for Board {
    fn from(string: String) -> Self {
        let cols = (((string.len()) as f64).sqrt()) as usize;
        let mut board = Board::new(cols);
        let cells = board.get_cells().clone();

        for (i, c) in string.as_str().char_indices() {
            if c == '1' {
                let (x, y, _) = cells[i].get_xyi();
                board.toggle_cell(x, y).unwrap();
            }
        }

        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_move() {
        let mut board: Board = Board::new(8);
        board.toggle_cell(&1, &2).unwrap();
        assert_eq!(board.get_cell_content(&1, &2).unwrap(), &CellContent::Queen);
        assert_eq!(board.get_cell_attacks(&4, &5).unwrap(), &1);
        assert_eq!(board.get_cell_content(&3, &1).unwrap(), &CellContent::Empty);
    }

    #[test]
    fn double_move() {
        let mut board: Board = Board::new(8);
        board.toggle_cell(&1, &2).unwrap();
        board.toggle_cell(&3, &1).unwrap();
        assert_eq!(board.get_cell_content(&1, &2).unwrap(), &CellContent::Queen);
        assert_eq!(board.get_cell_attacks(&2, &2).unwrap(), &2);
        assert_eq!(board.get_cell_content(&4, &3).unwrap(), &CellContent::Empty);
    }

    #[test]
    fn null_move() {
        let mut board: Board = Board::new(8);

        board.toggle_cell(&1, &2).unwrap();
        board.toggle_cell(&1, &2).unwrap();

        assert_eq!(board.get_signature(), &vec![0_u8; 8]);

        board
            .get_cells()
            .iter()
            .for_each(|c| assert_eq!(c.get_content(), &CellContent::Empty));
    }

    #[test]
    fn is_solved() {
        let mut board = Board::new(12);

        board.toggle_cell(&7, &1).unwrap();
        board.toggle_cell(&9, &2).unwrap();
        board.toggle_cell(&11, &3).unwrap();
        board.toggle_cell(&2, &4).unwrap();
        board.toggle_cell(&4, &5).unwrap();

        assert!(!board.is_solved());

        board.toggle_cell(&6, &6).unwrap();
        board.toggle_cell(&3, &7).unwrap();
        board.toggle_cell(&10, &8).unwrap();
        board.toggle_cell(&12, &9).unwrap();
        board.toggle_cell(&5, &10).unwrap();
        board.toggle_cell(&8, &11).unwrap();
        board.toggle_cell(&1, &12).unwrap();

        assert!(board.is_solved());

        assert_eq!(
            board.get_signature(),
            &vec![2_u8, 0, 8, 0, 36, 0, 16, 0, 64, 32, 0, 4, 0, 16, 128, 1, 8, 0]
        );
    }

    #[test]
    fn from_signature_to_string() {
        let mut board = Board::new(12);

        board.toggle_cell(&7, &1).unwrap();
        board.toggle_cell(&9, &2).unwrap();
        board.toggle_cell(&11, &3).unwrap();
        board.toggle_cell(&2, &4).unwrap();
        board.toggle_cell(&4, &5).unwrap();
        board.toggle_cell(&6, &6).unwrap();
        board.toggle_cell(&3, &7).unwrap();
        board.toggle_cell(&10, &8).unwrap();
        board.toggle_cell(&12, &9).unwrap();
        board.toggle_cell(&5, &10).unwrap();
        board.toggle_cell(&8, &11).unwrap();
        board.toggle_cell(&1, &12).unwrap();

        let signature = board.get_signature().clone();
        let board = Board::from(signature.clone());
        let board = board.to_string();
        let board = Board::from(board);

        assert_eq!(board.get_signature(), &signature);
    }
}
