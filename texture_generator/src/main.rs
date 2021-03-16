#[macro_use]
extern crate log;

use anyhow::Result;
use std::convert::TryInto;
use std::path::PathBuf;
use structopt::StructOpt;
use texture_generator::definition::generation::TextureDefinition;
use texture_generator::generation::data::{convert, Data};
use texture_generator::generation::TextureGenerator;
use texture_generator::utils::logging::init_logging;

#[derive(StructOpt)]
/// The arguments of the application.
struct Cli {
    /// The path of the texture definition.
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// The path of the output image.
    output: String,

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

    info!("Load definition");

    let definition = TextureDefinition::read(&args.input)?;
    let generator: TextureGenerator = definition.try_into()?;
    let color_path = format!("{}-color.png", args.output);
    let depth_path = format!("{}-depth.png", args.output);

    info!("Rendering");

    let data = generator.generate(args.size, args.size);

    info!("Save color to {:?}", color_path);

    let color_data = convert(&data.get_color_data());

    image::save_buffer(
        &color_path,
        &color_data,
        data.get_size().width(),
        data.get_size().height(),
        image::ColorType::Rgb8,
    )
    .unwrap();

    info!("Save depth to {:?}", depth_path);

    image::save_buffer(
        &depth_path,
        data.get_depth_data(),
        data.get_size().width(),
        data.get_size().height(),
        image::ColorType::L8,
    )
    .unwrap();

    info!("Finished");

    Ok(())
}
