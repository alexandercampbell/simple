
extern crate sdl2;
use sdl2::video::WindowPos;
use sdl2::video;

use event;
use event::Event;
use shape;

/// Windows can display graphics, play sounds, and return events.
pub struct Window {
    renderer:   sdl2::render::Renderer,
}

impl Window {
    pub fn new(name: &str, width: i32, height: i32) -> Window {
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
        self.renderer.drawer().set_draw_color(color_struct);
    }

    pub fn run(&self) {
        loop {
            sdl2::timer::delay(10);

            // Handle events
            let event = Event::from_sdl2_event(sdl2::event::poll_event());
            match event {
                Some(Event::Quit) => break,
                Some(Event::Keyboard{key: event::KeyCode::Escape, ..}) => break,
                _ => (),
            };

            self.renderer.drawer().clear();
            self.renderer.drawer().present();
        }
    }

    pub fn quit(&self) {
        sdl2::quit();
    }
}

// Drawing routines. These are mostly aliases onto renderer.drawer()
#[unstable]
impl Window {
    pub fn draw_rect(&self, rect: shape::Rect)      { self.renderer.drawer().draw_rect(&rect) }
    pub fn fill_rect(&self, rect: shape::Rect)      { self.renderer.drawer().fill_rect(&rect) }
    pub fn draw_point(&self, point: shape::Point)   { self.renderer.drawer().draw_point(point) }

    pub fn draw_polygon(&self, polygon: shape::Polygon) {
        self.renderer.drawer().draw_points(&polygon.points[])
    }
}

