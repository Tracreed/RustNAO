//! Example where errors are caught

use rustnao::{HandlerBuilder, Result};

async fn get_source(file: &str) -> Result<String> {
    let handle = HandlerBuilder::default()
        .api_key("")
        .db(999)
        .num_results(99)
        .build();
    handle.get_sauce_as_pretty_json(file, None, None).await
}

async fn get_source_string(file: &str) -> String {
    let result = get_source(file).await;
    match result {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}

#[tokio::main]
async fn main() {
    let file = "https://i.imgur.com/W42kkKS.jpg";
    let invalid_file = "https://j.jmgur.jpg";
    println!("{}", get_source_string(file).await);
    println!("{}", get_source_string(invalid_file).await);
}
