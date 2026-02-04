//! Thrift and Proto parser with Hertz HTTP server code generator
//!
//! This library provides:
//! - Concurrent parsing of .thrift and .proto files
//! - Git repository cloning/pulling for fetching IDL files
//! - Recursive include/import resolution
//! - Code generation for CloudWeGo Hertz HTTP server framework
//!
//! # Example
//!
//! ```rust,no_run
//! use thrift_proto_hertz_gen::{fetcher::Fetcher, generator::{HertzGenerator, Generator}};
//! use std::path::Path;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let fetcher = Fetcher::new();
//!     let project = fetcher.fetch_all(vec!["./idl".to_string()]).await?;
//!     
//!     let generator = HertzGenerator::new("github.com/example/myapp");
//!     generator.generate(&project, Path::new("./output"))?;
//!     
//!     Ok(())
//! }
//! ```

pub mod fetcher;
pub mod generator;
pub mod parser;

pub use fetcher::{Fetcher, FetcherConfig};
pub use generator::{Generator, HertzGenerator};
pub use parser::{parse_file, ParsedFile, ParsedProject};
