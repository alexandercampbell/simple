#![allow(unused)]

extern crate simple;

use simple::Window;

fn main() {
    let w = Window::new("November", 1920, 1080);
    w.run();
    w.quit();
}

