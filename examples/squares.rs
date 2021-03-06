/*!
 * This example uses the draw methods on simple::Window to display some rectangles bouncing around
 * on screen. You can click anywhere to add a new rectangle.
 */

extern crate rand;
use rand::random;

extern crate simple;
use simple::{Event, Rect, Window};

static SCREEN_WIDTH: u16 = 1280;
static SCREEN_HEIGHT: u16 = 720;

/// Return an f32 in the interval [0, upper_bound]
/// Used to generate random positions for Square.
fn rand_up_to(upper_bound: f32) -> f32 {
    random::<f32>().abs() * upper_bound
}

/// Square is our game object. It has a position, movement vector, and color.
#[derive(Debug, Copy, Clone)]
struct Square {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
    color: (u8, u8, u8, u8),
}

impl Square {
    /// Generate a Square with random speed and color starting at the point you specify
    fn new_at_position(x: f32, y: f32) -> Self {
        // generate a random angle and then use that angle to calculate the initial speed_x and
        // speed_y. We do this because it simplifies bouncing logic later in the update function.
        // The multiplication here is because random::<f32> appears to generate a value between 0
        // and 1, so we have to expand that range to [0, 2*PI] to get a full distribution of
        // possible angles.
        let angle: f32 = rand_up_to(3.141592 * 2.0);

        Square {
            x: x,
            y: y,
            speed_x: angle.sin() * 8.0,
            speed_y: angle.cos() * 8.0,
            color: (random(), random(), random(), 255), // color is totally random
        }
    }

    /// Generate a totally random new Square
    fn new() -> Self {
        Square::new_at_position(
            rand_up_to(SCREEN_WIDTH as f32),
            rand_up_to(SCREEN_HEIGHT as f32),
        )
    }

    /// Move the Square the distance it needs to travel for one frame.
    fn update(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;

        if self.x < 0.0 || self.x > SCREEN_WIDTH as f32 {
            self.speed_x *= -1f32;
        }
        if self.y < 0.0 || self.y > SCREEN_HEIGHT as f32 {
            self.speed_y *= -1f32;
        }
    }

    /// Blit a square representing this object onto the Window.
    fn draw(&self, app: &mut Window) {
        app.set_color(self.color.0, self.color.1, self.color.2, self.color.3);
        app.fill_rect(Rect::new(self.x as i32 - 32, self.y as i32 - 32, 64, 64));
    }
}

fn main() {
    // Create an application
    let mut app = Window::new("Squares", SCREEN_WIDTH, SCREEN_HEIGHT);

    // Create some objects to live in the application
    let mut squares = vec![Square::new(), Square::new(), Square::new()];

    // Run the game loop
    while app.next_frame() {
        // event handling
        while app.has_event() {
            match app.next_event() {
                // If the user clicks, we add a new Square at the position of the mouse event.
                Event::Mouse {
                    is_down: true,
                    mouse_x,
                    mouse_y,
                    ..
                } => squares.push(Square::new_at_position(mouse_x as f32, mouse_y as f32)),

                _ => (),
            }
        }

        app.clear();

        // update and draw
        for square in squares.iter_mut() {
            square.update();
            square.draw(&mut app);
        }
    }
}
