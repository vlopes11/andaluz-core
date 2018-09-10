extern crate andaluz_core;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use andaluz_core::board::Board;

#[no_mangle]
pub fn solve() -> i32 {
    let mut board = Board::new(8);
    let solved = board.solve();
    let jumps = board.jumps;

    if solved {
        jumps as i32
    } else {
        0
    }
}

#[no_mangle]
pub fn tst() -> Vec<i32> {
    vec![-1, 1]
}
