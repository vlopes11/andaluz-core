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
