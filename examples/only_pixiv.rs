//! Similar to the simple example, except now we only want to get results from Pixiv

use rustnao::{HandlerBuilder, Source};

#[tokio::main]
async fn main() {
    let api_key = "your_api_key";
    let file = "https://i.imgur.com/W42kkKS.jpg";

    let handle = HandlerBuilder::default()
        .api_key(api_key)
        .db_mask(vec![Source::Pixiv])
        .build();
        
    match handle.get_sauce(file, None, None).await {
        Ok(result) => {
            for i in result {
                println!("{:?}", i);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}