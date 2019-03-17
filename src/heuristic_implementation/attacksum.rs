use crate::cell::CellContent;
use crate::heuristic::Heuristic;
use crate::heuristic_implementation::HeuristicImplementation;

pub struct AttackSum {
    pub weigth: f64,
}

impl HeuristicImplementation for AttackSum {
    fn new(weigth: f64) -> Self {
        AttackSum { weigth }
    }

    fn to_heuristic<'a>(self) -> Heuristic<'a> {
        Heuristic::new(
            "AttackSum",
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
                sum_attack / maximum
            }),
        )
    }
}
