#![allow(unstable)]
#![allow(unused)]

extern crate sdl2;

pub struct Window {
    renderer:   sdl2::render::Renderer,
}

impl Window {
    pub fn new(name: &str, width: isize, height: isize) -> Result<Window> {
        sdl2::init(sdl2::INIT_EVERYTHING);

        let sdl_window = try!(sdl2::video::Window::new(name,
            WindowPos::PosCentered, WindowPos::PosCentered,
            width, height, video::BORDERLESS | video::RESIZABLE,
        ).unwrap());

        let renderer = try!(sdl2::render::Renderer::from_window(
            sdl_window,
            sdl2::render::RenderDriverIndex::Auto,
            sdl2::render::ACCELERATED,
        ).unwrap());

        Ok(Window{renderer: renderer})
    }

    pub fn run(&self) {
        loop {
            // Handle events
            match sdl2::event::poll_event() {
                sdl2::event::Event::Quit(_) => break,
                sdl2::event::Event::KeyDown(_, _, key, _, _, _) => {
                    if key == sdl2::keycode::KeyCode::Escape {
                        break;
                    }
                },
                _ => (),
            };
        }
    }

    pub fn quit(&self) {
        sdl2::quit();
    }
}

