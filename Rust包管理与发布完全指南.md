# Rust 包管理与发布完全指南

## 目录
- [1. Cargo 包管理深入](#1-cargo-包管理深入)
- [2. 依赖管理](#2-依赖管理)
- [3. Workspace 工作空间](#3-workspace-工作空间)
- [4. 发布到 crates.io](#4-发布到-cratesio)
- [5. 最佳实践](#5-最佳实践)
- [6. 实战案例](#6-实战案例)

---

## 1. Cargo 包管理深入

### 1.1 Cargo.toml 详解

```toml
# ============================================
# 包元数据（Package Metadata）
# ============================================
[package]
name = "my-awesome-crate"              # 包名（必需）
version = "0.1.0"                      # 版本（必需，遵循语义化版本）
authors = ["Your Name <you@example.com>"]
edition = "2021"                       # Rust 版本（2015/2018/2021）
rust-version = "1.70"                  # 最小 Rust 版本（MSRV）
description = "A short description"    # 简短描述
documentation = "https://docs.rs/my-awesome-crate"
homepage = "https://example.com"
repository = "https://github.com/user/repo"
readme = "README.md"
license = "MIT"                        # 或 "Apache-2.0"，或 "MIT OR Apache-2.0"
license-file = "LICENSE"               # 自定义协议文件
keywords = ["cli", "tool", "utility"]  # 最多 5 个
categories = ["command-line-utilities"] # 从 crates.io 类别列表选择
publish = true                         # 是否允许发布到 crates.io

# ============================================
# 依赖项（Dependencies）
# ============================================
[dependencies]
# 基本依赖
serde = "1.0"

# 带功能特性的依赖
tokio = { version = "1.35", features = ["full"] }

# 可选依赖（用于条件编译）
redis = { version = "0.24", optional = true }

# Git 依赖
my-lib = { git = "https://github.com/user/my-lib" }
my-lib-branch = { git = "https://github.com/user/my-lib", branch = "dev" }
my-lib-tag = { git = "https://github.com/user/my-lib", tag = "v1.0.0" }
my-lib-rev = { git = "https://github.com/user/my-lib", rev = "abc123" }

# 本地路径依赖
local-crate = { path = "../local-crate" }

# 重命名依赖
actix-web-old = { package = "actix-web", version = "3.0" }

# 平台特定依赖
[target.'cfg(windows)'.dependencies]
winapi = "0.3"

[target.'cfg(unix)'.dependencies]
libc = "0.2"

# 开发依赖（仅用于测试、示例、基准测试）
[dev-dependencies]
criterion = "0.5"
mockall = "0.11"

# 构建依赖（用于 build.rs）
[build-dependencies]
cc = "1.0"

# ============================================
# 功能特性（Features）
# ============================================
[features]
default = ["std"]                      # 默认启用的特性
std = []                               # 启用标准库
full = ["redis", "cache", "async"]    # 聚合特性
redis = ["dep:redis"]                  # 启用可选依赖
cache = []
async = ["tokio"]

# ============================================
# 构建配置（Profile）
# ============================================
[profile.dev]
opt-level = 0                          # 优化级别 0-3
debug = true                           # 是否包含调试信息
split-debuginfo = "unpacked"          # 调试信息格式
strip = false                          # 是否剥离符号
debug-assertions = true                # 是否启用调试断言
overflow-checks = true                 # 是否检查整数溢出
lto = false                           # 链接时优化
panic = 'unwind'                      # panic 策略：unwind 或 abort
incremental = true                     # 增量编译
codegen-units = 256                   # 代码生成单元数

[profile.release]
opt-level = 3                          # 最高优化
lto = true                            # 启用链接时优化
codegen-units = 1                     # 单个代码生成单元（更好的优化）
strip = true                          # 剥离符号（减小二进制大小）
panic = 'abort'                       # abort 策略（更小的二进制）

# 自定义配置
[profile.release-with-debug]
inherits = "release"
debug = true

# ============================================
# 二进制目标（Binary Targets）
# ============================================
[[bin]]
name = "my-cli"                       # 二进制名称
path = "src/bin/my-cli.rs"           # 源文件路径

[[bin]]
name = "my-server"
path = "src/bin/server.rs"

# ============================================
# 库目标（Library Target）
# ============================================
[lib]
name = "my_lib"                       # 库名称（默认为包名）
path = "src/lib.rs"                   # 源文件路径
crate-type = ["lib"]                  # 或 ["dylib", "staticlib", "cdylib"]

# ============================================
# 示例（Examples）
# ============================================
[[example]]
name = "basic"
path = "examples/basic.rs"

# ============================================
# 测试（Tests）
# ============================================
[[test]]
name = "integration"
path = "tests/integration_test.rs"

# ============================================
# 基准测试（Benchmarks）
# ============================================
[[bench]]
name = "my_benchmark"
path = "benches/my_benchmark.rs"
harness = false                       # 使用自定义基准框架

# ============================================
# 包含/排除文件
# ============================================
include = [
    "src/**/*",
    "Cargo.toml",
    "README.md",
    "LICENSE",
]

exclude = [
    ".github/**",
    "tests/**",
    "benches/**",
]

# ============================================
# 元数据（Metadata）
# ============================================
[package.metadata.docs.rs]
all-features = true                   # 在 docs.rs 上构建所有特性
rustdoc-args = ["--cfg", "docsrs"]   # 传递给 rustdoc 的参数

[package.metadata.release]
sign-commit = true
sign-tag = true
pre-release-commit-message = "Release {{version}}"
```

### 1.2 Cargo 命令详解

```bash
# ============================================
# 项目管理
# ============================================

# 创建新项目
cargo new my-project                  # 创建二进制项目
cargo new my-lib --lib               # 创建库项目
cargo new my-project --vcs git       # 指定版本控制系统
cargo new my-project --name my_name  # 指定包名

# 初始化现有目录
cargo init                           # 当前目录
cargo init --lib                     # 库项目

# ============================================
# 构建和编译
# ============================================

# 基本构建
cargo build                          # 调试构建
cargo build --release                # 发布构建
cargo build --target x86_64-pc-windows-gnu  # 交叉编译

# 检查（不生成二进制）
cargo check                          # 快速检查（推荐开发时使用）
cargo check --all-targets           # 检查所有目标

# 清理
cargo clean                          # 清理 target 目录
cargo clean --release               # 只清理 release

# ============================================
# 运行和测试
# ============================================

# 运行
cargo run                            # 运行默认二进制
cargo run --bin my-cli              # 运行指定二进制
cargo run --example basic           # 运行示例
cargo run --release                 # 运行发布版本
cargo run -- arg1 arg2              # 传递参数

# 测试
cargo test                          # 运行所有测试
cargo test test_name                # 运行特定测试
cargo test --lib                    # 只测试库
cargo test --test integration       # 运行特定集成测试
cargo test -- --nocapture          # 显示输出
cargo test -- --test-threads=1     # 单线程运行

# 基准测试
cargo bench                         # 运行基准测试
cargo bench bench_name             # 运行特定基准测试

# ============================================
# 文档
# ============================================

# 生成文档
cargo doc                           # 生成文档
cargo doc --open                    # 生成并打开文档
cargo doc --no-deps                # 不包含依赖文档
cargo doc --document-private-items # 包含私有项

# ============================================
# 依赖管理
# ============================================

# 更新依赖
cargo update                        # 更新所有依赖
cargo update -p serde              # 更新特定依赖

# 查看依赖树
cargo tree                          # 显示依赖树
cargo tree -i serde                # 显示反向依赖
cargo tree --depth 1               # 限制深度

# 搜索包
cargo search keyword                # 在 crates.io 搜索

# 添加依赖（需要 cargo-edit）
cargo add serde                     # 添加依赖
cargo add tokio -F full            # 添加带特性的依赖
cargo add --dev criterion          # 添加开发依赖

# ============================================
# 发布相关
# ============================================

# 登录 crates.io
cargo login <token>

# 打包
cargo package                       # 打包准备发布
cargo package --list               # 列出将要打包的文件
cargo package --allow-dirty        # 允许有未提交的更改

# 发布
cargo publish                       # 发布到 crates.io
cargo publish --dry-run            # 预演发布
cargo yank --vers 0.1.0           # 撤销版本

# ============================================
# 工具链管理（使用 rustup）
# ============================================

# 安装工具链
rustup install stable
rustup install nightly
rustup install 1.70.0

# 设置默认工具链
rustup default stable
rustup default nightly

# 更新
rustup update

# 组件管理
rustup component add rustfmt       # 代码格式化
rustup component add clippy        # 代码检查
rustup component add rust-src      # 源代码
rustup component add rust-analyzer # LSP

# 目标平台
rustup target add x86_64-pc-windows-gnu
rustup target list                 # 列出所有目标

# ============================================
# 格式化和检查
# ============================================

# 格式化
cargo fmt                          # 格式化代码
cargo fmt -- --check              # 检查格式（CI 用）

# Clippy 检查
cargo clippy                       # 运行 clippy
cargo clippy -- -D warnings       # 将警告视为错误
cargo clippy --fix                # 自动修复

# ============================================
# 其他工具
# ============================================

# 审计依赖安全性
cargo audit                        # 需要 cargo-audit

# 查看过时的依赖
cargo outdated                     # 需要 cargo-outdated

# 展开宏
cargo expand                       # 需要 cargo-expand

# 生成火焰图
cargo flamegraph                   # 需要 cargo-flamegraph

# 查看二进制大小
cargo bloat --release             # 需要 cargo-bloat

# 未使用的依赖
cargo udeps                        # 需要 cargo-udeps
```

---

## 2. 依赖管理

### 2.1 版本指定

```toml
[dependencies]
# 精确版本
serde = "=1.0.193"

# 插入符号要求（默认）
serde = "1.0"          # 等同于 "^1.0.0"，匹配 >=1.0.0, <2.0.0
serde = "^1.2.3"       # 匹配 >=1.2.3, <2.0.0

# 波浪号要求
serde = "~1.2.3"       # 匹配 >=1.2.3, <1.3.0
serde = "~1.2"         # 匹配 >=1.2.0, <1.3.0

# 通配符
serde = "1.*"          # 匹配 >=1.0.0, <2.0.0
serde = "1.2.*"        # 匹配 >=1.2.0, <1.3.0

# 比较要求
serde = ">= 1.0"
serde = "> 1.0, < 2.0"
serde = ">= 1.0, < 2.0"

# 多个要求
serde = ">= 1.0, < 1.5"
```

### 2.2 特性管理

```toml
[features]
# 默认特性
default = ["std", "serde"]

# 基本特性
std = []
serde = ["dep:serde", "dep:serde_json"]
async = ["tokio"]

# 特性组合
full = ["std", "serde", "async", "redis"]

# 可选依赖作为特性
redis = ["dep:redis"]
postgres = ["dep:sqlx/postgres"]

[dependencies]
# 可选依赖
serde = { version = "1.0", optional = true }
serde_json = { version = "1.0", optional = true }
tokio = { version = "1.0", optional = true }
redis = { version = "0.24", optional = true }

# 依赖的特性
sqlx = { version = "0.7", optional = true, default-features = false }

# 使用示例
# cargo build --features serde
# cargo build --features "serde async"
# cargo build --all-features
# cargo build --no-default-features
```

### 2.3 依赖覆盖

```toml
# 替换所有使用 serde 的版本
[patch.crates-io]
serde = { path = "../my-serde" }

# 替换 Git 依赖
[patch.'https://github.com/user/repo']
my-lib = { path = "../my-lib" }

# 使用本地注册表
[source.my-vendor-source]
directory = "vendor"

[source.crates-io]
replace-with = "my-vendor-source"
```

---

## 3. Workspace 工作空间

### 3.1 创建 Workspace

```toml
# 根目录 Cargo.toml
[workspace]
members = [
    "crates/core",
    "crates/cli",
    "crates/web",
]

# 排除某些目录
exclude = [
    "examples",
    "benchmarks",
]

# 工作空间级别的依赖
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }

# 工作空间级别的元数据
[workspace.package]
authors = ["Your Name <you@example.com>"]
edition = "2021"
license = "MIT"

# 工作空间级别的配置
[workspace.profile.release]
opt-level = 3
lto = true
```

### 3.2 成员包使用共享依赖

```toml
# crates/core/Cargo.toml
[package]
name = "my-core"
version = "0.1.0"
edition.workspace = true
license.workspace = true
authors.workspace = true

[dependencies]
# 使用工作空间依赖
serde.workspace = true
tokio.workspace = true

# 成员间依赖
my-utils = { path = "../utils" }
```

### 3.3 Workspace 结构示例

```
my-workspace/
├── Cargo.toml              # Workspace 根
├── Cargo.lock              # 锁定文件（整个 workspace 共享）
├── crates/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── cli/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── web/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
└── target/                 # 整个 workspace 共享
```

### 3.4 Workspace 命令

```bash
# 构建所有成员
cargo build --workspace
cargo build --all

# 测试所有成员
cargo test --workspace

# 运行特定成员
cargo run -p my-cli

# 发布特定成员
cargo publish -p my-core

# 更新工作空间依赖
cargo update --workspace
```

---

## 4. 发布到 crates.io

### 4.1 准备发布

#### 步骤 1：完善 Cargo.toml

```toml
[package]
name = "my-awesome-crate"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2021"
description = "一个很棒的 Rust 库"  # 必需
license = "MIT OR Apache-2.0"      # 推荐
readme = "README.md"
repository = "https://github.com/user/my-awesome-crate"
keywords = ["cli", "tool"]          # 最多 5 个
categories = ["command-line-utilities"]

[dependencies]
# 确保版本号正确
```

#### 步骤 2：编写文档

```rust
//! # My Awesome Crate
//!
//! 这是一个很棒的库，用于...
//!
//! ## 快速开始
//!
//! ```
//! use my_awesome_crate::MyStruct;
//!
//! let instance = MyStruct::new();
//! assert_eq!(instance.value(), 42);
//! ```
//!
//! ## 特性
//!
//! - 特性 1
//! - 特性 2

/// 主要的结构体
///
/// # 示例
///
/// ```
/// use my_awesome_crate::MyStruct;
///
/// let instance = MyStruct::new();
/// ```
pub struct MyStruct {
    value: i32,
}

impl MyStruct {
    /// 创建一个新实例
    ///
    /// # 示例
    ///
    /// ```
    /// use my_awesome_crate::MyStruct;
    ///
    /// let instance = MyStruct::new();
    /// assert_eq!(instance.value(), 42);
    /// ```
    pub fn new() -> Self {
        Self { value: 42 }
    }
    
    /// 获取值
    pub fn value(&self) -> i32 {
        self.value
    }
}
```

#### 步骤 3：编写 README.md

```markdown
# My Awesome Crate

[![Crates.io](https://img.shields.io/crates/v/my-awesome-crate.svg)](https://crates.io/crates/my-awesome-crate)
[![Documentation](https://docs.rs/my-awesome-crate/badge.svg)](https://docs.rs/my-awesome-crate)
[![License](https://img.shields.io/crates/l/my-awesome-crate.svg)](LICENSE)

一个很棒的 Rust 库。

## 安装

```toml
[dependencies]
my-awesome-crate = "0.1"
```

## 使用示例

```rust
use my_awesome_crate::MyStruct;

fn main() {
    let instance = MyStruct::new();
    println!("Value: {}", instance.value());
}
```

## 特性

- 特性 1
- 特性 2

## 许可证

MIT OR Apache-2.0
```

#### 步骤 4：选择许可证

创建 `LICENSE-MIT` 和 `LICENSE-APACHE` 文件，或选择其中之一。

### 4.2 发布流程

```bash
# 1. 确保代码已提交
git add .
git commit -m "Prepare for release v0.1.0"
git push

# 2. 登录 crates.io（获取 token：https://crates.io/settings/tokens）
cargo login <your-token>

# 3. 检查包内容
cargo package --list

# 4. 本地打包测试
cargo package

# 5. 预演发布
cargo publish --dry-run

# 6. 正式发布
cargo publish

# 7. 打标签
git tag v0.1.0
git push origin v0.1.0
```

### 4.3 版本更新

```bash
# 更新版本号（语义化版本）
# 0.1.0 -> 0.1.1  (补丁版本，向后兼容的 bug 修复)
# 0.1.0 -> 0.2.0  (次版本，向后兼容的新功能)
# 0.1.0 -> 1.0.0  (主版本，不兼容的 API 更改)

# 手动编辑 Cargo.toml 中的 version
# 或使用 cargo-release
cargo install cargo-release
cargo release patch  # 0.1.0 -> 0.1.1
cargo release minor  # 0.1.0 -> 0.2.0
cargo release major  # 0.1.0 -> 1.0.0
```

### 4.4 撤销版本

```bash
# 撤销（不删除，但不推荐使用）
cargo yank --vers 0.1.0

# 取消撤销
cargo yank --vers 0.1.0 --undo
```

---

## 5. 最佳实践

### 5.1 项目结构

```
my-crate/
├── Cargo.toml
├── Cargo.lock          # 库项目不提交，二进制项目提交
├── README.md
├── LICENSE
├── .gitignore
├── .github/
│   └── workflows/
│       └── ci.yml
├── src/
│   ├── lib.rs          # 库入口
│   ├── main.rs         # 二进制入口（可选）
│   ├── bin/            # 额外的二进制
│   │   └── tool.rs
│   └── module/
│       ├── mod.rs
│       └── submodule.rs
├── tests/              # 集成测试
│   └── integration_test.rs
├── benches/            # 基准测试
│   └── benchmark.rs
├── examples/           # 示例
│   └── example.rs
└── docs/               # 额外文档
    └── guide.md
```

### 5.2 .gitignore

```gitignore
# Rust
/target/
Cargo.lock  # 对于库项目

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db
```

### 5.3 CI/CD 示例（GitHub Actions）

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        components: rustfmt, clippy
    
    - name: Check format
      run: cargo fmt -- --check
    
    - name: Clippy
      run: cargo clippy -- -D warnings
    
    - name: Build
      run: cargo build --verbose
    
    - name: Run tests
      run: cargo test --verbose
    
    - name: Build docs
      run: cargo doc --no-deps

  coverage:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Install tarpaulin
      run: cargo install cargo-tarpaulin
    - name: Generate coverage
      run: cargo tarpaulin --out Xml
    - name: Upload coverage
      uses: codecov/codecov-action@v3
```

### 5.4 文档最佳实践

```rust
//! 模块级文档
//!
//! 使用 `//!` 作为模块或 crate 级文档注释

/// 公共 API 必须有文档注释
///
/// # 参数
///
/// * `value` - 输入值
///
/// # 返回值
///
/// 处理后的结果
///
/// # 示例
///
/// ```
/// use my_crate::process;
///
/// let result = process(42);
/// assert_eq!(result, 84);
/// ```
///
/// # Panics
///
/// 当输入为负数时会 panic
///
/// # Errors
///
/// 如果值过大会返回错误
///
/// # Safety
///
/// 这个函数是安全的，因为...
pub fn process(value: i32) -> i32 {
    value * 2
}
```

### 5.5 版本管理策略

```toml
# 语义化版本：MAJOR.MINOR.PATCH

# 0.x.y 阶段（初期开发）
# - 任何更改都可能破坏兼容性
# - 0.0.x: 快速迭代
# - 0.1.x -> 0.2.x: 可能有破坏性更改

# 1.0.0 发布标准
# - API 稳定
# - 文档完整
# - 测试覆盖充分
# - 已在生产环境使用

# 1.x.y 阶段
# - PATCH: 向后兼容的 bug 修复
# - MINOR: 向后兼容的新功能
# - MAJOR: 不兼容的 API 更改
```

---

## 6. 实战案例

### 6.1 创建一个 CLI 工具库

```bash
# 创建项目
cargo new my-cli-tool --bin

cd my-cli-tool
```

```toml
# Cargo.toml
[package]
name = "my-cli-tool"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A simple CLI tool"
license = "MIT"
repository = "https://github.com/user/my-cli-tool"
keywords = ["cli", "tool"]
categories = ["command-line-utilities"]

[dependencies]
clap = { version = "4.4", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```

```rust
// src/main.rs
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "my-cli-tool")]
#[command(about = "A simple CLI tool", long_about = None)]
struct Cli {
    /// Input file
    #[arg(short, long)]
    input: String,
    
    /// Output file
    #[arg(short, long)]
    output: Option<String>,
    
    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    if cli.verbose {
        println!("Processing: {}", cli.input);
    }
    
    // 处理逻辑
    
    Ok(())
}
```

### 6.2 发布完整流程

```bash
# 1. 完善文档
cargo doc --open

# 2. 运行测试
cargo test

# 3. 运行 clippy
cargo clippy -- -D warnings

# 4. 格式化代码
cargo fmt

# 5. 检查包内容
cargo package --list

# 6. 本地测试安装
cargo install --path .

# 7. 预演发布
cargo publish --dry-run

# 8. 提交代码
git add .
git commit -m "Release v0.1.0"
git tag v0.1.0
git push origin main --tags

# 9. 发布
cargo publish
```

---

## 总结

本指南涵盖了 Rust 包管理和发布的完整流程：

✅ **Cargo 深入** - 完整的 Cargo.toml 配置  
✅ **依赖管理** - 版本、特性、覆盖  
✅ **Workspace** - 多包项目管理  
✅ **发布流程** - 从准备到发布的完整步骤  
✅ **最佳实践** - 项目结构、CI/CD、文档  
✅ **实战案例** - CLI 工具开发  

**掌握这些内容，你就能熟练管理和发布 Rust 包！** 🚀
