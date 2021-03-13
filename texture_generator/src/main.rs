#[macro_use]
extern crate log;

use structopt::StructOpt;
use texture_generator::generation::component::Component;
use texture_generator::generation::data::{Data, RuntimeData};
use texture_generator::generation::layout::LayoutComponent;
use texture_generator::generation::rendering::RenderingComponent;
use texture_generator::math::aabb::AABB;
use texture_generator::math::color::{BLUE, WHITE};
use texture_generator::math::shape::Shape;
use texture_generator::math::size::Size;
use texture_generator::utils::logging::init_logging;

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
    init_logging();

    let args = Cli::from_args();

    info!("size={} output={:?}", args.size, args.output);

    let size = Size::new(args.size, args.size);
    let layout_size = args.size / 8;
    let aabb = AABB::with_size(size);
    let mut data = RuntimeData::new(size, WHITE);

    let circle = Shape::new_circle(layout_size / 3).unwrap();
    let rendering = RenderingComponent::new_shape(circle, BLUE);
    let component = Component::Rendering(Box::new(rendering));
    let layout = LayoutComponent::new_square("layout", layout_size, component).unwrap();

    info!("Start rendering");

    layout.generate(&mut data, &aabb);

    info!("Start saving");

    image::save_buffer(
        &args.output,
        data.get_color_data(),
        size.width(),
        size.height(),
        image::ColorType::Rgb8,
    )
    .unwrap();

    info!("Finished");
}
