# Rust 教程文档索引

欢迎来到 Rust 详解与实战教程！本文档索引帮助你快速找到需要的学习资源。

## 📚 文档分类

### 🎯 快速入门

如果你是 Rust 初学者，建议按以下顺序阅读：

1. **[学习路径](LEARNING_PATH.md)** - 从零开始的完整学习计划
2. **[快速参考](QUICK_REFERENCE.md)** - 核心概念速查表
3. **[可视化指南](VISUAL_GUIDE.md)** - 图解帮助理解

### 🔬 深度理解

对于想深入了解 Rust 底层原理的学习者：

1. **[核心底层原理详解](CORE_PRINCIPLES.md)** ⭐ 重点推荐
   - 所有权的内存布局
   - 借用检查器工作原理
   - 切片的胖指针结构
   - 生命周期的编译时本质
   - 零成本抽象的实现

2. **[可视化指南](VISUAL_GUIDE.md)**
   - ASCII 图示内存布局
   - 借用规则时间线
   - 生命周期决策树
   - 类型大小对比表

### 📖 专题指南

针对特定主题的详细说明：

1. **[类型标注指南](TYPE_ANNOTATIONS_GUIDE.md)**
   - let 何时需要类型声明
   - 泛型类型声明语法
   - Turbofish `::<T>` 用法
   - 实战最佳实践

2. **[类型标注速查卡](TYPE_ANNOTATIONS_CHEATSHEET.md)**
   - 一页纸快速参考
   - 决策树和记忆技巧

3. **[核心类型详解](CORE_TYPES.md)** ⭐ 新增
   - Box<T> 智能指针
   - Option<T> 可选值
   - Result<T, E> 错误处理
   - fmt::Display 格式化输出
   - 其他核心 Trait

4. **[核心类型速查卡](CORE_TYPES_CHEATSHEET.md)** ⭐ 新增
   - Box/Option/Result 快速参考
   - 常用方法速查
   - 决策树

### 🎮 互动示例

运行这些示例程序来实践学习：

```bash
# 1. 内存布局演示 - 查看各种类型的内存占用
cargo run --example memory_layout

# 2. 借用检查器演示 - 理解借用规则
cargo run --example borrow_checker

# 3. 生命周期深度剖析 - 掌握生命周期标注
cargo run --example lifetime_deep_dive

# 4. 类型标注演示 - 学习何时需要类型声明
cargo run --example type_annotations

# 5. 核心类型演示 - Box/Option/Result/Display 等
cargo run --example core_types
```

### 📝 代码示例

查看源代码中的详细示例：

- **基础语法**: `src/basics/`
- **数据类型**: `src/types/`
- **所有权系统**: `src/ownership/`
- **并发编程**: `src/concurrency/`
- **I/O 操作**: `src/io/`
- **网络编程**: `src/network/`
- **最佳实践**: `src/best_practices/`

## 🗺️ 学习路径推荐

### 初级（1-2 周）

```
第 1-3 天：基础概念
├─ 运行主程序：cargo run
├─ 阅读：快速参考手册
└─ 练习：基础语法示例

第 4-7 天：所有权系统
├─ 阅读：核心底层原理 - 所有权部分
├─ 运行：cargo run --example memory_layout
└─ 练习：src/ownership/ownership.rs

第 8-10 天：借用与引用
├─ 阅读：核心底层原理 - 借用部分
├─ 运行：cargo run --example borrow_checker
└─ 练习：src/ownership/borrowing.rs

第 11-14 天：生命周期
├─ 阅读：核心底层原理 - 生命周期部分
├─ 运行：cargo run --example lifetime_deep_dive
└─ 练习：src/ownership/lifetimes.rs
```

### 中级（2-3 周）

```
第 1 周：类型系统
├─ 结构体与枚举
├─ 泛型与 trait
└─ 错误处理

第 2 周：并发编程
├─ 多线程
├─ 消息传递
└─ 共享状态

第 3 周：异步编程
├─ async/await
├─ tokio 运行时
└─ 实战项目
```

### 高级（持续学习）

```
进阶主题：
├─ unsafe Rust
├─ 宏系统
├─ 高级 trait
├─ 生命周期子类型化
└─ 性能优化
```

## 📊 文档使用矩阵

不同场景下应该查看的文档：

| 场景 | 推荐文档 |
|------|---------|
| 忘记语法 | 快速参考手册 |
| 理解概念 | 核心底层原理详解 |
| 可视化理解 | 可视化指南 |
| 不知道怎么学 | 学习路径 |
| 编译错误 | 快速参考 + 核心原理 |
| 类型标注问题 | 类型标注指南 |
| 生命周期错误 | 生命周期深度剖析示例 |
| 借用检查错误 | 借用检查器示例 |
| 性能优化 | 核心原理 - 性能部分 |

## 🔍 快速查找

### 按主题查找

- **所有权**: [核心原理#1](CORE_PRINCIPLES.md#1-所有权-ownership-底层原理) | [可视化](VISUAL_GUIDE.md#所有权可视化)
- **借用**: [核心原理#2](CORE_PRINCIPLES.md#2-借用-borrowing-底层原理) | [示例](../examples/borrow_checker.rs)
- **切片**: [核心原理#3](CORE_PRINCIPLES.md#3-切片-slice-底层原理) | [可视化](VISUAL_GUIDE.md#切片内存图)
- **生命周期**: [核心原理#4](CORE_PRINCIPLES.md#4-生命周期-lifetime-底层原理) | [示例](../examples/lifetime_deep_dive.rs)
- **类型标注**: [类型标注指南](TYPE_ANNOTATIONS_GUIDE.md) | [速查卡](TYPE_ANNOTATIONS_CHEATSHEET.md)
- **核心类型**: [核心类型详解](CORE_TYPES.md) | [速查卡](CORE_TYPES_CHEATSHEET.md)

### 按问题查找

**Q: 为什么我的代码不能编译？**
- 所有权错误 → [核心原理 - 所有权](CORE_PRINCIPLES.md#1-所有权-ownership-底层原理)
- 借用错误 → [借用检查器示例](../examples/borrow_checker.rs)
- 生命周期错误 → [生命周期示例](../examples/lifetime_deep_dive.rs)

**Q: 我应该用 String 还是 &str？**
→ [快速参考 - 检查清单](QUICK_REFERENCE.md#检查清单)

**Q: 什么时候需要类型标注？**
→ [类型标注指南](TYPE_ANNOTATIONS_GUIDE.md)

**Q: Box/Option/Result 怎么用？**
→ [核心类型详解](CORE_TYPES.md)

**Q: 如何理解内存布局？**
→ [可视化指南](VISUAL_GUIDE.md#内存布局对比)

## 💡 学习建议

1. **循序渐进**：按照学习路径，不要跳过基础
2. **动手实践**：运行所有示例程序，修改代码观察结果
3. **理解原理**：不仅知道"是什么"，更要理解"为什么"
4. **反复阅读**：核心概念需要多次阅读才能深刻理解
5. **画图理解**：自己画内存布局图加深理解

## 🛠️ 实用命令

```bash
# 检查代码
cargo check

# 运行主程序
cargo run

# 运行示例
cargo run --example <name>

# 运行测试
cargo test

# 查看文档
cargo doc --open

# 格式化代码
cargo fmt

# Lint 检查
cargo clippy
```

## 📱 快速访问

- [返回主 README](../README.md)
- [查看源代码](../src/)
- [查看示例](../examples/)

---

**祝学习愉快！**🎉

如有问题或建议，欢迎提交 Issue。
