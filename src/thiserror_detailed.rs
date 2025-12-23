// Thiserror 详解 - 库错误处理完全指南
//
// thiserror 是 Rust 中用于定义自定义错误类型的库
// 本教程涵盖所有核心功能和实战技巧

use std::io;
use std::num::ParseIntError;
use thiserror::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Thiserror 错误处理详解 ===\n");
    
    // 1. Thiserror 基础
    demo_basics();
    
    // 2. derive(Error) 宏
    demo_derive_error()?;
    
    // 3. error 属性
    demo_error_attribute()?;
    
    // 4. from 属性
    demo_from_attribute()?;
    
    // 5. source 属性
    demo_source_attribute()?;
    
    // 6. transparent 错误
    demo_transparent()?;
    
    // 7. 错误枚举设计
    demo_error_enum_design()?;
    
    // 8. 与 anyhow 配合
    demo_with_anyhow()?;
    
    // 9. 实战案例
    demo_real_world_examples()?;
    
    // 10. 最佳实践
    demo_best_practices()?;
    
    println!("\n✅ 所有示例运行成功！");
    Ok(())
}

// ============================================
// 1. Thiserror 基础
// ============================================
fn demo_basics() {
    println!("--- 1. Thiserror 基础 ---\n");
    
    println!("什么是 Thiserror？");
    println!("  - 用于定义自定义错误类型的库");
    println!("  - 自动实现 std::error::Error trait");
    println!("  - 自动实现 Display trait");
    println!("  - 适合库开发（而非应用程序）\n");
    
    println!("核心特性:");
    println!("  📌 #[derive(Error)] - 自动实现 Error trait");
    println!("  📌 #[error(\"...\")] - 定义错误消息");
    println!("  📌 #[from] - 自动类型转换");
    println!("  📌 #[source] - 指定错误源");
    println!("  📌 #[error(transparent)] - 透明包装\n");
    
    println!("基础用法:");
    println!("  #[derive(Error, Debug)]");
    println!("  enum MyError {{");
    println!("      #[error(\"something went wrong\")]");
    println!("      SomethingWrong,");
    println!("  }}");
    println!();
    
    println!("使用场景:");
    println!("  ✓ 库开发");
    println!("  ✓ 精确的错误类型");
    println!("  ✓ 公共 API");
    println!("  ✗ 应用程序（用 anyhow）\n");
}

// ============================================
// 2. derive(Error) 宏
// ============================================
fn demo_derive_error() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 2. derive(Error) 宏 ---\n");
    
    println!("基础示例:");
    
    // 最简单的错误类型
    #[derive(Error, Debug)]
    #[error("simple error occurred")]
    struct SimpleError;
    
    // 使用错误
    fn may_fail() -> Result<(), SimpleError> {
        Err(SimpleError)
    }
    
    match may_fail() {
        Err(e) => println!("  错误: {}", e),
        _ => {}
    }
    println!();
    
    // 带字段的错误
    println!("带字段的错误:");
    
    #[derive(Error, Debug)]
    #[error("invalid value: {value}")]
    struct InvalidValue {
        value: i32,
    }
    
    let err = InvalidValue { value: -1 };
    println!("  {}", err);
    println!();
    
    // 枚举错误
    println!("枚举错误:");
    
    #[derive(Error, Debug)]
    enum MyError {
        #[error("IO error")]
        Io,
        #[error("parse error")]
        Parse,
        #[error("not found")]
        NotFound,
    }
    
    let errors = vec![MyError::Io, MyError::Parse, MyError::NotFound];
    for err in errors {
        println!("  {}", err);
    }
    println!();
    
    Ok(())
}

// ============================================
// 3. error 属性
// ============================================
fn demo_error_attribute() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 3. error 属性 ---\n");
    
    // 简单消息
    println!("简单消息:");
    
    #[derive(Error, Debug)]
    #[error("something went wrong")]
    struct BasicError;
    
    println!("  {}", BasicError);
    println!();
    
    // 格式化消息 - 使用字段
    println!("格式化消息 - 使用字段:");
    
    #[derive(Error, Debug)]
    #[error("invalid input: {input}")]
    struct InputError {
        input: String,
    }
    
    let err = InputError {
        input: "bad_value".to_string(),
    };
    println!("  {}", err);
    println!();
    
    // 复杂格式化
    println!("复杂格式化:");
    
    #[derive(Error, Debug)]
    #[error("range error: {value} not in [{min}, {max}]")]
    struct RangeError {
        value: i32,
        min: i32,
        max: i32,
    }
    
    let err = RangeError {
        value: 150,
        min: 0,
        max: 100,
    };
    println!("  {}", err);
    println!();
    
    // 变体特定消息
    println!("枚举变体特定消息:");
    
    #[derive(Error, Debug)]
    enum ValidationError {
        #[error("field '{field}' is required")]
        Required { field: String },
        
        #[error("field '{field}' must be at least {min} characters")]
        TooShort { field: String, min: usize },
        
        #[error("invalid email format: {0}")]
        InvalidEmail(String),
    }
    
    let errors = vec![
        ValidationError::Required {
            field: "username".to_string(),
        },
        ValidationError::TooShort {
            field: "password".to_string(),
            min: 8,
        },
        ValidationError::InvalidEmail("not-an-email".to_string()),
    ];
    
    for err in errors {
        println!("  {}", err);
    }
    println!();
    
    Ok(())
}

// ============================================
// 4. from 属性
// ============================================
fn demo_from_attribute() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 4. from 属性 ---\n");
    
    println!("自动类型转换:");
    println!("  #[from] 自动实现 From trait，启用 ? 操作符\n");
    
    #[derive(Error, Debug)]
    enum FileError {
        #[error("IO error: {0}")]
        Io(#[from] io::Error),
        
        #[error("parse error: {0}")]
        Parse(#[from] ParseIntError),
        
        #[error("custom error: {0}")]
        Custom(String),
    }
    
    fn read_number(path: &str) -> Result<i32, FileError> {
        // io::Error 自动转换为 FileError::Io
        let content = std::fs::read_to_string(path)?;
        
        // ParseIntError 自动转换为 FileError::Parse
        let number: i32 = content.trim().parse()?;
        
        Ok(number)
    }
    
    // 测试
    match read_number("nonexistent.txt") {
        Err(e) => {
            println!("  错误: {}", e);
            println!("  类型: {:?}", e);
        }
        _ => {}
    }
    println!();
    
    println!("等价的手动实现:");
    println!("  impl From<io::Error> for FileError {{");
    println!("      fn from(err: io::Error) -> Self {{");
    println!("          FileError::Io(err)");
    println!("      }}");
    println!("  }}");
    println!();
    
    Ok(())
}

// ============================================
// 5. source 属性
// ============================================
fn demo_source_attribute() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 5. source 属性 ---\n");
    
    println!("指定错误源（用于错误链）:");
    
    #[derive(Error, Debug)]
    enum DatabaseError {
        #[error("connection failed")]
        Connection {
            #[source]
            source: io::Error,
        },
        
        #[error("query failed: {query}")]
        Query {
            query: String,
            #[source]
            source: io::Error,
        },
    }
    
    // 创建嵌套错误
    let io_err = io::Error::new(io::ErrorKind::ConnectionRefused, "connection refused");
    let db_err = DatabaseError::Connection { source: io_err };
    
    println!("  错误: {}", db_err);
    
    // 访问错误源
    if let Some(source) = std::error::Error::source(&db_err) {
        println!("  根源: {}", source);
    }
    println!();
    
    println!("source vs from 的区别:");
    println!("  #[from] - 自动类型转换 + 设置 source");
    println!("  #[source] - 仅设置 source，不自动转换");
    println!();
    
    Ok(())
}

// ============================================
// 6. transparent 错误
// ============================================
fn demo_transparent() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 6. transparent 错误 ---\n");
    
    println!("透明包装 - 完全暴露内部错误:");
    
    #[derive(Error, Debug)]
    enum WrapperError {
        #[error(transparent)]
        Io(#[from] io::Error),
        
        #[error(transparent)]
        Parse(#[from] ParseIntError),
    }
    
    fn transparent_example() -> Result<i32, WrapperError> {
        let content = std::fs::read_to_string("missing.txt")?;
        let num: i32 = content.parse()?;
        Ok(num)
    }
    
    match transparent_example() {
        Err(e) => {
            println!("  错误: {}", e);
            println!("  说明: 显示的是原始 IO 错误，不是 WrapperError");
        }
        _ => {}
    }
    println!();
    
    println!("使用场景:");
    println!("  ✓ 简单包装其他错误");
    println!("  ✓ 不需要自定义消息");
    println!("  ✓ 完全暴露底层错误");
    println!();
    
    Ok(())
}

// ============================================
// 7. 错误枚举设计
// ============================================
fn demo_error_enum_design() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 7. 错误枚举设计 ---\n");
    
    // 完整的错误设计
    println!("完整的错误枚举设计:");
    
    #[derive(Error, Debug)]
    pub enum ConfigError {
        // 简单变体
        #[error("configuration not found")]
        NotFound,
        
        // 带数据的变体
        #[error("invalid port: {0}")]
        InvalidPort(u16),
        
        // 嵌套变体
        #[error("failed to read config file")]
        Io(#[from] io::Error),
        
        // 带多个字段
        #[error("validation failed for field '{field}': {reason}")]
        ValidationFailed { field: String, reason: String },
        
        // 包装其他错误
        #[error("parse error")]
        Parse(#[from] ParseIntError),
    }
    
    println!("  定义了 5 种错误变体");
    println!("  ✓ 简单错误");
    println!("  ✓ 带数据错误");
    println!("  ✓ IO 错误（自动转换）");
    println!("  ✓ 验证错误");
    println!("  ✓ 解析错误（自动转换）");
    println!();
    
    // 使用示例
    fn load_config(path: &str) -> Result<(), ConfigError> {
        if path.is_empty() {
            return Err(ConfigError::NotFound);
        }
        
        // IO 错误自动转换
        let _content = std::fs::read_to_string(path)?;
        
        Ok(())
    }
    
    match load_config("") {
        Err(e) => println!("  示例错误: {}", e),
        _ => {}
    }
    println!();
    
    // 错误匹配
    println!("错误匹配:");
    
    let err = ConfigError::InvalidPort(65000);
    
    match err {
        ConfigError::NotFound => {
            println!("  配置未找到");
        }
        ConfigError::InvalidPort(port) => {
            println!("  无效端口: {}", port);
        }
        ConfigError::ValidationFailed { field, reason } => {
            println!("  验证失败: {} - {}", field, reason);
        }
        _ => {
            println!("  其他错误");
        }
    }
    println!();
    
    Ok(())
}

// ============================================
// 8. 与 anyhow 配合
// ============================================
fn demo_with_anyhow() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 8. 与 Anyhow 配合 ---\n");
    
    println!("Thiserror vs Anyhow:");
    println!("┌──────────────┬────────────┬──────────┐");
    println!("│  特性         │ Thiserror  │ Anyhow   │");
    println!("├──────────────┼────────────┼──────────┤");
    println!("│  用途         │ 库         │ 应用     │");
    println!("│  错误类型     │ 具体       │ 统一     │");
    println!("│  模式匹配     │ ✓          │ ✗        │");
    println!("│  上下文       │ 需手动     │ 内置     │");
    println!("│  自动转换     │ #[from]    │ ?        │");
    println!("└──────────────┴────────────┴──────────┘");
    println!();
    
    println!("配合使用模式:");
    
    // 库代码使用 thiserror
    #[derive(Error, Debug)]
    pub enum LibError {
        #[error("lib error: {0}")]
        SomeError(String),
        
        #[error("IO error")]
        Io(#[from] io::Error),
    }
    
    pub fn lib_function() -> Result<(), LibError> {
        Err(LibError::SomeError("something wrong".to_string()))
    }
    
    // 应用代码使用 anyhow
    fn app_function() -> anyhow::Result<()> {
        // LibError 自动转换为 anyhow::Error
        lib_function()?;
        Ok(())
    }
    
    match app_function() {
        Err(e) => println!("  应用错误: {}", e),
        _ => {}
    }
    println!();
    
    println!("最佳实践:");
    println!("  - 库使用 thiserror（精确类型）");
    println!("  - 应用使用 anyhow（简化处理）");
    println!("  - 自动转换无缝衔接");
    println!();
    
    Ok(())
}

// ============================================
// 9. 实战案例
// ============================================
fn demo_real_world_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 9. 实战案例 ---\n");
    
    // 案例 1: 配置库
    println!("案例 1: 配置库错误\n");
    config_error_example()?;
    
    // 案例 2: HTTP 客户端库
    println!("\n案例 2: HTTP 客户端库错误\n");
    http_error_example()?;
    
    // 案例 3: 数据库库
    println!("\n案例 3: 数据库库错误\n");
    database_error_example()?;
    
    Ok(())
}

// 案例 1: 配置库错误
fn config_error_example() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Error, Debug)]
    pub enum ConfigError {
        #[error("config file not found at: {path}")]
        NotFound { path: String },
        
        #[error("failed to read config file: {path}")]
        ReadError {
            path: String,
            #[source]
            source: io::Error,
        },
        
        #[error("invalid config format")]
        ParseError(#[from] serde_json::Error),
        
        #[error("missing required field: {0}")]
        MissingField(String),
        
        #[error("invalid value for {field}: {value}")]
        InvalidValue { field: String, value: String },
    }
    
    fn load_config(path: &str) -> Result<(), ConfigError> {
        if path.is_empty() {
            return Err(ConfigError::NotFound {
                path: path.to_string(),
            });
        }
        
        match std::fs::read_to_string(path) {
            Err(e) => Err(ConfigError::ReadError {
                path: path.to_string(),
                source: e,
            }),
            Ok(_) => Ok(()),
        }
    }
    
    // 测试
    match load_config("") {
        Err(e) => {
            println!("  配置错误: {}", e);
            
            // 匹配具体错误类型
            match e {
                ConfigError::NotFound { path } => {
                    println!("  -> 路径: {}", path);
                    println!("  -> 建议: 检查文件路径");
                }
                _ => {}
            }
        }
        _ => {}
    }
    
    Ok(())
}

// 案例 2: HTTP 客户端库错误
fn http_error_example() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Error, Debug)]
    pub enum HttpError {
        #[error("connection failed to {url}")]
        Connection { url: String },
        
        #[error("request timeout after {timeout}s")]
        Timeout { timeout: u64 },
        
        #[error("HTTP {status}: {message}")]
        HttpStatus { status: u16, message: String },
        
        #[error("invalid URL: {0}")]
        InvalidUrl(String),
        
        #[error("network error")]
        Network(#[from] io::Error),
    }
    
    impl HttpError {
        // 便捷构造方法
        pub fn timeout(seconds: u64) -> Self {
            HttpError::Timeout { timeout: seconds }
        }
        
        pub fn not_found(url: String) -> Self {
            HttpError::HttpStatus {
                status: 404,
                message: url,
            }
        }
        
        // 错误分类
        pub fn is_timeout(&self) -> bool {
            matches!(self, HttpError::Timeout { .. })
        }
        
        pub fn is_client_error(&self) -> bool {
            matches!(self, HttpError::HttpStatus { status, .. } if *status >= 400 && *status < 500)
        }
    }
    
    // 使用示例
    let err = HttpError::timeout(30);
    println!("  HTTP 错误: {}", err);
    println!("  是超时? {}", err.is_timeout());
    
    let err = HttpError::not_found("http://example.com".to_string());
    println!("  HTTP 错误: {}", err);
    println!("  是客户端错误? {}", err.is_client_error());
    
    Ok(())
}

// 案例 3: 数据库库错误
fn database_error_example() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Error, Debug)]
    pub enum DatabaseError {
        #[error("connection pool exhausted")]
        PoolExhausted,
        
        #[error("connection failed: {0}")]
        ConnectionFailed(String),
        
        #[error("query failed: {query}")]
        QueryFailed {
            query: String,
            #[source]
            source: Box<dyn std::error::Error + Send + Sync>,
        },
        
        #[error("transaction failed")]
        TransactionFailed,
        
        #[error("record not found: {table} id={id}")]
        NotFound { table: String, id: i64 },
        
        #[error("duplicate key: {key}")]
        DuplicateKey { key: String },
    }
    
    impl DatabaseError {
        // 错误恢复建议
        pub fn recovery_suggestion(&self) -> &str {
            match self {
                DatabaseError::PoolExhausted => "Increase pool size or wait",
                DatabaseError::ConnectionFailed(_) => "Check network and credentials",
                DatabaseError::QueryFailed { .. } => "Review query syntax",
                DatabaseError::TransactionFailed => "Retry transaction",
                DatabaseError::NotFound { .. } => "Check if record exists",
                DatabaseError::DuplicateKey { .. } => "Use different key",
            }
        }
        
        // 是否可重试
        pub fn is_retryable(&self) -> bool {
            matches!(
                self,
                DatabaseError::PoolExhausted | DatabaseError::TransactionFailed
            )
        }
    }
    
    // 使用示例
    let err = DatabaseError::NotFound {
        table: "users".to_string(),
        id: 123,
    };
    
    println!("  数据库错误: {}", err);
    println!("  恢复建议: {}", err.recovery_suggestion());
    println!("  可重试? {}", err.is_retryable());
    
    Ok(())
}

// ============================================
// 10. 最佳实践
// ============================================
fn demo_best_practices() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 10. 最佳实践 ---\n");
    
    println!("1. 错误类型设计:");
    println!("   ✓ 使用枚举而非结构体");
    println!("   ✓ 每个变体表示一类错误");
    println!("   ✓ 包含必要的上下文信息");
    println!();
    
    println!("2. 错误消息:");
    println!("   ✓ 清晰描述问题");
    println!("   ✓ 包含相关数据");
    println!("   ✓ 避免技术术语（用户友好）");
    println!();
    
    println!("3. 使用 #[from]:");
    println!("   ✓ 简化错误转换");
    println!("   ✓ 启用 ? 操作符");
    println!("   ✓ 自动设置 source");
    println!();
    
    println!("4. 错误粒度:");
    println!("   ✓ 足够详细以支持调试");
    println!("   ✓ 支持模式匹配");
    println!("   ✓ 不要过度细分");
    println!();
    
    println!("5. 公共 API:");
    println!("   ✓ 错误类型作为公共 API 的一部分");
    println!("   ✓ 保持向后兼容");
    println!("   ✓ 文档化每个变体");
    println!();
    
    println!("6. 错误恢复:");
    println!("   ✓ 提供错误分类方法");
    println!("   ✓ 提供恢复建议");
    println!("   ✓ 标记可重试的错误");
    println!();
    
    // 好的示例
    println!("好的错误设计示例:\n");
    
    #[derive(Error, Debug)]
    pub enum GoodError {
        // 清晰的错误消息
        #[error("file '{path}' not found")]
        FileNotFound { path: String },
        
        // 自动转换
        #[error("IO error")]
        Io(#[from] io::Error),
        
        // 包含上下文
        #[error("invalid configuration: {field} = {value}")]
        InvalidConfig { field: String, value: String },
        
        // 透明包装
        #[error(transparent)]
        Other(#[from] Box<dyn std::error::Error + Send + Sync>),
    }
    
    impl GoodError {
        // 便捷方法
        pub fn file_not_found(path: impl Into<String>) -> Self {
            GoodError::FileNotFound { path: path.into() }
        }
        
        // 错误分类
        pub fn is_not_found(&self) -> bool {
            matches!(self, GoodError::FileNotFound { .. })
        }
        
        // 恢复建议
        pub fn suggestion(&self) -> &str {
            match self {
                GoodError::FileNotFound { .. } => "Check file path",
                GoodError::Io(_) => "Check file permissions",
                GoodError::InvalidConfig { .. } => "Fix configuration",
                GoodError::Other(_) => "Contact support",
            }
        }
    }
    
    println!("  特点:");
    println!("  ✓ 清晰的错误消息");
    println!("  ✓ 自动转换支持");
    println!("  ✓ 便捷构造方法");
    println!("  ✓ 错误分类方法");
    println!("  ✓ 恢复建议");
    println!();
    
    // 坏的示例
    println!("应避免的模式:\n");
    println!("  ❌ 过于笼统:");
    println!("     #[error(\"error\")]");
    println!("     Error,");
    println!();
    println!("  ❌ 缺少上下文:");
    println!("     #[error(\"invalid\")]");
    println!("     Invalid,");
    println!();
    println!("  ❌ 技术术语:");
    println!("     #[error(\"ENOENT\")]");
    println!("     SystemError,");
    println!();
    
    Ok(())
}

/*
=== 总结 ===

1. Thiserror 核心概念:

   宏:
   - #[derive(Error)] - 自动实现 Error trait
   - #[error("...")] - 定义错误消息
   - #[from] - 自动 From 实现
   - #[source] - 指定错误源
   - #[error(transparent)] - 透明包装

2. 错误设计原则:

   结构:
   - 使用枚举
   - 每个变体一类错误
   - 包含必要上下文
   
   消息:
   - 清晰描述问题
   - 包含相关数据
   - 用户友好

3. 与 Anyhow 对比:

   Thiserror:
   ✓ 库开发
   ✓ 具体错误类型
   ✓ 模式匹配
   ✓ 公共 API
   
   Anyhow:
   ✓ 应用开发
   ✓ 统一错误类型
   ✓ 简化处理
   ✓ 上下文链

4. 最佳实践:

   DO:
   ✓ 详细的错误消息
   ✓ 使用 #[from] 简化转换
   ✓ 提供便捷方法
   ✓ 支持错误分类
   ✓ 提供恢复建议
   
   DON'T:
   ✗ 过于笼统的错误
   ✗ 缺少上下文
   ✗ 使用技术术语
   ✗ 忽略错误源

5. 常用模式:

   配置错误:
   - NotFound
   - ParseError
   - ValidationError
   
   IO 错误:
   - ReadError
   - WriteError
   - PermissionDenied
   
   网络错误:
   - Connection
   - Timeout
   - HttpStatus

运行示例:
  cargo run --bin thiserror_detailed
*/
