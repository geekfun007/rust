# 架构设计文档

## 项目概述

本项目展示了如何构建一个混合 Rust-Python 库，利用 Rust 的性能优势和 Python 的易用性。

## 架构图

```
┌─────────────────────────────────────────────────────────────┐
│                     Python 应用层                            │
│  (用户代码 - 使用 import rust_py_example)                    │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Python API 层 (__init__.py)                     │
│  (导出 Rust 函数和类给 Python)                               │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│           PyO3 绑定层 (python_bindings.rs)                   │
│  - #[pyclass] 类型                                           │
│  - #[pyfunction] 函数                                        │
│  - Python ↔ Rust 类型转换                                   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Rust 核心库 (lib.rs)                            │
│  - 业务逻辑实现                                              │
│  - 数据结构                                                  │
│  - 算法实现                                                  │
└─────────────────────────────────────────────────────────────┘
```

## 模块划分

### 1. Rust 核心库 (`src/lib.rs`)

**职责**:
- 实现核心业务逻辑
- 定义数据结构
- 提供纯 Rust API

**设计原则**:
- 与 Python 解耦
- 可独立作为 Rust crate 使用
- 高性能、类型安全

**模块组织**:
```
lib.rs
├── User struct          # 用户数据结构
├── math module          # 数学计算
│   ├── fibonacci()
│   ├── is_prime()
│   └── factorial()
└── text module          # 字符串处理
    ├── reverse()
    ├── word_count()
    └── capitalize_words()
```

### 2. PyO3 绑定层 (`src/python_bindings.rs`)

**职责**:
- 将 Rust 功能暴露给 Python
- 处理 Python/Rust 类型转换
- 提供 Python 友好的 API

**设计模式**:
```rust
// 包装 Rust 类型
#[pyclass]
pub struct PyUser {
    // 内部使用 Rust 类型
    inner: crate::User,
}

// 包装 Rust 函数
#[pyfunction]
fn fibonacci(n: u32) -> u64 {
    crate::math::fibonacci(n)
}
```

**注意事项**:
- 避免在绑定层实现业务逻辑
- 处理好错误转换
- 提供清晰的 Python 文档字符串

### 3. Python 包层 (`python/rust_py_example/__init__.py`)

**职责**:
- 导入和导出 Rust 函数/类
- 提供 Python 文档
- 可选的 Python 包装函数

**示例**:
```python
from ._rust_py_example import (
    PyUser,
    fibonacci,
    # ...
)

# 可以添加纯 Python 的辅助函数
def helper_function():
    pass

__all__ = ["PyUser", "fibonacci", ...]
```

## 数据流

### Python 调用 Rust

```
Python 代码
    ↓
Python API 调用
    ↓
PyO3 边界
    ↓ (类型转换: Python → Rust)
Rust 函数执行
    ↓
返回 Rust 值
    ↓
PyO3 边界
    ↓ (类型转换: Rust → Python)
Python 接收结果
```

### 类型转换

```rust
// Python → Rust
i32 ← int
String ← str
Vec<T> ← list
HashMap<K,V> ← dict

// Rust → Python
int → i32
str → String
list → Vec<T>
dict → HashMap<K,V>
```

## 构建流程

### Rust 库构建

```
Cargo.toml 配置
    ↓
cargo build
    ↓
rustc 编译
    ↓
.rlib (Rust 静态库)
```

### Python 包构建

```
Cargo.toml + pyproject.toml
    ↓
maturin build
    ↓
rustc 编译 (cdylib)
    ↓
.so/.dylib/.dll (动态库)
    ↓
打包成 wheel
    ↓
.whl (Python wheel)
```

## 性能考虑

### 1. 最小化边界跨越

**不好的做法**:
```python
# Python 中循环调用 Rust 函数
for item in items:
    result = rust_func(item)  # 每次调用都有开销
```

**好的做法**:
```python
# 批量处理
results = rust_func_batch(items)  # 一次调用
```

### 2. 避免频繁的内存分配

**不好的做法**:
```rust
#[pyfunction]
fn process(s: String) -> String {
    // 每次都创建新 String
    s.clone() + " processed"
}
```

**好的做法**:
```rust
#[pyfunction]
fn process(s: &str) -> String {
    // 使用引用，减少复制
    format!("{} processed", s)
}
```

### 3. 并行处理

```rust
use rayon::prelude::*;

#[pyfunction]
fn parallel_sum(numbers: Vec<i64>) -> i64 {
    numbers.par_iter().sum()
}
```

## 错误处理

### Rust 错误 → Python 异常

```rust
use pyo3::exceptions::PyValueError;

#[pyfunction]
fn divide(a: i32, b: i32) -> PyResult<i32> {
    if b == 0 {
        Err(PyValueError::new_err("除数不能为零"))
    } else {
        Ok(a / b)
    }
}
```

### Python 中捕获

```python
try:
    result = divide(10, 0)
except ValueError as e:
    print(f"错误: {e}")
```

## 测试策略

### 1. Rust 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(10), 55);
    }
}
```

运行: `cargo test`

### 2. Python 集成测试

```python
import pytest
import rust_py_example as rpe

def test_fibonacci():
    assert rpe.fibonacci(10) == 55
```

运行: `pytest tests/`

### 3. 性能测试

```python
import time

def benchmark(func, args, iterations=1000):
    start = time.time()
    for _ in range(iterations):
        func(*args)
    return time.time() - start
```

## 发布策略

### 双重发布

1. **crates.io** (Rust 库)
   - 可以被其他 Rust 项目使用
   - 纯 Rust API

2. **PyPI** (Python 包)
   - Python 用户可以 `pip install`
   - 包含编译好的二进制

### 版本同步

保持 Cargo.toml 和 pyproject.toml 中的版本号一致：

```toml
# Cargo.toml
version = "0.1.0"

# pyproject.toml
version = "0.1.0"
```

## 最佳实践

### 1. 保持 Rust 核心与 Python 解耦

```rust
// ✅ 好：纯 Rust 实现
pub fn fibonacci(n: u32) -> u64 {
    // Rust 逻辑
}

// ❌ 不好：在核心库中依赖 PyO3
pub fn fibonacci(py: Python, n: u32) -> PyResult<u64> {
    // ...
}
```

### 2. 使用特性开关

```toml
[features]
default = []
python = ["pyo3"]
```

这样可以：
- 纯 Rust 项目不需要 PyO3 依赖
- Python 绑定可选

### 3. 文档完善

```rust
/// 计算斐波那契数列的第 n 项
///
/// # Arguments
/// * `n` - 数列索引
///
/// # Returns
/// 斐波那契数列的第 n 项
///
/// # Examples
/// ```
/// let result = fibonacci(10);
/// assert_eq!(result, 55);
/// ```
pub fn fibonacci(n: u32) -> u64 {
    // ...
}
```

### 4. 类型安全

```rust
// ✅ 使用类型系统
pub struct UserId(u64);
pub struct Email(String);

// ❌ 使用原始类型
pub fn create_user(id: u64, email: String) { }
```

## 扩展方向

### 1. 异步支持

```rust
use pyo3_asyncio;

#[pyfunction]
fn async_operation(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        // 异步代码
        Ok(())
    })
}
```

### 2. 全局解释器锁 (GIL) 释放

```rust
#[pyfunction]
fn cpu_intensive_task(py: Python, data: Vec<i32>) -> PyResult<i32> {
    py.allow_threads(|| {
        // 这里释放 GIL，允许其他 Python 线程运行
        data.iter().sum()
    })
}
```

### 3. 自定义错误类型

```rust
use pyo3::create_exception;

create_exception!(mymodule, CustomError, pyo3::exceptions::PyException);

#[pyfunction]
fn may_fail() -> PyResult<()> {
    Err(CustomError::new_err("自定义错误"))
}
```

## 性能基准

典型性能提升（相比纯 Python）:

- 数值计算: 10-100x
- 字符串处理: 5-20x
- 算法实现: 10-50x
- IO 操作: 1-3x (不明显)

## 相关工具

- **Maturin**: 构建和发布工具
- **PyO3**: Rust-Python 绑定
- **pyo3-pack** (已弃用): Maturin 的前身
- **setuptools-rust**: 另一个构建工具

## 参考资料

- [PyO3 用户指南](https://pyo3.rs/)
- [Maturin 文档](https://github.com/PyO3/maturin)
- [Rust 官方文档](https://doc.rust-lang.org/)
- [Python C API](https://docs.python.org/3/c-api/)
