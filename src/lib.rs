//!
//! simple
//! ======
//!
//! The simplest graphics library, inspired by LOVE2D. See the README for more information.
//!
//! Homepage: https://github.com/alexandercampbell/simple
//!

extern crate rand;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_sys;

// Re-export some of the symbols from the other modules.
pub use event::{Key,MouseButton,Event};
pub use shape::{Rect,Point,Polygon};
pub use window::{Window,Image};

mod event;
mod shape;
mod window;

