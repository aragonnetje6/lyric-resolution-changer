use chart_file_parser::chart::chart;
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

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Parsing failed")]
    ParseError(String),
}

fn main() -> Result<(), Error> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let cli = Cli::parse();
    let text = std::fs::read_to_string(cli.input_file)?;
    let mut chart = chart(&text)
        .map_err(|err| Error::ParseError(err.to_string()))?
        .1;
    chart.multiply(cli.multiplier);
    match cli.output_file {
        Some(file) => {
            let mut file = std::fs::File::create(file)?;
            file.write_all(chart.to_string().as_bytes())?;
        }
        None => {
            println!("{chart}");
        }
    }
    Ok(())
}
