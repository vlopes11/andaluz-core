extern crate stdweb;

use andaluz_core::board::Board;
use andaluz_wasm::canvas::Canvas;
use andaluz_wasm::console::Console;
use std::sync::{Arc, Mutex};
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ChangeEvent, ClickEvent};
use stdweb::web::html_element::InputElement;
use stdweb::web::{document, HtmlElement};

fn main() {
    let board = Board::new(8);
    let board = Arc::new(Mutex::new(board));

    stdweb::initialize();

    // threads arent implemented for wasm yet, check
    // https://rustwasm.github.io/2018/10/24/multithreading-rust-and-wasm.html
    let console = Console::new();
    let console = Arc::new(Mutex::new(console));

    let canvas = Canvas::new(board.clone(), console.clone());
    let canvas_selector = canvas.get_selector().clone();
    let canvas = Arc::new(Mutex::new(canvas));

    let canvas_clone = canvas.clone();
    canvas_selector.add_event_listener(move |event: ClickEvent| {
        let mut canvas = canvas_clone.lock().unwrap();
        let (x, y) = canvas.offset_to_xy(event.offset_x(), event.offset_y());
        canvas.toggle_cell(&x, &y);
    });

    let clear_button: HtmlElement = document()
        .query_selector("#andaluz-clear")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let canvas_clone = canvas.clone();
    let console_clone = console.clone();
    clear_button.add_event_listener(move |_: ClickEvent| {
        {
            let mut console = console_clone.lock().unwrap();
            console.clear();
        }
        {
            let canvas = canvas_clone.lock().unwrap();
            canvas.reset();
        }
    });

    let heuristic_attacksum: InputElement = document()
        .query_selector("#andaluz-attacksum")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let heuristic_attacksuminverse: InputElement = document()
        .query_selector("#andaluz-attacksuminverse")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let heuristic_horse: InputElement = document()
        .query_selector("#andaluz-horse")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let heuristic_prioritizecenter: InputElement = document()
        .query_selector("#andaluz-prioritizecenter")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let solve_button: HtmlElement = document()
        .query_selector("#andaluz-solve")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let canvas_clone = canvas.clone();
    solve_button.add_event_listener(move |_: ClickEvent| {
        // TODO - macro to fetch input value
        let attack_sum = match heuristic_attacksum.raw_value().parse::<f64>() {
            Ok(v) => v,
            _ => 0.0,
        };
        let attack_sum_inverse = match heuristic_attacksuminverse.raw_value().parse::<f64>() {
            Ok(v) => v,
            _ => 0.0,
        };
        let horse = match heuristic_horse.raw_value().parse::<f64>() {
            Ok(v) => v,
            _ => 0.0,
        };
        let prioritize_center = match heuristic_prioritizecenter.raw_value().parse::<f64>() {
            Ok(v) => v,
            _ => 0.0,
        };

        {
            let mut canvas = canvas_clone.lock().unwrap();
            canvas.solve(attack_sum, attack_sum_inverse, horse, prioritize_center);
        }
    });

    let input_cols: InputElement = document()
        .query_selector("#andaluz-cols")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let board_clone = board.clone();
    let cols = {
        let board = board_clone.lock().unwrap();
        board.get_cols().clone()
    };
    input_cols.set_raw_value(format!("{}", cols).as_str());
    let console_clone = console.clone();
    let canvas_clone = canvas.clone();
    let input_cols_clone = input_cols.clone();
    input_cols_clone.add_event_listener(move |_: ChangeEvent| {
        match input_cols.raw_value().parse::<f64>() {
            Ok(cols) => {
                let mut canvas = canvas_clone.lock().unwrap();
                canvas.set_cols(cols);
            }
            Err(_) => println(&console_clone, "Invalid number of cols!".to_string()),
        }
    });

    let input_maxjumps: InputElement = document()
        .query_selector("#andaluz-maxjumps")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let maxjumps = {
        let canvas = canvas.lock().unwrap();
        canvas.get_max_jumps().clone()
    };
    input_maxjumps.set_raw_value(format!("{}", maxjumps).as_str());
    let console_clone = console.clone();
    let canvas_clone = canvas.clone();
    let input_maxjumps_clone = input_maxjumps.clone();
    input_maxjumps_clone.add_event_listener(move |_: ChangeEvent| {
        match input_maxjumps.raw_value().parse::<f64>() {
            Ok(maxjumps) => {
                let mut canvas = canvas_clone.lock().unwrap();
                canvas.set_max_jumps(maxjumps);
            }
            Err(_) => println(
                &console_clone,
                "Invalid number of maximum jumps!".to_string(),
            ),
        }
    });

    stdweb::event_loop();
}

fn println(console: &Arc<Mutex<Console>>, txt: String) {
    let mut console = console.lock().unwrap();
    console.println(txt);
}
