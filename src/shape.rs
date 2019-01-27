extern crate sdl2;

// Might as well reuse the SDL2 structs wherever possible. Less memory being copied around.
pub use sdl2::rect::Point;
pub use sdl2::rect::Rect;

/// Polygon is a list of points with no special checking.
///
/// Polygon is mostly being set aside for now. May revisit in the future.
pub type Polygon = Vec<Point>;
