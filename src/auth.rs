use dirs;
use hyper::header::AUTHORIZATION;
use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;

const PREPARE_REPO_ACCESS: &str = "  
mutation prepareRepoAccess($datasetId: ID!) {
    prepareRepoAccess(datasetId: $datasetId) {
      token
      endpoint
    }
  }
";

fn repo_access_query(dataset_id: &str) -> Value {
    json!({
        "mutation": PREPARE_REPO_ACCESS.to_owned(),
        "variables": { "datasetId": dataset_id }
    })
}

#[derive(Serialize, Deserialize)]
struct OpenNeuroConfig {
    token: String,
    url: String,
}

pub async fn prepare_repo_access(dataset_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir();
    let mut home_dir = match home {
        Some(path) => path,
        None => {
            println!("Error: Path not found.");
            return Ok(());
        }
    };
    home_dir.push(".openneuro");
    // Maybe use https://crates.io/crates/config
    let data = fs::read_to_string(home_dir.as_path()).expect("Unable to read file");
    let config: OpenNeuroConfig =
        serde_json::from_str(&data).expect("JSON does not have correct format.");

    // HTTP request
    let client = reqwest::Client::new();
    let res = client
        .post(config.url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", config.token))
        .body(repo_access_query(dataset_id).to_string())
        .send()
        .await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    Ok(())
}
