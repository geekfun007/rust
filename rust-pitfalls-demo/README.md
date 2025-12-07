# Rust 注意事项实战项目

展示 Rust 常见陷阱和最佳实践的完整示例项目。

## 项目结构

```
rust-pitfalls-demo/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs                # 库入口
    ├── ownership.rs          # 所有权系统实战
    ├── borrowing.rs          # 借用检查器实战
    ├── lifetime.rs           # 生命周期实战
    ├── error_handling.rs     # 错误处理实战
    ├── performance.rs        # 性能优化实战
    └── concurrency.rs        # 并发编程实战
```

## 模块说明

### 1. ownership.rs - 所有权系统

展示：
- ✅ 值移动后的处理
- ✅ 部分移动的避免
- ✅ 循环中的所有权管理

**关键类型：**
- `Config` - 配置管理器
- `User` - 用户数据
- `LogProcessor` - 日志处理器

### 2. borrowing.rs - 借用检查器

展示：
- ✅ 多个可变借用的解决
- ✅ 可变/不可变借用共存
- ✅ 内部可变性（RefCell）
- ✅ 悬垂引用的避免

**关键类型：**
- `Cache` - 缓存系统
- `Database` - 数据库（内部可变性）
- `StringSlicer` - 字符串切片工具

### 3. lifetime.rs - 生命周期

展示：
- ✅ 生命周期注解
- ✅ 结构体生命周期
- ✅ 生命周期与引用

**关键类型：**
- `ImportantExcerpt` - 持有引用的结构体
- `StringAnalyzer` - 字符串分析器

### 4. error_handling.rs - 错误处理

展示：
- ✅ 自定义错误类型
- ✅ Error trait 实现
- ✅ From trait 转换
- ✅ Result 类型使用

**关键类型：**
- `AppError` - 应用错误
- `FileProcessor` - 文件处理器

### 5. performance.rs - 性能优化

展示：
- ✅ 避免不必要的克隆
- ✅ 预分配容量
- ✅ 使用迭代器
- ✅ Cow 的使用

**关键类型：**
- `StringProcessor` - 字符串处理器
- `VectorBuilder` - 向量构建器
- `ArraySummer` - 数组求和

### 6. concurrency.rs - 并发编程

展示：
- ✅ Arc + Mutex 模式
- ✅ Arc + RwLock 模式
- ✅ 线程安全

**关键类型：**
- `Counter` - 计数器（Mutex）
- `SharedData` - 共享数据（RwLock）

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test ownership
cargo test borrowing
cargo test lifetime
cargo test error_handling
cargo test performance
cargo test concurrency

# 查看测试输出
cargo test -- --nocapture
```

## 使用示例

### 所有权示例

```rust
use rust_pitfalls_demo::ownership::{Config, LogProcessor, LogEntry};

fn main() {
    // 配置管理
    let mut config = Config::new();
    config.set_setting("host".to_string(), "localhost".to_string());
    println!("{:?}", config.get_setting("host"));
    
    // 日志处理
    let mut processor = LogProcessor::new();
    processor.add_log(LogEntry::new(1000, "INFO", "启动"));
    processor.print_all();
}
```

### 借用示例

```rust
use rust_pitfalls_demo::borrowing::{Cache, Database};

fn main() {
    // 缓存系统
    let mut cache = Cache::new();
    cache.set("key1".to_string(), "value1".to_string());
    println!("{:?}", cache.get_and_track("key1"));
    
    // 内部可变性
    let db = Database::new();
    db.add("record".to_string());
    println!("记录数: {}", db.len());
}
```

### 错误处理示例

```rust
use rust_pitfalls_demo::error_handling::{FileProcessor, AppResult};

fn main() -> AppResult<()> {
    let age = FileProcessor::validate_age(25)?;
    println!("验证后的年龄: {}", age);
    Ok(())
}
```

## 学习要点

### 所有权规则
1. 每个值都有且只有一个所有者
2. 当所有者离开作用域，值被释放
3. 值可以被移动或借用

### 借用规则
1. 要么一个可变借用，要么任意多个不可变借用
2. 引用必须总是有效的

### 性能原则
1. 优先使用借用而不是克隆
2. 预分配容量避免重新分配
3. 使用迭代器而不是索引
4. 选择合适的字符串类型

### 并发原则
1. Arc 用于共享所有权
2. Mutex 用于独占访问
3. RwLock 用于读多写少
4. 统一锁顺序避免死锁

## 相关文档

- [Rust注意事项.md](../Rust注意事项.md) - 详细的原理解释
- [The Rust Book](https://doc.rust-lang.org/book/) - 官方教程
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - API 设计指南

## 贡献

欢迎提交 Issue 和 Pull Request！
