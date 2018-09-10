extern crate andaluz_core;

use std::mem;
use std::slice;
use std::os::raw::c_void;

use andaluz_core::square::{Square, SquareContent};
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
    let signature = board.signature;
    let byte_size = cols * cols * 4;
    let sl = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };
    for (i, c) in signature.chars().enumerate() {
        sl[i] = match c {
            '1' => 1,
            '0' => 0,
            _ => 2,
        }
    }
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
                    board.put_queen(&Square {x, y})
                        .unwrap_or(SquareContent::Empty)
                },
                _ => { SquareContent::Empty }
            };
        }
    };
    board
}
