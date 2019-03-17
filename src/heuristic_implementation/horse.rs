use crate::board::Board;
use crate::cell::CellContent;
use crate::heuristic::Heuristic;
use crate::heuristic_implementation::HeuristicImplementation;

pub struct Horse {
    pub weigth: f64,
}

impl Horse {
    fn check_jump(board: &Board, x: i32, y: i32) -> bool {
        let cols = *board.get_cols() as i32;

        if x < 1 || x > cols || y < 1 || y > cols {
            return false;
        }

        let (ux, uy) = (x as usize, y as usize);
        match board.get_cell_content(&ux, &uy) {
            Ok(c) => c == &CellContent::Queen,
            Err(_) => false,
        }
    }
}

impl HeuristicImplementation for Horse {
    fn new(weigth: f64) -> Self {
        Horse { weigth }
    }

    fn to_heuristic<'a>(self) -> Heuristic<'a> {
        Heuristic::new(
            "Horse",
            self.weigth,
            Box::new(|board, x, y| {
                let mut horse = false;
                let (sx, sy) = (*x as i32, *y as i32);

                horse = horse || Horse::check_jump(board, sx - 1, sy - 2);
                horse = horse || Horse::check_jump(board, sx - 2, sy - 1);
                horse = horse || Horse::check_jump(board, sx - 2, sy + 1);
                horse = horse || Horse::check_jump(board, sx - 1, sy + 2);
                horse = horse || Horse::check_jump(board, sx + 1, sy + 2);
                horse = horse || Horse::check_jump(board, sx + 2, sy + 1);
                horse = horse || Horse::check_jump(board, sx + 2, sy - 1);
                horse = horse || Horse::check_jump(board, sx + 1, sy - 2);

                if horse {
                    1.0
                } else {
                    0.0
                }
            }),
        )
    }
}
