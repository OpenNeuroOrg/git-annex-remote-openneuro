use hyper::body::Body;
use hyper::{Method, Request, Result};
use hyper_util::client::legacy::Client;
use std::fs;

use serde::{Deserialize, Serialize};

const PREPARE_REPO_ACCESS: &str = "  
mutation prepareRepoAccess($datasetId: ID!) {
    prepareRepoAccess(datasetId: $datasetId) {
      token
      endpoint
    }
  }
";

struct PrepareRepoAccessQueryVariables {
    datasetId: String,
}

#[derive(Serialize, Deserialize)]
struct PrepareRepoAccessQuery {
    mutation: String,
    variables: PrepareRepoAccessQueryVariables,
}

pub fn repo_access_query(dataset_id: String) -> Result<String> {
    let variables = PrepareRepoAccessQueryVariables {
        datasetId: dataset_id,
    };
    let query = PrepareRepoAccessQuery {
        mutation: PREPARE_REPO_ACCESS,
        variables: variables,
    };

    serde_json::to_string(&query)?;
}

pub fn prepare_repo_access() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Maybe use https://crates.io/crates/config
    let data = fs::read_to_string("~/.openneuro").expect("Unable to read file");
    let config_json: serde_json::Value =
        serde_json::from_str(&data).expect("JSON does not have correct format.");
    dbg!(config_json);

    Request::builder()
        .method(Method::POST)
        .uri("https://staging.openneuro.org/crn/graphql")
        .header("content-type", "application/json")
        .body(Body::from(repo_access_query("ds001150")))?;
}

#[tokio::main]
pub async fn request_access(dataset_id: String) {
    let req = prepare_repo_access();
    let client = Client::new();
    let resp = client.request(req).await?;

    println!("Response: {}", resp.status());
}
