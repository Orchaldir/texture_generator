#[macro_use]
extern crate log;

use anyhow::Result;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use texture_generation::definition::generation::process::PostProcessDefinition;
use texture_generation::definition::generation::TextureDefinition;
use texture_generation::generation::process::PostProcess;
use texture_generation::generation::TextureGenerator;
use texture_generation::utils::error::DefinitionError;
use texture_generation::utils::logging::init_logging;

#[derive(StructOpt)]
#[structopt(name = "texture_generator")]
/// The arguments of the application.
struct Cli {
    /// The path of the texture definition.
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// The path of the output images.
    output: String,

    /// The path of the post processing definition.
    #[structopt(default_value = "")]
    post_processing: PathBuf,

    /// The size of the output images.
    #[structopt(default_value = "1024")]
    size: u32,
}

fn load_post_processing(path: &Path) -> Result<Vec<PostProcess>, DefinitionError> {
    if path.exists() {
        let definition = PostProcessDefinition::read(path)?;
        let post_processes = definition.into_iter().map(|d| d.into()).collect();
        Ok(post_processes)
    } else {
        Ok(Vec::default())
    }
}

fn main() -> Result<()> {
    init_logging();

    let args = Cli::from_args();

    info!(
        "size={} input={:?} output={:?} post_processing={:?}",
        args.size, args.input, args.output, args.post_processing
    );

    info!("Load texture definition");

    let definition = TextureDefinition::read(&args.input)?;
    let generator: TextureGenerator = definition.convert(args.size)?;
    let color_path = format!("{}-color.png", args.output);
    let depth_path = format!("{}-depth.png", args.output);

    info!("Load post processing definition");

    let post_processes: Vec<PostProcess> = load_post_processing(&args.post_processing)?;

    info!("Texture generation");

    let mut data = generator.generate();

    info!("Post processing. N={}", post_processes.len());

    data.apply(&post_processes);

    data.save_color_image(&color_path);
    data.save_depth_image(&depth_path);

    info!("Finished");

    Ok(())
}
