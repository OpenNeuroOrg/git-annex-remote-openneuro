use reqwest;
use serde_json::{json, Value};

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

pub async fn prepare_repo_access(
    dataset_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://staging.openneuro.org/crn/graphql")
        .body(repo_access_query(dataset_id).to_string())
        .send()
        .await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    Ok(())
}
