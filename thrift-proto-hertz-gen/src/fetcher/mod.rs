//! File fetcher module with git support and concurrent include resolution

use anyhow::{Context, Result};
use async_recursion::async_recursion;
use dashmap::DashMap;
use git2::{FetchOptions, RemoteCallbacks, Repository};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Semaphore;
use tracing::{debug, info, warn};
use url::Url;

use crate::parser::{self, ParsedFile, ParsedProject};

/// Configuration for the fetcher
#[derive(Debug, Clone)]
pub struct FetcherConfig {
    /// Maximum concurrent file reads
    pub max_concurrent_reads: usize,
    /// Include search paths
    pub include_paths: Vec<PathBuf>,
    /// Whether to clone git repos if needed
    pub allow_git_clone: bool,
    /// Temp directory for git clones
    pub temp_dir: PathBuf,
}

impl Default for FetcherConfig {
    fn default() -> Self {
        Self {
            max_concurrent_reads: 16,
            include_paths: vec![],
            allow_git_clone: true,
            temp_dir: std::env::temp_dir().join("thrift-proto-hertz-gen"),
        }
    }
}

/// Concurrent file fetcher
pub struct Fetcher {
    config: FetcherConfig,
    /// Cache of already parsed files
    parsed_cache: Arc<DashMap<PathBuf, ParsedFile>>,
    /// Semaphore for controlling concurrent reads
    semaphore: Arc<Semaphore>,
}

impl Fetcher {
    /// Create a new fetcher with default configuration
    pub fn new() -> Self {
        Self::with_config(FetcherConfig::default())
    }

    /// Create a new fetcher with custom configuration
    pub fn with_config(config: FetcherConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent_reads));
        Self {
            config,
            parsed_cache: Arc::new(DashMap::new()),
            semaphore,
        }
    }

    /// Clone or update a git repository
    pub fn git_clone_or_pull(&self, url: &str, target_path: &Path) -> Result<()> {
        info!("Git operation for: {}", url);
        
        if target_path.exists() {
            // Try to pull
            let repo = Repository::open(target_path)
                .context("Failed to open existing repository")?;
            
            let mut remote = repo.find_remote("origin")
                .context("Failed to find remote 'origin'")?;
            
            let mut callbacks = RemoteCallbacks::new();
            callbacks.transfer_progress(|progress| {
                debug!(
                    "Transfer: {}/{} objects",
                    progress.received_objects(),
                    progress.total_objects()
                );
                true
            });
            
            let mut fetch_opts = FetchOptions::new();
            fetch_opts.remote_callbacks(callbacks);
            
            remote.fetch(&["main", "master"], Some(&mut fetch_opts), None)
                .or_else(|_| remote.fetch(&["HEAD"], Some(&mut FetchOptions::new()), None))
                .context("Failed to fetch from remote")?;
            
            info!("Updated repository at {:?}", target_path);
        } else {
            // Clone
            std::fs::create_dir_all(target_path)?;
            
            let mut callbacks = RemoteCallbacks::new();
            callbacks.transfer_progress(|progress| {
                debug!(
                    "Cloning: {}/{} objects",
                    progress.received_objects(),
                    progress.total_objects()
                );
                true
            });
            
            let mut fetch_opts = FetchOptions::new();
            fetch_opts.remote_callbacks(callbacks);
            
            let mut builder = git2::build::RepoBuilder::new();
            builder.fetch_options(fetch_opts);
            
            builder.clone(url, target_path)
                .context("Failed to clone repository")?;
            
            info!("Cloned repository to {:?}", target_path);
        }
        
        Ok(())
    }

    /// Resolve an include path relative to the current file
    fn resolve_include_path(&self, include: &str, current_file: &Path) -> Option<PathBuf> {
        let current_dir = current_file.parent().unwrap_or(Path::new("."));
        
        // First try relative to current file
        let relative_path = current_dir.join(include);
        if relative_path.exists() {
            return Some(relative_path.canonicalize().unwrap_or(relative_path));
        }
        
        // Then try include paths
        for include_path in &self.config.include_paths {
            let candidate = include_path.join(include);
            if candidate.exists() {
                return Some(candidate.canonicalize().unwrap_or(candidate));
            }
        }
        
        // Try as absolute path
        let abs_path = PathBuf::from(include);
        if abs_path.exists() {
            return Some(abs_path.canonicalize().unwrap_or(abs_path));
        }
        
        None
    }

    /// Read and parse a single file
    async fn read_and_parse(&self, path: &Path) -> Result<ParsedFile> {
        let _permit = self.semaphore.acquire().await?;
        
        debug!("Reading file: {:?}", path);
        
        let content = fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read file: {:?}", path))?;
        
        let parsed = parser::parse_file(&content, path)
            .map_err(|e| anyhow::anyhow!("Parse error in {:?}: {}", path, e))?;
        
        Ok(parsed)
    }

    /// Recursively fetch and parse a file and all its includes
    #[async_recursion]
    pub async fn fetch_recursive(&self, path: PathBuf) -> Result<()> {
        // Check cache
        if self.parsed_cache.contains_key(&path) {
            return Ok(());
        }

        // Parse the file
        let parsed = self.read_and_parse(&path).await?;
        let includes = parsed.includes.clone();
        
        // Cache the result
        self.parsed_cache.insert(path.clone(), parsed);
        
        // Process includes concurrently
        let mut tasks = Vec::new();
        for include in includes {
            if let Some(resolved_path) = self.resolve_include_path(&include.path, &path) {
                if !self.parsed_cache.contains_key(&resolved_path) {
                    let fetcher = self.clone_inner();
                    tasks.push(tokio::spawn(async move {
                        fetcher.fetch_recursive(resolved_path).await
                    }));
                }
            } else {
                warn!("Could not resolve include: {} from {:?}", include.path, path);
            }
        }
        
        // Wait for all includes to be processed
        for task in tasks {
            task.await??;
        }
        
        Ok(())
    }

    /// Clone internal state for async operations
    fn clone_inner(&self) -> Self {
        Self {
            config: self.config.clone(),
            parsed_cache: Arc::clone(&self.parsed_cache),
            semaphore: Arc::clone(&self.semaphore),
        }
    }

    /// Fetch from a git URL or local path
    pub async fn fetch_entry(&self, entry: &str) -> Result<Vec<PathBuf>> {
        let mut entry_files = Vec::new();
        
        // Check if it's a git URL
        if entry.starts_with("http://") || entry.starts_with("https://") || entry.starts_with("git@") {
            if self.config.allow_git_clone {
                // Parse the URL to get repo name
                let repo_name = if entry.starts_with("git@") {
                    entry.split(':').last()
                        .and_then(|s| s.strip_suffix(".git"))
                        .unwrap_or("repo")
                        .replace('/', "_")
                } else {
                    let parsed_url = Url::parse(entry).ok();
                    let segments: Option<Vec<_>> = parsed_url
                        .as_ref()
                        .and_then(|u| u.path_segments())
                        .map(|s| s.collect());
                    segments
                        .and_then(|s| s.last().cloned())
                        .map(|s| s.trim_end_matches(".git").to_string())
                        .unwrap_or_else(|| "repo".to_string())
                };
                
                let target_path = self.config.temp_dir.join(&repo_name);
                
                // Clone or pull in a blocking task
                let url = entry.to_string();
                let target = target_path.clone();
                let fetcher = self.clone_inner();
                tokio::task::spawn_blocking(move || {
                    fetcher.git_clone_or_pull(&url, &target)
                }).await??;
                
                // Find all .thrift and .proto files
                entry_files = find_idl_files(&target_path).await?;
            }
        } else {
            // Local path
            let path = PathBuf::from(entry);
            if path.is_dir() {
                entry_files = find_idl_files(&path).await?;
            } else if path.is_file() {
                entry_files.push(path);
            }
        }
        
        Ok(entry_files)
    }

    /// Fetch all entry points and their includes concurrently
    pub async fn fetch_all(&self, entries: Vec<String>) -> Result<ParsedProject> {
        let mut all_files = Vec::new();
        
        // Fetch entry files
        for entry in &entries {
            let files = self.fetch_entry(entry).await?;
            all_files.extend(files);
        }
        
        info!("Found {} entry files to process", all_files.len());
        
        // Process all files recursively and concurrently
        let mut tasks = Vec::new();
        for file in all_files {
            let fetcher = self.clone_inner();
            tasks.push(tokio::spawn(async move {
                fetcher.fetch_recursive(file).await
            }));
        }
        
        // Wait for all tasks
        for task in tasks {
            task.await??;
        }
        
        // Build the parsed project
        let root_path = entries.first()
            .map(|e| PathBuf::from(e))
            .unwrap_or_else(|| PathBuf::from("."));
        
        let mut project = ParsedProject::new(root_path);
        for entry in self.parsed_cache.iter() {
            project.files.push(entry.value().clone());
        }
        
        info!("Successfully parsed {} files", project.files.len());
        
        Ok(project)
    }
}

/// Find all .thrift and .proto files in a directory
async fn find_idl_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    
    while let Some(current) = stack.pop() {
        let mut entries = fs::read_dir(&current).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_type = entry.file_type().await?;
            
            if file_type.is_dir() {
                stack.push(path);
            } else if file_type.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "thrift" || ext == "proto" {
                        files.push(path);
                    }
                }
            }
        }
    }
    
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_fetcher_config() {
        let config = FetcherConfig::default();
        assert_eq!(config.max_concurrent_reads, 16);
    }
}
