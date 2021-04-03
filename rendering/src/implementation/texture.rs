use glium::backend::Facade;
use image::io::Reader;

pub fn load_texture<F: Facade>(
    display: &F,
    file: &str,
) -> Result<glium::texture::Texture2d, glium::texture::TextureCreationError> {
    let path = "resources/image/";
    let image = Reader::open([path, file].concat())
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, image)
}
