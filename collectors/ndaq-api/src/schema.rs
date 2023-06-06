use serde::{Deserialize};
use std::collections::HashMap;

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

// #[derive(Deserialize, Debug)]
// #[serde(tag = "type", rename_all = "snake_case")]
// pub struct PriceRecord {
//     pub date: String,
//     pub price: f64
// }

// 2. SEC
#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct CompanyHeaders {
    pub cik_str: u32,
    pub ticker: String,
    pub title: String
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct Companies {
    #[serde(flatten)]
    pub companies: HashMap<String, CompanyHeaders>
}

impl Companies {
    pub fn get_cik_str(&self, ticker: &str) -> Option<u32> {
        for (_, company) in &self.companies {
            if company.ticker == ticker {
                return Some(company.cik_str);
            }
        }
        None
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub struct CompanyInfo {
    pub cik: u32,
    pub entityName: String,
    // pub 
}