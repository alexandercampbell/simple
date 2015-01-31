
extern crate sdl2;
use sdl2::video::WindowPos;
use sdl2::video;

/// handle_error is a helper that (for now) just prints out the message that you choose to the
/// screen.
fn handle_error(msg: String) {
    println!("SDL2 Error detected: {}", msg);
}

/// Windows can display graphics, play sounds, and return events.
pub struct Window {
    renderer:   sdl2::render::Renderer,
}

impl Window {
    pub fn new(name: &str, width: isize, height: isize) -> Window {
        sdl2::init(sdl2::INIT_EVERYTHING);

        let sdl_window = video::Window::new(name,
            WindowPos::PosCentered, WindowPos::PosCentered,
            width, height, video::BORDERLESS | video::RESIZABLE,
        ).unwrap();

        let renderer = sdl2::render::Renderer::from_window(
            sdl_window,
            sdl2::render::RenderDriverIndex::Auto,
            sdl2::render::ACCELERATED,
        ).unwrap();

        Window{renderer: renderer}
    }

    pub fn set_color(&self, red: u8, green: u8, blue: u8, alpha: u8) {
        let color_struct = sdl2::pixels::Color::RGBA(red, green, blue, alpha);
        match self.renderer.set_draw_color(color_struct) {
            Ok(_) => (),
            Err(s) => handle_error(s),
        }
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

