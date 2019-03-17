use crate::board::Board;
use crate::heuristic::Heuristic;
use crate::heuristic_implementation::HeuristicImplementation;
use std::fmt;

#[derive(Debug)]
pub struct Classifier<'a> {
    total_weigth: f64,
    heuristics: Vec<Heuristic<'a>>,
}

impl<'a> Classifier<'a> {
    pub fn new() -> Self {
        Classifier {
            total_weigth: 0.0,
            heuristics: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heuristics.is_empty()
    }

    pub fn get_heuristics(&self) -> &Vec<Heuristic> {
        &self.heuristics
    }

    pub fn push_heuristic(&mut self, hi: impl HeuristicImplementation) {
        let heuristic = hi.to_heuristic();
        self.total_weigth += heuristic.weigth();
        self.heuristics.push(heuristic);
    }

    pub fn score(&self, board: &Board, x: &usize, y: &usize) -> f64 {
        let heuristic = self
            .heuristics
            .iter()
            .map(|h| h.score(board, x, y))
            .sum::<f64>();

        heuristic / self.total_weigth
    }
}

impl<'a> fmt::Display for Classifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.heuristics)
    }
}
