use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PropertySource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Property {
    pub value: String,
    pub source: PropertySource,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct DatasetProperties {
    pub used: Property,
    pub available: Property,
    pub referenced: Property,
    pub mountpoint: Property,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Dataset {
    pub name: String,
    #[serde(rename = "type")]
    pub dataset_type: String,
    pub pool: String,
    pub createtxg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    pub properties: DatasetProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct OutputVersion {
    pub command: String,
    pub vers_major: u32,
    pub vers_minor: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ZfsListOutput {
    pub output_version: OutputVersion,
    pub datasets: std::collections::HashMap<String, Dataset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ZfsStats {
    pub pools: Vec<String>,
    pub filesystems: Vec<Dataset>,
    pub snapshots: Vec<Dataset>,
    pub bookmarks: Vec<Dataset>,
    pub total_used: String,
    pub total_available: String,
}

