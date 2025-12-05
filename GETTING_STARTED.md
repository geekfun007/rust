# 快速入门指南 🚀

欢迎使用 Rust-Python 集成项目！本指南将帮助你在 5 分钟内开始使用。

## 📦 前置要求

### 必需
- **Rust** (1.70+): https://rustup.rs/
- **Python** (3.7+): https://www.python.org/downloads/
- **pip**: 随 Python 一起安装

### 检查安装

```bash
# 检查 Rust
rustc --version
cargo --version

# 检查 Python
python3 --version
pip --version
```

如果 Rust 未安装，运行：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## ⚡ 快速开始（3 步）

### 步骤 1: 克隆项目

```bash
git clone https://github.com/yourusername/rust-py-example.git
cd rust-py-example
```

### 步骤 2: 自动设置（推荐）

```bash
./scripts/setup.sh
```

这个脚本会：
- ✅ 检查依赖
- ✅ 安装 Python 工具
- ✅ 构建 Rust 库
- ✅ 安装 Python 包
- ✅ 运行测试

### 步骤 3: 运行示例

```bash
# Python 示例
python examples/python_example.py

# Rust 示例
cargo run --example rust_only_example

# 性能测试
python scripts/benchmark.py
```

## 🎯 手动设置（如果自动设置失败）

### 1. 安装 Python 工具

```bash
pip install maturin pytest
```

### 2. 构建项目

```bash
# 开发模式（快速，方便开发）
maturin develop

# 或者 Release 模式（优化，用于性能测试）
maturin develop --release
```

### 3. 验证安装

```bash
# 测试导入
python -c "import rust_py_example; print('✅ 安装成功!')"

# 运行测试
pytest tests/ -v
```

## 💻 第一个示例

创建文件 `my_first_example.py`:

```python
import rust_py_example as rpe

# 1. 创建用户
user = rpe.PyUser("小明", 20, "xiaoming@example.com")
print(f"用户: {user.description()}")
print(f"是否成年: {user.is_adult()}")

# 2. 计算斐波那契数列
result = rpe.fibonacci(10)
print(f"斐波那契(10) = {result}")

# 3. 判断质数
is_prime = rpe.is_prime(17)
print(f"17 是质数吗? {is_prime}")

# 4. 字符串处理
text = "hello world"
reversed_text = rpe.reverse_string(text)
print(f"反转: {reversed_text}")

# 5. 批量处理
numbers = [1, 2, 3, 4, 5]
results = rpe.fibonacci_batch(numbers)
print(f"批量计算: {results}")
```

运行：
```bash
python my_first_example.py
```

输出：
```
用户: 小明 is 20 years old, email: xiaoming@example.com
是否成年: True
斐波那契(10) = 55
17 是质数吗? True
反转: dlrow olleh
批量计算: [1, 1, 2, 3, 5]
```

## 🛠️ 开发工作流

### 修改代码后重新构建

```bash
# 1. 修改 src/lib.rs 或 src/python_bindings.rs
vim src/lib.rs

# 2. 重新构建
maturin develop --release

# 3. 测试
python my_example.py
```

### 使用 Make 命令（推荐）

```bash
make dev    # 重新构建
make test   # 运行所有测试
make clean  # 清理构建文件
make help   # 查看所有命令
```

## 📚 下一步

### 基础学习
1. 📖 阅读 [README.md](README.md) - 完整文档
2. 👀 查看 [examples/](examples/) - 更多示例
3. 🧪 运行 [tests/test_python.py](tests/test_python.py) - 了解测试

### 进阶学习
1. 🏗️ [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) - 架构设计
2. 📦 [docs/PUBLISHING_GUIDE.md](docs/PUBLISHING_GUIDE.md) - 发布流程
3. ⚡ [docs/QUICK_REFERENCE.md](docs/QUICK_REFERENCE.md) - API 参考

### 实践项目
1. 🔧 添加新功能到 `src/lib.rs`
2. 🐍 在 `src/python_bindings.rs` 中添加 Python 绑定
3. ✅ 添加测试到 `tests/test_python.py`
4. 📝 更新文档

## 🐛 遇到问题？

### 常见问题

#### 1. `ImportError: No module named 'rust_py_example'`

**解决**:
```bash
maturin develop --release
```

#### 2. `cargo: command not found`

**解决**:
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 3. `maturin: command not found`

**解决**:
```bash
pip install maturin
```

#### 4. 编译错误

**解决**:
```bash
# 清理并重建
cargo clean
maturin develop --release
```

#### 5. 性能不如预期

**原因**: 可能使用了 debug 构建

**解决**:
```bash
# 确保使用 --release
maturin develop --release
```

### 获取帮助

- 📖 查看完整文档: [README.md](README.md)
- 🐛 报告问题: [GitHub Issues](https://github.com/yourusername/rust-py-example/issues)
- 💬 讨论: [GitHub Discussions](https://github.com/yourusername/rust-py-example/discussions)

## ✅ 检查清单

完成以下检查确认一切正常：

- [ ] Rust 已安装 (`rustc --version`)
- [ ] Python 已安装 (`python3 --version`)
- [ ] Maturin 已安装 (`maturin --version`)
- [ ] 项目已克隆
- [ ] 依赖已安装 (`./scripts/setup.sh` 或手动安装)
- [ ] Python 包已构建 (`maturin develop --release`)
- [ ] 可以导入包 (`python -c "import rust_py_example"`)
- [ ] 测试通过 (`pytest tests/`)
- [ ] 示例可运行 (`python examples/python_example.py`)

## 🎉 完成！

恭喜！你已经成功设置了 Rust-Python 集成项目。

**接下来可以：**
- 🔍 探索示例代码
- 🧪 运行性能测试: `python scripts/benchmark.py`
- 💡 阅读架构文档: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- 🚀 开始开发自己的功能

**有用的命令：**
```bash
make help               # 查看所有 make 命令
cargo doc --open        # 打开 Rust API 文档
python scripts/benchmark.py  # 运行性能测试
```

---

**祝你编码愉快！** 💻✨

如果这个项目对你有帮助，请给个 ⭐️：
https://github.com/yourusername/rust-py-example
