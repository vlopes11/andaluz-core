use crate::console::Console;
use andaluz_core::board::Board;
use andaluz_core::heuristic::Heuristic;
use andaluz_core::heuristic_implementation::attacksum::AttackSum;
use andaluz_core::heuristic_implementation::attacksuminverse::AttackSumInverse;
use andaluz_core::heuristic_implementation::horse::Horse;
use andaluz_core::heuristic_implementation::prioritizecenter::PrioritizeCenter;
use andaluz_core::heuristic_implementation::HeuristicImplementation;
use andaluz_core::solver::Solver;
use std::sync::{Arc, Mutex};
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::ClickEvent;
use stdweb::web::html_element::{CanvasElement, ImageElement};
use stdweb::web::{document, window, CanvasRenderingContext2d};

#[derive(Debug, Clone)]
pub struct Canvas {
    width: f64,
    height: f64,
    cols: f64,
    square_size: f64,
    max_jumps: f64,
    selector: CanvasElement,
    context: CanvasRenderingContext2d,
    queen_svg: ImageElement,
    attack_svg: ImageElement,
    board: Arc<Mutex<Board>>,
    console: Arc<Mutex<Console>>,
}

impl Canvas {
    pub fn new(board: Arc<Mutex<Board>>, console: Arc<Mutex<Console>>) -> Self {
        let selector: CanvasElement = document()
            .query_selector("#andaluz-canvas")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let queen_svg: ImageElement = document()
            .query_selector("#andaluz-queen")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let attack_svg: ImageElement = document()
            .query_selector("#andaluz-attack")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let (width, height) = (selector.width() as f64, selector.height() as f64);

        let context: CanvasRenderingContext2d = selector.get_context().unwrap();

        let mut canvas = Canvas {
            selector,
            context,
            width,
            height,
            square_size: 0.0,
            cols: 8.0,
            max_jumps: 100000.0,
            board,
            queen_svg,
            attack_svg,
            console,
        };

        canvas.set_cols(8.0);
        canvas
    }

    pub fn set_cols(&mut self, cols: f64) {
        {
            let mut board = self.board.lock().unwrap();
            board.resize(cols as usize);
        }
        self.square_size = self.width / cols;
        self.cols = cols;

        self.redraw();
    }

    pub fn reset(&self) {
        {
            let mut board = self.board.lock().unwrap();
            board.reset();
        };
        self.redraw();
    }

    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.width, self.height);
    }

    pub fn redraw(&self) {
        self.clear();

        let c = self.cols as usize;

        for x in 1..=c {
            for y in 1..=c {
                self.draw_square(&(x as f64), &(y as f64));
            }
        }
    }

    pub fn solve(
        &mut self,
        attack_sum: f64,
        attack_sum_inverse: f64,
        horse: f64,
        prioritize_center: f64,
    ) {
        let (solved, jumps) = {
            let mut board = self.board.lock().unwrap();
            let mut solver = Solver::new();
            solver.set_max_jumps(self.max_jumps as u32);

            if attack_sum > 0.0 {
                solver.push_heuristic(AttackSum::new(attack_sum));
            }

            if attack_sum_inverse > 0.0 {
                solver.push_heuristic(AttackSumInverse::new(attack_sum_inverse));
            }

            if horse > 0.0 {
                solver.push_heuristic(Horse::new(horse));
            }

            if prioritize_center > 0.0 {
                solver.push_heuristic(PrioritizeCenter::new(prioritize_center));
            }

            let result = solver.solve(&mut board).unwrap();
            (result.is_solved(), result.get_jumps().clone())
        };

        if solved {
            self.println("Solution found!".to_string());
        } else {
            self.println("No solution found!".to_string());
        }

        self.println(format!("Jumps performed: {}", jumps));
        self.redraw();
    }

    fn draw_square(&self, x: &f64, y: &f64) {
        let top_left_x = self.square_size * (x - 1.0);
        let top_left_y = self.square_size * (self.cols - y);

        let dark = is_even(x) && is_even(y) || !is_even(x) && !is_even(y);
        let color = if dark { "#566573" } else { "#eaecee" };

        self.context.set_fill_style_color(color);
        self.context.fill_rect(
            top_left_x.clone(),
            top_left_y.clone(),
            self.square_size,
            self.square_size,
        );

        let (is_queen, is_attacked) = {
            let board = self.board.lock().unwrap();
            let cell = board.get_cell(&(*x as usize), &(*y as usize)).unwrap();

            let is_queen = cell.is_queen();
            let is_attacked = cell.is_attacked();

            (is_queen, is_attacked)
        };

        if is_queen {
            self.context
                .draw_image_d(
                    self.queen_svg.clone(),
                    top_left_x.clone(),
                    top_left_y.clone(),
                    self.square_size,
                    self.square_size,
                )
                .unwrap();
        } else if is_attacked {
            self.context
                .draw_image_d(
                    self.attack_svg.clone(),
                    top_left_x.clone(),
                    top_left_y.clone(),
                    self.square_size,
                    self.square_size,
                )
                .unwrap();
        }
    }

    pub fn get_selector(&self) -> &CanvasElement {
        &self.selector
    }

    pub fn println(&self, txt: String) {
        let mut console = self.console.lock().unwrap();
        console.println(txt);
    }

    pub fn offset_to_xy(&self, offset_x: f64, offset_y: f64) -> (f64, f64) {
        let x = (offset_x / self.square_size).trunc() + 1.0;
        let y = self.cols - (offset_y / self.square_size).trunc();
        (x, y)
    }

    pub fn toggle_cell(&mut self, x: &f64, y: &f64) {
        let solved = {
            let board = self.board.clone();
            let mut board = board.lock().unwrap();
            board.try_toggle_cell(&(*x as usize), &(*y as usize));
            board.is_solved()
        };

        if solved {
            self.println("Solved!".to_string());
        }
        self.redraw();
    }

    pub fn get_max_jumps(&self) -> &f64 {
        &self.max_jumps
    }

    pub fn set_max_jumps(&mut self, max_jumps: f64) {
        self.max_jumps = max_jumps
    }
}

fn is_even(n: &f64) -> bool {
    *n as i32 % 2 == 0
}
