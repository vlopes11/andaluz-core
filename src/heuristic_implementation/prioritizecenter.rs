use crate::heuristic::Heuristic;
use crate::heuristic_implementation::HeuristicImplementation;

pub struct PrioritizeCenter {
    pub weigth: f64,
}

impl HeuristicImplementation for PrioritizeCenter {
    fn new(weigth: f64) -> Self {
        PrioritizeCenter { weigth }
    }

    fn to_heuristic<'a>(self) -> Heuristic<'a> {
        Heuristic::new(
            "PrioritizeCenter",
            self.weigth,
            Box::new(|board, x, y| {
                let mid = board.get_cols() / 2;
                if x == &mid && y == &mid {
                    1.0
                } else {
                    0.0
                }
            }),
        )
    }
}
