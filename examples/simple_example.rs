use rustnao::HandlerBuilder;

#[tokio::main]
async fn main() {
    let api_key = "your_api_key";
    let file = "https://i.imgur.com/W42kkKS.jpg";

    let handle = HandlerBuilder::default()
        .api_key(api_key)
        .num_results(10)
        .build();

    handle.set_min_similarity(45);

    match handle.get_sauce(file, None, None).await {
        Ok(result) => {
            for sauce in result {
                println!("{:?}", sauce);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
