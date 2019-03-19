extern crate stdweb;

use andaluz_wasm::canvas::Canvas;
use andaluz_wasm::console::Console;

fn main() {
    stdweb::initialize();

    let mut console = Console::new();
    let mut canvas = Canvas::new();
    console.println("Andaluz nQueen Wasm initialized...");

    stdweb::event_loop();
}
