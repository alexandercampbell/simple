/*!
 * This example is a complete playable game of breakout.
 */

extern crate rand;
extern crate num;
use num::Float;

extern crate simple;
use simple::*;

const SCREEN_WIDTH:i32 = 1024;
const SCREEN_HEIGHT:i32 = 768;

struct Ball {
    rect: Rect,
    speed: i32,
    angle: f32,
}

impl Ball {
    fn new(xpos: i32, ypos: i32) -> Ball {
        Ball{
            rect: Rect{
                x: xpos - 16,
                y: ypos - 16,
                w: 32,
                h: 32,
            },
            speed: 6,
            angle: rand::random(),
        }
    }

    fn update(&mut self) {
        // bounds check against the edges of the screen
        let bounce_x = self.rect.x < 0 || self.rect.x+self.rect.w > SCREEN_WIDTH;
        let bounce_y = self.rect.y < 0 || self.rect.y+self.rect.h > SCREEN_HEIGHT;
        self.bounce(bounce_x, bounce_y);

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
        // early out
        if !(x_bounce || y_bounce) { return; }

        let x_vel = self.angle.sin() * if x_bounce { -1.0 } else { 1.0 };
        let y_vel = self.angle.cos() * if y_bounce { -1.0 } else { 1.0 };

        self.angle = x_vel.atan2(y_vel);
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
    let mut ball = Ball::new(
        SCREEN_WIDTH/2,
        SCREEN_HEIGHT/2,
    );

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

