//! CLI for thrift-proto-hertz-gen
//!
//! Concurrent parser for .thrift and .proto files with Hertz HTTP server code generation

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use thrift_proto_hertz_gen::{
    fetcher::{Fetcher, FetcherConfig},
    generator::{Generator, HertzGenerator},
};

#[derive(Parser)]
#[command(name = "hertz-gen")]
#[command(author = "geekfun007")]
#[command(version = "0.1.0")]
#[command(about = "Generate Hertz HTTP server from Thrift/Proto IDL files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Hertz HTTP server code from IDL files
    Generate {
        /// Input paths (local directories, files, or git URLs)
        #[arg(short, long, required = true, num_args = 1..)]
        input: Vec<String>,

        /// Output directory for generated code
        #[arg(short, long, default_value = "./gen")]
        output: PathBuf,

        /// Go module name for the generated code
        #[arg(short, long, default_value = "github.com/example/hertz-server")]
        module: String,

        /// Additional include paths for resolving imports
        #[arg(short = 'I', long, num_args = 0..)]
        include: Vec<PathBuf>,

        /// Maximum number of concurrent file reads
        #[arg(long, default_value = "16")]
        concurrency: usize,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Parse IDL files and output AST as JSON
    Parse {
        /// Input paths (local directories, files, or git URLs)
        #[arg(short, long, required = true, num_args = 1..)]
        input: Vec<String>,

        /// Additional include paths for resolving imports
        #[arg(short = 'I', long, num_args = 0..)]
        include: Vec<PathBuf>,

        /// Output file for JSON AST (defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Fetch IDL files from git repository
    Fetch {
        /// Git repository URL
        #[arg(short, long)]
        url: String,

        /// Target directory for cloned repository
        #[arg(short, long)]
        target: PathBuf,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            input,
            output,
            module,
            include,
            concurrency,
            verbose,
        } => {
            setup_logging(verbose);
            generate(input, output, module, include, concurrency).await
        }
        Commands::Parse {
            input,
            include,
            output,
            verbose,
        } => {
            setup_logging(verbose);
            parse(input, include, output).await
        }
        Commands::Fetch {
            url,
            target,
            verbose,
        } => {
            setup_logging(verbose);
            fetch(url, target).await
        }
    }
}

fn setup_logging(verbose: bool) {
    let level = if verbose { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}

async fn generate(
    input: Vec<String>,
    output: PathBuf,
    module: String,
    include: Vec<PathBuf>,
    concurrency: usize,
) -> Result<()> {
    info!("Starting Hertz code generation");
    info!("Input: {:?}", input);
    info!("Output: {:?}", output);
    info!("Module: {}", module);

    // Configure fetcher
    let config = FetcherConfig {
        max_concurrent_reads: concurrency,
        include_paths: include,
        allow_git_clone: true,
        temp_dir: std::env::temp_dir().join("thrift-proto-hertz-gen"),
    };

    let fetcher = Fetcher::with_config(config);

    // Fetch and parse all files
    info!("Fetching and parsing IDL files...");
    let project = fetcher
        .fetch_all(input)
        .await
        .context("Failed to fetch and parse IDL files")?;

    info!(
        "Parsed {} files with {} services and {} structs",
        project.files.len(),
        project.all_services().len(),
        project.all_structs().len()
    );

    // Generate code
    info!("Generating Hertz HTTP server code...");
    let generator = HertzGenerator::new(&module);
    generator
        .generate(&project, &output)
        .context("Failed to generate code")?;

    info!("Code generation completed successfully!");
    info!("Output directory: {:?}", output);

    Ok(())
}

async fn parse(
    input: Vec<String>,
    include: Vec<PathBuf>,
    output: Option<PathBuf>,
) -> Result<()> {
    info!("Parsing IDL files...");

    // Configure fetcher
    let config = FetcherConfig {
        include_paths: include,
        ..Default::default()
    };

    let fetcher = Fetcher::with_config(config);

    // Fetch and parse all files
    let project = fetcher
        .fetch_all(input)
        .await
        .context("Failed to fetch and parse IDL files")?;

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&project)
        .context("Failed to serialize parsed project to JSON")?;

    // Output
    if let Some(output_path) = output {
        std::fs::write(&output_path, &json)
            .context("Failed to write JSON output")?;
        info!("Wrote parsed AST to {:?}", output_path);
    } else {
        println!("{}", json);
    }

    Ok(())
}

async fn fetch(url: String, target: PathBuf) -> Result<()> {
    info!("Fetching from: {}", url);
    info!("Target: {:?}", target);

    let fetcher = Fetcher::new();
    
    // Use blocking task for git operations
    let url_clone = url.clone();
    let target_clone = target.clone();
    tokio::task::spawn_blocking(move || {
        fetcher.git_clone_or_pull(&url_clone, &target_clone)
    })
    .await?
    .context("Git operation failed")?;

    info!("Successfully fetched repository to {:?}", target);

    Ok(())
}
