# Rust 详解与实战教程

这是一个全面的 Rust 编程语言教程，包含语法、数据类型、核心特性、并发编程、I/O 操作和网络编程的实战示例。

## 目录结构

```
rust_tutorial/
├── src/
│   ├── main.rs                    # 主程序入口
│   ├── basics/
│   │   ├── mod.rs                 # 基础语法模块
│   │   ├── syntax.rs              # 语法基础
│   │   ├── data_types.rs          # 数据类型
│   │   ├── functions.rs           # 函数和逻辑
│   │   └── control_flow.rs        # 控制流
│   ├── types/
│   │   ├── mod.rs                 # 类型模块
│   │   ├── primitives.rs          # 基本类型 (int, float, bool)
│   │   ├── strings.rs             # String vs str
│   │   ├── collections.rs         # tuple, array, vec
│   │   ├── structs.rs             # 结构体与方法
│   │   ├── enums.rs               # 枚举与方法
│   │   ├── datetime.rs            # 日期时间
│   │   ├── regex.rs               # 正则表达式
│   │   └── errors.rs              # 错误处理
│   ├── ownership/
│   │   ├── mod.rs                 # 所有权模块
│   │   ├── ownership.rs           # 所有权
│   │   ├── borrowing.rs           # 借用
│   │   ├── slices.rs              # 切片
│   │   └── lifetimes.rs           # 生命周期
│   ├── concurrency/
│   │   ├── mod.rs                 # 并发模块
│   │   ├── threads.rs             # 线程
│   │   ├── async_await.rs         # 异步编程
│   │   └── channels.rs            # 消息传递
│   ├── io/
│   │   ├── mod.rs                 # I/O 模块
│   │   ├── filesystem.rs          # 文件系统操作
│   │   └── files.rs               # 文件读写
│   ├── network/
│   │   ├── mod.rs                 # 网络模块
│   │   ├── tcp_server.rs          # TCP 服务器
│   │   ├── tcp_client.rs          # TCP 客户端
│   │   ├── http_server.rs         # HTTP 服务器
│   │   └── http_client.rs         # HTTP 客户端
│   └── best_practices/
│       ├── mod.rs                 # 最佳实践模块
│       └── guidelines.rs          # 编程规范与注意事项
├── examples/                       # 独立示例程序
├── Cargo.toml                     # 项目配置
└── README.md                      # 本文件
```

## 快速开始

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 运行示例

```bash
# 编译并运行主程序
cargo run

# 运行特定示例
cargo run --example <示例名称>

# 运行测试
cargo test

# 构建发布版本
cargo build --release
```

## 学习路径

### 1. 基础语法 (basics/)
- 变量与常量
- 数据类型
- 函数定义
- 控制流 (if, loop, while, for)
- 模式匹配

### 2. 数据类型详解 (types/)
- **基本类型**: i8, i16, i32, i64, i128, u8-u128, f32, f64, bool, char
- **字符串**: String vs &str 的区别与使用
- **集合类型**: tuple, array, Vec, HashMap, HashSet
- **自定义类型**: struct 结构体与方法实现
- **枚举**: enum 与模式匹配
- **时间处理**: chrono 库的使用
- **正则表达式**: regex 库的使用
- **错误处理**: Result, Option, 自定义错误类型

### 3. 核心特性 (ownership/)
- **所有权 (Ownership)**: Rust 最重要的特性
- **借用 (Borrowing)**: 不可变借用与可变借用
- **切片 (Slices)**: 引用集合的一部分
- **生命周期 (Lifetimes)**: 引用的作用域

### 4. 逻辑与函数 (basics/functions.rs)
- 函数定义与调用
- 闭包 (Closures)
- 迭代器 (Iterators)
- 高阶函数

### 5. 并发编程 (concurrency/)
- **多线程**: thread::spawn, Arc, Mutex
- **异步编程**: async/await, tokio
- **消息传递**: channel, mpsc

### 6. I/O 操作 (io/)
- 文件读写
- 目录操作
- 路径处理
- 标准输入输出

### 7. 网络编程 (network/)
- TCP 服务器与客户端
- HTTP 服务器 (使用 tokio + warp/axum)
- HTTP 客户端 (使用 reqwest)
- WebSocket 通信

### 8. 最佳实践 (best_practices/)
- 代码组织与模块化
- 错误处理最佳实践
- 性能优化技巧
- 安全编程指南
- 测试策略
- 文档编写

## 重要概念速查

### 所有权规则
1. 每个值都有一个所有者
2. 值在任一时刻只能有一个所有者
3. 当所有者离开作用域，值将被丢弃

### 借用规则
1. 在任意时刻，要么只有一个可变引用，要么有任意数量的不可变引用
2. 引用必须总是有效的

### String vs &str
- `String`: 可增长的、堆分配的字符串类型
- `&str`: 字符串切片，通常指向某处的 UTF-8 编码字符串数据

### Vec vs Array
- `array`: 固定长度，栈分配 `[T; N]`
- `Vec`: 动态长度，堆分配 `Vec<T>`

## 常用依赖库

```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }  # 异步运行时
serde = { version = "1.0", features = ["derive"] }  # 序列化/反序列化
chrono = "0.4"                                      # 日期时间
regex = "1.10"                                      # 正则表达式
reqwest = { version = "0.11", features = ["json"] } # HTTP 客户端
axum = "0.7"                                        # Web 框架
anyhow = "1.0"                                      # 错误处理
thiserror = "1.0"                                   # 错误派生宏
```

## 学习建议

1. **按顺序学习**: 从基础语法开始，逐步深入到所有权系统
2. **动手实践**: 每个概念都有对应的可运行示例
3. **理解所有权**: 这是 Rust 的核心，需要多加练习
4. **利用编译器**: Rust 编译器的错误提示非常详细，是最好的老师
5. **阅读标准库**: 标准库代码是学习 Rust 最佳实践的好材料

## 常见问题

### 1. 什么时候使用 String，什么时候使用 &str？
- 需要修改字符串或拥有所有权时使用 `String`
- 只是读取或传递字符串时使用 `&str`

### 2. 如何选择 Box, Rc, Arc？
- `Box<T>`: 单一所有权，堆分配
- `Rc<T>`: 多个所有权，单线程
- `Arc<T>`: 多个所有权，多线程

### 3. 何时使用 unwrap() vs expect() vs ?
- 原型开发时可以用 `unwrap()`
- 生产代码用 `expect()` 或 `?` 操作符
- `?` 适合在返回 Result 的函数中传播错误

## 资源链接

- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)
- [Rust 异步编程](https://rust-lang.github.io/async-book/)

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个教程！

## 许可证

MIT License

## 文档索引

### 核心文档
- 📘 [核心底层原理详解](docs/CORE_PRINCIPLES.md) - 深入理解 ownership/borrowing/slice/lifetime
- 📋 [快速参考手册](docs/QUICK_REFERENCE.md) - 常用概念和语法速查
- 🎨 [可视化指南](docs/VISUAL_GUIDE.md) - 内存布局和概念图示
- 🗺️ [学习路径](docs/LEARNING_PATH.md) - 推荐的学习顺序
- 📝 [类型标注指南](docs/TYPE_ANNOTATIONS_GUIDE.md) - let 类型声明与泛型
- 🎯 [类型标注速查卡](docs/TYPE_ANNOTATIONS_CHEATSHEET.md) - 快速决策参考

### 示例程序
```bash
# 内存布局与底层原理
cargo run --example memory_layout

# 借用检查器演示
cargo run --example borrow_checker

# 生命周期深度剖析
cargo run --example lifetime_deep_dive

# 类型标注与泛型
cargo run --example type_annotations
```

## 专题示例

除了主教程外，还提供了以下专题示例：

### 类型标注与泛型
详细说明 let 何时需要类型声明，以及泛型类型如何使用：

```bash
# 运行类型标注示例
cargo run --example type_annotations

# 查看详细文档
cat docs/TYPE_ANNOTATIONS_GUIDE.md
```

**涵盖内容：**
- let 变量何时需要类型标注
- 编译器类型推断规则
- 泛型类型声明（结构体、枚举、函数）
- Turbofish 语法 `::<T>` 详解
- 复杂泛型场景（生命周期、trait 约束）
- 实战最佳实践

### 核心底层原理深度剖析

深入理解 Rust 的核心概念和底层实现：

```bash
# 查看底层原理详解文档
cat docs/CORE_PRINCIPLES.md

# 查看核心类型详解
cat docs/CORE_TYPES.md

# 运行内存布局演示
cargo run --example memory_layout

# 运行借用检查器演示
cargo run --example borrow_checker

# 运行生命周期深度演示
cargo run --example lifetime_deep_dive

# 运行核心类型演示
cargo run --example core_types
```

**涵盖内容：**

#### 1. 所有权 (Ownership)
- 栈与堆的内存布局
- Move vs Copy vs Clone 的底层实现
- Drop trait 与 RAII 机制
- 零成本抽象的实现原理

#### 2. 借用 (Borrowing)
- 引用的内存表示（瘦指针 vs 胖指针）
- 借用检查器的工作原理
- 非词法作用域生命周期 (NLL)
- 内部可变性 (RefCell/Cell)
- 分割借用

#### 3. 切片 (Slice)
- 切片的胖指针结构（ptr + len）
- 字符串切片 vs 数组切片
- 动态大小类型 (DST)
- 零成本抽象验证

#### 4. 生命周期 (Lifetime)
- 生命周期的编译时本质
- 生命周期消除规则（三大规则）
- 结构体中的生命周期
- 生命周期约束与子类型化
- 'static 生命周期
- 高阶 trait 约束 (HRTB)

#### 5. 性能分析
- 类型大小与内存对齐
- 引用 vs 克隆的性能对比
- 迭代器零成本抽象
- 编译器优化（内联、SIMD）

### 核心类型与 Trait 详解

深入理解 Rust 的内置类型和常用 trait：

```bash
# 查看核心类型详解文档
cat docs/CORE_TYPES.md

# 运行核心类型演示
cargo run --example core_types
```

**涵盖内容：**

#### 1. Box<T> - 智能指针
- 堆内存分配
- 递归类型支持
- Trait 对象（动态分派）
- 性能开销分析

#### 2. Option<T> - 可选值
- 替代空指针
- 空指针优化
- 丰富的组合器方法
- ? 操作符

#### 3. Result<T, E> - 错误处理
- 可恢复错误
- 错误传播链
- 与 Option 转换
- 最佳实践

#### 4. fmt::Display - 格式化输出
- Display vs Debug
- 自定义格式化
- format! 宏家族
- 其他格式化 trait

#### 5. 其他核心 Trait
- From & Into（类型转换）
- Default（默认值）
- Clone & Copy（复制）
- Drop（析构）
- Iterator（迭代器）
