#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod schema;
mod endpoints;
mod vault;

use crate::vault::*;
use crate::schema::*;
use crate::endpoints::*;
use serde::{Deserialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

const DB_CODE: &str = "WIKI";
const DS_CODE: &str = "AAPL";



#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {

    let client = reqwest::Client::new();

    // Create a path for the "live" folder
    let current_dir = env::current_dir()?;
    let live_folder_path = current_dir.join("live");
    
    // Check if the folder already exists
    if live_folder_path.is_dir() {
        fs::remove_dir_all(&live_folder_path)?;
        println!("Existing contents of 'live' folder cleared.");
    } else {
        ();
    }

    fs::create_dir(&live_folder_path)?;
    println!("'live' folder created successfully.");

    // GET Nasdaq data
    for (code_number, description, statement) in MAPCODES.iter().take(10) {
        println!("Retrieving mapcode: {}", code_number);
        let url_summ = 
            format!("https://data.nasdaq.com/api/v3/datasets/{}/{}.json?mapcode={}?api_key={}.json", DB_CODE, DS_CODE, code_number, API_KEY);
        
        let resp_summ = client.get(&url_summ).send().await?;
        let bytes = resp_summ.bytes().await?;
        
        let file_name = format!("{} - {}.json", statement, description);
        let file_path: PathBuf = live_folder_path.join(&file_name);
        
        // Create the parent directory if it doesn't exist <<< Is this needed?
        if let Some(parent_dir) = file_path.parent() {
          if !parent_dir.exists() {
              fs::create_dir_all(parent_dir)?;
          }
        }

        fs::write(&file_path, &bytes)?;
        println!("{:?}", file_path);
    }
    
    println!("Data retrieval and file creation completed.");

    Ok(())
}