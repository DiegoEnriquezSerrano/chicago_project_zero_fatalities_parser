#![recursion_limit = "2000"]

use anyhow::{Context, Result};
use chicago_project_zero_fatalities_parser::json_parser::parse_file;
use clap::Parser;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    input_path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let geo_json_result: String = match parse_file(args.input_path.display().to_string()) {
        Ok(content) => content,
        Err(error) => {
            return Err(error.into());
        }
    };

    let mut file = File::create("output/output.json").expect("Failed to create file.");
    file.write_all(geo_json_result.as_bytes())
        .with_context(|| format!("failed to write to output file"))?;

    Ok(())
}
