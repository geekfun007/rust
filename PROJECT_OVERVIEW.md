# 项目总览

## 🎯 项目目的

这是一个完整的示例项目，演示如何：
1. **开发 Rust 库** - 创建高性能的 Rust 核心库
2. **发布到 crates.io** - 让其他 Rust 项目可以使用
3. **创建 Python 绑定** - 使用 PyO3 将 Rust 功能暴露给 Python
4. **发布到 PyPI** - 让 Python 用户可以通过 `pip install` 安装

## 📁 项目结构详解

```
rust-py-example/
│
├── 📄 配置文件
│   ├── Cargo.toml              # Rust 项目配置
│   ├── pyproject.toml          # Python 项目配置 (PEP 518)
│   ├── Makefile                # 构建任务自动化
│   ├── requirements.txt        # Python 运行时依赖
│   └── requirements-dev.txt    # Python 开发依赖
│
├── 📚 文档
│   ├── README.md               # 主文档（中文）
│   ├── CHANGELOG.md            # 版本更新记录
│   ├── CONTRIBUTING.md         # 贡献指南
│   ├── LICENSE                 # 开源许可证
│   ├── PROJECT_OVERVIEW.md     # 本文件
│   └── docs/
│       ├── ARCHITECTURE.md     # 架构设计文档
│       ├── PUBLISHING_GUIDE.md # 发布指南
│       └── QUICK_REFERENCE.md  # 快速参考
│
├── 🦀 Rust 源代码
│   └── src/
│       ├── lib.rs              # Rust 核心库
│       └── python_bindings.rs  # PyO3 Python 绑定
│
├── 🐍 Python 源代码
│   └── python/
│       └── rust_py_example/
│           └── __init__.py     # Python 包初始化
│
├── 📝 示例代码
│   └── examples/
│       ├── rust_only_example.rs    # Rust 库使用示例
│       └── python_example.py       # Python 使用示例
│
├── 🧪 测试
│   └── tests/
│       └── test_python.py      # Python 单元测试
│       # Rust 测试在 src/lib.rs 中
│
├── 🔧 脚本
│   └── scripts/
│       ├── setup.sh            # 快速设置脚本
│       └── benchmark.py        # 性能基准测试
│
└── ⚙️ CI/CD
    └── .github/
        └── workflows/
            └── ci.yml          # GitHub Actions CI 配置
```

## 🔑 核心组件

### 1. Rust 核心库 (`src/lib.rs`)

**功能模块**:
- ✅ **User 结构体**: 用户数据管理
- ✅ **math 模块**: 数学计算
  - `fibonacci(n)`: 斐波那契数列
  - `is_prime(n)`: 质数判断
  - `factorial(n)`: 阶乘计算
- ✅ **text 模块**: 字符串处理
  - `reverse(s)`: 字符串反转
  - `word_count(s)`: 单词统计
  - `capitalize_words(s)`: 首字母大写

**特点**:
- 纯 Rust 实现，无 Python 依赖
- 可独立作为 Rust crate 使用
- 包含完整的单元测试

### 2. Python 绑定 (`src/python_bindings.rs`)

**导出内容**:
- **PyUser 类**: User 结构体的 Python 包装
- **数学函数**: fibonacci, is_prime, factorial
- **字符串函数**: reverse_string, word_count, capitalize_words
- **批量处理**: fibonacci_batch, filter_primes

**特点**:
- 使用 PyO3 实现
- 完整的类型转换
- Python 友好的 API

### 3. Python 包 (`python/rust_py_example/__init__.py`)

**作用**:
- 导入 Rust 函数和类
- 提供统一的 Python API
- 可添加纯 Python 辅助函数

## 🚀 快速开始

### 最小安装（5 分钟）

```bash
# 1. 克隆项目
git clone https://github.com/yourusername/rust-py-example.git
cd rust-py-example

# 2. 运行设置脚本
./scripts/setup.sh

# 3. 运行示例
python examples/python_example.py
```

### 手动安装

```bash
# 1. 安装 Rust (如果未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 安装 Python 工具
pip install maturin

# 3. 开发模式安装
maturin develop --release

# 4. 测试
pytest tests/
```

## 📊 使用示例

### Python 中使用

```python
import rust_py_example as rpe

# 创建用户
user = rpe.PyUser("Alice", 25, "alice@example.com")
print(user.description())  # Alice is 25 years old, email: alice@example.com
print(user.is_adult())      # True

# 数学计算
print(rpe.fibonacci(10))    # 55
print(rpe.is_prime(17))     # True
print(rpe.factorial(5))     # 120

# 字符串处理
print(rpe.reverse_string("hello"))      # olleh
print(rpe.word_count("hello world"))    # 2

# 批量处理
numbers = [0, 1, 2, 5, 10]
print(rpe.fibonacci_batch(numbers))     # [0, 1, 1, 5, 55]
```

### Rust 中使用

```rust
use rust_py_example::{User, math, text};

fn main() {
    // 用户管理
    let user = User::new("Alice".into(), 25, "alice@example.com".into());
    println!("{}", user.description());
    
    // 数学计算
    println!("fibonacci(10) = {}", math::fibonacci(10));
    println!("is_prime(17) = {}", math::is_prime(17));
    
    // 字符串处理
    println!("reversed = {}", text::reverse("hello"));
}
```

## 📈 性能对比

运行基准测试：
```bash
python scripts/benchmark.py
```

典型结果（Rust vs Python）:
- 斐波那契计算: **~10x 更快**
- 质数判断: **~15x 更快**
- 字符串处理: **~5x 更快**

## 🛠️ 开发工作流

### 修改代码

```bash
# 1. 编辑 Rust 代码
vim src/lib.rs

# 2. 运行 Rust 测试
cargo test

# 3. 重新构建 Python 包
maturin develop --release

# 4. 运行 Python 测试
pytest tests/
```

### 添加新功能

1. 在 `src/lib.rs` 中实现 Rust 功能
2. 在 `src/python_bindings.rs` 中添加 Python 绑定
3. 在 `python/rust_py_example/__init__.py` 中导出
4. 在 `tests/test_python.py` 中添加测试
5. 更新文档

### 代码检查

```bash
# Rust
cargo fmt      # 格式化
cargo clippy   # Lint 检查
cargo test     # 测试

# Python
black python/ tests/       # 格式化
flake8 python/ tests/      # Lint 检查
pytest tests/              # 测试
```

## 📦 发布流程

### 准备发布

```bash
# 1. 更新版本号（Cargo.toml 和 pyproject.toml）
vim Cargo.toml pyproject.toml

# 2. 更新 CHANGELOG.md
vim CHANGELOG.md

# 3. 运行所有测试
make test

# 4. 构建
make build
```

### 发布到 TestPyPI（推荐先测试）

```bash
maturin publish --repository testpypi
```

### 正式发布

```bash
# 发布到 PyPI
maturin publish

# 发布到 crates.io
cargo publish

# 创建 Git 标签
git tag -a v0.1.0 -m "Release 0.1.0"
git push origin --tags
```

## 🧪 测试

### Rust 测试

```bash
cargo test              # 运行所有测试
cargo test fibonacci    # 运行特定测试
cargo test -- --nocapture  # 显示输出
```

### Python 测试

```bash
pytest tests/           # 运行所有测试
pytest tests/ -v        # 详细输出
pytest tests/test_python.py::TestUser  # 运行特定测试类
```

### 性能测试

```bash
python scripts/benchmark.py
```

## 📚 文档

- **README.md**: 完整的使用指南（你正在看的文档）
- **docs/ARCHITECTURE.md**: 架构设计
- **docs/PUBLISHING_GUIDE.md**: 详细的发布流程
- **docs/QUICK_REFERENCE.md**: 命令和 API 速查表
- **CONTRIBUTING.md**: 贡献指南

生成 Rust API 文档：
```bash
cargo doc --open
```

## 🔧 Make 命令

```bash
make help           # 显示所有命令
make install        # 安装依赖
make dev            # 开发模式安装
make test           # 运行所有测试
make build          # 构建项目
make clean          # 清理构建文件
make format         # 格式化代码
make lint           # 代码检查
make example-rust   # 运行 Rust 示例
make example-python # 运行 Python 示例
```

## 🌟 特性

### ✅ 完整的项目结构
- Rust 核心库
- PyO3 Python 绑定
- 完整的测试套件
- 详细的文档

### ✅ 开发工具
- Makefile 自动化
- CI/CD 配置
- 代码格式化和检查
- 性能基准测试

### ✅ 发布支持
- crates.io 发布配置
- PyPI 发布配置
- 版本管理工具
- 多平台支持

### ✅ 最佳实践
- 代码组织
- 错误处理
- 性能优化
- 文档完善

## 🎓 学习资源

### 官方文档
- [Rust 官方书](https://doc.rust-lang.org/book/)
- [PyO3 用户指南](https://pyo3.rs/)
- [Maturin 文档](https://github.com/PyO3/maturin)

### 相关项目
- [pydantic-core](https://github.com/pydantic/pydantic-core) - 使用 Rust 加速的 Python 库
- [polars](https://github.com/pola-rs/polars) - Rust 实现的 DataFrame 库
- [ruff](https://github.com/astral-sh/ruff) - Rust 编写的 Python linter

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

步骤：
1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📝 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 💡 FAQ

### Q: 为什么要用 Rust？
A: Rust 提供接近 C/C++ 的性能，同时保证内存安全，非常适合性能关键的组件。

### Q: PyO3 vs C 扩展？
A: PyO3 提供更安全、更符合人体工程学的 API，无需手动管理引用计数。

### Q: 什么时候应该使用 Rust？
A: CPU 密集型计算、需要高性能的算法、系统级编程。不适合 IO 密集型任务。

### Q: 如何调试？
A: Rust 使用 `println!` 和 `dbg!`，Python 使用 `print` 和 `pdb`。

### Q: 性能提升有多大？
A: 通常是 5-100x，具体取决于任务类型。运行 `python scripts/benchmark.py` 查看。

## 🔗 相关链接

- GitHub: https://github.com/yourusername/rust-py-example
- PyPI: https://pypi.org/project/rust-py-example/
- crates.io: https://crates.io/crates/rust-py-example
- 文档: https://docs.rs/rust-py-example/

## 📞 联系方式

- 作者: Your Name
- Email: your.email@example.com
- GitHub: @yourusername

---

**祝你使用愉快！如果觉得这个项目有用，请给个 ⭐️**
