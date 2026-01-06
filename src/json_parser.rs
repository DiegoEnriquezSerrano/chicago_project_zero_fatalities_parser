use anyhow::{Context, Result};
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, JsonObject, JsonValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct GeoJsonParser {
    pub geojson: GeoJson,
}

impl GeoJsonParser {
    pub fn parse_file<P>(path: P) -> Result<Self, anyhow::Error>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        let json = Self::read_from_file(path)?;
        Self::parse_raw(json)
    }
    pub fn pull_and_parse() -> Result<Self, anyhow::Error> {
        let raw_data =
            reqwest::blocking::get("https://data.cityofchicago.org/resource/gzaz-isa6.json")?
                .text()
                .context("Failed to get vision zero data.")?;
        Self::parse_raw(raw_data)
    }

    pub fn parse_raw(text: String) -> Result<Self, anyhow::Error> {
        let inputs = Self::get_input_list(text)?;
        let features = Self::convert_to_feature_list(inputs)?;

        Ok(Self::make_feature_collection(features))
    }

    fn make_feature_collection(input_features: Vec<Feature>) -> Self {
        let feature_collection = FeatureCollection {
            bbox: None,
            features: input_features,
            foreign_members: None,
        };

        Self {
            geojson: GeoJson::from(feature_collection),
        }
    }

    fn convert_to_feature_list(
        entries: Vec<VisionZeroData>,
    ) -> Result<Vec<Feature>, anyhow::Error> {
        let mut feature_vec: Vec<Feature> = Vec::new();

        for input_entry in entries {
            let feature: Feature = input_entry.try_into()?;
            feature_vec.push(feature);
        }

        Ok(feature_vec)
    }

    fn get_input_list(json: String) -> Result<Vec<VisionZeroData>, anyhow::Error> {
        serde_json::from_str(&json).with_context(|| format!("could not parse json from `{}`", json))
    }

    fn read_from_file<P>(path: P) -> Result<String, anyhow::Error>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        std::fs::read_to_string(&path).with_context(|| format!("could not read file `{:?}`", path))
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct VisionZeroData {
    crash_circumstances: Option<String>,
    crash_date: String,
    crash_location: String,
    #[serde(rename = "geocoded_column")]
    geometry: Geometry,
    latitude: String,
    longitude: String,
    victim: String,
}

impl From<VisionZeroData> for JsonObject {
    fn from(value: VisionZeroData) -> Self {
        let mut properties = JsonObject::new();
        properties.insert(
            "crash_circumstances".to_string(),
            JsonValue::from(value.crash_circumstances.as_deref()),
        );
        properties.insert("crash_date".to_string(), Value::from(value.crash_date));
        properties.insert(
            "crash_location".to_string(),
            Value::from(value.crash_location),
        );
        properties.insert("latitude".to_string(), Value::from(value.latitude));
        properties.insert("longitude".to_string(), Value::from(value.longitude));
        properties.insert("victim".to_string(), Value::from(value.victim));
        properties
    }
}

impl TryInto<Feature> for VisionZeroData {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Feature, Self::Error> {
        let geojson = GeoJson::Feature(Feature {
            bbox: None,
            foreign_members: None,
            geometry: Some(self.geometry.clone()),
            id: None,
            properties: Some(self.into()),
        });

        Feature::try_from(geojson).context("Failed to parse Feature from GeoJSON.")
    }
}
