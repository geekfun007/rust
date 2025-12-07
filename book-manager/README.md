# 图书管理系统 - Rust 模块化示例

完整的模块化项目示例，展示 Rust 的模块组织最佳实践。

## 项目结构

```
book-manager/
├── Cargo.toml
├── src/
│   ├── lib.rs              # 库入口
│   ├── models/             # 数据模型
│   │   ├── mod.rs
│   │   ├── book.rs
│   │   └── author.rs
│   ├── services/           # 业务逻辑
│   │   ├── mod.rs
│   │   ├── book_service.rs
│   │   └── author_service.rs
│   └── utils/              # 工具函数
│       ├── mod.rs
│       └── validators.rs
└── examples/
    └── basic_usage.rs      # 使用示例
```

## 模块说明

### models - 数据模型
- `Book`: 图书模型，包含书籍信息和方法
- `Author`: 作者模型

### services - 业务逻辑
- `BookService`: 图书服务，处理图书相关业务
- `AuthorService`: 作者服务

### utils - 工具模块
- `validators`: 数据验证函数

## 使用方法

```bash
# 运行示例
cargo run --example basic_usage

# 运行测试
cargo test

# 查看文档
cargo doc --open
```

## 代码示例

```rust
use book_manager::{Book, Author, BookService};

fn main() {
    let author = Author::new(1, "鲁迅".to_string());
    let book = Book::new(1, "狂人日记".to_string(), author.id, 29.99);
    
    let service = BookService::new();
    service.display_book(&book);
}
```

## 学习要点

1. **模块组织**: 清晰的目录结构
2. **可见性控制**: pub, pub(crate) 的使用
3. **重新导出**: 简化 API
4. **测试**: 单元测试和集成测试
