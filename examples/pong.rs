
extern crate simple;
use simple::Window;

#[allow(unused)]
fn main() {
    let mut app = Window::new("November", 1920, 1080);

    while app.next_frame() {
        loop {
            let event = app.next_event();
            match event {
                Some(simple::Event::Mouse{is_down, mouse_x, mouse_y, ..}) => {
                    println!("Mouse {} {} {}", mouse_x, mouse_y,
                        if is_down { "down" } else { "up" });
                },

                None    => break,
                _       => (),
            }
        }
    }
}

