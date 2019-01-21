/*!
 * This example demonstrates how to load an image and draw it onto the screen.
 */

extern crate num;
use num::Float;

extern crate simple;

fn main() {
    let mut window = simple::Window::new("Los Angeles", 640, 480);
    let mut pic = window.load_image(include_bytes!("city.jpg")).unwrap();

    let mut frame_number: u64 = 0;

    while window.next_frame() {
        window.clear();

        let sine = (frame_number as f32 / 150.0).sin().abs();
        let color = (sine * 255f32) as u8;

        window.set_color(100 + color / 3, color, 255 - color, 255);
        window.draw_image(&mut pic, 0, 0);

        frame_number += 1;
    }
}
