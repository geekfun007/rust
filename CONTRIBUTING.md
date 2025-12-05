# 贡献指南

感谢你考虑为本项目做出贡献！

## 开发环境设置

1. **安装依赖**
   ```bash
   # 安装 Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # 安装 Python 工具
   pip install maturin pytest black flake8
   ```

2. **克隆并设置项目**
   ```bash
   git clone https://github.com/yourusername/rust-py-example.git
   cd rust-py-example
   maturin develop --release
   ```

## 代码规范

### Rust 代码
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 添加适当的文档注释 (`///`)
- 为新功能编写测试

```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy -- -D warnings

# 运行测试
cargo test
```

### Python 代码
- 使用 `black` 格式化代码
- 使用 `flake8` 进行代码检查
- 遵循 PEP 8 规范
- 为新功能编写测试

```bash
# 格式化代码
black python/ tests/

# 代码检查
flake8 python/ tests/

# 运行测试
pytest tests/ -v
```

## 提交流程

1. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **编写代码和测试**
   - 实现你的功能
   - 添加测试
   - 确保所有测试通过

3. **提交更改**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

4. **推送并创建 PR**
   ```bash
   git push origin feature/your-feature-name
   ```

## Commit 消息规范

使用语义化的 commit 消息：

- `feat:` 新功能
- `fix:` 修复 bug
- `docs:` 文档更新
- `style:` 代码格式调整
- `refactor:` 代码重构
- `test:` 添加或修改测试
- `chore:` 构建过程或辅助工具的变动

示例：
```
feat: add fibonacci batch calculation function
fix: correct prime number detection for edge cases
docs: update installation instructions
```

## 报告问题

发现 bug 或有功能建议？请创建 Issue 并包含：

1. 问题描述
2. 重现步骤
3. 预期行为
4. 实际行为
5. 环境信息（OS、Python 版本、Rust 版本）

## 行为准则

- 尊重所有贡献者
- 建设性地讨论
- 接受不同观点
- 专注于项目目标

感谢你的贡献！🎉
