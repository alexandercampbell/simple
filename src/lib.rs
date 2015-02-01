//!
//! simple
//! ======
//!
//! The simplest graphics library, inspired by LOVE2D. See the README for more information.
//!
//!
//! ```
//! let app = simple::Window::new("hello world", 1920, 1080);
//!
//! app.set_color(255, 0, 255, 255);
//! app.draw_rect(simple::Rect{
//!     x: 100,
//!     y: 110,
//!     w: 120,
//!     h: 130,
//! });
//!
//! while app.next_frame() {}
//! ```
//!

extern crate sdl2;
extern crate "sdl2-sys" as sdl2_sys;

// Re-export some of the symbols in event.rs
pub use event::{KeyCode,MouseButton,Event};
pub use shape::{Rect,Point,Polygon};
pub use window::Window;

mod event;
mod shape;
mod window;

