/*!
 * This example is a complete playable game of breakout.
 */

extern crate simple;
use simple::*;

fn main() {
    let mut app = Window::new("Breakout", 1024, 768);
    while app.next_frame() {};
}

