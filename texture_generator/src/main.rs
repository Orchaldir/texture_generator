#[macro_use]
extern crate log;

use anyhow::Result;
use std::convert::TryInto;
use std::path::PathBuf;
use structopt::StructOpt;
use texture_generator::definition::generation::TextureDefinition;
use texture_generator::generation::data::Data;
use texture_generator::generation::TextureGenerator;
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

    info!("Load definition");

    let definition = TextureDefinition::read(&args.input)?;
    let generator: TextureGenerator = definition.try_into()?;

    info!("Start rendering");

    let data = generator.generate(args.size, args.size);

    info!("Start saving");

    image::save_buffer(
        &args.output,
        data.get_color_data(),
        data.get_size().width(),
        data.get_size().height(),
        image::ColorType::Rgb8,
    )
    .unwrap();

    info!("Finished");

    Ok(())
}
