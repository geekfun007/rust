//! thrift-to-ts CLI
//! 
//! 将 Thrift IDL 文件转换为 TypeScript 类型定义

mod ast;
mod codegen;
mod lexer;
mod parser;

use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::codegen::TypeScriptGenerator;
use crate::lexer::Lexer;
use crate::parser::Parser;

/// Thrift to TypeScript type definition generator
#[derive(ClapParser, Debug)]
#[command(name = "thrift-to-ts")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input Thrift file or directory
    #[arg(short, long)]
    input: PathBuf,
    
    /// Output directory for TypeScript files
    #[arg(short, long)]
    output: PathBuf,
    
    /// Generate readonly properties
    #[arg(long, default_value = "false")]
    readonly: bool,
    
    /// Don't add export keyword
    #[arg(long = "no-export", default_value = "false")]
    no_export: bool,
    
    /// Watch for file changes
    #[arg(short, long, default_value = "false")]
    watch: bool,
    
    /// Verbose output
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // 确保输出目录存在
    fs::create_dir_all(&cli.output)
        .context("Failed to create output directory")?;
    
    // 获取所有 .thrift 文件
    let thrift_files = collect_thrift_files(&cli.input)?;
    
    if thrift_files.is_empty() {
        println!("No .thrift files found in {:?}", cli.input);
        return Ok(());
    }
    
    if cli.verbose {
        println!("Found {} Thrift file(s)", thrift_files.len());
    }
    
    // 处理每个文件
    for input_path in thrift_files {
        if cli.verbose {
            println!("Processing: {:?}", input_path);
        }
        
        match process_file(&input_path, &cli) {
            Ok(output_path) => {
                println!("✓ Generated: {:?}", output_path);
            }
            Err(e) => {
                eprintln!("✗ Error processing {:?}: {}", input_path, e);
            }
        }
    }
    
    Ok(())
}

/// 收集所有 .thrift 文件
fn collect_thrift_files(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if path.is_file() {
        if path.extension().map_or(false, |ext| ext == "thrift") {
            files.push(path.clone());
        }
    } else if path.is_dir() {
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();
            if entry_path.is_file()
                && entry_path.extension().map_or(false, |ext| ext == "thrift")
            {
                files.push(entry_path.to_path_buf());
            }
        }
    }
    
    Ok(files)
}

/// 处理单个 Thrift 文件
fn process_file(input_path: &PathBuf, cli: &Cli) -> Result<PathBuf> {
    // 读取文件内容
    let content = fs::read_to_string(input_path)
        .with_context(|| format!("Failed to read file: {:?}", input_path))?;
    
    // 词法分析
    let mut lexer = Lexer::new(&content);
    let tokens = lexer.tokenize()
        .with_context(|| format!("Lexer error in {:?}", input_path))?;
    
    // 语法分析
    let mut parser = Parser::new(tokens);
    let document = parser.parse()
        .with_context(|| format!("Parser error in {:?}", input_path))?;
    
    // 代码生成
    let mut generator = TypeScriptGenerator::new();
    generator.readonly = cli.readonly;
    generator.export = !cli.no_export;
    
    let ts_code = generator.generate(&document);
    
    // 计算输出文件路径
    let file_stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    
    let output_path = cli.output.join(format!("{}.ts", file_stem));
    
    // 写入文件
    fs::write(&output_path, ts_code)
        .with_context(|| format!("Failed to write file: {:?}", output_path))?;
    
    Ok(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_end_to_end() {
        let input = r#"
            namespace js example
            
            enum UserStatus {
                ACTIVE = 1,
                INACTIVE = 2
            }
            
            struct User {
                1: required i64 id
                2: required string name
                3: optional string email
                4: required UserStatus status
            }
            
            struct CreateUserRequest {
                1: required string name
                2: optional string email
            }
            
            service UserService {
                User getUser(1: i64 id)
                User createUser(1: CreateUserRequest request)
                void deleteUser(1: i64 id)
            }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let document = parser.parse().unwrap();
        
        let mut generator = TypeScriptGenerator::new();
        let ts_code = generator.generate(&document);
        
        // 验证生成的代码包含预期内容
        assert!(ts_code.contains("enum UserStatus"));
        assert!(ts_code.contains("ACTIVE = 1"));
        assert!(ts_code.contains("interface User"));
        assert!(ts_code.contains("id: bigint"));
        assert!(ts_code.contains("name: string"));
        assert!(ts_code.contains("email?: string"));
        assert!(ts_code.contains("status: UserStatus"));
        assert!(ts_code.contains("interface UserService"));
        assert!(ts_code.contains("getUser(id: bigint): Promise<User>"));
    }
}
