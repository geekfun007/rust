# Makefile for rust-py-example

.PHONY: help install dev test test-rust test-python clean build release format lint doc

# 默认目标
help:
	@echo "可用命令："
	@echo "  make install       - 安装依赖"
	@echo "  make dev          - 开发模式安装"
	@echo "  make test         - 运行所有测试"
	@echo "  make test-rust    - 运行 Rust 测试"
	@echo "  make test-python  - 运行 Python 测试"
	@echo "  make build        - 构建项目"
	@echo "  make release      - 发布构建"
	@echo "  make format       - 格式化代码"
	@echo "  make lint         - 代码检查"
	@echo "  make doc          - 生成文档"
	@echo "  make clean        - 清理构建文件"
	@echo "  make example-rust - 运行 Rust 示例"
	@echo "  make example-python - 运行 Python 示例"

# 安装依赖
install:
	@echo "安装 Rust 依赖..."
	cargo fetch
	@echo "安装 Python 依赖..."
	pip install maturin pytest black flake8

# 开发模式安装
dev:
	@echo "开发模式安装..."
	maturin develop --release

# 运行所有测试
test: test-rust test-python

# 运行 Rust 测试
test-rust:
	@echo "运行 Rust 测试..."
	cargo test

# 运行 Python 测试
test-python: dev
	@echo "运行 Python 测试..."
	pytest tests/ -v

# 构建项目
build:
	@echo "构建 Rust 库..."
	cargo build --release
	@echo "构建 Python wheel..."
	maturin build --release

# 发布构建
release:
	@echo "发布构建..."
	cargo build --release
	maturin build --release

# 格式化代码
format:
	@echo "格式化 Rust 代码..."
	cargo fmt
	@echo "格式化 Python 代码..."
	black python/ tests/ examples/*.py

# 代码检查
lint:
	@echo "检查 Rust 代码..."
	cargo clippy -- -D warnings
	@echo "检查 Python 代码..."
	flake8 python/ tests/ examples/*.py --max-line-length=100

# 生成文档
doc:
	@echo "生成 Rust 文档..."
	cargo doc --no-deps --open

# 清理构建文件
clean:
	@echo "清理构建文件..."
	cargo clean
	rm -rf target/
	rm -rf dist/
	rm -rf build/
	rm -rf *.egg-info
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name "*.pyc" -delete
	find . -type f -name "*.so" -delete

# 运行 Rust 示例
example-rust:
	@echo "运行 Rust 示例..."
	cargo run --example rust_only_example

# 运行 Python 示例
example-python: dev
	@echo "运行 Python 示例..."
	python examples/python_example.py

# 发布到 crates.io
publish-crates:
	@echo "发布到 crates.io..."
	cargo publish

# 发布到 PyPI (测试)
publish-pypi-test:
	@echo "发布到 TestPyPI..."
	maturin build --release
	maturin publish --repository testpypi

# 发布到 PyPI
publish-pypi:
	@echo "发布到 PyPI..."
	maturin build --release
	maturin publish

# 完整的发布流程
publish-all: test build publish-crates publish-pypi
	@echo "发布完成！"

# 检查版本一致性
check-version:
	@echo "检查版本号..."
	@grep "^version" Cargo.toml
	@grep "^version" pyproject.toml
