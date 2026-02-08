//! Handler module of rustnao.  The handler for the SauceNAO API calls.

mod error;
pub use error::Error;
/// A specialized Result type for rustnao.
pub type Result<T> = error::Result<T>;

mod constants;
pub use constants::Source;

mod sauce;
pub use sauce::Sauce;

mod deserialize;
use deserialize::SauceResult;

use reqwest::Client;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use url::Url;

/// A builder to create a Handler for RustNAO usage.
/// ## Example
/// ```
/// use rustnao::{HandlerBuilder, Source};
/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db_mask(vec![Source::Pixiv]).build();
/// ```
#[derive(Default, Debug, Clone)]
pub struct HandlerBuilder {
    api_key: Option<String>,
    testmode: Option<bool>,
    db_mask: Option<Vec<Source>>,
    db_mask_i: Option<Vec<Source>>,
    db: Option<u32>,
    num_results: Option<u32>,
    min_similarity: Option<f64>,
    empty_filter_enabled: Option<bool>,
}

impl HandlerBuilder {
    /// Sets the API key used for searches for the Handler.  If this is not set then a blank API key is used, instead of your personal one.
    pub fn api_key(&mut self, api_key: &str) -> &mut HandlerBuilder {
        self.api_key = Some(api_key.to_string());
        self
    }

    /// Sets whether testmode should be enabled on searches for the Handler.
    pub fn testmode(&mut self, testmode: bool) -> &mut HandlerBuilder {
        self.testmode = Some(testmode);
        self
    }

    /// Sets which database indices you want included on search for the Handler.
    pub fn db_mask(&mut self, db_mask: Vec<Source>) -> &mut HandlerBuilder {
        self.db_mask = Some(db_mask);
        self
    }

    /// Sets which database indices you want excluded on search for the Handler.
    pub fn db_mask_i(&mut self, db_mask_i: Vec<Source>) -> &mut HandlerBuilder {
        self.db_mask_i = Some(db_mask_i);
        self
    }

    /// Sets a database index to be searched for the Handler.
    pub fn db(&mut self, db: u32) -> &mut HandlerBuilder {
        self.db = Some(db);
        self
    }

    /// Sets the maximum number of results you want returned on search for the Handler.
    pub fn num_results(&mut self, num_results: u32) -> &mut HandlerBuilder {
        self.num_results = Some(num_results);
        self
    }

    /// Sets the minimum similarity for results by default for the Handler.
    pub fn min_similarity<T: Into<f64>>(&mut self, min_similarity: T) -> &mut HandlerBuilder {
        self.min_similarity = Some(min_similarity.into());
        self
    }

    /// Sets whether to enable an empty filter by default for the Handler.
    pub fn empty_filter_enabled(&mut self, empty_filter_enabled: bool) -> &mut HandlerBuilder {
        self.empty_filter_enabled = Some(empty_filter_enabled);
        self
    }

    /// Builds the HandlerBuilder, returning a Handler that can be used to search.
    pub fn build(&self) -> Handler {
        let api_key = self.api_key.as_deref().unwrap_or("");
        let testmode = self.testmode.map(|x| if x { 1 } else { 0 });

        let result = Handler::new(
            api_key,
            testmode,
            self.db_mask.clone(),
            self.db_mask_i.clone(),
            self.db,
            self.num_results,
        );
        if let Some(x) = self.min_similarity {
            result.set_min_similarity(x);
        }
        if let Some(x) = self.empty_filter_enabled {
            result.set_empty_filter(x);
        }
        result
    }
}

/// A handler struct to make SauceNAO API calls with.
#[derive(Debug)]
pub struct Handler {
    api_key: String,
    output_type: i32,
    testmode: Option<u32>,
    db_mask: Option<Vec<Source>>,
    db_mask_i: Option<Vec<Source>>,
    db: Option<u32>,
    num_results: Option<u32>,
    short_limit: AtomicU32,
    long_limit: AtomicU32,
    short_left: AtomicU32,
    long_left: AtomicU32,
    min_similarity: Mutex<f64>,
    empty_filter_enabled: AtomicBool,
    client: Client,
}

impl Handler {
    /// Generates a bitmask from a given vector of Sources.
    fn generate_bitmask(&self, mask: Vec<Source>) -> u64 {
        let mut res: u64 = 0;
        for m in mask {
            let index = m as u32;
            res |= 1 << index;
        }
        res
    }

    /// Generates a url from the given image path/url
    fn generate_url(&self, image_path: &str, num_results: Option<u32>) -> Result<String> {
        let mut request_url = Url::parse(constants::API_URL)?;
        request_url
            .query_pairs_mut()
            .append_pair("api_key", self.api_key.as_str());
        request_url
            .query_pairs_mut()
            .append_pair("output_type", self.output_type.to_string().as_str());

        if let Some(val) = self.db {
            request_url
                .query_pairs_mut()
                .append_pair("db", val.to_string().as_str());
        }

        if let Some(val) = &self.db_mask {
            if !val.is_empty() {
                request_url.query_pairs_mut().append_pair(
                    "dbmask",
                    self.generate_bitmask(val.clone()).to_string().as_str(),
                );
            }
        }
        if let Some(val) = &self.db_mask_i {
            if !val.is_empty() {
                request_url.query_pairs_mut().append_pair(
                    "dbmaski",
                    self.generate_bitmask(val.clone()).to_string().as_str(),
                );
            }
        }

        request_url.query_pairs_mut().append_pair(
            "testmode",
            self.testmode.unwrap_or(0).to_string().as_str(),
        );

        let res_count = num_results.or(self.num_results).unwrap_or(999);
        request_url
            .query_pairs_mut()
            .append_pair("numres", res_count.to_string().as_str());

        if image_path.starts_with("https://") || image_path.starts_with("http://") {
            request_url.query_pairs_mut().append_pair("url", image_path);
        }

        Ok(request_url.to_string())
    }

    fn new(
        api_key: &str, testmode: Option<u32>, db_mask: Option<Vec<Source>>,
        db_mask_i: Option<Vec<Source>>, db: Option<u32>, num_results: Option<u32>,
    ) -> Handler {
        Handler {
            api_key: api_key.to_string(),
            output_type: 2,
            testmode,
            db_mask,
            db_mask_i,
            db,
            num_results,
            short_limit: AtomicU32::new(12),
            long_limit: AtomicU32::new(200),
            short_left: AtomicU32::new(12),
            long_left: AtomicU32::new(200),
            min_similarity: Mutex::new(0.0),
            empty_filter_enabled: AtomicBool::new(false),
            client: Client::new(),
        }
    }

    /// Sets the minimum similarity threshold.
    pub fn set_min_similarity<T: Into<f64>>(&self, min_similarity: T) {
        if let Ok(mut lock) = self.min_similarity.lock() {
            *lock = min_similarity.into();
        }
    }

    /// Sets whether empty URL results should be filtered.
    pub fn set_empty_filter(&self, enabled: bool) {
        self.empty_filter_enabled.store(enabled, Ordering::SeqCst);
    }

    /// Gets the remaining short limit.
    pub fn get_current_short_limit(&self) -> u32 {
        self.short_left.load(Ordering::SeqCst)
    }

    /// Gets the remaining long limit.
    pub fn get_current_long_limit(&self) -> u32 {
        self.long_left.load(Ordering::SeqCst)
    }

    fn process_results(
        &self, returned_sauce: SauceResult, min_similarity: Option<f64>,
    ) -> Result<Vec<Sauce>> {
        let mut ret_sauce: Vec<Sauce> = Vec::new();

        if returned_sauce.header.status >= 0 {
            self.short_left
                .store(returned_sauce.header.short_remaining, Ordering::SeqCst);
            self.long_left
                .store(returned_sauce.header.long_remaining, Ordering::SeqCst);
            self.short_limit
                .store(returned_sauce.header.short_limit.parse()?, Ordering::SeqCst);
            self.long_limit
                .store(returned_sauce.header.long_limit.parse()?, Ordering::SeqCst);

            if let Some(results) = returned_sauce.results {
                let actual_min_sim =
                    min_similarity.unwrap_or_else(|| *self.min_similarity.lock().unwrap());
                
                for res in results {
                    let sauce_min_sim: f64 = res.header.similarity.parse()?;
                    if sauce_min_sim >= actual_min_sim
                        && (!self.empty_filter_enabled.load(Ordering::SeqCst)
                            || !res.data.ext_urls.is_empty())
                    {
                        let source = Source::from_u32(res.header.index_id);
                        let site_name = source.map(|s| s.name()).unwrap_or(&res.header.index_name).to_string();

                        ret_sauce.push(sauce::new_sauce(
                            res.data.ext_urls,
                            res.data.title,
                            site_name,
                            res.header.index_id,
                            res.header.index_id, // SauceNAO returns index_id
                            sauce_min_sim as f32,
                            res.header.thumbnail,
                            match serde_json::to_value(&res.data.additional_fields) {
                                Ok(x) => Some(x),
                                Err(_) => None,
                            },
                            res.data.source,
                            res.data.creator,
                            res.data.eng_name,
                            res.data.jp_name,
                        ));
                    }
                }
            }
            Ok(ret_sauce)
        } else {
            Err(Error::InvalidCode {
                code: returned_sauce.header.status,
                message: returned_sauce.header.message,
            })
        }
    }

    /// Asynchronously returns a Result of either a vector of Sauce objects.
    pub async fn get_sauce(
        &self, image_path: &str, num_results: Option<u32>, min_similarity: Option<f64>,
    ) -> Result<Vec<Sauce>> {
        let url_string = self.generate_url(image_path, num_results)?;

        let response = if !(image_path.starts_with("https://") || image_path.starts_with("http://")) {
            let data = std::fs::read(image_path)?;
            let file_name = std::path::Path::new(image_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("image.jpg")
                .to_string();
            let part = reqwest::multipart::Part::bytes(data).file_name(file_name);
            let form = reqwest::multipart::Form::new().part("file", part);
            self.client.post(&url_string).multipart(form).send().await?
        } else {
            self.client.post(&url_string).send().await?
        };

        let returned_sauce: SauceResult = response.json().await?;
        self.process_results(returned_sauce, min_similarity)
    }

    /// Returns a string representing a vector of Sauce objects as a serialized JSON.
    pub async fn get_sauce_as_json(
        &self, image_path: &str, num_results: Option<u32>, min_similarity: Option<f64>,
    ) -> Result<String> {
        let ret_sauce = self
            .get_sauce(image_path, num_results, min_similarity)
            .await?;
        Ok(serde_json::to_string(&ret_sauce)?)
    }

    /// Returns a string representing a vector of Sauce objects as a serialized pretty JSON.
    pub async fn get_sauce_as_pretty_json(
        &self, image_path: &str, num_results: Option<u32>, min_similarity: Option<f64>,
    ) -> Result<String> {
        let ret_sauce = self
            .get_sauce(image_path, num_results, min_similarity)
            .await?;
        Ok(serde_json::to_string_pretty(&ret_sauce)?)
    }
}

/// A trait to convert to JSON and pretty JSON strings.
pub trait ToJSON {
    /// Converts to a Result containing a JSON string.
    fn to_json(&self) -> Result<String>;
    /// Converts to a Result containing a pretty JSON string.
    fn to_json_pretty(&self) -> Result<String>;
}

impl ToJSON for Vec<Sauce> {
    fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}