//! # RustNAO
//!
//! A Rust implementation of a wrapper for the SauceNAO API.
//!
//! ## Installation
//! Add the following to your ``Cargo.toml`` file:
//! ```toml
//! [dependencies]
//! rustnao = "0.4.0"
//! ```
//!
//! ## Examples
//! Here's a simple example:
//! ```no_run
//! use rustnao::{Handler, HandlerBuilder, Sauce, Result};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = "your_api_key";
//!     let file = "https://i.imgur.com/W42kkKS.jpg";
//!
//!     // Specifying our key, only want to see Pixiv and Sankaku using a mask, and 15 results at most
//!     let handle = HandlerBuilder::default().api_key(api_key).db_mask(vec![Handler::PIXIV, Handler::SANKAKU_CHANNEL]).num_results(15).build();
//!
//!     // Set the minimum similarity to 45.
//!     handle.set_min_similarity(45);
//!
//!     // Returns a vector of Sauce objects if successful
//!     let result : Result<Vec<Sauce>> = handle.get_sauce(file, None, None).await;
//! }
//! ```

#![deny(missing_docs)]

mod handler;
pub use handler::{Error, Handler, HandlerBuilder, Result, Sauce, ToJSON};