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

    println!("output={:?} size={}", args.output, args.size)
}
