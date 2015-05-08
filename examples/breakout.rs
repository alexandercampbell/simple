/*!
 * This example is a very simple, but playable, game of Breakout. The game is controlled with the
 * mouse.
 *
 * The physics are wonky becuase I'm not good at collision code.
 */

extern crate rand;
extern crate num;
use num::Float;

extern crate simple;
use simple::*;

const SCREEN_WIDTH:i32 = 640;
const SCREEN_HEIGHT:i32 = 480;

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
     * the ball to be stuck outside of the window.
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

    /**
     * Transform the current angle of the ball. If x_bounce is true, the ball bounces as if it has
     * it a vertical wall (the velocity as measured in the x dimension is inverted). The y_bounce
     * is identical, but for the y dimension.
     */
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

/**
 * Blocks are the things along the top of the screen that you try to hit with the ball :)
 */
struct Block {
    rect: Rect,
    color: u8,
}
const BLOCK_WIDTH:i32 = 48;
const BLOCK_HEIGHT:i32 = 32;

impl Block {
    /**
     * Each newly created block gets its own randomly generated color.
     */
    fn new(x: i32, y: i32) -> Block {
        Block{
            rect: Rect{
                x: x,
                y: y,
                w: BLOCK_WIDTH,
                h: BLOCK_HEIGHT,
            },
            color: rand::random(),
        }
    }

    fn draw(&self, app: &mut Window) {
        app.set_color(self.color, 255 - self.color, 255, 255);
        app.fill_rect(self.rect);
    }
}

fn main() {
    let mut app = Window::new("Breakout Demo", SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16);
    let mut ball = Ball::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
    let mut player = Rect{x: 0, y: SCREEN_HEIGHT - 32, w: 100, h: 8};

    /*
     * Set up the blocks along the top of the screen.
     */
    let mut blocks = vec![];
    let spacing_x = BLOCK_WIDTH*3/2;
    let spacing_y = BLOCK_HEIGHT*3/2;
    for i in 1..(SCREEN_WIDTH/spacing_x) {
        for j in 1..(SCREEN_HEIGHT/spacing_y/2) {
            blocks.push(Block::new(i * spacing_x, j * spacing_y));
        }
    }

    /*
     * Main loop
     */
    while app.next_frame() {
        app.clear();

        // smooth glide the paddle towards the mouse cursor
        player.x += (app.mouse_position().0 - player.w/2 - player.x) / 3;

        ball.update();
        ball.draw(&mut app);

        if ball.intersects(&player) {
            ball.bounce(false, true);

            // A hack because we don't actually have physics capabilities. Prevent the ball from
            // getting stuck inside the paddle.
            ball.rect.y = player.y - ball.rect.h;
        }

        // Keep only those blocks that are not touching the ball.
        blocks = blocks.into_iter().filter(|ref b| {
            b.draw(&mut app);
            !ball.intersects(&(b.rect))
        }).collect();

        app.set_color(255,255,255,255);
        app.fill_rect(player);
    };
}

