use dirs;
use reqwest;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::{json, Number, Value};
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
        "query": PREPARE_REPO_ACCESS.to_owned(),
        "variables": { "datasetId": dataset_id }
    })
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenNeuroConfig {
    url: String,
    apikey: String,
    error_reporting: bool,
}

#[derive(Serialize, Deserialize)]
struct PrepareRepoAccessQuery {
    endpoint: Number,
    token: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrepareRepoAccessData {
    prepare_repo_access: PrepareRepoAccessQuery,
}

#[derive(Serialize, Deserialize)]
struct PrepareRepoAccess {
    data: PrepareRepoAccessData,
}

pub async fn prepare_repo_access(
    dataset_id: &str,
) -> Result<(String, i64), Box<dyn std::error::Error>> {
    let home = dirs::home_dir();
    let mut home_dir = match home {
        Some(path) => path,
        None => {
            panic!("Error: Home path not found.");
        }
    };
    home_dir.push(".openneuro");
    // Maybe use https://crates.io/crates/config
    let data = fs::read_to_string(home_dir.as_path()).expect("Unable to read file");
    dbg!(&data);
    let config: OpenNeuroConfig =
        serde_json::from_str(&data).expect("JSON does not have correct format.");

    // HTTP request
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}crn/graphql", config.url))
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", config.apikey))
        .body(repo_access_query(dataset_id).to_string())
        .send()
        .await?;

    let auth_response: PrepareRepoAccess = res.json::<PrepareRepoAccess>().await?;

    Ok((
        auth_response.data.prepare_repo_access.token,
        auth_response.data.prepare_repo_access.endpoint.as_i64().unwrap(),
    ))
}
