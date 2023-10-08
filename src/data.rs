use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BOP {
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub track: String,
    #[serde(rename = "carModel")]
    pub car_model: u32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ballastKg")]
    pub ballast_kg: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictor: Option<i32>,
}