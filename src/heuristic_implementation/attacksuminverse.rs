use crate::cell::CellContent;
use crate::heuristic::Heuristic;
use crate::heuristic_implementation::HeuristicImplementation;

pub struct AttackSumInverse {
    pub weigth: f64,
}

impl HeuristicImplementation for AttackSumInverse {
    fn new(weigth: f64) -> Self {
        AttackSumInverse { weigth }
    }

    fn to_heuristic<'a>(self) -> Heuristic<'a> {
        Heuristic::new(
            "AttackSumInverse",
            self.weigth,
            Box::new(|board, _x, _y| {
                let sum_attack: f64 = board
                    .get_cells()
                    .iter()
                    .map(|c| match c.get_content() {
                        &CellContent::Attack(a) => a as f64,
                        _ => 0.0,
                    })
                    .sum();
                let maximum = 4.0 * (board.get_cols() * board.get_cols()) as f64;
                (maximum - sum_attack) / maximum
            }),
        )
    }
}
