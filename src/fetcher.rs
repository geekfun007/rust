use anyhow::{Context, Result};
use std::path::Path;
use url::Url;

/// Handles fetching IDL files from local filesystem or remote URLs
pub struct Fetcher {
    client: reqwest::blocking::Client,
}

impl Fetcher {
    pub fn new() -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self { client })
    }

    /// Fetch content from a URL or local file path
    pub fn fetch(&self, source: &str) -> Result<String> {
        // Try to parse as URL first
        if let Ok(url) = Url::parse(source) {
            if url.scheme() == "http" || url.scheme() == "https" {
                return self.fetch_url(source);
            } else if url.scheme() == "file" {
                return self.fetch_file(url.path());
            }
        }
        
        // Otherwise treat as local file path
        self.fetch_file(source)
    }

    /// Fetch content from a URL
    fn fetch_url(&self, url: &str) -> Result<String> {
        println!("Fetching from URL: {}", url);
        let response = self.client
            .get(url)
            .send()
            .context(format!("Failed to fetch from URL: {}", url))?;
        
        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch {}: HTTP {}", url, response.status());
        }
        
        let content = response
            .text()
            .context(format!("Failed to read response from {}", url))?;
        
        Ok(content)
    }

    /// Fetch content from a local file
    fn fetch_file(&self, path: &str) -> Result<String> {
        println!("Reading from file: {}", path);
        std::fs::read_to_string(path)
            .context(format!("Failed to read file: {}", path))
    }

    /// Resolve a relative include path based on the parent source
    pub fn resolve_path(&self, parent_source: &str, include_path: &str) -> String {
        // If include_path is already absolute or a URL, return it as is
        if include_path.starts_with("http://") 
            || include_path.starts_with("https://")
            || include_path.starts_with("file://")
            || Path::new(include_path).is_absolute() {
            return include_path.to_string();
        }

        // Try to parse parent as URL
        if let Ok(parent_url) = Url::parse(parent_source) {
            if parent_url.scheme() == "http" || parent_url.scheme() == "https" {
                // Join relative path with parent URL
                if let Ok(resolved) = parent_url.join(include_path) {
                    return resolved.to_string();
                }
            }
        }

        // Handle as file path
        let parent_path = Path::new(parent_source);
        if let Some(parent_dir) = parent_path.parent() {
            let resolved = parent_dir.join(include_path);
            return resolved.to_string_lossy().to_string();
        }

        // Fallback to include path as is
        include_path.to_string()
    }
}

impl Default for Fetcher {
    fn default() -> Self {
        Self::new().expect("Failed to create default fetcher")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_path_relative() {
        let fetcher = Fetcher::new().unwrap();
        
        let resolved = fetcher.resolve_path("/base/dir/file.idl", "include/other.idl");
        assert_eq!(resolved, "/base/dir/include/other.idl");
    }

    #[test]
    fn test_resolve_path_url() {
        let fetcher = Fetcher::new().unwrap();
        
        let resolved = fetcher.resolve_path(
            "https://example.com/idl/base.idl",
            "include/other.idl"
        );
        assert_eq!(resolved, "https://example.com/idl/include/other.idl");
    }

    #[test]
    fn test_resolve_path_absolute() {
        let fetcher = Fetcher::new().unwrap();
        
        let resolved = fetcher.resolve_path("/base/dir/file.idl", "/absolute/path.idl");
        assert_eq!(resolved, "/absolute/path.idl");
    }
}
