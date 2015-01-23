
use engine::Engine;
mod engine;

#[allow(unused)]
fn main() {
    let e = Engine::new(1920, 1080);
    e.run();
    e.quit();
}

