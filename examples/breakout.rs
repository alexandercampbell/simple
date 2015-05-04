/*!
 * This example is a complete playable game of breakout.
 */

extern crate simple;
use simple::*;

fn main() {
    let mut app = Window::new("Breakout", 1024, 768);
    let mut player = Rect{
        x: 0,
        y: 700,
        w: 100,
        h: 16,
    };

    while app.next_frame() {
        app.clear();

        // move the paddle to the mouse cursor
        player.x += (app.mouse_position().0 - player.w/2 - player.x) / 3;

        app.set_color(255,255,255,255);
        app.fill_rect(player);
    };
}

