//! Now, we wish to filter out our results such that we have a minimum similarity and we reject any empty sources

use rustnao::{HandlerBuilder, Sauce};

#[tokio::main]
async fn main() {
    let api_key = "your_api_key";
    let file = "https://i.imgur.com/W42kkKS.jpg";

    let handle = HandlerBuilder::default().api_key(api_key).build();
    handle.set_min_similarity(61.31);
    
    match handle.get_sauce(file, None, None).await {
        Ok(result) => {
            let filtered: Vec<Sauce> = result
                .into_iter()
                .filter(|sauce| !sauce.has_empty_url())
                .collect(); // Remove empty results
            for i in filtered {
                println!("{:?}", i);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
