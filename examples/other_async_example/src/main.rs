use rustnao::{Handler, HandlerBuilder, Source};

fn main() {
    let handle = HandlerBuilder::default()
        .api_key("key")
        .num_results(999)
        .db_mask(vec![Source::Pixiv, Source::SankakuChannel])
        .build();

    let file = "https://i.imgur.com/W42kkKS.jpg";

    // Example of running the async code in a synchronous main using tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let result = handle.get_sauce_as_pretty_json(file, None, None).await;
        println!("Result... {}", result.unwrap());
    });
}
