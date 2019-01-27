/*!
 * This example is a mouse accuracy test.
 * Click within the square as many times as possible within the time limit.
 */

use std::time::*;

extern crate simple;
use simple::*;

extern crate rand;
use rand::random;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const RECT_SIZE: u32 = 32;

const COUNTDOWN_TIME_MILLIS: u32 = 1_500;
const GAME_TIME_MILLIS: u32 = 8_000;

fn random_point() -> Point {
    Point::new(
        (random::<u32>() % (WIDTH - RECT_SIZE)) as i32,
        (random::<u32>() % (HEIGHT - RECT_SIZE)) as i32,
    )
}

fn main() {
    let mut app = Window::new("Mouse Accuracy Game", WIDTH as u16, HEIGHT as u16);
    let mut successes = 0;
    let mut misses = 0;
    let mut next_position = random_point();
    let mut rect = Rect::new(next_position.x(), next_position.y(), RECT_SIZE, RECT_SIZE);

    let program_start_time = SystemTime::now();

    while app.next_frame() {
        app.clear_to_color(130, 130, 130);

        // move the rect closer to the next_position by 1/5th of the current distance
        rect = Rect::from_center(
            Point::new(
                rect.center().x() + (next_position.x() - rect.center().x()) / 5,
                rect.center().y() + (next_position.y() - rect.center().y()) / 5,
            ),
            RECT_SIZE,
            RECT_SIZE,
        );

        let elapsed = program_start_time.elapsed().unwrap();
        let millis_since_start =
            (elapsed.as_secs() as f64 * 1000.0) as u32 + elapsed.subsec_millis();

        if millis_since_start < COUNTDOWN_TIME_MILLIS {
            // Countdown phase
            app.set_color(255, 255, 255, 255);
            app.print("Get Ready!", WIDTH as i32 / 2 - 50, HEIGHT as i32 / 2 - 30);
            app.print(
                &format!("{}", 3 - (millis_since_start * 2 / 1000)),
                WIDTH as i32 / 2 - 10,
                HEIGHT as i32 / 2,
            );
        } else if millis_since_start < COUNTDOWN_TIME_MILLIS + GAME_TIME_MILLIS {
            // Gameplay phase
            app.set_color(255, 255, 255, 255);
            app.fill_rect(rect);
            app.set_color(0, 0, 0, 255);
            app.draw_rect(rect);
            app.set_color(255, 255, 255, 255);
            app.print(
                &format!("Successes: {}  Misses: {}", successes, misses),
                15,
                15,
            );
            app.print(
                &format!(
                    "Seconds Remaining: {}",
                    (GAME_TIME_MILLIS - (millis_since_start - COUNTDOWN_TIME_MILLIS)) / 1000
                ),
                15,
                HEIGHT as i32 - 25,
            );
        } else {
            // Score screen
            app.set_color(255, 255, 255, 255);
            app.print("Time's up!", WIDTH as i32 / 2 - 40, HEIGHT as i32 / 2 - 30);
            app.print(
                &format!(
                    "You had {} accurate clicks and {} misses",
                    successes, misses,
                ),
                WIDTH as i32 / 5,
                HEIGHT as i32 / 2,
            );
        }

        while app.has_event() {
            match app.next_event() {
                Event::Mouse {
                    is_down: true,
                    button: MouseButton::Left,
                    mouse_x,
                    mouse_y,
                } => {
                    // gameplay phase, and player clicked on rectangle correctly
                    if millis_since_start > COUNTDOWN_TIME_MILLIS
                        && millis_since_start < COUNTDOWN_TIME_MILLIS + GAME_TIME_MILLIS
                    {
                        if rect.contains_point(Point::new(mouse_x, mouse_y)) {
                            successes += 1;
                        } else {
                            misses += 1;
                        }
                        next_position = random_point();
                    }
                }
                _ => (),
            }
        }
    }
}
