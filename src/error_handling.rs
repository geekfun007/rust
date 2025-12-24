// Rust 错误处理详解
// 
// 涵盖：Result、Option、?, unwrap, expect, 自定义错误、thiserror, anyhow

use std::fs::File;
use std::io;
use std::num::ParseIntError;
use std::fmt;

// 使用 thiserror 简化自定义错误
use thiserror::Error;
// 使用 anyhow 处理通用错误
use anyhow::{Context, Result as AnyhowResult, anyhow};

fn main() {
    println!("=== Rust 错误处理实战 ===\n");
    
    // 1. Result 和 Option 基础
    demo_result_option_basics();
    
    // 2. ? 操作符
    demo_question_mark_operator();
    
    // 3. unwrap 和 expect
    demo_unwrap_expect();
    
    // 4. 错误传播和转换
    demo_error_propagation();
    
    // 5. 自定义错误类型
    demo_custom_errors();
    
    // 6. thiserror 库
    demo_thiserror();
    
    // 7. anyhow 库（快速原型开发）
    demo_anyhow();
    
    // 8. 实战案例
    demo_real_world_cases();
}

// ============================================
// 1. Result 和 Option 基础
// ============================================
fn demo_result_option_basics() {
    println!("--- 1. Result 和 Option 基础 ---");
    
    // Option: 可能有值，可能没有
    fn find_user(id: u32) -> Option<String> {
        if id == 1 {
            Some("张三".to_string())
        } else {
            None
        }
    }
    
    match find_user(1) {
        Some(name) => println!("找到用户: {}", name),
        None => println!("用户不存在"),
    }
    
    // Result: 可能成功，可能失败（带错误信息）
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    println!();
}

// ============================================
// 2. ? 操作符 - 简化错误传播
// ============================================
fn demo_question_mark_operator() {
    println!("--- 2. ? 操作符 ---");
    
    // ? 操作符自动传播错误
    fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
        let n = s.parse::<i32>()?; // 如果失败，自动返回 Err
        Ok(n * 2)
    }
    
    match parse_and_double("42") {
        Ok(n) => println!("42 的两倍是: {}", n),
        Err(e) => println!("解析错误: {}", e),
    }
    
    match parse_and_double("abc") {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("解析错误: {}", e),
    }
    
    // ? 也可以用于 Option
    fn get_first_last(vec: Vec<i32>) -> Option<(i32, i32)> {
        let first = vec.first()?; // 如果为 None，自动返回 None
        let last = vec.last()?;
        Some((*first, *last))
    }
    
    println!("非空数组: {:?}", get_first_last(vec![1, 2, 3]));
    println!("空数组: {:?}", get_first_last(vec![]));
    
    println!();
}

// ============================================
// 3. unwrap 和 expect
// ============================================
fn demo_unwrap_expect() {
    println!("--- 3. unwrap 和 expect ---");
    
    // unwrap: 如果是 Err/None，会 panic
    let result: Result<i32, &str> = Ok(42);
    let value = result.unwrap();
    println!("unwrap Ok: {}", value);
    
    // expect: 类似 unwrap，但可以自定义 panic 消息
    let result: Result<i32, &str> = Ok(100);
    let value = result.expect("应该是 Ok 值");
    println!("expect Ok: {}", value);
    
    // 注意：生产环境应避免 unwrap，除非你确定不会失败
    // 不好的做法（会 panic）:
    // let result: Result<i32, &str> = Err("错误");
    // result.unwrap(); // 程序崩溃！
    
    // 更安全的替代方案
    let result: Result<i32, &str> = Err("错误");
    let value = result.unwrap_or(0); // 提供默认值
    println!("unwrap_or: {}", value);
    
    let value = result.unwrap_or_else(|e| {
        println!("发生错误: {}, 使用默认值", e);
        -1
    });
    println!("unwrap_or_else: {}", value);
    
    println!();
}

// ============================================
// 4. 错误传播和转换
// ============================================
fn demo_error_propagation() {
    println!("--- 4. 错误传播和转换 ---");
    
    // map_err: 转换错误类型
    fn read_number_from_string(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|e| format!("解析失败: {}", e)) // ParseIntError -> String
    }
    
    match read_number_from_string("123") {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("{}", e),
    }
    
    // and_then: 链式处理 Result
    fn process(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|e| format!("解析错误: {}", e))
            .and_then(|n| {
                if n > 0 {
                    Ok(n * 2)
                } else {
                    Err("数字必须是正数".to_string())
                }
            })
    }
    
    println!("处理 '10': {:?}", process("10"));
    println!("处理 '-5': {:?}", process("-5"));
    
    // or_else: 提供替代方案
    let result: Result<i32, &str> = Err("主要方法失败");
    let final_result: Result<i32, &str> = result.or_else(|_| {
        println!("尝试备用方法...");
        Ok(42) // 备用成功
    });
    println!("最终结果: {:?}", final_result);
    
    println!();
}

// ============================================
// 5. 自定义错误类型
// ============================================

// 方式 1: 手动实现 Error trait
#[derive(Debug)]
enum MyError {
    IoError(io::Error),
    ParseError(ParseIntError),
    ValidationError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO 错误: {}", e),
            MyError::ParseError(e) => write!(f, "解析错误: {}", e),
            MyError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
        }
    }
}

impl std::error::Error for MyError {}

// 实现 From trait 简化错误转换
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::IoError(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::ParseError(err)
    }
}

fn demo_custom_errors() {
    println!("--- 5. 自定义错误类型 ---");
    
    fn read_and_parse_file(path: &str) -> Result<i32, MyError> {
        // 模拟读取文件（实际应该用真实的文件操作）
        if path.is_empty() {
            return Err(MyError::ValidationError("路径不能为空".to_string()));
        }
        
        let content = "123";
        let number = content.parse::<i32>()?; // 自动转换 ParseIntError -> MyError
        
        Ok(number)
    }
    
    match read_and_parse_file("data.txt") {
        Ok(n) => println!("读取到数字: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    println!();
}

// ============================================
// 6. thiserror 库 - 简化自定义错误
// ============================================

#[derive(Error, Debug)]
enum AppError {
    #[error("IO 错误: {0}")]
    Io(#[from] io::Error),
    
    #[error("解析错误: {0}")]
    Parse(#[from] ParseIntError),
    
    #[error("验证失败: {message}")]
    Validation { message: String },
    
    #[error("用户 {user_id} 未找到")]
    UserNotFound { user_id: u32 },
    
    #[error("数据库错误")]
    Database,
}

fn demo_thiserror() {
    println!("--- 6. thiserror 库 ---");
    
    fn validate_age(age: &str) -> Result<u32, AppError> {
        let age: u32 = age.parse()?; // 自动转换 ParseIntError
        
        if age < 18 {
            return Err(AppError::Validation {
                message: "年龄必须大于等于 18".to_string(),
            });
        }
        
        Ok(age)
    }
    
    match validate_age("25") {
        Ok(age) => println!("有效年龄: {}", age),
        Err(e) => println!("{}", e),
    }
    
    match validate_age("15") {
        Ok(age) => println!("有效年龄: {}", age),
        Err(e) => println!("{}", e),
    }
    
    // 演示其他错误类型
    let error = AppError::UserNotFound { user_id: 123 };
    println!("{}", error);
    
    println!();
}

// ============================================
// 7. anyhow 库 - 快速原型开发
// ============================================
fn demo_anyhow() {
    println!("--- 7. anyhow 库 ---");
    
    // anyhow::Result 可以包含任何实现了 Error trait 的错误
    fn process_data(input: &str) -> AnyhowResult<i32> {
        let num: i32 = input
            .parse()
            .context("无法解析输入为数字")?; // 添加上下文信息
        
        if num < 0 {
            return Err(anyhow!("数字不能为负数: {}", num));
        }
        
        if num > 100 {
            return Err(anyhow!("数字不能大于 100"));
        }
        
        Ok(num * 2)
    }
    
    match process_data("50") {
        Ok(result) => println!("处理结果: {}", result),
        Err(e) => println!("错误: {:?}", e),
    }
    
    match process_data("abc") {
        Ok(result) => println!("处理结果: {}", result),
        Err(e) => {
            println!("错误: {}", e);
            // 打印完整的错误链
            for cause in e.chain().skip(1) {
                println!("  原因: {}", cause);
            }
        }
    }
    
    // anyhow 特别适合快速开发，不需要定义专门的错误类型
    fn quick_operation() -> AnyhowResult<()> {
        let _n: i32 = "123".parse().context("解析失败")?;
        // 可以直接返回任何错误类型
        Ok(())
    }
    
    let _ = quick_operation();
    
    println!();
}

// ============================================
// 8. 实战案例
// ============================================

#[derive(Error, Debug)]
enum ConfigError {
    #[error("配置文件不存在: {path}")]
    FileNotFound { path: String },
    
    #[error("配置文件读取失败")]
    ReadError(#[from] io::Error),
    
    #[error("配置格式错误: {0}")]
    FormatError(String),
    
    #[error("缺少必需的配置项: {0}")]
    MissingField(String),
}

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u64,
}

fn demo_real_world_cases() {
    println!("--- 8. 实战案例 ---");
    
    // 案例 1: 读取和解析配置文件
    fn load_config(path: &str) -> Result<Config, ConfigError> {
        // 检查文件是否存在（模拟）
        if path.is_empty() {
            return Err(ConfigError::FileNotFound {
                path: path.to_string(),
            });
        }
        
        // 模拟读取文件内容
        let content = "host=localhost\nport=8080\ntimeout=30";
        
        // 解析配置
        let mut host = None;
        let mut port = None;
        let mut timeout = None;
        
        for line in content.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                continue;
            }
            
            match parts[0] {
                "host" => host = Some(parts[1].to_string()),
                "port" => {
                    port = Some(
                        parts[1]
                            .parse()
                            .map_err(|_| ConfigError::FormatError("port 必须是数字".to_string()))?
                    )
                },
                "timeout" => {
                    timeout = Some(
                        parts[1]
                            .parse()
                            .map_err(|_| ConfigError::FormatError("timeout 必须是数字".to_string()))?
                    )
                },
                _ => {}
            }
        }
        
        Ok(Config {
            host: host.ok_or_else(|| ConfigError::MissingField("host".to_string()))?,
            port: port.ok_or_else(|| ConfigError::MissingField("port".to_string()))?,
            timeout: timeout.unwrap_or(60),
        })
    }
    
    match load_config("config.txt") {
        Ok(config) => println!("配置加载成功: {:?}", config),
        Err(e) => println!("配置加载失败: {}", e),
    }
    
    // 案例 2: 多层错误处理
    println!("\n案例 2: 数据验证管道");
    data_validation_pipeline();
    
    // 案例 3: 错误恢复
    println!("\n案例 3: 错误恢复");
    error_recovery_example();
}

fn data_validation_pipeline() {
    fn validate_and_process(input: &str) -> AnyhowResult<i32> {
        // 第一步: 清理输入
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("输入不能为空"));
        }
        
        // 第二步: 解析
        let number: i32 = trimmed
            .parse()
            .context("输入必须是有效的整数")?;
        
        // 第三步: 验证范围
        if !(1..=100).contains(&number) {
            return Err(anyhow!("数字必须在 1-100 之间，得到: {}", number));
        }
        
        // 第四步: 业务逻辑
        let result = number * 2;
        
        Ok(result)
    }
    
    let test_cases = vec!["  42  ", "150", "abc", ""];
    
    for input in test_cases {
        match validate_and_process(input) {
            Ok(result) => println!("  '{}' -> {}", input, result),
            Err(e) => println!("  '{}' -> 错误: {}", input, e),
        }
    }
}

fn error_recovery_example() {
    // 尝试多个数据源，直到成功
    fn fetch_data_with_fallback() -> Result<String, String> {
        // 主数据源
        let primary = fetch_from_primary();
        if primary.is_ok() {
            println!("  从主数据源获取成功");
            return primary;
        }
        
        // 备用数据源 1
        println!("  主数据源失败，尝试备用源 1...");
        let secondary = fetch_from_secondary();
        if secondary.is_ok() {
            println!("  从备用源 1 获取成功");
            return secondary;
        }
        
        // 备用数据源 2（缓存）
        println!("  备用源 1 失败，尝试缓存...");
        let cache = fetch_from_cache();
        if cache.is_ok() {
            println!("  从缓存获取成功");
            return cache;
        }
        
        Err("所有数据源都失败了".to_string())
    }
    
    fn fetch_from_primary() -> Result<String, String> {
        Err("主服务器连接超时".to_string())
    }
    
    fn fetch_from_secondary() -> Result<String, String> {
        Err("备用服务器不可用".to_string())
    }
    
    fn fetch_from_cache() -> Result<String, String> {
        Ok("缓存数据".to_string())
    }
    
    match fetch_data_with_fallback() {
        Ok(data) => println!("  最终数据: {}", data),
        Err(e) => println!("  完全失败: {}", e),
    }
}
