extern crate andaluz_core;

use std::mem;
use std::slice;
use std::os::raw::c_void;

use andaluz_core::square::{Square, SquareContent};
use andaluz_core::board::Board;

#[no_mangle]
pub fn solve(pointer: *mut u8, cols: usize) -> i32 {
    let mut board = board_from_pointer(pointer, cols);
    board.solve();
    board_to_pointer(&board, pointer);
    board.jumps as i32
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn fill(pointer: *mut u8, cols: usize) {
    let byte_size = cols * cols * 4;
    let sl = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };
    for i in 0..byte_size {
        sl[i] = 0 as u8;
    }
}

#[no_mangle]
pub extern "C" fn put_queen(pointer: *mut u8, cols: usize, x: usize, y: usize) {
    let mut board = board_from_pointer(pointer, cols);
    board.put_queen(&Square {x, y});
    board_to_pointer(&board, pointer);
}

fn board_from_pointer(pointer: *mut u8, cols: usize) -> Board {
    let byte_size = cols * cols * 4;
    let sl = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };
    let mut board = Board::new(cols);
    for x in 0..cols {
        for y in 0..cols {
            let i = x + cols * y;
            match sl[i] {
                1 => {
                    board.put_queen(&Square {x: x + 1, y: y + 1})
                        .unwrap_or(SquareContent::Empty)
                },
                _ => { SquareContent::Empty }
            };
        }
    };
    board
}

fn board_to_pointer(board: &Board, pointer: *mut u8) {
    let cols = board.cols;
    let byte_size = cols * cols * 4;
    let sl = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };
    for x in 0..cols {
        for y in 0..cols {
            let i = x + cols * y;
            sl[i] = match board.get_square_content(&Square {x: x+1, y: y+1}) {
                SquareContent::Queen => 1 as u8,
                SquareContent::Empty => 0 as u8,
                SquareContent::Attacked => 2 as u8,
            }
        }
    }
}
