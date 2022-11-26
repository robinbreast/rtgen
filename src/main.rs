use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    /// input data file path
    #[clap(short, long)]
    input: String,
    /// template directory
    temp_dir: String,
    /// output directory
    output_dir: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if path::Path::new(&args.output_dir).exists() {
        println!(
            "`{}` is already existed. select other folder as output",
            args.output_dir
        );
    } else {
        fs::create_dir_all(path)
    }
    Ok(())
}
