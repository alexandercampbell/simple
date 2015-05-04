/*!
 * This example is a complete playable game of breakout.
 */

extern crate rand;
extern crate num;
use num::Float;

extern crate simple;
use simple::*;

struct Ball {
    rect: Rect,
    speed: i32,
    angle: f32,
}

impl Ball {
    fn new(xpos: i32, ypos: i32) -> Ball {
        Ball{
            rect: Rect{
                x: xpos/2 - 16,
                y: ypos/2 - 16,
                w: 32,
                h: 32,
            },
            speed: 5,
            angle: rand::random(),
        }
    }

    fn update(&mut self) {
        self.rect.x += (self.speed as f32 * self.angle.sin()) as i32;
        self.rect.y += (self.speed as f32 * self.angle.cos()) as i32;
    }

    fn draw(&self, app: &mut Window) {
        app.set_color(255, 0, 255, 255);
        app.fill_rect(self.rect);
    }

    fn intersects(&self, other: &Rect) -> bool {
        self.rect.has_intersection(other)
    }

    fn bounce(&mut self, x_bounce: bool, y_bounce: bool) {
        if x_bounce {
            // this is almost definitely the wrong formula. TODO: revisit
            self.angle = 3.141592 - self.angle;
        }
        if y_bounce {
            self.angle = 3.141592 - self.angle;
        }
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
    let mut ball = Ball::new(1024/2, 768/2);

    while app.next_frame() {
        app.clear();

        // move the paddle to the mouse cursor
        player.x += (app.mouse_position().0 - player.w/2 - player.x) / 3;

        ball.update();
        ball.draw(&mut app);

        if ball.intersects(&player) {
            ball.bounce(false, true);
        }

        app.set_color(255,255,255,255);
        app.fill_rect(player);
    };
}

