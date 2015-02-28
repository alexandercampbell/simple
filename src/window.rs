
use std;

extern crate sdl2;
use sdl2::video::{self,WindowPos};

use event::{self,Event};
use shape;

///
/// A Window can display graphics, play sounds, and handle events.
///
/// Creating multiple Windows is untested!
///
pub struct Window {
    context:        sdl2::sdl::Sdl,
    renderer:       sdl2::render::Renderer,
    running:        bool,
    event_queue:    std::vec::Vec<Event>,

    // timing fields
    target_ticks_per_frame:     u32,
    ticks_at_previous_frame:    u32,
}

/// Top-level Running / Creation Methods
/// ------------------------------------
impl Window {
    /// Intialize a new running window. `name` is used as a caption.
    pub fn new(name: &str, width: i32, height: i32) -> Self {
        let sdl_context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();

        let sdl_window = video::Window::new(
            name, WindowPos::PosCentered, WindowPos::PosCentered,
            width, height, video::SHOWN,
        ).unwrap();

        let renderer = sdl2::render::Renderer::from_window(
            sdl_window,
            sdl2::render::RenderDriverIndex::Auto,
            sdl2::render::ACCELERATED,
        ).unwrap();

        let window = Window{
            context:                    sdl_context,
            renderer:                   renderer,
            running:                    true,
            event_queue:                vec![],
            target_ticks_per_frame:     (1000.0 / 60.0) as u32,
            ticks_at_previous_frame:    0,
        };
        window.clear();
        window
    }

    /// Redrawing and update the display, while maintaining a consistent framerate and updating the
    /// event queue. You should draw your objects immediately before you call this function. NOTE:
    /// This function returns false if the program should terminate.
    pub fn next_frame(&mut self) -> bool {
        if !self.running {
            return false;
        }

        self.renderer.drawer().present();

        let mut current_ticks = sdl2::timer::get_ticks();
        while current_ticks - self.ticks_at_previous_frame < self.target_ticks_per_frame {
            sdl2::timer::delay(5);
            current_ticks = sdl2::timer::get_ticks();
        }
        self.ticks_at_previous_frame = current_ticks;

        // Handle events
        let sdl_event = self.context.event_pump().poll_event();
        match sdl_event {
            Some(sdl_event) => {
                let event = Event::from_sdl2_event(sdl_event);
                match event {
                    Some(Event::Quit) => self.quit(),
                    Some(Event::Keyboard{key: event::KeyCode::Escape, ..})  => self.quit(),

                    // any other unrecognized event
                    Some(e) => (self.event_queue.push(e)),
                    None => (),
                };
            },
            _ => (),
        }

        true
    }

    /// Return true when there is an event waiting in the queue for processing.
    pub fn has_event(&self) -> bool { self.event_queue.len() > 0 }

    /// Get the next event from the queue. NOTE: If the event queue on the Window is empty, this
    /// function will panic. Call `has_event()` to find out if there is an event ready for
    /// processing.
    ///
    /// Note that events are handled in a first-in-first-out order. If a user presses three keys 1,
    /// 2, 3 during a frame, then the next three calls to next_event will return 1, 2, 3 in the
    /// same order.
    pub fn next_event(&mut self) -> Event { self.event_queue.remove(0) }

    /// This does not actually cause the program to exit. It just means that next_frame will return
    /// false on the next call.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

/// Drawing Methods
/// ---------------
impl Window {
    /// Windows have a color set on them at all times. This color is applied to every draw
    /// operation. To "unset" the color, call set_color with (255,255,255,255)
    pub fn set_color(&self, red: u8, green: u8, blue: u8, alpha: u8) {
        let color_struct = sdl2::pixels::Color::RGBA(red, green, blue, alpha);
        self.renderer.drawer().set_draw_color(color_struct);
    }

    // These functions are just aliases onto self.renderer.drawer() as you can see.
    pub fn draw_rect(&self, rect: shape::Rect)      { self.renderer.drawer().draw_rect(rect) }
    pub fn fill_rect(&self, rect: shape::Rect)      { self.renderer.drawer().fill_rect(rect) }
    pub fn draw_point(&self, point: shape::Point)   { self.renderer.drawer().draw_point(point) }

    #[unstable]
    pub fn draw_polygon(&self, polygon: shape::Polygon) {
        self.renderer.drawer().draw_points(&polygon.points[..])
    }

    /// Clear the screen to black. This will set the Window's draw color to (0,0,0,255)
    pub fn clear(&self) {
        self.set_color(0, 0, 0, 255);
        self.renderer.drawer().clear();
    }
}

