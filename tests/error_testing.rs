use rustnao::{Handler, HandlerBuilder, Source};

const FILE: &str = "https://i.imgur.com/W42kkKS.jpg";
const INVALID_URL: &str = "https://j.jmgur.com";
const INVALID_FILE: &str = "./fake_file.png";

/// Creates a handler for testing purposes
fn create_handler(
    db_mask: Vec<Source>, db_mask_i: Vec<Source>, db_option: Option<u32>, numres: u32,
) -> Handler {
    let mut api_key = "".to_string();

    let data = std::fs::read_to_string("config.json");
    if data.is_ok() {
        if let Ok(val) = data {
            let json: serde_json::Value =
                serde_json::from_str(val.as_str()).expect("JSON not well formatted.");
            let json_api_key = json["api_key"].as_str();

            if let Some(key) = json_api_key {
                api_key = key.to_string();
            }
        }
    }

    let mut builder = HandlerBuilder::default();
    builder.db_mask(db_mask)
        .db_mask_i(db_mask_i)
        .num_results(numres)
        .api_key(api_key.as_str());

    if let Some(db) = db_option {
        builder.db(db);
    }

    builder.build()
}

/// Tests an invalid URL
#[tokio::test]
async fn test_invalid_url() {
    let handler = create_handler(vec![], vec![], Some(999), 999);
    let result = handler.get_sauce(INVALID_URL, None, None).await;
    assert!(result.is_err());
}

/// Tests an invalid URL
#[tokio::test]
async fn test_invalid_url_json() {
    let handler = create_handler(vec![], vec![], None, 999);
    let result = handler.get_sauce_as_json(INVALID_URL, None, None).await;
    assert!(result.is_err());
}

/// Tests an invalid file
#[tokio::test]
async fn test_invalid_file() {
    let handler = create_handler(vec![], vec![], Some(999), 999);
    let result = handler.get_sauce(INVALID_FILE, None, None).await;
    assert!(result.is_err());
}

/// Tests an invalid file
#[tokio::test]
async fn test_invalid_file_json() {
    let handler = create_handler(vec![], vec![], Some(999), 999);
    let result = handler.get_sauce(INVALID_FILE, None, None).await;
    assert!(result.is_err());
}

/// Tests an invalid number of results
#[tokio::test]
async fn test_invalid_num_results() {
    let handle = create_handler(vec![], vec![], Some(999), 2);
    let vec = handle.get_sauce(FILE, Some(1000), None).await;
    assert!(vec.is_err());
}

/// Tests an invalid minimum similarity option (upper)
#[tokio::test]
async fn test_invalid_min_similarity_upper() {
    let handle = create_handler(vec![], vec![], Some(999), 2);
    let vec_two = handle.get_sauce(FILE, None, Some(100.1)).await;
    assert!(vec_two.is_err());
}

/// Tests an invalid minimum similarity option (lower)
#[tokio::test]
async fn test_invalid_min_similarity_lower() {
    let handle = create_handler(vec![], vec![], Some(999), 2);
    let vec_two = handle.get_sauce(FILE, None, Some(-0.1)).await;
    assert!(vec_two.is_err());
}