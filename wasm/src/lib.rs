extern crate andaluz_core;

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
