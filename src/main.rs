use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path;

/// command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// input source file path
    #[clap(short, long)]
    input_source: Option<String>,
    /// input context json file path
    #[clap(short, long)]
    input_json: String,
    /// template directory
    #[clap(short, long)]
    template_dir: Option<String>,
    /// output directory
    #[clap(short, long)]
    output_dir: Option<String>,
    /// output json file path
    #[clap(short, long)]
    output_json: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // read input json file
    let input_json_file = fs::File::open(&args.input_json).unwrap();
    let mut input_json_obj: serde_json::Value = serde_json::from_reader(input_json_file).unwrap();

    // read source file
    if let Some(input_source_filepath) = &args.input_source {
        let input_source = fs::read_to_string(&input_source_filepath)
            .with_context(|| format!("failed to open file `{}`", &input_source_filepath))?;
        if let Some(regset) = input_json_obj.get("regset").cloned() {
            if regset.is_array() {
                for item in regset.as_array().unwrap() {
                    // Convert the item to a HashMap
                    let item_map: HashMap<String, serde_json::Value> =
                        serde_json::from_value(item.clone()).unwrap();
                    // Access the values in the HashMap
                    let name = item_map.get("name").unwrap().to_string();
                    let regex_str = item_map.get("regex").unwrap().to_string();
                    let _ =
                        update_json_object(&mut input_json_obj, &input_source, &name, &regex_str);
                }
            }
        }
    }
    /*
        if path::Path::new(&args.output_dirname).exists() {
            println!(
                "`{}` is already existed. select other folder as output",
                args.output_dirname
            );
        } else {
            fs::create_dir_all(&args.output_dirname)
                .with_context(|| format!("failed to create folder `{}`", args.output_dirname))?;
        }
    */
    Ok(())
}

/// capture values using regex
///
fn update_json_object(
    json_object: &mut serde_json::Value,
    input_str: &str,
    dataset_name: &str,
    regex_str: &str,
) -> Result<(), String> {
    let map = json_object
        .as_object_mut()
        .ok_or("JSON object is not a map")?;
    let regex = match Regex::new(regex_str) {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to compile regex {}: {}", regex_str, e)),
    };
    let mut result = Vec::new();
    let capture_names = regex.capture_names().flatten().collect::<Vec<_>>();
    if !capture_names.is_empty() {
        for capture in regex.captures_iter(input_str) {
            let mut line_result = serde_json::Map::new();
            for name in &capture_names {
                if let Some(value) = capture.name(name) {
                    line_result.insert(
                        name.to_string(),
                        serde_json::Value::String(value.as_str().to_string()),
                    );
                }
            }
            result.push(serde_json::Value::Object(line_result));
        }
    }
    map.insert(dataset_name.to_string(), serde_json::Value::Array(result));
    Ok(())
}
