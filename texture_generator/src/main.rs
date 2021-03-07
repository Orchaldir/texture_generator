use structopt::StructOpt;

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

    let pink = image::Rgb([255u8, 0, 128]);
    let image = image::ImageBuffer::from_pixel(args.size, args.size, pink);

    image.save(&args.output).unwrap();
}
