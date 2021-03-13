#[macro_use]
extern crate log;

use anyhow::Result;
use std::convert::TryInto;
use std::path::PathBuf;
use structopt::StructOpt;
use texture_generator::definition::generation::TextureDefinition;
use texture_generator::generation::data::{Data, RuntimeData};
use texture_generator::generation::TextureGenerator;
use texture_generator::math::aabb::AABB;
use texture_generator::math::color::WHITE;
use texture_generator::math::size::Size;
use texture_generator::utils::logging::init_logging;

#[derive(StructOpt)]
struct Cli {
    /// The path of the texture definition.
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// The path of the output image.
    #[structopt(parse(from_os_str))]
    output: PathBuf,

    /// The size of the output image.
    #[structopt(default_value = "1024")]
    size: u32,
}

fn main() -> Result<()> {
    init_logging();

    let args = Cli::from_args();

    info!(
        "size={} input{:?} output={:?}",
        args.size, args.input, args.output
    );

    let size = Size::new(args.size, args.size);
    let aabb = AABB::with_size(size);
    let mut data = RuntimeData::new(size, WHITE);

    info!("Load definition");

    let definition = TextureDefinition::read(&args.input)?;
    let generator: TextureGenerator = definition.try_into()?;

    info!("Start rendering");

    generator.component.generate(&mut data, &aabb);

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

    Ok(())
}
