use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path;

/// tmpgen command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// input content json file name
    #[clap(short, long)]
    content_filename: String,
    /// template directory
    #[clap(short, long)]
    template_dirname: String,
    /// output directory
    #[clap(short, long)]
    output_dirname: String,
}

/// tmpgen content json file format
#[derive(Debug, Deserialize)]
struct Data {
    name: String,
    value: String,
}
#[derive(Debug, Deserialize)]
struct Group {
    name: String,
    data: Vec<Data>,
}
#[derive(Debug, Deserialize)]
struct Content {
    #[serde(default = "default_dataset")]
    dataset: Vec<Data>,
    #[serde(default = "default_groupset")]
    groupset: Vec<Group>,
}
fn default_dataset() -> Vec<Data> {
    Vec::new()
}
fn default_groupset() -> Vec<Group> {
    Vec::new()
}

fn main() -> Result<()> {
    let args = Args::parse();
    if path::Path::new(&args.output_dirname).exists() {
        println!(
            "`{}` is already existed. select other folder as output",
            args.output_dirname
        );
    } else {
        fs::create_dir_all(&args.output_dirname)
            .with_context(|| format!("failed to create folder `{}`", args.output_dirname))?;
    }
    Ok(())
}
