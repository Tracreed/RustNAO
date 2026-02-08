//! Similar to the simple example, except now it's JSON

use rustnao::{HandlerBuilder, Source};

#[tokio::main]
async fn main() {
    let api_key = "your_api_key";
    let file = "https://i.imgur.com/W42kkKS.jpg";

    // Specifying our key, only want to see Pixiv and Sankaku using a mask, and 999 results at most
    let handle = HandlerBuilder::default()
        .api_key(api_key)
        .num_results(999)
        .db_mask(vec![Source::Pixiv, Source::SankakuChannel])
        .build();
        
    match handle.get_sauce_as_pretty_json(file, None, None).await {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}