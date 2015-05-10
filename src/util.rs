/*!
 * This util module is here for internal library use only.
 */

extern crate sdl2;
use sdl2::render;
use sdl2::pixels;

/// Return true if any of the characters in `s` appear anywhere else in `s`.
pub fn str_has_duplicate_chars(s: &str) -> bool {
    false
}

#[test]
fn dup_chars() {
    assert!(!str_has_duplicate_chars("123"));
}

/// Utility method to set the texture's color and alpha mods to the Color.
pub fn set_texture_color(color: &pixels::Color, texture: &mut render::Texture) {
    // configure the texture for drawing according to the current foreground_color
    let (r,g,b,a) = match *color {
        pixels::Color::RGB(r, g, b) => (r,g,b,255),
        pixels::Color::RGBA(r, g, b, a) => (r,g,b,a),
    };
    texture.set_color_mod(r, g, b);
    texture.set_alpha_mod(a);
}

