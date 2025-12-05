#!/bin/bash
# 快速设置脚本

set -e

echo "=========================================="
echo "Rust-Python 项目快速设置"
echo "=========================================="
echo ""

# 检查 Rust 是否安装
if ! command -v rustc &> /dev/null; then
    echo "⚠️  未检测到 Rust，正在安装..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust 安装完成"
else
    echo "✅ Rust 已安装: $(rustc --version)"
fi

# 检查 Python 是否安装
if ! command -v python3 &> /dev/null; then
    echo "❌ 未检测到 Python 3，请先安装 Python 3.7+"
    exit 1
else
    echo "✅ Python 已安装: $(python3 --version)"
fi

# 升级 pip
echo ""
echo "📦 升级 pip..."
python3 -m pip install --upgrade pip

# 安装 Maturin
echo ""
echo "📦 安装 Maturin..."
pip install maturin

# 安装开发依赖
echo ""
echo "📦 安装开发依赖..."
pip install -r requirements-dev.txt

# 获取 Rust 依赖
echo ""
echo "📦 获取 Rust 依赖..."
cargo fetch

# 运行测试
echo ""
echo "🧪 运行 Rust 测试..."
cargo test

# 构建 Python 包
echo ""
echo "🔨 构建 Python 包（开发模式）..."
maturin develop --release

# 运行 Python 测试
echo ""
echo "🧪 运行 Python 测试..."
pytest tests/ -v

echo ""
echo "=========================================="
echo "✅ 设置完成！"
echo "=========================================="
echo ""
echo "可用命令："
echo "  make help          - 查看所有 make 命令"
echo "  make dev           - 开发模式安装"
echo "  make test          - 运行所有测试"
echo "  make example-rust  - 运行 Rust 示例"
echo "  make example-python - 运行 Python 示例"
echo ""
echo "开始开发："
echo "  1. 编辑 src/lib.rs 添加 Rust 功能"
echo "  2. 编辑 src/python_bindings.rs 添加 Python 绑定"
echo "  3. 运行 'make dev' 重新构建"
echo "  4. 运行 'make test' 测试"
echo ""
