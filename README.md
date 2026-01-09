# Rust 实战项目集

🦀 这个仓库包含两个 Rust 实战项目，演示了 Rust 在不同场景下的应用。

## 项目列表

### 1. 📄 thrift-to-ts - Thrift 解析器 + TypeScript 类型生成器

一个使用 Rust 开发的 CLI 工具，用于解析 Apache Thrift IDL 文件并生成 TypeScript 类型定义。

**核心功能：**
- 完整的 Thrift IDL 词法分析器 (Lexer)
- 完整的 Thrift IDL 语法解析器 (Parser)
- TypeScript 类型定义代码生成器
- 支持所有 Thrift 基本类型、容器类型、struct、enum、union、exception、service

**使用示例：**
```bash
cd thrift-to-ts

# 编译
cargo build --release

# 运行
./target/release/thrift-to-ts -i examples/user.thrift -o output/ -v

# 查看帮助
./target/release/thrift-to-ts --help
```

**输入 (Thrift):**
```thrift
struct User {
    1: required i64 id
    2: required string name
    3: optional string email
}
```

**输出 (TypeScript):**
```typescript
export interface User {
  id: bigint;
  name: string;
  email?: string;
}
```

📁 [查看 thrift-to-ts 项目](./thrift-to-ts/)

---

### 2. 🐍 py-rust-module - PyO3 Python 扩展模块

一个使用 PyO3 开发的 Python 扩展模块，演示了如何用 Rust 为 Python 编写高性能扩展。

**核心功能：**
- 将 Rust 函数导出到 Python
- 定义 Python 类 (PyClass)
- 实现 Python 特殊方法 (`__repr__`, `__add__`, `__eq__` 等)
- JSON 序列化/反序列化 (Serde 集成)
- 集合类型转换

**包含的组件：**
- 基础函数：`add`, `factorial`, `fibonacci`, `quicksort`, `binary_search` 等
- Vector2D 类：2D 向量运算
- Matrix 类：矩阵运算
- User 类：带 JSON 序列化
- Statistics 类：统计计算
- Counter 类：类似 Python `collections.Counter`
- JSON 工具：`parse_json`, `to_json`

**使用示例：**
```bash
cd py-rust-module

# 安装 maturin
pip install maturin

# 开发模式安装
python -m venv .venv
source .venv/bin/activate
maturin develop --release

# 运行测试
pytest tests/ -v
```

**Python 使用：**
```python
import py_rust_module as pm

# 基础函数
print(pm.fibonacci(10))  # 55
print(pm.quicksort([3, 1, 4, 1, 5]))  # [1, 1, 3, 4, 5]

# Vector2D
v = pm.Vector2D(3.0, 4.0)
print(v.length())  # 5.0

# Statistics
stats = pm.Statistics([1.0, 2.0, 3.0, 4.0, 5.0])
print(stats.mean())  # 3.0

# Counter
c = pm.Counter.from_iterable(["a", "b", "a", "c", "a"])
print(c.most_common(2))  # [("a", 3), ("b", 1)]
```

📁 [查看 py-rust-module 项目](./py-rust-module/)

---

## 项目结构

```
.
├── README.md                    # 本文件
├── thrift-to-ts/               # Thrift → TypeScript 生成器
│   ├── Cargo.toml
│   ├── README.md
│   ├── src/
│   │   ├── main.rs             # CLI 入口
│   │   ├── lexer.rs            # 词法分析器
│   │   ├── parser.rs           # 语法解析器
│   │   ├── ast.rs              # 抽象语法树
│   │   └── codegen.rs          # TypeScript 代码生成
│   ├── examples/               # 示例 Thrift 文件
│   └── output/                 # 生成的 TypeScript 文件
│
└── py-rust-module/             # PyO3 Python 扩展
    ├── Cargo.toml
    ├── pyproject.toml
    ├── README.md
    ├── src/
    │   └── lib.rs              # Rust 源代码
    └── tests/
        └── test_module.py      # Python 测试
```

## 技术栈

| 项目 | 主要依赖 |
|------|----------|
| thrift-to-ts | clap (CLI), thiserror (错误处理), walkdir (文件遍历) |
| py-rust-module | pyo3 (Python 绑定), serde (序列化), serde_json |

## 环境要求

- Rust 1.70+
- Python 3.8+ (py-rust-module)
- Cargo
- maturin (py-rust-module)

## 快速开始

```bash
# 克隆仓库
git clone <repository-url>
cd rust

# 编译所有项目
cd thrift-to-ts && cargo build --release && cd ..
cd py-rust-module && cargo build --release && cd ..

# 运行测试
cd thrift-to-ts && cargo test && cd ..
cd py-rust-module && cargo test && cd ..
```

## 学习资源

### Rust 基础
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### 编译器/解析器
- [Crafting Interpreters](https://craftinginterpreters.com/)
- [Writing a Parser in Rust](https://adriann.github.io/rust_parser.html)

### PyO3
- [PyO3 官方文档](https://pyo3.rs/)
- [Maturin 文档](https://www.maturin.rs/)

## License

MIT License
