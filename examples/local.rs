//! A simple example with a local file

use rustnao::HandlerBuilder;

#[tokio::main]
async fn main() {
    let api_key = "your_api_key";
    let file = "./tests/test.jpg";

    let handle = HandlerBuilder::default().api_key(api_key).build();
    match handle.get_sauce(file, None, None).await {
        Ok(result) => {
            for i in result {
                println!("{:?}", i);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
