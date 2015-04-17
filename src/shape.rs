
extern crate sdl2;
extern crate sdl2_sys;

use std;

// Might as well reuse the SDL2 structs wherever possible
pub use sdl2_sys::rect::Rect;
pub use sdl2_sys::rect::Point;

/// Polygon is mostly being set aside for now. May revisit in the future.
pub struct Polygon {
    pub points: std::vec::Vec<Point>,
}
