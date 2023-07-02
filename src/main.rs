use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// input source file path
    #[clap(short, long)]
    input_source: Option<String>,
    /// input context json file path
    #[clap(short, long)]
    context_json: String,
    /// template directory
    #[clap(short, long)]
    template_dir: Option<String>,
    /// output directory
    #[clap(short, long)]
    output_dir: Option<String>,
    /// debug json file path
    #[clap(short, long)]
    debug_json: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // read input json file
    let context_json = fs::File::open(&args.context_json)
        .expect(&format!("failed to open {}", &args.context_json));
    let mut context_json: serde_json::Value = serde_json::from_reader(context_json).unwrap();

    // read source file
    if let Some(input_source_filepath) = &args.input_source {
        let input_source = fs::read_to_string(&input_source_filepath)
            .with_context(|| format!("failed to open file `{}`", &input_source_filepath))?;
        if let Some(regset) = context_json.get("regset").cloned() {
            if regset.is_array() {
                for item in regset.as_array().unwrap() {
                    // Convert the item to a HashMap
                    let item_map: HashMap<String, serde_json::Value> =
                        serde_json::from_value(item.clone()).unwrap();
                    // Access the values in the HashMap
                    let name = item_map.get("name").unwrap().to_string();
                    let regex_str = item_map.get("regex").unwrap().to_string();
                    let _ = update_json_object(&mut context_json, &input_source, &name, &regex_str);
                }
            }
        }
    }
    if let Some(debug_json) = &args.debug_json {
        let dirpath = Path::new(&debug_json)
            .parent()
            .unwrap_or_else(|| Path::new("./"));
        if !dirpath.exists() {
            fs::create_dir_all(&dirpath).with_context(|| {
                format!("failed to create folder `{}`", dirpath.to_string_lossy())
            })?;
        }
        let _ = generate_json(&context_json, debug_json);
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

pub fn generate_json(json_object: &serde_json::Value, filepath: &String) -> Result<()> {
    let file = fs::File::create(filepath)
        .with_context(|| format!("failed to create file `{}`", filepath))?;
    serde_json::to_writer(file, json_object)?;
    Ok(())
}
