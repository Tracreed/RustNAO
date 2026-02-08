use rustnao::{Error, HandlerBuilder, Source};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handle = HandlerBuilder::default()
        .api_key("key")
        .num_results(999)
        .db_mask(vec![Source::Pixiv, Source::SankakuChannel])
        .build();

    let file = "https://i.imgur.com/W42kkKS.jpg";

    let result = handle.get_sauce_as_pretty_json(file, None, None).await?;
    println!("Result... {}", result);

    Ok(())
}
