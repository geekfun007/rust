# Rust 包管理与 Python 集成完整指南

这是一个完整的示例项目，展示如何：
1. 开发 Rust 库
2. 发布 Rust 包到 crates.io
3. 使用 PyO3 创建 Python 绑定
4. 在 Python 中安装和使用 Rust 包

## 📋 目录

- [项目结构](#项目结构)
- [环境要求](#环境要求)
- [快速开始](#快速开始)
- [Rust 开发](#rust-开发)
- [Python 绑定](#python-绑定)
- [发布流程](#发布流程)
- [使用示例](#使用示例)
- [性能对比](#性能对比)
- [常见问题](#常见问题)

## 📁 项目结构

```
rust-py-example/
├── Cargo.toml              # Rust 项目配置
├── pyproject.toml          # Python 项目配置
├── LICENSE                 # 许可证
├── README.md              # 项目文档
├── .gitignore             # Git 忽略文件
├── src/                   # Rust 源代码
│   ├── lib.rs            # Rust 核心库
│   └── python_bindings.rs # PyO3 Python 绑定
├── python/                # Python 包代码
│   └── rust_py_example/
│       └── __init__.py   # Python 包初始化
├── examples/              # 示例代码
│   ├── rust_only_example.rs  # Rust 库使用示例
│   └── python_example.py     # Python 使用示例
└── tests/                 # 测试代码
    └── test_python.py    # Python 单元测试
```

## 🔧 环境要求

### Rust 开发
- Rust 1.70+ (安装: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Cargo (随 Rust 一起安装)

### Python 绑定
- Python 3.7+
- pip
- maturin (安装: `pip install maturin`)

### 可选工具
- pytest (测试): `pip install pytest`
- twine (发布): `pip install twine`

## 🚀 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/yourusername/rust-py-example.git
cd rust-py-example
```

### 2. 构建和测试 Rust 库

```bash
# 运行 Rust 测试
cargo test

# 构建 Rust 库
cargo build --release

# 运行 Rust 示例
cargo run --example rust_only_example
```

### 3. 构建和安装 Python 包

```bash
# 开发模式安装（推荐用于开发）
maturin develop --release

# 或者构建 wheel 包
maturin build --release

# 安装构建的包
pip install target/wheels/*.whl
```

### 4. 运行 Python 示例

```bash
python examples/python_example.py
```

### 5. 运行 Python 测试

```bash
pytest tests/test_python.py -v
```

## 🦀 Rust 开发

### Cargo.toml 配置详解

```toml
[package]
name = "rust-py-example"       # 包名
version = "0.1.0"               # 版本号
edition = "2021"                # Rust 版本
authors = ["Your Name <email>"] # 作者信息
description = "..."             # 包描述
license = "MIT"                 # 许可证
repository = "..."              # 代码仓库
keywords = ["python", "rust"]   # 关键词
categories = ["api-bindings"]   # 分类

[lib]
name = "rust_py_example"
crate-type = ["cdylib", "rlib"]  # cdylib 用于动态链接（Python）
                                  # rlib 用于 Rust 静态链接

[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"], optional = true }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[features]
default = []
python = ["pyo3"]  # Python 功能开关
```

### Rust 库功能

本项目实现了以下功能模块：

#### 1. 用户管理 (User)
```rust
let user = User::new("张三".to_string(), 25, "zhangsan@example.com".to_string());
println!("{}", user.description());
println!("是否成年: {}", user.is_adult());
```

#### 2. 数学计算 (math)
```rust
use rust_py_example::math;

// 斐波那契数列
let fib = math::fibonacci(10);  // 55

// 判断质数
let is_prime = math::is_prime(17);  // true

// 阶乘
let fact = math::factorial(5);  // 120
```

#### 3. 字符串处理 (text)
```rust
use rust_py_example::text;

// 反转字符串
let reversed = text::reverse("hello");  // "olleh"

// 统计单词
let count = text::word_count("hello world");  // 2

// 首字母大写
let capitalized = text::capitalize_words("hello world");  // "Hello World"
```

### Rust 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_fibonacci

# 显示输出
cargo test -- --nocapture

# 生成测试覆盖率
cargo tarpaulin --out Html
```

### Rust 文档

```bash
# 生成并打开文档
cargo doc --open

# 只生成文档
cargo doc --no-deps
```

## 🐍 Python 绑定

### PyO3 绑定实现

PyO3 是一个 Rust 库，用于创建 Python 绑定：

```rust
use pyo3::prelude::*;

#[pyclass]
pub struct PyUser {
    #[pyo3(get, set)]
    pub name: String,
    pub age: u32,
}

#[pymethods]
impl PyUser {
    #[new]
    fn new(name: String, age: u32) -> Self {
        PyUser { name, age }
    }
    
    fn description(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

#[pyfunction]
fn fibonacci(n: u32) -> u64 {
    // 实现...
}

#[pymodule]
fn _rust_py_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyUser>()?;
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
```

### Python 使用示例

```python
import rust_py_example as rpe

# 创建用户
user = rpe.PyUser("Alice", 25, "alice@example.com")
print(user.description())
print(user.is_adult())

# 数学计算
print(rpe.fibonacci(10))      # 55
print(rpe.is_prime(17))       # True
print(rpe.factorial(5))       # 120

# 字符串处理
print(rpe.reverse_string("hello"))           # "olleh"
print(rpe.word_count("hello world"))         # 2
print(rpe.capitalize_words("hello world"))   # "Hello World"

# 批量处理
numbers = [0, 1, 2, 5, 10]
print(rpe.fibonacci_batch(numbers))  # [0, 1, 1, 5, 55]

primes = rpe.filter_primes([2, 3, 4, 5, 6, 7, 8, 9])
print(primes)  # [2, 3, 5, 7]
```

### Maturin 使用

Maturin 是一个构建和发布 Rust Python 包的工具：

```bash
# 开发模式（快速迭代）
maturin develop

# 开发模式 + Release 优化
maturin develop --release

# 构建 wheel 包
maturin build --release

# 构建并发布到 PyPI
maturin publish

# 指定 Python 版本
maturin build --release -i python3.9

# 构建多个 Python 版本
maturin build --release -i python3.8 -i python3.9 -i python3.10
```

## 📦 发布流程

### 发布到 crates.io (Rust 包)

1. **注册 crates.io 账号**
   - 访问 https://crates.io/
   - 使用 GitHub 账号登录

2. **获取 API Token**
   ```bash
   # 在 crates.io 网站生成 token
   # 然后运行：
   cargo login <your-api-token>
   ```

3. **准备发布**
   ```bash
   # 检查包配置
   cargo package --list
   
   # 本地构建测试
   cargo package
   
   # 检查构建的包
   cargo package --list
   ```

4. **发布**
   ```bash
   # 发布到 crates.io
   cargo publish
   
   # 如果需要，可以先试运行
   cargo publish --dry-run
   ```

5. **版本管理**
   ```bash
   # 更新版本号（在 Cargo.toml 中）
   version = "0.1.1"
   
   # 重新发布
   cargo publish
   ```

### 发布到 PyPI (Python 包)

1. **注册 PyPI 账号**
   - 访问 https://pypi.org/
   - 注册账号

2. **配置凭证**
   ```bash
   # 创建 ~/.pypirc 文件
   cat > ~/.pypirc << EOF
   [distutils]
   index-servers =
       pypi
       testpypi
   
   [pypi]
   username = __token__
   password = <your-pypi-token>
   
   [testpypi]
   repository = https://test.pypi.org/legacy/
   username = __token__
   password = <your-testpypi-token>
   EOF
   ```

3. **构建包**
   ```bash
   # 构建 wheel 包
   maturin build --release
   
   # 构建多个 Python 版本
   maturin build --release -i python3.8 -i python3.9 -i python3.10 -i python3.11
   ```

4. **测试发布（推荐）**
   ```bash
   # 发布到 TestPyPI
   maturin publish --repository testpypi
   
   # 从 TestPyPI 安装测试
   pip install --index-url https://test.pypi.org/simple/ rust-py-example
   ```

5. **正式发布**
   ```bash
   # 发布到 PyPI
   maturin publish
   
   # 或使用 twine
   twine upload target/wheels/*
   ```

6. **验证安装**
   ```bash
   # 安装你的包
   pip install rust-py-example
   
   # 测试导入
   python -c "import rust_py_example; print(rust_py_example.fibonacci(10))"
   ```

### 版本管理最佳实践

1. **语义化版本控制**
   - MAJOR.MINOR.PATCH (例如: 1.2.3)
   - MAJOR: 不兼容的 API 变更
   - MINOR: 向后兼容的功能新增
   - PATCH: 向后兼容的问题修复

2. **同步版本号**
   ```bash
   # 确保 Cargo.toml 和 pyproject.toml 中的版本号一致
   # Cargo.toml
   version = "0.1.0"
   
   # pyproject.toml
   version = "0.1.0"
   ```

3. **更新日志**
   创建 CHANGELOG.md 记录每个版本的变更：
   ```markdown
   # Changelog
   
   ## [0.1.0] - 2025-12-05
   ### Added
   - 初始版本发布
   - 用户管理功能
   - 数学计算功能
   - 字符串处理功能
   ```

4. **Git 标签**
   ```bash
   # 创建版本标签
   git tag -a v0.1.0 -m "Release version 0.1.0"
   git push origin v0.1.0
   ```

## 📊 性能对比

Rust 相比纯 Python 实现通常有显著的性能提升：

```python
# 运行 examples/python_example.py 查看性能对比
# 典型结果（计算 fibonacci(30) 10000 次）：
# Rust 版本耗时: 0.0234 秒
# Python 版本耗时: 0.2156 秒
# 性能提升: 9.21x
```

### 何时使用 Rust

✅ **适合使用 Rust 的场景：**
- CPU 密集型计算
- 需要高性能的算法
- 需要处理大量数据
- 并发/并行处理
- 系统级编程

❌ **不需要 Rust 的场景：**
- IO 密集型操作（网络、文件）
- 简单的业务逻辑
- 原型快速开发
- 团队不熟悉 Rust

## 🛠️ 开发工作流

### 1. 修改 Rust 代码

```bash
# 编辑 src/ 下的代码
vim src/lib.rs

# 运行 Rust 测试
cargo test

# 重新构建 Python 包
maturin develop --release
```

### 2. 修改 Python 绑定

```bash
# 编辑 src/python_bindings.rs
vim src/python_bindings.rs

# 重新构建
maturin develop --release

# 测试
python examples/python_example.py
```

### 3. 添加新功能

1. 在 `src/lib.rs` 中实现 Rust 功能
2. 在 `src/python_bindings.rs` 中添加 Python 绑定
3. 在 `python/rust_py_example/__init__.py` 中导出
4. 添加测试到 `tests/test_python.py`
5. 更新文档

### 4. 持续集成

创建 `.github/workflows/ci.yml`：

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-python@v2
        with:
          python-version: '3.9'
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
      - run: pip install maturin pytest
      - run: maturin develop
      - run: pytest tests/
```

## 📚 常见问题

### Q1: 如何调试 Rust 代码？

```bash
# 使用 println! 调试
println!("Debug: {:?}", variable);

# 使用 dbg! 宏
dbg!(variable);

# 使用 RUST_BACKTRACE
RUST_BACKTRACE=1 cargo test

# 使用 rust-lldb 或 rust-gdb
rust-lldb target/debug/rust_py_example
```

### Q2: Python 导入失败怎么办？

```bash
# 确保已安装
pip list | grep rust-py-example

# 重新构建
maturin develop --release

# 检查 Python 路径
python -c "import sys; print(sys.path)"

# 检查模块位置
python -c "import rust_py_example; print(rust_py_example.__file__)"
```

### Q3: 如何处理跨平台编译？

```bash
# 安装 cross
cargo install cross

# 编译 Windows
cross build --target x86_64-pc-windows-gnu

# 编译 macOS (需要在 macOS 上)
cargo build --target x86_64-apple-darwin

# 编译 Linux
cargo build --target x86_64-unknown-linux-gnu

# 使用 maturin 跨平台构建
maturin build --release --target x86_64-pc-windows-gnu
```

### Q4: 如何优化性能？

```toml
# 在 Cargo.toml 中配置
[profile.release]
opt-level = 3           # 最大优化
lto = true             # 链接时优化
codegen-units = 1      # 单个代码生成单元
panic = 'abort'        # 减小二进制大小
strip = true           # 去除符号信息
```

### Q5: 如何处理内存和生命周期？

- 使用 `'static` 生命周期处理长期存在的数据
- 使用 `Arc<T>` 和 `Mutex<T>` 处理共享状态
- 避免在 Python 绑定中使用复杂的生命周期
- 优先使用 `String` 而不是 `&str` 在 PyO3 中

### Q6: 如何添加异步支持？

```rust
// 在 Cargo.toml 中添加
[dependencies]
tokio = { version = "1", features = ["full"] }
pyo3-asyncio = "0.20"

// 在代码中使用
#[pyfunction]
fn async_function(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        // 异步代码
        Ok(())
    })
}
```

## 🔗 相关资源

### 官方文档
- [Rust 官方网站](https://www.rust-lang.org/)
- [Cargo 文档](https://doc.rust-lang.org/cargo/)
- [PyO3 文档](https://pyo3.rs/)
- [Maturin 文档](https://github.com/PyO3/maturin)

### 学习资源
- [Rust 程序设计语言](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [PyO3 用户指南](https://pyo3.rs/latest/)

### 社区
- [Rust 中文社区](https://rustcc.cn/)
- [Rust 官方论坛](https://users.rust-lang.org/)
- [PyO3 Gitter](https://gitter.im/PyO3/Lobby)

## 📄 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🤝 贡献

欢迎贡献！请遵循以下步骤：

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 👥 作者

- Your Name - [GitHub](https://github.com/yourusername)

## 🙏 致谢

- [PyO3](https://github.com/PyO3/pyo3) - 优秀的 Rust-Python 绑定库
- [Maturin](https://github.com/PyO3/maturin) - 强大的构建工具
- Rust 和 Python 社区的所有贡献者
