#![recursion_limit = "2000"]

use anyhow::Context;
use chicago_project_zero_fatalities_parser::json_parser::GeoJsonParser;
use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<(), anyhow::Error> {
    println!("Pulling fresh Chicago Vision Zero Fatalities json data.");
    io::stdout().flush().unwrap();

    let geo_json_result: String = match GeoJsonParser::pull_and_parse() {
        Ok(content) => content.geojson.to_string(),
        Err(error) => Err(error)?,
    };

    println!("Writing parsed Chicago Vision Zero Fatalities GeoJSON to file.");
    io::stdout().flush().unwrap();

    let mut file = File::create("output/output.json").context("Failed to create file.")?;
    file.write_all(geo_json_result.as_bytes())
        .with_context(|| format!("failed to write geojson to file: {}", geo_json_result))?;

    println!("Finished writing to file!");
    io::stdout().flush().unwrap();

    Ok(())
}
