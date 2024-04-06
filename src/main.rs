#![deny(warnings)]
#![warn(rust_2018_idioms)]

mod auth;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    println!("VERSION 1");

    let ds_key = auth::request_access();
    Ok(())
}

