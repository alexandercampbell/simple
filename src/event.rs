
extern crate sdl2;
use sdl2::event::Event as SDL_Event;
pub use sdl2::scancode::ScanCode as Key;
pub use sdl2::mouse::Mouse as MouseButton;

/// Event is an enumeration of the effects that a user can have on a running Window.
#[derive(Copy, Clone, PartialEq)]
pub enum Event {
    Keyboard{is_down: bool, key: Key},
    Mouse{is_down: bool, button: MouseButton, mouse_x: i32, mouse_y: i32},
    Quit,
}

impl Event {
    pub fn from_sdl2_event(e: SDL_Event) -> Option<Event> {
        match e {
            // Quit
            SDL_Event::Quit{..} => Some(Event::Quit),

            // Keyboard
            SDL_Event::KeyDown{scancode: key, ..}  => Some(Event::Keyboard{is_down: true, key: key}),
            SDL_Event::KeyUp{scancode: key, ..}    => Some(Event::Keyboard{is_down: false, key: key}),

            // Mouse
            SDL_Event::MouseButtonDown{mouse_btn: button, x, y, ..} =>
                Some(Event::Mouse{is_down: true, button: button, mouse_x: x, mouse_y: y}),
            SDL_Event::MouseButtonUp{mouse_btn: button, x, y, ..} =>
                Some(Event::Mouse{is_down: false, button: button, mouse_x: x, mouse_y: y}),

            _ => None,
        }
    }
}

#[test]
fn test_from_sdl2_event() {
    fn test(input: SDL_Event, expected: Event) {
        assert!(Event::from_sdl2_event(input).unwrap() == expected);
    }

    test(SDL_Event::Quit{timestamp: 0}, Event::Quit);

    // TODO: Test more comprehensively.
}

/// Convert from an sdl2::mouse::Mouse enum into an sdl2::mouse::MouseState bitflag.
pub fn mousebutton_to_mousestate(button: MouseButton) -> Option<sdl2::mouse::MouseState> {
    Some(match button {
        MouseButton::Left => sdl2::mouse::LEFTMOUSESTATE,
        MouseButton::Right => sdl2::mouse::RIGHTMOUSESTATE,
        MouseButton::Middle => sdl2::mouse::MIDDLEMOUSESTATE,
        MouseButton::X1 => sdl2::mouse::X1MOUSESTATE,
        MouseButton::X2 => sdl2::mouse::X2MOUSESTATE,
        _ => return None,
    })
}

#[test]
fn test_mousebutton_to_mousestate() {
    assert!(mousebutton_to_mousestate(MouseButton::Left) == Some(sdl2::mouse::LEFTMOUSESTATE));
    assert!(mousebutton_to_mousestate(MouseButton::X1) == Some(sdl2::mouse::X1MOUSESTATE));
    assert!(mousebutton_to_mousestate(MouseButton::X2) == Some(sdl2::mouse::X2MOUSESTATE));
    assert!(mousebutton_to_mousestate(MouseButton::Unknown(5)) == None);
}

