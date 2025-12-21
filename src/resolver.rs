use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::fs;

use crate::fetcher::Fetcher;
use crate::parser::IdlParser;

/// Represents a fetched IDL entry with its content and metadata
#[derive(Debug, Clone)]
pub struct IdlEntry {
    pub source: String,
    pub content: String,
    pub includes: Vec<String>,
    pub local_path: PathBuf,
}

/// Recursively fetches IDL entries and their includes
pub struct IdlResolver {
    fetcher: Fetcher,
    parser: IdlParser,
    output_dir: PathBuf,
}

impl IdlResolver {
    pub fn new(output_dir: PathBuf) -> Result<Self> {
        Ok(Self {
            fetcher: Fetcher::new()?,
            parser: IdlParser::new(),
            output_dir,
        })
    }

    /// Fetch an IDL entry and all its includes recursively
    pub fn fetch_recursive(&self, entry_source: &str) -> Result<Vec<IdlEntry>> {
        let mut all_entries = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(entry_source.to_string());
        
        while let Some(current_source) = queue.pop_front() {
            // Skip if already visited
            if visited.contains(&current_source) {
                continue;
            }
            
            visited.insert(current_source.clone());
            
            println!("\nProcessing: {}", current_source);
            
            // Fetch the content
            let content = self.fetcher.fetch(&current_source)
                .context(format!("Failed to fetch {}", current_source))?;
            
            // Extract includes
            let includes = self.parser.extract_includes(&content);
            println!("Found {} include(s)", includes.len());
            
            // Resolve include paths relative to current source
            let resolved_includes: Vec<String> = includes.iter()
                .map(|inc| self.fetcher.resolve_path(&current_source, inc))
                .collect();
            
            // Save to local file
            let local_path = self.save_to_local(&current_source, &content)?;
            
            // Create entry
            let entry = IdlEntry {
                source: current_source.clone(),
                content,
                includes: resolved_includes.clone(),
                local_path,
            };
            
            all_entries.push(entry);
            
            // Add includes to queue for processing
            for resolved_include in resolved_includes {
                if !visited.contains(&resolved_include) {
                    queue.push_back(resolved_include);
                }
            }
        }
        
        println!("\n✓ Successfully fetched {} IDL file(s)", all_entries.len());
        Ok(all_entries)
    }

    /// Save IDL content to local filesystem
    fn save_to_local(&self, source: &str, content: &str) -> Result<PathBuf> {
        // Generate a local file path based on the source
        let local_path = self.generate_local_path(source);
        
        // Create parent directories if needed
        if let Some(parent) = local_path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create directory: {:?}", parent))?;
        }
        
        // Write content to file
        fs::write(&local_path, content)
            .context(format!("Failed to write file: {:?}", local_path))?;
        
        println!("Saved to: {}", local_path.display());
        
        Ok(local_path)
    }

    /// Generate a local file path for a given source
    fn generate_local_path(&self, source: &str) -> PathBuf {
        // Try to extract a meaningful filename from the source
        let filename = if let Ok(url) = url::Url::parse(source) {
            // For URLs, use the path segments
            url.path_segments()
                .and_then(|mut segments| segments.next_back())
                .unwrap_or("downloaded.idl")
                .to_string()
        } else {
            // For file paths, use the filename
            Path::new(source)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("file.idl")
                .to_string()
        };
        
        self.output_dir.join(filename)
    }

    /// Create a dependency graph showing the include relationships
    pub fn create_dependency_graph(entries: &[IdlEntry]) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        
        for entry in entries {
            graph.insert(entry.source.clone(), entry.includes.clone());
        }
        
        graph
    }

    /// Print a summary of fetched entries
    pub fn print_summary(entries: &[IdlEntry]) {
        println!("\n{}", "=".repeat(60));
        println!("FETCH SUMMARY");
        println!("{}", "=".repeat(60));
        
        for (idx, entry) in entries.iter().enumerate() {
            println!("\n[{}] Source: {}", idx + 1, entry.source);
            println!("    Local:  {}", entry.local_path.display());
            println!("    Size:   {} bytes", entry.content.len());
            if !entry.includes.is_empty() {
                println!("    Includes:");
                for include in &entry.includes {
                    println!("      - {}", include);
                }
            }
        }
        
        println!("\n{}", "=".repeat(60));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_local_path() {
        let temp_dir = std::env::temp_dir();
        let resolver = IdlResolver::new(temp_dir).unwrap();
        
        let path = resolver.generate_local_path("https://example.com/api/base.idl");
        assert_eq!(path.file_name().unwrap().to_str().unwrap(), "base.idl");
        
        let path = resolver.generate_local_path("/some/path/to/file.idl");
        assert_eq!(path.file_name().unwrap().to_str().unwrap(), "file.idl");
    }
}
