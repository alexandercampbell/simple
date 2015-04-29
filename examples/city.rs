/**
 * This example demonstrates how to load an image and draw it onto the screen.
 */

use std::path::Path;

extern crate simple;
use simple::Window;

fn main() {
    let mut window = Window::new("Los Angeles", 640, 480);
    let pic = window.load_image(Path::new("examples/los_angeles.jpg")).unwrap();

    while window.next_frame() {
        window.draw_image(&pic, 0, 0);
    }
}

