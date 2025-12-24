# Rust 模块化管理完全指南

## 目录
- [什么是模块](#什么是模块)
- [核心关键字](#核心关键字)
- [可见性控制](#可见性控制)
- [use 导入](#use-导入)
- [路径系统](#路径系统)
- [文件组织](#文件组织)
- [实战项目](#实战项目)
- [最佳实践](#最佳实践)

---

## 什么是模块

模块是 Rust 代码组织的基本单位，用于：
- ✅ 组织代码结构
- ✅ 控制可见性
- ✅ 避免命名冲突
- ✅ 管理作用域

---

## 核心关键字

### 1. mod - 定义模块

```rust
// 内联模块
mod my_module {
    pub fn function() {
        println!("模块函数");
    }
}

// 文件模块（会查找 my_module.rs 或 my_module/mod.rs）
mod my_module;
```

### 2. pub - 公开可见

```rust
mod my_module {
    pub fn public_fn() {}      // 公开
    fn private_fn() {}         // 私有
}
```

### 3. use - 导入路径

```rust
use std::collections::HashMap;
use my_module::function as my_fn;
```

### 4. crate - crate 根

```rust
// 从 crate 根开始的绝对路径
use crate::models::User;
```

### 5. super - 父模块

```rust
// 访问父模块
super::parent_function();
```

### 6. self - 当前模块

```rust
// 当前模块的项
use self::my_function;
```

---

## 可见性控制

### 可见性级别

| 修饰符 | 可见范围 | 适用场景 |
|--------|---------|---------|
| (无) | 私有 | 默认，仅当前模块 |
| `pub` | 公开 | 所有人可见 |
| `pub(crate)` | Crate 内 | 整个 crate 内可见 |
| `pub(super)` | 父模块 | 父模块及其子模块 |
| `pub(in path)` | 指定路径 | 特定路径可见 |

### 示例

```rust
mod outer {
    // 默认私有
    fn private() {}
    
    // 公开
    pub fn public() {}
    
    // Crate 内可见
    pub(crate) fn crate_visible() {}
    
    // 父模块可见
    pub(super) fn super_visible() {}
    
    mod inner {
        pub fn test() {
            // 可以访问 super_visible
            super::super_visible();
        }
    }
}
```

### 结构体和枚举的可见性

```rust
mod my_mod {
    // 公开结构体，但字段默认私有
    pub struct PublicStruct {
        pub public_field: i32,    // 公开字段
        private_field: i32,        // 私有字段
    }
    
    impl PublicStruct {
        // 提供构造函数
        pub fn new(value: i32) -> Self {
            PublicStruct {
                public_field: value,
                private_field: value * 2,
            }
        }
        
        // 访问私有字段的方法
        pub fn get_private(&self) -> i32 {
            self.private_field
        }
    }
    
    // 公开枚举，所有变体自动公开
    pub enum Status {
        Active,
        Inactive,
    }
}
```

---

## use 导入

### 基础导入

```rust
// 单个导入
use std::collections::HashMap;

// 多个导入
use std::collections::{HashMap, HashSet, BTreeMap};

// 导入所有
use std::collections::*;  // ⚠️ 谨慎使用

// 重命名
use std::io::Result as IoResult;
use std::fmt::Result as FmtResult;
```

### 嵌套导入

```rust
// 分组导入
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    fmt,
};
```

### 重导出

```rust
// lib.rs
mod internal_module;

// 重导出，使外部可以直接使用
pub use internal_module::{Type1, Type2};
```

### 导入建议

✅ **推荐**:
```rust
use std::collections::HashMap;
use crate::models::User;
```

❌ **避免**:
```rust
use std::*;  // 过于宽泛
use super::super::super::function;  // 路径过深
```

---

## 路径系统

### 绝对路径 vs 相对路径

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径（推荐）
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

### 路径关键字

#### crate - 根路径

```rust
// 从 crate 根开始
use crate::models::User;
```

#### super - 父模块

```rust
mod parent {
    pub fn parent_fn() {}
    
    mod child {
        pub fn call_parent() {
            // 访问父模块
            super::parent_fn();
        }
    }
}
```

#### self - 当前模块

```rust
mod my_mod {
    pub fn function() {}
    
    pub fn caller() {
        // 显式当前模块
        self::function();
    }
}
```

### 路径选择建议

| 场景 | 推荐路径 | 原因 |
|------|---------|------|
| 稳定API | 绝对路径 | 重构时不易出错 |
| 相关模块 | 相对路径 | 更简洁 |
| 父模块 | `super::` | 清晰表达关系 |

---

## 文件组织

### 模式 1: 单文件模块

**适用**: 小型模块（< 200 行）

```
src/
├── main.rs
├── config.rs       // mod config;
└── utils.rs        // mod utils;
```

**main.rs**:
```rust
mod config;
mod utils;

fn main() {
    config::load();
    utils::helper();
}
```

---

### 模式 2: 目录模块（旧式）

**适用**: 大型模块

```
src/
├── main.rs
└── models/
    ├── mod.rs      // 模块根
    ├── user.rs
    └── post.rs
```

**main.rs**:
```rust
mod models;  // 查找 models/mod.rs

use models::User;
```

**models/mod.rs**:
```rust
mod user;
mod post;

pub use user::User;
pub use post::Post;
```

---

### 模式 3: 新式目录模块（推荐）

**Rust 2018+ 推荐模式**

```
src/
├── main.rs
├── models.rs       // 模块根
└── models/
    ├── user.rs
    └── post.rs
```

**main.rs**:
```rust
mod models;
```

**models.rs**:
```rust
mod user;
mod post;

pub use user::User;
pub use post::Post;
```

**优点**:
- ✅ 避免大量 `mod.rs` 文件
- ✅ 更清晰的模块入口
- ✅ 与单文件模块一致的风格

---

## 实战项目

### 完整项目结构

```
modular_project/
├── Cargo.toml
└── src/
    ├── main.rs          // 入口
    ├── config.rs        // 配置
    ├── models/          // 数据模型
    │   ├── mod.rs
    │   ├── user.rs
    │   └── post.rs
    ├── handlers/        // 请求处理
    │   ├── mod.rs
    │   ├── user_handler.rs
    │   └── post_handler.rs
    ├── services/        // 业务逻辑
    │   ├── mod.rs
    │   ├── auth_service.rs
    │   └── database_service.rs
    └── utils/           // 工具函数
        ├── mod.rs
        ├── validation.rs
        └── formatting.rs
```

### main.rs - 入口文件

```rust
// 声明模块
mod config;
mod models;
mod handlers;
mod services;
mod utils;

// 导入需要的类型
use config::Config;
use models::User;
use handlers::user_handler;
use services::auth_service;

fn main() {
    let config = Config::load();
    let user = User::new(1, "Alice", "alice@example.com");
    
    if auth_service::authenticate("token") {
        match user_handler::get_user(1) {
            Ok(user) => println!("用户: {}", user.name()),
            Err(e) => println!("错误: {}", e),
        }
    }
}
```

### models/mod.rs - 模块根

```rust
// 声明子模块
mod user;
mod post;

// 重导出公共类型
pub use user::User;
pub use post::Post;
```

### models/user.rs - 用户模型

```rust
pub struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    pub fn new(id: u32, name: &str, email: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
        }
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}
```

### handlers/mod.rs - 处理器根

```rust
pub mod user_handler;
pub mod post_handler;

// 定义通用类型
pub type Result<T> = std::result::Result<T, String>;
```

### handlers/user_handler.rs - 用户处理器

```rust
use crate::models::User;
use super::Result;

pub fn get_user(id: u32) -> Result<User> {
    if id > 0 {
        Ok(User::new(id, "Alice", "alice@example.com"))
    } else {
        Err("Invalid user ID".to_string())
    }
}
```

---

## 最佳实践

### 1. 模块层次设计

✅ **推荐结构**:
```
src/
├── main.rs/lib.rs    // 根
├── config.rs         // 配置
├── error.rs          // 错误类型
├── models/           // 数据层
├── services/         // 业务层
├── handlers/         // 控制层
└── utils/            // 工具层
```

❌ **避免**:
```
src/
├── main.rs
└── everything/       // 过于扁平
    ├── user_model.rs
    ├── user_handler.rs
    ├── post_model.rs
    └── post_handler.rs
```

---

### 2. 可见性原则

✅ **最小可见性**:
```rust
// 默认私有
fn helper() {}

// 仅在需要时公开
pub fn public_api() {
    helper();  // 内部使用
}
```

❌ **过度公开**:
```rust
// 不必要的 pub
pub fn internal_helper() {}  // ❌
```

---

### 3. use 导入规范

✅ **清晰的导入**:
```rust
use std::collections::HashMap;
use crate::models::{User, Post};
use crate::services::auth;
```

❌ **避免**:
```rust
use std::*;  // ❌ 过于宽泛
use super::super::super::module;  // ❌ 过深
```

---

### 4. 重导出策略

✅ **lib.rs 重导出**:
```rust
// lib.rs
pub mod models;
pub mod services;

// 重导出常用类型
pub use models::{User, Post};
pub use services::{Error, Result};
```

**优点**: 用户可以直接 `use mylib::User` 而不是 `use mylib::models::User`

---

### 5. prelude 模式

```rust
// src/prelude.rs
pub use crate::{
    error::{Error, Result},
    models::{User, Post},
    services::AuthService,
};

// src/lib.rs
pub mod prelude;

// 用户代码
use mylib::prelude::*;
```

---

### 6. 模块文档

```rust
//! 用户模块
//!
//! 提供用户相关的数据结构和操作。
//!
//! # 示例
//!
//! ```
//! use mylib::models::User;
//!
//! let user = User::new(1, "Alice", "alice@example.com");
//! ```

pub struct User {
    // ...
}
```

---

### 7. 避免循环依赖

❌ **错误**:
```
models/user.rs  → depends on → handlers/user_handler.rs
                                      ↓
handlers/user_handler.rs → depends on → models/user.rs
```

✅ **正确**:
```
单向依赖:
handlers → models
services → models
utils    → (独立)
```

---

### 8. 模块大小建议

| 模块大小 | 建议 |
|---------|------|
| < 200 行 | 单文件 |
| 200-1000 行 | 考虑拆分 |
| > 1000 行 | 必须拆分为子模块 |

---

## 常见问题

### Q1: mod.rs vs 新式组织？

**A**: 推荐新式（Rust 2018+）

```
✅ 新式（推荐）:
src/
├── models.rs
└── models/
    └── user.rs

❌ 旧式（避免）:
src/
└── models/
    ├── mod.rs
    └── user.rs
```

---

### Q2: 何时使用 pub(crate)？

**A**: 内部 API，crate 内共享但不对外暴露

```rust
// lib.rs
pub(crate) fn internal_api() {
    // crate 内可用，但不会导出
}
```

---

### Q3: 路径太长怎么办？

**A**: 使用 use 简化

```rust
// ❌ 冗长
crate::models::user::User::new()

// ✅ 简洁
use crate::models::User;
User::new()
```

---

### Q4: 如何组织测试模块？

**A**: 使用 `#[cfg(test)]`

```rust
// src/models/user.rs
pub struct User { /* ... */ }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new(1, "Alice", "alice@example.com");
        assert_eq!(user.id(), 1);
    }
}
```

---

## 快速参考

### 关键字速查

```rust
mod my_module;              // 声明模块
pub mod my_module;          // 公开模块

use crate::models::User;    // 绝对路径
use super::parent_fn;       // 父模块
use self::my_fn;            // 当前模块

pub fn public() {}          // 公开函数
pub(crate) fn crate_fn() {} // Crate 内可见
pub(super) fn super_fn() {} // 父模块可见
```

### 文件组织速查

```
单文件:  src/module.rs
目录:    src/module.rs + src/module/
旧式:    src/module/mod.rs
```

---

## 总结

### 核心概念

1. **模块系统**: 组织代码的基础
2. **可见性**: 控制 API 边界
3. **路径**: 导航模块树
4. **文件组织**: 物理结构映射

### 最佳实践

✅ 清晰的模块层次  
✅ 最小可见性原则  
✅ 使用新式组织（Rust 2018+）  
✅ 重导出常用类型  
✅ 避免循环依赖  
✅ 适当拆分大模块  

### 学习路径

1. 理解 mod, pub, use 关键字
2. 掌握可见性控制
3. 学习文件组织模式
4. 实践模块化项目
5. 优化项目结构

---

**运行示例**:
```bash
# 概念教程
cargo run --bin mod_detailed

# 实战项目
cd examples/modular_project && cargo run
```

**查看项目结构**:
```bash
cd examples/modular_project
tree src/
```
