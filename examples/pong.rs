
extern crate simple;
use simple::Window;
use simple::Event;

#[derive(Copy,Clone)]
struct Paddle {
    rect:   simple::Rect,
}

#[allow(unused)]
fn main() {

    // Create an application
    let mut app = Window::new("November", 1920, 1080);

    while app.next_frame() {

        while app.has_event() {
            match app.next_event() {
                Event::Mouse{is_down, mouse_x, mouse_y, ..} => {
                    println!("Mouse {} {} {}", mouse_x, mouse_y,
                        if is_down { "down" } else { "up" });
                },
                _  => (),
            }
        }

    }

}

