use anyhow::{Context, Result};
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, JsonObject, JsonValue};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug, Deserialize, Serialize)]
pub struct InputJson<'a> {
    #[serde(rename = "geocoded_column")]
    geometry: Geometry,
    crash_circumstances: Option<String>,
    crash_date: &'a str,
    crash_location: &'a str,
    latitude: &'a str,
    longitude: &'a str,
    victim: &'a str,
}

pub fn parse_file(filename: String) -> Result<String, anyhow::Error> {
    let json = std::fs::read_to_string(&filename)
        .with_context(|| format!("could not read file `{}`", filename))?;

    let input_vec = from_str::<Vec<InputJson>>(&json).expect("Failed to parse JSON from file.");
    let feature_vec = transform_to_geojson_feature_vec(input_vec);
    let geojson = make_feature_collection(feature_vec);

    return to_string(&geojson).with_context(|| format!("Failed to convert file to JSON"));
}

fn transform_to_geojson_feature_vec(input_vec: Vec<InputJson>) -> Vec<Feature> {
    let mut feature_vec: Vec<Feature> = Vec::new();

    for input_entry in &input_vec {
        let geojson = GeoJson::Feature(Feature {
            bbox: None,
            geometry: Some(input_entry.geometry.clone()),
            id: None,
            properties: Some(geo_properties_from_input_json(&input_entry)),
            foreign_members: None,
        });

        let feature = Feature::try_from(geojson).expect("Failed to parse Feature from GeoJSON");

        feature_vec.push(feature);
    }

    return feature_vec;
}

fn make_feature_collection(input_features: Vec<Feature>) -> GeoJson {
    let feature_collection = FeatureCollection {
        bbox: None,
        features: input_features,
        foreign_members: None,
    };

    return GeoJson::from(feature_collection);
}

fn geo_properties_from_input_json(input_json: &InputJson) -> JsonObject {
    let mut properties = JsonObject::new();

    properties.insert(
        "crash_circumstances".to_string(),
        JsonValue::from(input_json.crash_circumstances.as_ref().map(String::as_str)),
    );
    properties.insert(
        "crash_date".to_string(),
        JsonValue::from(input_json.crash_date),
    );
    properties.insert(
        "crash_location".to_string(),
        JsonValue::from(input_json.crash_location),
    );
    properties.insert("latitude".to_string(), JsonValue::from(input_json.latitude));

    properties.insert(
        "longitude".to_string(),
        JsonValue::from(input_json.longitude),
    );
    properties.insert("victim".to_string(), JsonValue::from(input_json.victim));

    return properties;
}
