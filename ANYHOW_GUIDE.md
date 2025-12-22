# Rust Anyhow 完全指南

## 目录
- [什么是 Anyhow](#什么是-anyhow)
- [核心概念](#核心概念)
- [核心 API](#核心-api)
- [Context - 错误上下文](#context---错误上下文)
- [错误链与回溯](#错误链与回溯)
- [实战案例](#实战案例)
- [Anyhow vs Thiserror](#anyhow-vs-thiserror)
- [最佳实践](#最佳实践)
- [常见问题](#常见问题)

---

## 什么是 Anyhow

**Anyhow** 是 Rust 生态中最流行的应用层错误处理库，由 [dtolnay](https://github.com/dtolnay) 开发。

### 核心价值

```rust
// 传统方式 - 需要定义错误类型
fn traditional() -> Result<String, Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("file.txt")?;
    let num: i32 = file.parse()?;
    Ok(format!("Number: {}", num))
}

// Anyhow - 简洁统一
use anyhow::Result;

fn with_anyhow() -> Result<String> {
    let file = std::fs::read_to_string("file.txt")?;
    let num: i32 = file.parse()?;
    Ok(format!("Number: {}", num))
}
```

### 主要特性

✅ **统一的错误类型** - `anyhow::Error` 可以包装任何错误  
✅ **自动类型转换** - 任何 `impl std::error::Error` 的类型自动转换  
✅ **丰富的上下文** - 轻松添加错误发生的背景信息  
✅ **错误链** - 追踪错误的完整传播路径  
✅ **回溯支持** - 可选的堆栈跟踪  
✅ **零成本** - 编译时优化，无运行时开销  

---

## 核心概念

### 1. Result<T> 类型

```rust
use anyhow::Result;

// 这个类型定义
type Result<T> = std::result::Result<T, anyhow::Error>;

// 简化函数签名
fn process() -> Result<String> {
    // 任何错误都自动转换为 anyhow::Error
    Ok("success".to_string())
}
```

**对比**：

| 传统 | Anyhow |
|------|--------|
| `Result<T, io::Error>` | `Result<T>` |
| `Result<T, ParseIntError>` | `Result<T>` |
| `Result<T, Box<dyn Error>>` | `Result<T>` |

### 2. 统一的错误类型

```rust
use anyhow::{Result, Context};

fn mixed_operations() -> Result<()> {
    // io::Error 自动转换
    let content = std::fs::read_to_string("file.txt")?;
    
    // ParseIntError 自动转换
    let num: i32 = content.parse()?;
    
    // 自定义错误也可以转换
    validate_number(num)?;
    
    Ok(())
}
```

### 3. main 函数的返回类型

```rust
use anyhow::Result;

fn main() -> Result<()> {
    // 错误会自动打印到 stderr 并设置退出码
    let config = load_config("app.conf")?;
    run_app(config)?;
    Ok(())
}
```

**输出示例**：
```
Error: Failed to load configuration

Caused by:
    0: Failed to read config file
    1: No such file or directory (os error 2)
```

---

## 核心 API

### bail! 宏

**作用**：立即返回错误

```rust
use anyhow::{bail, Result};

fn validate_age(age: i32) -> Result<()> {
    if age < 0 {
        bail!("年龄不能为负数");
    }
    if age > 150 {
        bail!("年龄 {} 超出合理范围", age);
    }
    Ok(())
}
```

**等价于**：
```rust
return Err(anyhow::anyhow!("年龄不能为负数"));
```

### ensure! 宏

**作用**：条件断言，失败时返回错误

```rust
use anyhow::{ensure, Result};

fn divide(a: f64, b: f64) -> Result<f64> {
    ensure!(b != 0.0, "除数不能为零");
    Ok(a / b)
}

fn validate_username(name: &str) -> Result<()> {
    ensure!(!name.is_empty(), "用户名不能为空");
    ensure!(name.len() >= 3, "用户名至少 3 个字符");
    ensure!(name.len() <= 20, "用户名最多 20 个字符");
    Ok(())
}
```

**等价于**：
```rust
if b == 0.0 {
    bail!("除数不能为零");
}
```

### anyhow! 宏

**作用**：创建错误对象（不立即返回）

```rust
use anyhow::{anyhow, Result};

fn process(value: i32) -> Result<()> {
    if value < 0 {
        return Err(anyhow!("值不能为负数: {}", value));
    }
    Ok(())
}

// 存储错误
fn collect_errors() -> Vec<anyhow::Error> {
    vec![
        anyhow!("第一个错误"),
        anyhow!("第二个错误"),
    ]
}
```

---

## Context - 错误上下文

### 为什么需要 Context？

```rust
// ❌ 不好 - 错误信息不明确
fn load_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
// 错误: No such file or directory

// ✅ 好 - 清晰的错误上下文
fn load_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config file")?;
    
    Ok(config)
}
// 错误: Failed to read config file: No such file or directory
```

### context() vs with_context()

#### context() - 立即求值

```rust
use anyhow::{Context, Result};

fn read_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .context("Failed to read file")  // 立即创建字符串
        //       ^^^^^^^^^^^^^^^^^^^^^^  总是分配内存
}
```

**适用场景**：
- 静态字符串
- 简单的错误消息

#### with_context() - 惰性求值

```rust
use anyhow::{Context, Result};

fn read_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))
        //            ^^                                           ^^
        //            闭包 - 仅在错误发生时执行
}
```

**适用场景**：
- 动态构建的消息
- 需要格式化的字符串
- 包含计算的上下文

**性能对比**：

| 方法 | 成功时 | 失败时 |
|------|--------|--------|
| `context()` | 分配字符串 | 返回错误 |
| `with_context()` | 零开销 | 分配字符串并返回 |

**建议**：优先使用 `with_context()` 处理动态消息

---

## 错误链与回溯

### 错误链

多层 `context` 会形成**错误链**，记录错误的完整传播路径。

```rust
use anyhow::{Context, Result};

fn level_3() -> Result<()> {
    std::fs::read_to_string("missing.txt")
        .context("Level 3: File read failed")?;
    Ok(())
}

fn level_2() -> Result<()> {
    level_3().context("Level 2: Config loading failed")?;
    Ok(())
}

fn level_1() -> Result<()> {
    level_2().context("Level 1: Application initialization failed")?;
    Ok(())
}

fn main() {
    if let Err(e) = level_1() {
        // 打印错误链
        eprintln!("Error: {:?}", e);
        
        // 遍历错误链
        for (i, cause) in e.chain().enumerate() {
            eprintln!("  {}: {}", i, cause);
        }
    }
}
```

**输出**：
```
Error: Level 1: Application initialization failed

Caused by:
    0: Level 1: Application initialization failed
    1: Level 2: Config loading failed
    2: Level 3: File read failed
    3: No such file or directory (os error 2)
```

### 访问根本原因

```rust
use std::io;
use anyhow::Result;

fn analyze_error(result: Result<()>) {
    if let Err(e) = result {
        // 获取最底层的错误
        if let Some(io_err) = e.root_cause().downcast_ref::<io::Error>() {
            match io_err.kind() {
                io::ErrorKind::NotFound => eprintln!("文件不存在"),
                io::ErrorKind::PermissionDenied => eprintln!("权限不足"),
                _ => eprintln!("其他 IO 错误"),
            }
        }
    }
}
```

### 回溯 (Backtrace)

启用回溯功能：

```bash
# 设置环境变量
export RUST_BACKTRACE=1

# 运行程序
cargo run
```

在代码中访问回溯：

```rust
use anyhow::Result;

fn main() -> Result<()> {
    if let Err(e) = risky_operation() {
        eprintln!("Error: {}", e);
        if let Some(backtrace) = e.backtrace() {
            eprintln!("Backtrace:\n{}", backtrace);
        }
    }
    Ok(())
}
```

---

## 实战案例

### 案例 1: 配置文件加载器

```rust
use anyhow::{Context, Result, ensure};
use std::fs;

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u64,
}

fn load_config(path: &str) -> Result<Config> {
    // 读取文件
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path))?;
    
    // 解析配置
    let mut host = None;
    let mut port = None;
    let mut timeout = None;
    
    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split('=').collect();
        ensure!(
            parts.len() == 2,
            "Invalid format at line {}: {}",
            line_num + 1,
            line
        );
        
        let key = parts[0].trim();
        let value = parts[1].trim();
        
        match key {
            "host" => host = Some(value.to_string()),
            "port" => {
                port = Some(value.parse()
                    .with_context(|| format!("Invalid port: {}", value))?);
            }
            "timeout" => {
                timeout = Some(value.parse()
                    .with_context(|| format!("Invalid timeout: {}", value))?);
            }
            _ => anyhow::bail!("Unknown config key: {}", key),
        }
    }
    
    // 验证必需字段
    let host = host.context("Missing required field: host")?;
    let port = port.context("Missing required field: port")?;
    let timeout = timeout.unwrap_or(30);
    
    Ok(Config { host, port, timeout })
}
```

### 案例 2: CLI 工具

```rust
use anyhow::{bail, ensure, Context, Result};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    ensure!(args.len() >= 2, "Usage: {} <command> [args...]", args[0]);
    
    let command = &args[1];
    let cmd_args = &args[2..];
    
    match command.as_str() {
        "add" => cmd_add(cmd_args)?,
        "sub" => cmd_sub(cmd_args)?,
        "mul" => cmd_mul(cmd_args)?,
        "div" => cmd_div(cmd_args)?,
        _ => bail!("Unknown command: {}", command),
    }
    
    Ok(())
}

fn cmd_add(args: &[String]) -> Result<()> {
    ensure!(args.len() == 2, "add requires exactly 2 arguments");
    
    let a: i32 = args[0].parse()
        .context("First argument must be an integer")?;
    let b: i32 = args[1].parse()
        .context("Second argument must be an integer")?;
    
    println!("Result: {}", a + b);
    Ok(())
}

fn cmd_div(args: &[String]) -> Result<()> {
    ensure!(args.len() == 2, "div requires exactly 2 arguments");
    
    let a: f64 = args[0].parse()
        .context("First argument must be a number")?;
    let b: f64 = args[1].parse()
        .context("Second argument must be a number")?;
    
    ensure!(b != 0.0, "Division by zero");
    
    println!("Result: {}", a / b);
    Ok(())
}
```

### 案例 3: 数据处理管道

```rust
use anyhow::{Context, Result, ensure};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Record {
    id: u32,
    name: String,
    score: f64,
}

fn parse_csv_line(line: &str, line_num: usize) -> Result<Record> {
    let parts: Vec<&str> = line.split(',').collect();
    
    ensure!(
        parts.len() == 3,
        "Line {}: expected 3 columns, got {}",
        line_num,
        parts.len()
    );
    
    let id: u32 = parts[0].trim().parse()
        .with_context(|| format!("Line {}: invalid ID '{}'", line_num, parts[0]))?;
    
    let name = parts[1].trim();
    ensure!(!name.is_empty(), "Line {}: name cannot be empty", line_num);
    
    let score: f64 = parts[2].trim().parse()
        .with_context(|| format!("Line {}: invalid score '{}'", line_num, parts[2]))?;
    
    ensure!(
        (0.0..=100.0).contains(&score),
        "Line {}: score {} out of range [0, 100]",
        line_num,
        score
    );
    
    Ok(Record {
        id,
        name: name.to_string(),
        score,
    })
}

fn process_csv_file(path: &str) -> Result<Vec<Record>> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open file: {}", path))?;
    
    let reader = BufReader::new(file);
    let mut records = Vec::new();
    
    for (i, line) in reader.lines().enumerate() {
        let line = line.context("Failed to read line")?;
        let line = line.trim();
        
        if line.is_empty() {
            continue;
        }
        
        let record = parse_csv_line(line, i + 1)
            .context("CSV parsing failed")?;
        records.push(record);
    }
    
    Ok(records)
}
```

---

## Anyhow vs Thiserror

### 使用场景

| 特性 | Anyhow | Thiserror |
|------|--------|-----------|
| **目标** | 应用程序 | 库 |
| **错误类型** | 统一 (`anyhow::Error`) | 自定义枚举 |
| **类型转换** | 自动 | 手动（通过 `From`） |
| **上下文** | 内置 (`Context`) | 需要手动实现 |
| **错误匹配** | 不支持 | 支持（模式匹配） |
| **公共 API** | 不推荐 | 推荐 |

### 代码对比

#### Anyhow (应用程序)

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = load_config("app.conf")?;
    run_server(config)?;
    Ok(())
}

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read config")?;
    
    let config = parse_config(&content)
        .context("Failed to parse config")?;
    
    Ok(config)
}
```

#### Thiserror (库)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file")]
    ReadError(#[from] std::io::Error),
    
    #[error("Failed to parse config")]
    ParseError(#[from] toml::de::Error),
    
    #[error("Invalid port: {0}")]
    InvalidPort(u16),
}

pub fn load_config(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    
    if config.port == 0 {
        return Err(ConfigError::InvalidPort(config.port));
    }
    
    Ok(config)
}

// 用户可以匹配具体的错误类型
match load_config("app.conf") {
    Err(ConfigError::InvalidPort(port)) => {
        eprintln!("Invalid port: {}", port);
    }
    Err(e) => eprintln!("Other error: {}", e),
    Ok(config) => { /* ... */ }
}
```

### 选择指南

```
你在开发什么？
├─ 应用程序（CLI、服务器、脚本）
│  └─ 使用 Anyhow
│     - 简化错误处理
│     - 快速开发
│     - 统一错误类型
│
└─ 库（供他人使用）
   └─ 使用 Thiserror
      - 精确的错误类型
      - 可模式匹配
      - 清晰的 API
```

---

## 最佳实践

### 1. 总是添加 Context

```rust
// ❌ 不好 - 错误信息不明确
fn bad() -> Result<()> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(())
}

// ✅ 好 - 清晰的错误上下文
fn good() -> Result<()> {
    let content = std::fs::read_to_string("file.txt")
        .context("Failed to read configuration file")?;
    Ok(())
}
```

### 2. 使用 with_context 处理动态信息

```rust
// ❌ 不好 - 总是分配内存
fn bad(path: &str) -> Result<()> {
    std::fs::read_to_string(path)
        .context(format!("Failed to read: {}", path))?;
    Ok(())
}

// ✅ 好 - 仅在错误时分配
fn good(path: &str) -> Result<()> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read: {}", path))?;
    Ok(())
}
```

### 3. 优先使用 ensure! 而不是 if + bail!

```rust
// ❌ 不好 - 冗长
fn bad(x: i32) -> Result<()> {
    if x <= 0 {
        bail!("x must be positive");
    }
    Ok(())
}

// ✅ 好 - 简洁
fn good(x: i32) -> Result<()> {
    ensure!(x > 0, "x must be positive");
    Ok(())
}
```

### 4. 构建清晰的错误链

```rust
// ✅ 好 - 多层上下文
fn process() -> Result<()> {
    load_database()
        .context("Failed to load database")?;
    
    validate_data()
        .context("Data validation failed")?;
    
    save_results()
        .context("Failed to save results")?;
    
    Ok(())
}
```

### 5. main 函数返回 Result

```rust
use anyhow::Result;

// ✅ 好 - 错误自动打印
fn main() -> Result<()> {
    run_app()?;
    Ok(())
}

// ❌ 不好 - 需要手动处理
fn main() {
    if let Err(e) = run_app() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

### 6. 不要过度使用 unwrap

```rust
// ❌ 不好 - 可能 panic
fn bad() -> Result<()> {
    let num = "123".parse::<i32>().unwrap();
    Ok(())
}

// ✅ 好 - 优雅地处理错误
fn good() -> Result<()> {
    let num: i32 = "123".parse()
        .context("Failed to parse number")?;
    Ok(())
}
```

---

## 常见问题

### Q1: Anyhow 适合库开发吗？

**A**: ❌ 不推荐

原因：
- 库用户无法匹配具体的错误类型
- 失去了类型安全的优势
- 推荐使用 `thiserror` 定义明确的错误类型

### Q2: context() 和 with_context() 有什么区别？

**A**: 性能差异

| 方法 | 求值时机 | 适用场景 |
|------|----------|----------|
| `context()` | 立即 | 静态字符串 |
| `with_context()` | 惰性 | 动态消息 |

```rust
// context - 总是创建字符串
.context(format!("Error: {}", x))  // 即使成功也会分配

// with_context - 仅在错误时创建
.with_context(|| format!("Error: {}", x))  // 成功时零开销
```

### Q3: 如何访问底层的具体错误类型？

**A**: 使用 `downcast_ref()`

```rust
use std::io;

if let Err(e) = operation() {
    if let Some(io_err) = e.downcast_ref::<io::Error>() {
        // 处理 io::Error
        match io_err.kind() {
            io::ErrorKind::NotFound => { /* ... */ }
            _ => { /* ... */ }
        }
    }
}
```

### Q4: 如何在库和应用中共用代码？

**A**: 库用 thiserror，应用层转换为 anyhow

```rust
// 库代码 (mylib)
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibError {
    #[error("Something went wrong")]
    SomeError,
}

pub fn lib_function() -> Result<(), LibError> {
    // ...
}

// 应用代码
use anyhow::Result;

fn app_function() -> Result<()> {
    // LibError 自动转换为 anyhow::Error
    mylib::lib_function()?;
    Ok(())
}
```

### Q5: bail! 和 return Err 有什么区别？

**A**: 只是语法糖

```rust
// 这两种写法完全等价
bail!("error message");
return Err(anyhow!("error message"));

// bail! 更简洁，是推荐写法
```

### Q6: 如何打印错误的完整链？

**A**: 使用 `chain()` 迭代器

```rust
if let Err(e) = operation() {
    eprintln!("Error: {}", e);
    
    eprintln!("\nCaused by:");
    for (i, cause) in e.chain().skip(1).enumerate() {
        eprintln!("  {}: {}", i, cause);
    }
}
```

### Q7: Anyhow 有运行时开销吗？

**A**: ❌ 几乎没有

- 错误路径（Error path）的开销可以忽略
- 成功路径（Happy path）零开销
- `with_context` 使用闭包避免不必要的分配

---

## 快速参考

### 导入

```rust
use anyhow::{
    anyhow,      // 创建错误对象
    bail,        // 立即返回错误
    ensure,      // 条件断言
    Context,     // 添加上下文 trait
    Result,      // Result<T, anyhow::Error>
};
```

### 常用模式

#### 返回错误

```rust
// 立即返回
bail!("error message");

// 条件返回
ensure!(condition, "error message");

// 创建并返回
return Err(anyhow!("error message"));
```

#### 添加上下文

```rust
// 静态消息
operation()?.context("Failed to do something")?;

// 动态消息
operation()?.with_context(|| format!("Failed: {}", detail))?;
```

#### 错误链

```rust
// 遍历错误链
for cause in error.chain() {
    eprintln!("{}", cause);
}

// 获取根本原因
let root = error.root_cause();
```

---

## 总结

### Anyhow 的核心价值

1. **简化** - 统一的错误类型，简化函数签名
2. **灵活** - 自动转换任何错误类型
3. **信息丰富** - 错误上下文和完整的错误链
4. **零成本** - 成功路径无性能开销
5. **开发友好** - 清晰的错误信息，易于调试

### 何时使用

✅ **适合**:
- 应用程序开发
- CLI 工具
- 服务器应用
- 快速原型
- 脚本和工具

❌ **不适合**:
- 库开发（用 thiserror）
- 需要精确错误类型匹配
- 公共 API
- 需要细粒度错误处理

### 关键要点

1. 总是使用 `context` 或 `with_context` 添加错误上下文
2. 优先使用 `with_context` 处理动态消息（性能更好）
3. 使用 `ensure!` 而不是 `if + bail!`
4. `main` 函数返回 `Result<()>` 自动处理错误
5. 构建清晰的错误链，便于调试

---

**运行示例代码**:
```bash
cargo run --bin anyhow_detailed
```

**官方文档**: https://docs.rs/anyhow/
