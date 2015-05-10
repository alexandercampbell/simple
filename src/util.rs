/*!
 * This util module is here for internal library use only.
 */

use std::collections::HashMap;

extern crate sdl2;
use sdl2::render;
use sdl2::pixels;

/// Return true if any of the characters in `s` appear anywhere else in `s`.
pub fn string_has_duplicate_chars(s: String) -> bool {
    // A very naive implementation, obviously, but this function doesn't need to be quick.
    let mut h = HashMap::new();
    for ch in s.chars() {
        if h.contains_key(&ch) {
            return true;
        }
        h.insert(ch, true);
    }
    false
}

#[test]
fn dup_chars() {
    assert!(!string_has_duplicate_chars("123".to_string()));
    assert!(!string_has_duplicate_chars("".to_string()));
    assert!(!string_has_duplicate_chars("aAbBcC".to_string())); // case sensitivity check

    assert!(string_has_duplicate_chars("11".to_string()));
    assert!(string_has_duplicate_chars("250 asdf 5".to_string()));

    // real-world test case
    assert!(!string_has_duplicate_chars(
    " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,!?-+/():;%&`'*#=[]\""
    .to_string()));
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

