#![allow(unstable)]
#![allow(unused)]

use engine::Engine;
mod engine;

fn main() {
    let e = Engine::new(1920, 1080);
    e.run();
    e.quit();
}

