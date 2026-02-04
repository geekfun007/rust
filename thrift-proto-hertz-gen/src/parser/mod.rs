//! Parser module for Thrift and Proto files

pub mod proto;
pub mod thrift;
pub mod types;

pub use proto::parse_proto_file;
pub use thrift::parse_thrift_file;
pub use types::*;

use std::path::Path;

/// Detect file type based on extension
pub fn detect_file_type(path: &Path) -> Option<FileType> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("thrift") => Some(FileType::Thrift),
        Some("proto") => Some(FileType::Proto),
        _ => None,
    }
}

/// Parse a file based on its type
pub fn parse_file(content: &str, path: &Path) -> Result<ParsedFile, String> {
    let file_type = detect_file_type(path)
        .ok_or_else(|| format!("Unknown file type for: {:?}", path))?;

    match file_type {
        FileType::Thrift => parse_thrift_file(content, path.to_path_buf()),
        FileType::Proto => parse_proto_file(content, path.to_path_buf()),
    }
}
