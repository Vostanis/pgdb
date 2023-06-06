#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod schema;
mod endpoints;
mod vault;

use crate::vault::*;
use crate::schema::*;
use crate::endpoints::*;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::path::Path;

use serde::{Deserialize};
use serde_json::{Value, from_reader};
use tokio_postgres::{Config, NoTls};
use tokio_postgres::types::ToSql;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type;

const DB_CODE: &str = "WIKI";
const DS_CODE: &str = "AAPL";

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {

    // Init clients for HTTPS requests and PostgreSQL
    let http_client = reqwest::Client::new();
    let (pg_client, conn) = tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;


    // Directories
    let dir_ndaq = env::current_dir()?;
    let dir_coll = dir_ndaq.parent().unwrap();
    let dir_pgdb = dir_coll.parent().unwrap();


    // Staging Data
    let dir_stage = dir_pgdb.join("stage");
    if dir_stage.is_dir() {
        fs::remove_dir_all(&dir_stage)?;
        println!("Existing contents of 'stage' folder cleared.");
    } else {
        ();
    }
    fs::create_dir(&dir_stage)?;
    println!("'stage' folder created successfully.");


    // GET Nasdaq data
    for (code_number, description, statement) in MAPCODES.iter().take(10) {
        println!("Retrieving mapcode: {}", code_number);
        let url_summ = 
            format!("https://data.nasdaq.com/api/v3/datasets/{}/{}.json?mapcode={}?api_key={}.json", DB_CODE, DS_CODE, code_number, API_KEY);
        let resp_summ = client.get(&url_summ).send().await?;
        let bytes = resp_summ.bytes().await?;
        
        let file_name = format!("{} - {}.json", statement, description);
        let file_path: PathBuf = dir_stage.join(&file_name);
        // let file_path: PathBuf = live_folder_path.join(&file_name);
        
        if let Some(parent_dir) = file_path.parent() {
          if !parent_dir.exists() {
              fs::create_dir_all(parent_dir)?;
          }
        }

        fs::write(&file_path, &bytes)?;
        println!("{:?}", file_path);
    }
    println!("Data retrieval and file creation completed.");

    // Import staged data into PG Server


    Ok(())
}

async fn load_json_files_to_pg() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = Config::new()
        .host("localhost")
        .user("your_username")
        .password("your_password")
        .dbname("your_database")
        .connect(NoTls)
        .await?;

    // Create a table in your database to store the JSON data
    client.batch_execute("CREATE TABLE IF NOT EXISTS json_data (data jsonb)").await?;

    let stage_folder = "./stage/"; // Path to your stage folder containing the JSON files

    let entries = std::fs::read_dir(stage_folder)?;
    for entry in entries {
        let file = entry?.path();
        let file_name = file.file_name().unwrap().to_str().unwrap().to_owned();

        let file = File::open(file)?;
        let reader = BufReader::new(file);

        let json_data: Value = from_reader(reader)?;

        // Convert the JSON data to a format suitable for insertion
        let json_data = json_data.to_string();
        let json_data: &(dyn ToSql + Sync) = &json_data;

        client.execute("INSERT INTO json_data (data) VALUES ($1)", &[json_data]).await?;
        println!("Inserted {} into the database", file_name);
    }

    connection.await?;
    Ok(())
}
