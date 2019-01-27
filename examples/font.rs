/*!
 * This example shows off some of the font-rendering capabilities of the library.
 */

extern crate simple;
use simple::*;

fn main() {
    let mut app = Window::new("Image Font Demo", 640, 480);
    while app.next_frame() {
        app.clear_to_color(32, 64, 32);

        app.set_color(255, 255, 255, 255);
        app.print("Hello world!", 32, 32);
        app.print("This example demonstrates ImageFont rendering :)", 32, 64);
        app.print("You can even write symbols: !@#$%^&*()", 32, 96);

        app.set_color(0, 255, 255, 255);
        app.print("16777216 possible rendering colors!", 32, 128);
    }
}
