
extern crate sdl2;
extern crate sdl2_image;

use sdl2::surface::Surface;
use sdl2_image::LoadSurface;

pub struct Image {
    surface:    Surface,
}

impl Image {
    /// Load the Image at the Path you specify.
    ///
    /// Do not call this function until after you have created a Window.
    pub fn from_file(filename: Path) -> Result<Image,String> {
        let surface:Surface = match LoadSurface::from_file(&filename) {
            Ok(surf) => surf,
            Err(msg) => return Err(msg),
        };

        Ok(Image{
            surface: surface,
        })
    }
}

#[test]
fn image_from_file() {
    match Image::from_file(Path::new("/dev/null")) {
        Ok(_)   => panic!("Shouldn't be possible to load an image from /dev/null"),
        Err(_)  => (),
    }
}


