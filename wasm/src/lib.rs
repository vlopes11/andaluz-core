extern crate andaluz_core;

use std::mem;
use std::slice;
use std::os::raw::c_void;

use andaluz_core::board::Board;
use andaluz_core::solver::Solver;
use andaluz_core::heuristic_implementation::attacksum::AttackSum;
use andaluz_core::heuristic_implementation::bruteforce::BruteForce;
use andaluz_core::heuristic_implementation::horse::Horse;
use andaluz_core::heuristic_implementation::prioritizecenter::PrioritizeCenter;
use andaluz_core::heuristic_implementation::HeuristicImplementation;

#[no_mangle]
pub fn solve(pointer: *mut u8, cols: usize, max_jumps: i32, bruteforce: i32, attacksum: i32, horse: i32) -> i32 {
    let mut board = board_from_pointer(pointer, cols);
    let mut solver = Solver::new();

    if max_jumps > 0 {
        solver.set_max_jumps(max_jumps as u32);
    }

    if bruteforce > 0 {
        solver.push_heuristic(BruteForce::new(bruteforce as f64));
    }
    
    solver.push_heuristic(PrioritizeCenter::new((bruteforce.max(attacksum).max(horse)) as f64));

    if attacksum > 0 {
        solver.push_heuristic(AttackSum::new(attacksum as f64));
    }

    if horse > 0 {
        solver.push_heuristic(Horse::new(horse as f64));
    }

    match solver.solve(&mut board) {
        Ok(result) => {
            board_to_pointer(&board, pointer);
            *result.get_jumps() as i32
        },
        Err(_) => {
            // TODO - report the error
            0
        },
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
    match board.toggle_cell(&x, &y) {
        Ok(_) => board_to_pointer(&board, pointer),
        Err(_) => {
            // TODO - Report the error
        },
    }
}

fn board_from_pointer(pointer: *mut u8, cols: usize) -> Board {
    let byte_size = cols * cols * 4;
    let sl = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };
    let mut board = Board::new(cols);

    for x in 0..cols {
        for y in 0..cols {
            let i = x + cols * y;
            
            if sl[i] == 1 {
                board.toggle_cell(&x, &y).unwrap_or(());
            }
        }
    };

    board
}

fn board_to_pointer(board: &Board, pointer: *mut u8) {
    let cols = *board.get_cols();
    let byte_size = cols * cols * 4;
    let sl = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    for x in 0..cols {
        for y in 0..cols {
            let i = x + cols * y;
            sl[i] = match board.get_cell(&(x+1), &(y+1)) {
                Ok(c) if c.is_queen() => 1,
                Ok(c) if c.is_attacked() => 2,
                _ => 0,
            }
        }
    }
}
