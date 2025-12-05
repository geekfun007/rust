# 更新日志

本文档记录项目的所有重要变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

### 计划中
- 添加更多数学函数
- 支持异步操作
- 添加并发处理示例

## [0.1.0] - 2025-12-05

### 新增
- 初始版本发布
- Rust 核心库实现
  - 用户管理模块
  - 数学计算模块（斐波那契、质数判断、阶乘）
  - 字符串处理模块（反转、单词统计、首字母大写）
- PyO3 Python 绑定
  - PyUser 类
  - 数学函数绑定
  - 字符串处理函数绑定
  - 批量处理函数
- 完整的示例代码
  - Rust 使用示例
  - Python 使用示例
- 测试套件
  - Rust 单元测试
  - Python 单元测试
- 文档
  - 详细的 README 文档
  - 贡献指南
  - CI/CD 配置
  - Makefile 构建脚本

### 技术栈
- Rust 2021 edition
- PyO3 0.20
- Maturin 1.0+
- Python 3.7+

[未发布]: https://github.com/yourusername/rust-py-example/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/rust-py-example/releases/tag/v0.1.0
