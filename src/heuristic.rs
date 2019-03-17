use crate::board::Board;
use std::fmt;

pub struct Heuristic<'a> {
    label: &'a str,
    weigth: f64,
    implementation: Box<Fn(&Board, &usize, &usize) -> f64>,
}

impl<'a> fmt::Debug for Heuristic<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.label, self.weigth)
    }
}

impl<'a> Heuristic<'a> {
    pub fn new(
        label: &'a str,
        weigth: f64,
        implementation: Box<Fn(&Board, &usize, &usize) -> f64>,
    ) -> Self {
        Heuristic {
            label,
            weigth,
            implementation,
        }
    }

    pub fn weigth(&self) -> &f64 {
        &self.weigth
    }

    pub fn score(&self, board: &Board, x: &usize, y: &usize) -> f64 {
        self.weigth * (self.implementation)(board, x, y)
    }
}
