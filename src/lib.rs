//! simple
//! ======
//!
//! The simplest graphics library, inspired by LOVE2D. See the README for more information.
//!
//! Homepage: https://github.com/alexandercampbell/simple

extern crate rand;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_sys;

// Re-export some of the symbols from the other modules.
pub use event::{Event};
pub use shape::{Polygon,Rect,Point};
pub use window::{Window,Image,Font};

// rustdoc has some bugs right now and the below code works around this. Rust issue link:
// https://github.com/rust-lang/rust/issues/24305
//
// The specific issue is that renamed re-exports show up as their original names. This is a problem
// because we re-export a couple SDL2 structs under slightly different names.
pub use event::Key as Key;
pub use event::MouseButton as MouseButton;

mod event;
mod shape;
mod window;

