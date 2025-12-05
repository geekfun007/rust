# 发布指南

本指南详细说明如何将你的 Rust-Python 项目发布到 crates.io 和 PyPI。

## 📋 目录

- [准备工作](#准备工作)
- [发布到 crates.io](#发布到-cratesio)
- [发布到 PyPI](#发布到-pypi)
- [版本管理](#版本管理)
- [持续发布](#持续发布)
- [故障排除](#故障排除)

## 准备工作

### 1. 完善项目信息

确保以下文件配置正确：

**Cargo.toml**
```toml
[package]
name = "your-package-name"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
description = "简短描述（一句话）"
license = "MIT"
repository = "https://github.com/username/repo"
homepage = "https://github.com/username/repo"
documentation = "https://docs.rs/your-package-name"
readme = "README.md"
keywords = ["python", "rust", "pyo3"]  # 最多 5 个
categories = ["api-bindings"]  # 见 https://crates.io/categories
```

**pyproject.toml**
```toml
[project]
name = "your-package-name"
version = "0.1.0"
description = "简短描述"
readme = "README.md"
requires-python = ">=3.7"
license = { text = "MIT" }
authors = [{ name = "Your Name", email = "your.email@example.com" }]
```

### 2. 编写文档

- **README.md**: 必须包含
  - 项目简介
  - 安装方法
  - 快速开始示例
  - 功能列表
  - 许可证信息

- **CHANGELOG.md**: 记录版本变更

- **LICENSE**: 选择合适的许可证

### 3. 测试检查

```bash
# 运行所有测试
make test

# 检查 Rust 代码
cargo clippy -- -D warnings
cargo fmt -- --check

# 检查 Python 代码
black --check python/ tests/
flake8 python/ tests/
```

## 发布到 crates.io

### 1. 注册账号

访问 https://crates.io/ 并使用 GitHub 账号登录。

### 2. 获取 API Token

1. 登录后，访问 https://crates.io/settings/tokens
2. 点击 "New Token"
3. 输入 token 名称（如 "cli"）
4. 复制生成的 token

### 3. 登录 Cargo

```bash
cargo login <your-api-token>
```

Token 会保存在 `~/.cargo/credentials.toml`

### 4. 验证包配置

```bash
# 检查包内容
cargo package --list

# 本地构建测试
cargo package

# 检查生成的包
ls target/package/
```

### 5. 发布

```bash
# 试运行（不实际发布）
cargo publish --dry-run

# 正式发布
cargo publish
```

### 6. 验证发布

访问 https://crates.io/crates/your-package-name 确认发布成功。

## 发布到 PyPI

### 1. 注册账号

访问以下网站注册账号：
- PyPI: https://pypi.org/account/register/
- TestPyPI: https://test.pypi.org/account/register/ (用于测试)

### 2. 配置 API Token

1. 登录 PyPI
2. 访问 https://pypi.org/manage/account/token/
3. 创建新 token
4. 复制 token

### 3. 配置凭证文件

创建 `~/.pypirc`:

```ini
[distutils]
index-servers =
    pypi
    testpypi

[pypi]
username = __token__
password = pypi-AgEIcHlwaS5vcmc... (你的 PyPI token)

[testpypi]
repository = https://test.pypi.org/legacy/
username = __token__
password = pypi-AgENdGVzdC5weXBpLm9yZw... (你的 TestPyPI token)
```

设置权限：
```bash
chmod 600 ~/.pypirc
```

### 4. 构建 Wheel 包

```bash
# 构建单个平台
maturin build --release

# 构建多个 Python 版本
maturin build --release -i python3.8 -i python3.9 -i python3.10 -i python3.11

# 查看构建的包
ls target/wheels/
```

### 5. 测试发布（推荐）

```bash
# 发布到 TestPyPI
maturin publish --repository testpypi

# 从 TestPyPI 安装测试
pip install --index-url https://test.pypi.org/simple/ your-package-name

# 测试导入
python -c "import your_package_name; print('Success!')"
```

### 6. 正式发布

```bash
# 发布到 PyPI
maturin publish

# 或使用 twine
twine upload target/wheels/*
```

### 7. 验证发布

访问 https://pypi.org/project/your-package-name/ 确认发布成功。

### 8. 测试安装

```bash
# 创建新环境测试
python -m venv test_env
source test_env/bin/activate
pip install your-package-name
python -c "import your_package_name; print('Works!')"
deactivate
rm -rf test_env
```

## 版本管理

### 语义化版本控制

遵循 [SemVer](https://semver.org/lang/zh-CN/) 规范：

- **主版本号 (MAJOR)**: 不兼容的 API 变更
- **次版本号 (MINOR)**: 向后兼容的功能新增
- **修订号 (PATCH)**: 向后兼容的问题修复

示例：`1.2.3`
- 1 = 主版本
- 2 = 次版本
- 3 = 修订版本

### 更新版本号

1. **更新 Cargo.toml**
   ```toml
   version = "0.2.0"
   ```

2. **更新 pyproject.toml**
   ```toml
   version = "0.2.0"
   ```

3. **更新 CHANGELOG.md**
   ```markdown
   ## [0.2.0] - 2025-12-05
   ### Added
   - 新功能 A
   - 新功能 B
   
   ### Fixed
   - 修复 bug X
   ```

4. **创建 Git 标签**
   ```bash
   git add .
   git commit -m "chore: bump version to 0.2.0"
   git tag -a v0.2.0 -m "Release version 0.2.0"
   git push origin main
   git push origin v0.2.0
   ```

### 版本更新脚本

创建 `scripts/bump_version.sh`:

```bash
#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "用法: $0 <version>"
    echo "示例: $0 0.2.0"
    exit 1
fi

NEW_VERSION=$1

echo "更新版本号到 $NEW_VERSION..."

# 更新 Cargo.toml
sed -i.bak "s/^version = .*/version = \"$NEW_VERSION\"/" Cargo.toml

# 更新 pyproject.toml
sed -i.bak "s/^version = .*/version = \"$NEW_VERSION\"/" pyproject.toml

# 更新 Python __init__.py
sed -i.bak "s/__version__ = .*/__version__ = \"$NEW_VERSION\"/" python/rust_py_example/__init__.py

# 清理备份文件
rm -f Cargo.toml.bak pyproject.toml.bak python/rust_py_example/__init__.py.bak

echo "✅ 版本号已更新到 $NEW_VERSION"
echo ""
echo "下一步："
echo "1. 更新 CHANGELOG.md"
echo "2. git add ."
echo "3. git commit -m 'chore: bump version to $NEW_VERSION'"
echo "4. git tag -a v$NEW_VERSION -m 'Release version $NEW_VERSION'"
echo "5. git push origin main --tags"
```

## 持续发布

### GitHub Actions 自动发布

创建 `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release-crates:
    name: Release to crates.io
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo publish --token ${{ secrets.CARGO_TOKEN }}

  release-pypi:
    name: Release to PyPI
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v3
      - uses: PyO3/maturin-action@v1
        with:
          command: build
          args: --release --out dist
      - uses: PyO3/maturin-action@v1
        if: matrix.os == 'ubuntu-latest'
        with:
          command: publish
          args: --username __token__ --password ${{ secrets.PYPI_TOKEN }}
```

配置 Secrets：
1. 在 GitHub 仓库设置中添加 secrets
2. `CARGO_TOKEN`: crates.io API token
3. `PYPI_TOKEN`: PyPI API token

### 发布流程

```bash
# 1. 更新版本号
./scripts/bump_version.sh 0.2.0

# 2. 更新 CHANGELOG
vim CHANGELOG.md

# 3. 提交并打标签
git add .
git commit -m "chore: release version 0.2.0"
git tag -a v0.2.0 -m "Release version 0.2.0"

# 4. 推送（触发自动发布）
git push origin main --tags
```

## 故障排除

### 常见问题

#### 1. Cargo 发布失败："already uploaded"

**原因**: 已经发布过相同版本

**解决**:
```bash
# 更新版本号后重新发布
# 注意：crates.io 不允许删除或覆盖已发布版本
```

#### 2. PyPI 发布失败："File already exists"

**原因**: 已经上传过相同文件名的包

**解决**:
```bash
# 更新版本号或重新构建
maturin build --release
```

#### 3. Maturin 找不到 Python

**原因**: Python 解释器路径问题

**解决**:
```bash
# 显式指定 Python
maturin build --release -i python3.9

# 或使用 which 查找
maturin build --release -i $(which python3)
```

#### 4. 编译错误："linking with `cc` failed"

**原因**: 缺少编译工具链

**解决**:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# macOS
xcode-select --install

# Windows
# 安装 Visual Studio Build Tools
```

#### 5. 导入错误："ImportError: undefined symbol"

**原因**: ABI 不兼容

**解决**:
```bash
# 确保使用相同的 Python 版本编译和运行
# 重新构建
maturin develop --release
```

### 检查清单

发布前确认：

- [ ] 所有测试通过
- [ ] 代码格式化和 lint 检查通过
- [ ] 版本号已更新（Cargo.toml 和 pyproject.toml）
- [ ] CHANGELOG.md 已更新
- [ ] README.md 准确完整
- [ ] LICENSE 文件存在
- [ ] 文档齐全
- [ ] 示例代码可运行
- [ ] Git 标签已创建

## 最佳实践

1. **始终先发布到 TestPyPI** 测试
2. **使用 CI/CD** 自动化发布流程
3. **保持版本号同步** (Cargo.toml 和 pyproject.toml)
4. **详细的 CHANGELOG** 记录所有变更
5. **语义化版本** 遵循 SemVer 规范
6. **测试多平台** 至少测试 Linux、macOS、Windows
7. **测试多 Python 版本** 支持的所有版本都要测试
8. **完善的文档** 让用户容易上手
9. **示例代码** 展示主要功能
10. **响应式维护** 及时处理 issues 和 PRs

## 相关资源

- [crates.io 发布指南](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [PyPI 打包用户指南](https://packaging.python.org/tutorials/packaging-projects/)
- [Maturin 用户指南](https://github.com/PyO3/maturin)
- [语义化版本规范](https://semver.org/lang/zh-CN/)
- [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)
