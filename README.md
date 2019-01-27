
simple
======

[![Build Status](https://travis-ci.org/alexandercampbell/simple.svg?branch=master)](https://travis-ci.org/alexandercampbell/simple)

Simple is supposed to be a completely obvious and intuitive game library for
Rust. Simple was inspired by [LOVE2D](http://love2d.org). I wrote Simple because
I liked the ease-of-use that LOVE2D provided but I wanted the advantage of a
typed compiler (Rust).

Simple is a layer built on top of the
[`sdl2`](https://github.com/Rust-SDL2/rust-sdl2) package. You will need some
development C libraries. On MacOS, you can use [Brew](https://brew.sh/) to
install them with `brew install sdl2`. On Linux, look for a `libsdl2-dev`
package in your package manager.

Hello World
-----------

```rust
let mut app = simple::Window::new("hello world", 1920, 1080);

app.set_color(255, 0, 255, 255);
app.draw_rect(simple::Rect{
    x: 100,
    y: 110,
    w: 120,
    h: 130,
});

while app.next_frame() {}
```

Examples
--------

Check out the `examples/` directory for some mini-programs that use Simple.

```sh
cargo run --example font
```


Maintainer
----------

Alexander Campbell <alexanderhcampbell@gmail.com>

