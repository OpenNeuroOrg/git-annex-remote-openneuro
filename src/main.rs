#![deny(warnings)]
#![warn(rust_2018_idioms)]

mod auth;

#[tokio::main]
async fn main() {
    let dataset_id = "ds001150";
    if let Err(err) = auth::prepare_repo_access(dataset_id).await {
        println!("Error: {}", err);
    }
}
