//! A simple example using ToJSON trait

use rustnao::{HandlerBuilder, ToJSON};

#[tokio::main]
async fn main() {
    let api_key = "your_api_key";
    let file = "https://s5.mangadex.org/data/f2cf04ff9fbd5374c20d1cd5a555efed/x2.png";

    let handle = HandlerBuilder::default().api_key(api_key).build();
    handle.set_empty_filter(true);
    
    match handle.get_sauce(file, None, None).await {
        Ok(result) => {
            println!("{}", result.to_json_pretty().unwrap());
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
