/*!
 * This example is a complete playable game of breakout.
 */

extern crate rand;
extern crate num;
use num::Float;

extern crate simple;
use simple::*;

struct Ball {
    x: i32,
    y: i32,
    speed: i32,
    angle: f32,
}

impl Ball {
    fn update(&mut self) {
        self.x += (self.speed as f32 * self.angle.sin()) as i32;
        self.y += (self.speed as f32 * self.angle.cos()) as i32;
    }

    fn draw(&self, app: &mut Window) {
        app.set_color(255, 0, 255, 255);
        app.fill_rect(Rect{
            x: self.x - 16,
            y: self.y - 16,
            w: 32,
            h: 32,
        });
    }
}

fn main() {
    let mut app = Window::new("Breakout", 1024, 768);
    let mut player = Rect{
        x: 0,
        y: 700,
        w: 100,
        h: 16,
    };
    let mut ball = Ball{
        x: 1024/2,
        y: 768/2,
        speed: 5,
        angle: rand::random(),
    };

    while app.next_frame() {
        app.clear();

        // move the paddle to the mouse cursor
        player.x += (app.mouse_position().0 - player.w/2 - player.x) / 3;

        ball.update();
        ball.draw(&mut app);

        app.set_color(255,255,255,255);
        app.fill_rect(player);
    };
}

