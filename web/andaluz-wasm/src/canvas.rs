use andaluz_core::board::Board;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::ClickEvent;
use stdweb::web::html_element::{CanvasElement, ImageElement};
use stdweb::web::{document, window, CanvasRenderingContext2d};

pub struct Canvas {
    width: f64,
    height: f64,
    cols: f64,
    square_size: f64,
    selector: CanvasElement,
    context: CanvasRenderingContext2d,
    queen_svg: ImageElement,
    attack_svg: ImageElement,
    board: Board,
}

impl Canvas {
    pub fn new() -> Self {
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
            board: Board::new(8),
            queen_svg,
            attack_svg,
        };

        canvas.set_cols(8.0);
        
        // TODO - impl send (probably arc/mutex?)
        // canvas.add_event_listener(|event: ClickEvent| {});

        canvas
    }

    pub fn set_cols(&mut self, cols: f64) {
        self.board = Board::new(cols as usize);
        self.square_size = self.width / cols;
        self.cols = cols;

        self.redraw();
    }

    pub fn clear(&mut self) {
        self.context.clear_rect(0.0, 0.0, self.width, self.height);
    }

    pub fn redraw(&mut self) {
        self.clear();

        let c = self.cols as usize;

        for x in 1..=c {
            for y in 1..=c {
                self.draw_square(&(x as f64), &(y as f64));
            }
        }
    }

    pub fn draw_square(&mut self, x: &f64, y: &f64) {
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

        let cell = self.board.get_cell(&(*x as usize), &(*y as usize)).unwrap();

        if cell.is_queen() {
            self.context
                .draw_image_d(
                    self.queen_svg.clone(),
                    top_left_x.clone(),
                    top_left_y.clone(),
                    self.square_size,
                    self.square_size,
                )
                .unwrap();
        } else if cell.is_attacked() {
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
}

fn is_even(n: &f64) -> bool {
    *n as i32 % 2 == 0
}
