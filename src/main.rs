mod parser;
mod fetcher;
mod resolver;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use resolver::IdlResolver;

#[derive(Parser, Debug)]
#[command(name = "idl-fetcher")]
#[command(about = "Fetch IDL entries and their includes recursively", long_about = None)]
struct Args {
    /// Source IDL file (URL or local path)
    #[arg(value_name = "SOURCE")]
    source: String,

    /// Output directory for downloaded files
    #[arg(short, long, value_name = "DIR", default_value = "./idl_output")]
    output: PathBuf,

    /// Show dependency graph
    #[arg(short, long)]
    graph: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("IDL Fetcher - Recursive Include Resolver");
        println!("Source: {}", args.source);
        println!("Output: {}\n", args.output.display());
    }

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output)?;

    // Create resolver and fetch recursively
    let resolver = IdlResolver::new(args.output)?;
    let entries = resolver.fetch_recursive(&args.source)?;

    // Print summary
    IdlResolver::print_summary(&entries);

    // Print dependency graph if requested
    if args.graph {
        println!("\nDEPENDENCY GRAPH:");
        println!("{}", "=".repeat(60));
        let graph = IdlResolver::create_dependency_graph(&entries);
        for (source, includes) in &graph {
            println!("\n{}", source);
            if includes.is_empty() {
                println!("  (no includes)");
            } else {
                for include in includes {
                    println!("  └─> {}", include);
                }
            }
        }
        println!();
    }

    Ok(())
}
