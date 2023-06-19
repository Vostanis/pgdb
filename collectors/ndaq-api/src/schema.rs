use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct NasdaqResponse {
    pub dataset: Dataset
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct Dataset {
    pub id: i32,
    pub dataset_code: String,
    pub database_code: String,
    pub name: String,
    pub description: String,
    pub refreshed_at: String,
    pub start_date: String,
    pub end_date: String,
    pub data: Vec<(String, f64)>
}