use clap::Parser;
use std::{io::Write, path::PathBuf};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Parser)]
struct Cli {
    /// .chart file to be used
    input_file: PathBuf,

    /// .chart file to be written to
    #[arg(short, long)]
    output_file: Option<PathBuf>,

    /// Factor to multiply resolution by
    multiplier: u32,
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    let cli = Cli::parse();
    let text = std::fs::read_to_string(cli.input_file).unwrap();
    let mut chart = chart(&text).unwrap().1;
    chart.multiply_res(cli.multiplier);
    match cli.output_file {
        Some(file) => {
            let mut file = std::fs::File::create(file).unwrap();
            file.write_all(chart.to_string().as_bytes()).unwrap();
        }
        None => {
            // println!("{chart}");
        }
    }
}
