
extern crate sdl2;
use sdl2::event::Event as SDL_Event;
pub use sdl2::keycode::KeyCode;
pub use sdl2::mouse::Mouse as MouseButton;

#[derive(Copy,PartialEq)]
pub enum Event {
    /// The boolean at the beginning of Event::Keyboard is true iff the key is down.
    Keyboard(bool, KeyCode),
    /// The boolean at the beginning of Event::Mouse is true iff the key is down.
    Mouse(bool, MouseButton, i32, i32),
    Quit,
}

impl Event {
    pub fn from_sdl2_event(e: SDL_Event) -> Option<Event> {
        match e {
            // Quit
            SDL_Event::Quit(_) => Some(Event::Quit),

            // Keyboard
            SDL_Event::KeyDown(_, _, key, _, _, _)  => Some(Event::Keyboard(true, key)),
            SDL_Event::KeyUp(_, _, key, _, _, _)    => Some(Event::Keyboard(false, key)),

            // Mouse
            SDL_Event::MouseButtonDown(_, _, _, button, x, y) =>
                Some(Event::Mouse(true, button, x, y)),
            SDL_Event::MouseButtonUp(_, _, _, button, x, y) =>
                Some(Event::Mouse(false, button, x, y)),

            _ => None,
        }
    }
}

#[test]
fn test_from_sdl2_event() {
    fn test(input: SDL_Event, expected: Event) {
        assert!(Event::from_sdl2_event(input).unwrap() == expected);
    }

    test(SDL_Event::Quit(0), Event::Quit);

    // NOTE: can't test more comprehensively because SDL_Events have embedded sdl::video::Window
    // instances and I can't think of a way to get a window instance for a simple test.
}

