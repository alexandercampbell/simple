//!
//! simple
//! ======
//!
//! The simplest graphics library, inspired by LOVE2D. See the README for more information.
//!

extern crate rand;
extern crate sdl2;
extern crate sdl2_image;
extern crate "sdl2-sys" as sdl2_sys;

// Re-export some of the symbols in event.rs
pub use event::{KeyCode,MouseButton,Event};
pub use shape::{Rect,Point,Polygon};
pub use window::Window;
pub use window::Image;

mod event;
mod shape;
mod window;

