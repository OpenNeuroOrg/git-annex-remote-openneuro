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

const VALID_RESPONSE: &str = r#"
{
    "data": {
        "prepareRepoAccess": {"token": "abcdefg", "endpoint": 0}
    },
    "errors": [],
    "extensions": {
        "openneuro": {
            "version": "4.22.0"
        }
    }
}
"#;

const ERROR_RESPONSE: &str = r#"
{
    "data": {
        "prepareRepoAccess": null
    },
    "errors": [
        {
            "extensions": {
                "code": "INTERNAL_SERVER_ERROR",
                "stacktrace": [
                    "Error: You do not have access to modify this dataset.",
                    "    at checkDatasetWrite (/srv/packages/openneuro-server/dist/graphql/permissions.js:139:15)",
                    "    at process.processTicksAndRejections (node:internal/process/task_queues:95:5)",
                    "    at async prepareRepoAccess (/srv/packages/openneuro-server/dist/graphql/resolvers/git.js:17:5)"
                ]
            },
            "locations": [
                {
                    "column": 5,
                    "line": 3
                }
            ],
            "message": "You do not have access to modify this dataset.",
            "path": [
                "prepareRepoAccess"
            ]
        }
    ],
    "extensions": {
        "openneuro": {
            "version": "4.22.0"
        }
    }
}
"#;

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
#[derive(Debug)]
struct PrepareRepoAccessQuery {
    endpoint: Number,
    token: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
struct PrepareRepoAccessData {
    prepare_repo_access: Option<PrepareRepoAccessQuery>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Error {
    message: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct PrepareRepoAccess {
    data: PrepareRepoAccessData,
    errors: Vec<Error>,
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
    /* Test lines for working offline */
    // let auth_response: PrepareRepoAccess = serde_json::from_str(ERROR_RESPONSE)?;
    // let _auth_response: PrepareRepoAccess = serde_json::from_str(VALID_RESPONSE)?;

    let PrepareRepoAccess { data, errors } = auth_response;

    let resdata = match data.prepare_repo_access {
        Some(data) => data,
        None => {
            panic!("Error: {}", errors[0].message);
        }
    };

    Ok((
        resdata.token,
        resdata.endpoint.as_i64().unwrap(),
    ))
}
