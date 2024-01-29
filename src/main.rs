mod display;
mod parser;

use clap::Parser;
use std::{io::Write, path::PathBuf};

use crate::parser::chart;

#[derive(Parser)]
struct Cli {
    /// .chart file to be used
    input_file: PathBuf,

    /// .chart file to be written to
    output_file: PathBuf,

    /// Factor to multiply resolution by
    multiplier: u32,
}

fn main() {
    let cli = Cli::parse();
    let text = std::fs::read_to_string(cli.input_file).unwrap();
    let mut chart = chart(&text).unwrap().1;
    chart.multiply_res(cli.multiplier);
    let mut file = std::fs::File::create(cli.output_file).unwrap();
    file.write_all(chart.to_string().as_bytes()).unwrap();
    println!("{chart}");
}
