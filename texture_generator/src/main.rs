use structopt::StructOpt;
use texture_generator::generation::rendering::RenderComponent;
use texture_generator::generation::{RuntimeData, RuntimeDataImpl};
use texture_generator::math::aabb::AABB;
use texture_generator::math::color::{BLUE, WHITE};
use texture_generator::math::shape::Shape;
use texture_generator::math::size::Size;

#[derive(StructOpt)]
struct Cli {
    /// The path of the output image.
    #[structopt(parse(from_os_str))]
    output: std::path::PathBuf,

    /// The size of the output image.
    #[structopt(default_value = "1024")]
    size: u32,
}

fn main() {
    let args = Cli::from_args();

    println!(
        "Save image of size {0}*{0} to {1:?}",
        args.size, args.output
    );

    let size = Size::new(args.size, args.size);
    let aabb = AABB::with_size(size);
    let mut data = RuntimeDataImpl::new(size, WHITE);

    let circle = Shape::new_circle(args.size / 3);
    let renderer = RenderComponent::new_shape(circle, BLUE);

    renderer.render(&mut data, &aabb);

    image::save_buffer(
        &args.output,
        data.get_colors(),
        size.width(),
        size.height(),
        image::ColorType::Rgb8,
    )
    .unwrap()
}
