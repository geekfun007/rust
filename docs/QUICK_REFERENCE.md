# 快速参考

## 常用命令速查表

### 🦀 Rust 开发

```bash
# 构建项目
cargo build                    # Debug 构建
cargo build --release          # Release 构建

# 运行测试
cargo test                     # 运行所有测试
cargo test test_name           # 运行特定测试
cargo test -- --nocapture      # 显示 println! 输出

# 代码检查
cargo check                    # 快速类型检查
cargo clippy                   # Lint 检查
cargo fmt                      # 格式化代码

# 文档
cargo doc                      # 生成文档
cargo doc --open              # 生成并打开文档

# 运行示例
cargo run --example rust_only_example

# 清理
cargo clean                    # 清理构建文件
```

### 🐍 Python 开发

```bash
# 安装 Maturin
pip install maturin

# 开发模式安装
maturin develop                # Debug 模式
maturin develop --release      # Release 模式

# 构建 Wheel
maturin build                  # 构建
maturin build --release        # Release 构建

# 运行测试
pytest tests/                  # 运行所有测试
pytest tests/ -v               # 详细输出
pytest tests/test_python.py::TestUser  # 运行特定测试类

# 代码检查
black python/ tests/           # 格式化
flake8 python/ tests/          # Lint 检查

# 运行示例
python examples/python_example.py
```

### 📦 Make 命令

```bash
make help           # 显示所有可用命令
make install        # 安装所有依赖
make dev            # 开发模式安装
make test           # 运行所有测试
make build          # 构建项目
make clean          # 清理构建文件
make example-rust   # 运行 Rust 示例
make example-python # 运行 Python 示例
```

## PyO3 常用模式

### 导出 Python 类

```rust
use pyo3::prelude::*;

#[pyclass]
pub struct MyClass {
    #[pyo3(get, set)]
    pub field: String,
}

#[pymethods]
impl MyClass {
    #[new]
    fn new(field: String) -> Self {
        MyClass { field }
    }
    
    fn method(&self) -> String {
        format!("Value: {}", self.field)
    }
}
```

### 导出 Python 函数

```rust
#[pyfunction]
fn my_function(arg: i32) -> PyResult<i32> {
    Ok(arg * 2)
}
```

### 导出 Python 模块

```rust
#[pymodule]
fn my_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    m.add_function(wrap_pyfunction!(my_function, m)?)?;
    Ok(())
}
```

### 处理 Python 对象

```rust
use pyo3::types::{PyDict, PyList};

#[pyfunction]
fn process_list(py: Python, list: &PyList) -> PyResult<Vec<i32>> {
    list.iter()
        .map(|item| item.extract::<i32>())
        .collect()
}

#[pyfunction]
fn create_dict(py: Python) -> PyResult<&PyDict> {
    let dict = PyDict::new(py);
    dict.set_item("key", "value")?;
    Ok(dict)
}
```

### 错误处理

```rust
use pyo3::exceptions::PyValueError;

#[pyfunction]
fn divide(a: i32, b: i32) -> PyResult<i32> {
    if b == 0 {
        Err(PyValueError::new_err("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
```

## 类型映射

### Rust → Python

| Rust 类型 | Python 类型 |
|-----------|-------------|
| `i32`, `i64`, `u32`, `u64` | `int` |
| `f32`, `f64` | `float` |
| `bool` | `bool` |
| `String`, `&str` | `str` |
| `Vec<T>` | `list` |
| `HashMap<K, V>` | `dict` |
| `Option<T>` | `Optional[T]` 或 `None` |
| `Result<T, E>` | `T` 或抛出异常 |
| `()` | `None` |

### Python → Rust

```rust
// 基本类型
fn example(
    int_arg: i32,
    float_arg: f64,
    string_arg: String,
    bool_arg: bool,
) -> PyResult<()> {
    // ...
}

// 集合类型
fn example_collections(
    list_arg: Vec<i32>,
    dict_arg: HashMap<String, i32>,
    optional_arg: Option<String>,
) -> PyResult<()> {
    // ...
}

// Python 对象
fn example_py_objects(
    py: Python,
    list: &PyList,
    dict: &PyDict,
    any: &PyAny,
) -> PyResult<()> {
    // ...
}
```

## 性能优化技巧

### 1. Release 构建

```bash
# 总是使用 --release 进行性能测试
maturin develop --release
cargo build --release
```

### 2. 配置优化选项

```toml
[profile.release]
opt-level = 3        # 最大优化
lto = true          # 链接时优化
codegen-units = 1   # 单个代码生成单元
```

### 3. 避免频繁的 Python/Rust 边界跨越

```rust
// ❌ 不好：多次调用
#[pyfunction]
fn process_items(items: Vec<i32>) -> Vec<i32> {
    items.iter().map(|&x| x * 2).collect()
}
// 在 Python 中循环调用这个函数

// ✅ 好：批量处理
#[pyfunction]
fn process_items_batch(items: Vec<i32>) -> Vec<i32> {
    items.iter().map(|&x| x * 2).collect()
}
// 一次传入所有数据
```

### 4. 使用并行处理

```rust
use rayon::prelude::*;

#[pyfunction]
fn parallel_process(items: Vec<i32>) -> Vec<i32> {
    items.par_iter().map(|&x| x * 2).collect()
}
```

## 调试技巧

### Rust 调试

```rust
// 使用 dbg! 宏
dbg!(variable);

// 使用 println!
println!("Debug: {:?}", variable);

// 环境变量
RUST_BACKTRACE=1 cargo test
RUST_LOG=debug cargo run
```

### Python 调试

```python
# 使用 pdb
import pdb; pdb.set_trace()

# 打印类型
print(type(obj))

# 打印所有属性
print(dir(obj))

# 查看文档
help(rust_py_example.fibonacci)
```

## 常见错误及解决方法

### 1. "ImportError: undefined symbol"

```bash
# 重新构建
maturin develop --release --force
```

### 2. "error: linking with `cc` failed"

```bash
# Ubuntu
sudo apt-get install build-essential

# macOS
xcode-select --install
```

### 3. "PyO3 version mismatch"

```bash
# 清理并重建
cargo clean
maturin develop --release
```

### 4. Python 找不到模块

```python
# 检查安装
import sys
print(sys.path)

# 重新安装
pip uninstall rust-py-example
maturin develop --release
```

## 发布快速指南

```bash
# 1. 更新版本号
# 编辑 Cargo.toml 和 pyproject.toml

# 2. 运行测试
make test

# 3. 构建
maturin build --release

# 4. 发布到 TestPyPI（测试）
maturin publish --repository testpypi

# 5. 发布到 PyPI
maturin publish

# 6. 发布到 crates.io
cargo publish
```

## 有用的 Cargo.toml 配置

```toml
# 工作区
[workspace]
members = ["crate1", "crate2"]

# 特性
[features]
default = ["feature1"]
feature1 = []
python = ["pyo3"]

# 开发依赖
[dev-dependencies]
criterion = "0.5"  # 性能测试

# 示例
[[example]]
name = "example_name"
path = "examples/example.rs"

# 基准测试
[[bench]]
name = "benchmark_name"
harness = false
```

## 有用的链接

- [Rust 官方文档](https://doc.rust-lang.org/)
- [PyO3 用户指南](https://pyo3.rs/)
- [Maturin 文档](https://github.com/PyO3/maturin)
- [Cargo 文档](https://doc.rust-lang.org/cargo/)
- [crates.io](https://crates.io/)
- [PyPI](https://pypi.org/)
