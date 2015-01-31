extern crate sdl2;

// Re-export some of the symbols in event.rs
pub use event::{KeyCode,MouseButton,Event};
pub use window::Window;

mod event;
mod window;

