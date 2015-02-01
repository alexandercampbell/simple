
use std;

extern crate sdl2;
use sdl2::video::{self,WindowPos};

use event::{self,Event};
use shape;

/// Windows can display graphics, play sounds, and return events.
pub struct Window {
    renderer:           sdl2::render::Renderer,
    running:            bool,
    event_queue:        std::vec::Vec<Event>,
    ticks_per_frame:    u32,
}

/// Top-level running / creation methods.
impl Window {
    pub fn new(name: &str, width: i32, height: i32) -> Self {
        sdl2::init(sdl2::INIT_EVERYTHING);

        let sdl_window = video::Window::new(
            name, WindowPos::PosCentered, WindowPos::PosCentered,
            width, height, video::SHOWN,
        ).unwrap();

        let renderer = sdl2::render::Renderer::from_window(
            sdl_window,
            sdl2::render::RenderDriverIndex::Auto,
            sdl2::render::ACCELERATED,
        ).unwrap();

        Window{
            renderer:           renderer,
            running:            true,
            event_queue:        vec![],
            ticks_per_frame:    (60.0 / 1000.0) as u32,
        }
    }

    /// Do most of the heavy lifting in redrawing and updating the display.
    pub fn next_frame(&mut self) -> bool {
        if !self.running {
            return false;
        }

        self.renderer.drawer().present();

        let mut ticks = sdl2::timer::get_ticks();
        while ticks < self.ticks_per_frame {
            sdl2::timer::delay(5);
            ticks = sdl2::timer::get_ticks();
        }

        // Handle events
        let event = Event::from_sdl2_event(sdl2::event::poll_event());
        match event {
            Some(Event::Quit) => self.quit(),
            Some(Event::Keyboard{key: event::KeyCode::Escape, ..})  => self.quit(),

            // any other unrecognized event
            Some(e) => (self.event_queue.push(e)),
            None => (),
        };

        self.set_color(0, 0, 0, 255);
        self.renderer.drawer().clear();

        true
    }

    pub fn has_event(&self) -> bool { self.event_queue.len() > 0 }
    pub fn next_event(&mut self) -> Event { self.event_queue.remove(0) }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

/// Drawing routines. These are mostly aliases onto renderer.drawer()
impl Window {
    pub fn draw_rect(&self, rect: shape::Rect)      { self.renderer.drawer().draw_rect(&rect) }
    pub fn fill_rect(&self, rect: shape::Rect)      { self.renderer.drawer().fill_rect(&rect) }
    pub fn draw_point(&self, point: shape::Point)   { self.renderer.drawer().draw_point(point) }

    #[unstable]
    pub fn draw_polygon(&self, polygon: shape::Polygon) {
        self.renderer.drawer().draw_points(&polygon.points[])
    }

    /// Windows have a color set on them at all times. This color is applied to every draw
    /// operation. To "unset" the color, call set_color with (255,255,255,255)
    pub fn set_color(&self, red: u8, green: u8, blue: u8, alpha: u8) {
        let color_struct = sdl2::pixels::Color::RGBA(red, green, blue, alpha);
        self.renderer.drawer().set_draw_color(color_struct);
    }
}

// Dtor for Window.
impl std::ops::Drop for Window {
    /// close the window
    fn drop(&mut self) {
        sdl2::quit();
    }
}

