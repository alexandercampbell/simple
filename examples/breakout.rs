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

/**
 * Ball is a very simple representation of a rectangle with a speed and angle. Ball contains some
 * methods to update and draw. Note that the Ball is actually drawn as a square.
 */
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
                w: 24,
                h: 24,
            },
            speed: 9,
            angle: rand::random(),
        }
    }

    fn update(&mut self) {
        // bounds check against the edges of the screen
        let bounce_x = self.rect.x < 0 || self.rect.x+self.rect.w > SCREEN_WIDTH;
        let bounce_y = self.rect.y < 0 || self.rect.y+self.rect.h > SCREEN_HEIGHT;
        self.bounce(bounce_x, bounce_y);
        self.clamp_on_screen();

        self.rect.x += (self.speed as f32 * self.angle.sin()) as i32;
        self.rect.y += (self.speed as f32 * self.angle.cos()) as i32;
    }

    /**
     * Clamp the ball onto the screen.
     *
     * This is really annoying but it's required because sometimes the angle fuzzing can cause
     * the ball to be stuct outside of the window.
     */
    fn clamp_on_screen(&mut self) {
        // X-dimension
        self.rect.x = if self.rect.x < 0 { 0 } else { self.rect.x };
        self.rect.x = if self.rect.x+self.rect.w > SCREEN_WIDTH {
            SCREEN_WIDTH-self.rect.w
        } else {
            self.rect.x
        };

        // Y-dimension
        self.rect.y = if self.rect.y < 0 { 0 } else { self.rect.y };
        self.rect.y = if self.rect.y+self.rect.h > SCREEN_HEIGHT {
            SCREEN_HEIGHT-self.rect.h
        } else {
            self.rect.y
        };
    }

    fn draw(&self, app: &mut Window) {
        app.set_color(255, 255, 0, 255);
        app.fill_rect(self.rect);
    }

    fn bounce(&mut self, x_bounce: bool, y_bounce: bool) {
        // early out
        if !(x_bounce || y_bounce) { return; }

        let x_vel = self.angle.sin() * if x_bounce { -1.0 } else { 1.0 };
        let y_vel = self.angle.cos() * if y_bounce { -1.0 } else { 1.0 };

        self.angle = x_vel.atan2(y_vel);
        let fuzzing = rand::random::<u8>() as f32 / 750.0;
        self.angle += fuzzing;
    }

    fn intersects(&self, other: &Rect) -> bool { self.rect.has_intersection(other) }
}

fn main() {
    let mut app = Window::new("Breakout", 1024, 768);
    let mut entities = vec![
        Ball::new(SCREEN_WIDTH*2/3, SCREEN_HEIGHT/2),
        Ball::new(SCREEN_WIDTH/3, SCREEN_HEIGHT/2),
    ];
    let mut player = Rect{x: 0, y: 700, w: 100, h: 8};

    while app.next_frame() {
        app.clear();

        // smooth glide the paddle towards the mouse cursor
        player.x += (app.mouse_position().0 - player.w/2 - player.x) / 3;

        for entity in entities.iter_mut() {
            entity.update();
            entity.draw(&mut app);

            if entity.intersects(&player) {
                entity.bounce(false, true);

                // A hack because we don't actually have physics capabilities. Prevent the ball from
                // getting stuck inside the paddle.
                entity.rect.y = player.y - entity.rect.h;
            }
        }

        app.set_color(255,255,255,255);
        app.fill_rect(player);
    };
}

