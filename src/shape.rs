
extern crate sdl2;
extern crate sdl2_sys;

// Might as well reuse the SDL2 structs wherever possible. Less memory being copied around.
pub use sdl2_sys::rect::Rect;
pub use sdl2_sys::rect::Point;

/// Polygon is a list of points with no special checking.
///
/// Polygon is mostly being set aside for now. May revisit in the future.
pub type Polygon = Vec<Point>;

