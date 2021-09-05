use crate::mandelbrot;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Mandelbrot", version = "1.0.0")]
struct MandelbrotCLI {
    #[structopt(short, long, default_value = "1080")]
    height: u32,

    #[structopt(short, long, default_value = "1920")]
    width: u32,

    #[structopt(short, long, default_value = "mandelbrot.jpeg")]
    filename: String,
}

pub fn run() {
    let args = MandelbrotCLI::from_args();
    mandelbrot::generate(args.width, args.height)
        .save(&args.filename)
        .unwrap_or_else(|error| eprintln!("{}", error));
}
