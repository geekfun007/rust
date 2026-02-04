//! Code generator module for Hertz HTTP server

pub mod hertz;

pub use hertz::HertzGenerator;

use crate::parser::ParsedProject;
use anyhow::Result;
use std::path::Path;

/// Trait for code generators
pub trait Generator {
    /// Generate code from parsed project
    fn generate(&self, project: &ParsedProject, output_dir: &Path) -> Result<()>;
}
