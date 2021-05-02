use crate::generation::data::texture::Data;
use crate::math::color::convert;

/// Save the color image.
pub fn save_color_image(texture: &dyn Data, path: &str) {
    info!("Save color to {:?}", path);

    let size = texture.get_size();
    let color_data = convert(texture.get_color_data());

    image::save_buffer(
        path,
        &color_data,
        size.width(),
        size.height(),
        image::ColorType::Rgb8,
    )
    .unwrap();
}

/// Save the depth image.
pub fn save_depth_image(texture: &dyn Data, path: &str) {
    info!("Save depth to {:?}", path);

    let size = texture.get_size();

    image::save_buffer(
        path,
        texture.get_depth_data(),
        size.width(),
        size.height(),
        image::ColorType::L8,
    )
    .unwrap();
}
