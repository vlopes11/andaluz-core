use crate::heuristic::Heuristic;
use crate::heuristic_implementation::HeuristicImplementation;

pub struct BruteForce {
    pub weigth: f64,
}

impl HeuristicImplementation for BruteForce {
    fn new(weigth: f64) -> Self {
        BruteForce { weigth }
    }

    fn to_heuristic<'a>(self) -> Heuristic<'a> {
        Heuristic::new("BruteForce", self.weigth, Box::new(|_board, _x, _y| 1.0))
    }
}
