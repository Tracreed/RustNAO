use rustnao::{Handler, HandlerBuilder, Sauce, Source};

const FILE: &str = "https://i.imgur.com/W42kkKS.jpg";
const LOCAL_FILE: &str = "./tests/test.jpg";

/// Creates a handler for testing purposes
fn create_handler(
    db_mask: Vec<Source>, db_mask_i: Vec<Source>, db_option: Option<u32>, numres: u32,
) -> Handler {
    let mut api_key = "".to_string();

    let data = std::fs::read_to_string("config.json");
    if let Ok(val) = data {
        let json: serde_json::Value =
            serde_json::from_str(val.as_str()).expect("JSON not well formatted.");
        let json_api_key = json["api_key"].as_str();

        if let Some(key) = json_api_key {
            api_key = key.to_string();
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

/// Tests handler creation
#[test]
fn test_check_handler_creation() {
    create_handler(vec![], vec![], Some(999), 999);
}

/// Tests short and long limit checks (which should change after a search)
#[tokio::test]
async fn test_get_short_and_long_limits() {
    let handle = create_handler(vec![], vec![], Some(999), 999);
    let cur_short_before = handle.get_current_short_limit();
    let cur_long_before = handle.get_current_long_limit();
    let result = handle.get_sauce(FILE, None, None).await;
    if result.is_ok() {
        assert!(cur_short_before > handle.get_current_short_limit());
        assert!(cur_long_before > handle.get_current_long_limit());
    }
}

/// Tests searching for filtering empty sourced URLs
#[tokio::test]
async fn test_filter_empty_sauce() {
    let handle = create_handler(vec![], vec![], Some(999), 999);
    let vec: rustnao::Result<Vec<Sauce>> = handle.get_sauce(FILE, None, None).await;
    if let Ok(vec_unwrap) = vec {
        let only_empty: Vec<Sauce> = vec_unwrap
            .into_iter()
            .filter(|sauce| !sauce.has_empty_url())
            .collect();
        for o in only_empty {
            assert!(!o.ext_urls.is_empty());
        }
    }
}

/// Tests local searching (local file)
#[tokio::test]
async fn test_local() {
    let handle = create_handler(vec![], vec![], Some(999), 2);
    let vec = handle.get_sauce_as_json(LOCAL_FILE, None, None).await;
    match vec {
        Ok(result) => println!("Passed, {}", result),
        Err(result) => println!("out, {}", result),
    }
}

/// Tests setting a max number of results
#[tokio::test]
async fn test_limiting() {
    let handle = create_handler(vec![], vec![], Some(999), 2);
    let vec = handle.get_sauce(FILE, None, None).await;
    if let Ok(vec_unwrap) = vec {
        assert!(vec_unwrap.len() <= 2);
    }
}

/// Tests db bit masks
#[tokio::test]
async fn test_db_bit_mask() {
    let handle = create_handler(vec![Source::SankakuChannel], vec![], None, 999);
    let vec = handle.get_sauce(FILE, None, None).await;
    if let Ok(vec_unwrap) = vec {
        for v in vec_unwrap {
            assert!(v.index >= 27, "saw {}", v.index);
        }
    }
}

/// Tests db bit mask for exclusion
#[tokio::test]
async fn test_db_bit_mask_i() {
    let handle = create_handler(
        vec![],
        vec![
            Source::HMagazines,
            Source::HGameCG,
            Source::DoujinshiDB,
            Source::Pixiv,
            Source::NicoNicoSeiga,
            Source::Danbooru,
            Source::Drawr,
            Source::Nijie,
            Source::Yandere,
            Source::Shutterstock,
            Source::Fakku,
        ],
        None,
        999,
    );
    let vec = handle.get_sauce(FILE, None, None).await;
    if let Ok(vec_unwrap) = vec {
        for v in vec_unwrap {
            assert!(v.index >= 11);
        }
    }
}

/// Tests min similarity and capping the number of results
#[tokio::test]
async fn test_min_similarity_and_num_results() {
    let handle = create_handler(vec![], vec![], Some(999), 2);
    let vec = handle.get_sauce(LOCAL_FILE, Some(5), Some(50_f64)).await;
    if let Ok(vec_unwrap) = vec {
        let res = vec_unwrap;
        assert!(res.len() <= 5);
        for v in res {
            assert!(v.similarity >= 49.0);
        }
    }
}
