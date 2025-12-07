# Rust 语言详解

## 目录
- [1. 命令行操作](#1-命令行操作)
- [2. 模块化](#2-模块化)
- [3. 语法基础](#3-语法基础)
- [4. 数据类型](#4-数据类型)
- [5. 逻辑控制](#5-逻辑控制)
- [6. 函数](#6-函数)
- [7. IO 操作](#7-io-操作)
- [8. HTTP 编程](#8-http-编程)
- [9. 异步编程](#9-异步编程)

---

## 1. 命令行操作

### 1.1 安装 Rust

```bash
# 安装 Rust（Unix-like 系统）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 更新 Rust
rustup update

# 查看版本
rustc --version
cargo --version
```

### 1.2 Cargo 基础命令

```bash
# 创建新项目
cargo new my_project         # 创建二进制项目
cargo new my_lib --lib       # 创建库项目

# 构建项目
cargo build                  # 调试版本
cargo build --release        # 发布版本

# 运行项目
cargo run                    # 运行项目
cargo run --release          # 运行发布版本

# 测试
cargo test                   # 运行测试
cargo test test_name         # 运行特定测试

# 文档
cargo doc                    # 生成文档
cargo doc --open             # 生成并打开文档

# 其他
cargo check                  # 快速检查代码（不生成可执行文件）
cargo clean                  # 清理构建产物
cargo fmt                    # 格式化代码
cargo clippy                 # 代码检查（需要安装）
```

### 1.3 项目结构

```
my_project/
├── Cargo.toml              # 项目配置文件
├── Cargo.lock              # 依赖锁定文件
├── src/
│   ├── main.rs             # 二进制项目入口
│   ├── lib.rs              # 库项目入口
│   └── bin/                # 额外的二进制文件
├── tests/                  # 集成测试
├── benches/                # 性能测试
└── examples/               # 示例代码
```

---

## 2. 模块化系统详解

### 2.1 模块基础

```rust
// 定义模块
mod network {
    pub fn connect() {
        println!("连接网络");
    }
    
    fn internal_function() {
        println!("内部函数");
    }
    
    // 嵌套模块
    pub mod server {
        pub fn start() {
            println!("启动服务器");
        }
    }
}

// 使用模块
fn main() {
    network::connect();
    network::server::start();
}
```

### 2.2 文件模块系统

```rust
// src/lib.rs 或 src/main.rs
mod utils;          // 引入 src/utils.rs
mod database;       // 引入 src/database.rs 或 src/database/mod.rs

// src/utils.rs
pub fn helper_function() {
    println!("辅助函数");
}

// src/database/mod.rs
pub mod connection;
pub mod query;
```

### 2.3 使用 use 关键字

```rust
use std::collections::HashMap;
use std::io::{self, Write};  // 导入多个
use std::fmt::Result;
use std::io::Result as IoResult;  // 别名

// 重新导出
pub use self::network::connect;

// glob 导入（不推荐过度使用）
use std::collections::*;
```

### 2.4 模块可见性

```rust
pub mod outer {
    pub fn public_function() {}
    
    fn private_function() {}
    
    pub mod inner {
        pub fn inner_public() {}
        
        pub(crate) fn crate_visible() {}      // 整个 crate 可见
        pub(super) fn parent_visible() {}      // 父模块可见
        pub(in crate::outer) fn outer_visible() {}  // 指定路径可见
    }
}
```

---

## 2.5 实战：完整的模块化项目示例

### 示例 1：图书管理系统

**项目结构：**
```
book_manager/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── models/
    │   ├── mod.rs
    │   ├── book.rs
    │   └── author.rs
    ├── services/
    │   ├── mod.rs
    │   ├── book_service.rs
    │   └── author_service.rs
    └── utils/
        ├── mod.rs
        └── validators.rs
```

**src/main.rs:**
```rust
mod models;
mod services;
mod utils;

use models::{Book, Author};
use services::{BookService, AuthorService};

fn main() {
    // 创建作者
    let author = Author::new(1, "鲁迅".to_string());
    
    // 创建图书
    let book = Book::new(
        1,
        "狂人日记".to_string(),
        author.id,
        29.99,
    );
    
    // 使用服务
    let book_service = BookService::new();
    book_service.display_book(&book);
    
    let author_service = AuthorService::new();
    author_service.display_author(&author);
}
```

**src/models/mod.rs:**
```rust
pub mod book;
pub mod author;

// 重新导出，方便使用
pub use book::Book;
pub use author::Author;
```

**src/models/book.rs:**
```rust
use crate::utils::validators;

#[derive(Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author_id: u32,
    pub price: f64,
}

impl Book {
    pub fn new(id: u32, title: String, author_id: u32, price: f64) -> Self {
        assert!(validators::validate_price(price), "价格必须为正数");
        
        Self {
            id,
            title,
            author_id,
            price,
        }
    }
    
    pub fn set_price(&mut self, new_price: f64) {
        if validators::validate_price(new_price) {
            self.price = new_price;
        } else {
            panic!("无效的价格");
        }
    }
    
    pub fn apply_discount(&mut self, discount: f64) {
        assert!(discount >= 0.0 && discount <= 1.0, "折扣必须在 0-1 之间");
        self.price = self.price * (1.0 - discount);
    }
}
```

**src/models/author.rs:**
```rust
#[derive(Debug, Clone)]
pub struct Author {
    pub id: u32,
    pub name: String,
}

impl Author {
    pub fn new(id: u32, name: String) -> Self {
        assert!(!name.is_empty(), "作者名不能为空");
        
        Self { id, name }
    }
}
```

**src/services/mod.rs:**
```rust
pub mod book_service;
pub mod author_service;

pub use book_service::BookService;
pub use author_service::AuthorService;
```

**src/services/book_service.rs:**
```rust
use crate::models::Book;

pub struct BookService {
    // 可以添加数据库连接等字段
}

impl BookService {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn display_book(&self, book: &Book) {
        println!("=== 图书信息 ===");
        println!("ID: {}", book.id);
        println!("标题: {}", book.title);
        println!("作者ID: {}", book.author_id);
        println!("价格: ¥{:.2}", book.price);
    }
    
    pub fn calculate_total(&self, books: &[Book]) -> f64 {
        books.iter().map(|b| b.price).sum()
    }
}

impl Default for BookService {
    fn default() -> Self {
        Self::new()
    }
}
```

**src/services/author_service.rs:**
```rust
use crate::models::Author;

pub struct AuthorService;

impl AuthorService {
    pub fn new() -> Self {
        Self
    }
    
    pub fn display_author(&self, author: &Author) {
        println!("=== 作者信息 ===");
        println!("ID: {}", author.id);
        println!("姓名: {}", author.name);
    }
}
```

**src/utils/mod.rs:**
```rust
pub mod validators;

// 可以直接在这里定义一些工具函数
pub fn format_price(price: f64) -> String {
    format!("¥{:.2}", price)
}
```

**src/utils/validators.rs:**
```rust
pub fn validate_price(price: f64) -> bool {
    price > 0.0
}

pub fn validate_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 100
}
```

---

### 示例 2：用户认证系统

**项目结构：**
```
auth_system/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── auth/
    │   ├── mod.rs
    │   ├── login.rs
    │   ├── register.rs
    │   └── password.rs
    ├── models/
    │   ├── mod.rs
    │   └── user.rs
    └── config/
        ├── mod.rs
        └── database.rs
```

**src/lib.rs:**
```rust
pub mod auth;
pub mod models;
pub mod config;

// 导出常用类型
pub use models::User;
pub use auth::{login, register, change_password};
```

**src/auth/mod.rs:**
```rust
mod login;
mod register;
mod password;

// 重新导出主要功能
pub use login::login;
pub use register::register;
pub use password::change_password;

// 内部使用的辅助函数
pub(crate) fn hash_password(password: &str) -> String {
    // 简化示例，实际应使用 bcrypt 等
    format!("hashed_{}", password)
}

pub(crate) fn verify_password(password: &str, hash: &str) -> bool {
    hash == format!("hashed_{}", password)
}
```

**src/auth/login.rs:**
```rust
use crate::models::User;
use super::{verify_password};

pub fn login(username: &str, password: &str) -> Result<User, String> {
    // 这里应该从数据库查询用户
    // 简化示例
    
    let user = User {
        id: 1,
        username: username.to_string(),
        password_hash: super::hash_password("correct_password"),
    };
    
    if verify_password(password, &user.password_hash) {
        Ok(user)
    } else {
        Err("密码错误".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_login_success() {
        let result = login("alice", "correct_password");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_login_failure() {
        let result = login("alice", "wrong_password");
        assert!(result.is_err());
    }
}
```

**src/auth/register.rs:**
```rust
use crate::models::User;
use super::hash_password;

pub fn register(username: &str, password: &str) -> Result<User, String> {
    // 验证用户名
    if username.len() < 3 {
        return Err("用户名至少 3 个字符".to_string());
    }
    
    // 验证密码
    if password.len() < 8 {
        return Err("密码至少 8 个字符".to_string());
    }
    
    // 创建用户
    let user = User {
        id: generate_user_id(),
        username: username.to_string(),
        password_hash: hash_password(password),
    };
    
    Ok(user)
}

fn generate_user_id() -> u32 {
    // 简化示例
    1
}
```

**src/auth/password.rs:**
```rust
use crate::models::User;
use super::{hash_password, verify_password};

pub fn change_password(
    user: &mut User,
    old_password: &str,
    new_password: &str,
) -> Result<(), String> {
    // 验证旧密码
    if !verify_password(old_password, &user.password_hash) {
        return Err("旧密码错误".to_string());
    }
    
    // 验证新密码
    if new_password.len() < 8 {
        return Err("新密码至少 8 个字符".to_string());
    }
    
    // 更新密码
    user.password_hash = hash_password(new_password);
    
    Ok(())
}
```

**src/models/mod.rs:**
```rust
mod user;

pub use user::User;
```

**src/models/user.rs:**
```rust
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub(crate) password_hash: String,  // 只在 crate 内可见
}

impl User {
    pub fn new(id: u32, username: String, password_hash: String) -> Self {
        Self {
            id,
            username,
            password_hash,
        }
    }
    
    pub fn display(&self) {
        println!("用户 ID: {}", self.id);
        println!("用户名: {}", self.username);
        // 不显示密码哈希
    }
}
```

**src/main.rs:**
```rust
use auth_system::{register, login, change_password};

fn main() {
    println!("=== 用户认证系统 ===\n");
    
    // 注册用户
    match register("alice", "password123") {
        Ok(user) => {
            println!("注册成功！");
            user.display();
        }
        Err(e) => println!("注册失败: {}", e),
    }
    
    println!();
    
    // 登录
    match login("alice", "correct_password") {
        Ok(user) => {
            println!("登录成功！");
            user.display();
        }
        Err(e) => println!("登录失败: {}", e),
    }
    
    println!();
    
    // 修改密码
    let mut user = register("bob", "oldpassword123").unwrap();
    match change_password(&mut user, "oldpassword123", "newpassword456") {
        Ok(_) => println!("密码修改成功！"),
        Err(e) => println!("密码修改失败: {}", e),
    }
}
```

---

### 示例 3：电商购物车系统

**项目结构：**
```
shopping_cart/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── cart/
    │   ├── mod.rs
    │   ├── cart.rs
    │   └── item.rs
    ├── product/
    │   ├── mod.rs
    │   └── product.rs
    └── discount/
        ├── mod.rs
        └── calculator.rs
```

**完整实现（关键文件）：**

**src/cart/mod.rs:**
```rust
mod cart;
mod item;

pub use cart::Cart;
pub use item::CartItem;
```

**src/cart/cart.rs:**
```rust
use crate::product::Product;
use crate::discount::DiscountCalculator;
use super::CartItem;

pub struct Cart {
    items: Vec<CartItem>,
    discount_calculator: DiscountCalculator,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            discount_calculator: DiscountCalculator::new(),
        }
    }
    
    pub fn add_item(&mut self, product: Product, quantity: u32) {
        if let Some(item) = self.items.iter_mut().find(|i| i.product.id == product.id) {
            item.quantity += quantity;
        } else {
            self.items.push(CartItem::new(product, quantity));
        }
    }
    
    pub fn remove_item(&mut self, product_id: u32) {
        self.items.retain(|item| item.product.id != product_id);
    }
    
    pub fn subtotal(&self) -> f64 {
        self.items.iter()
            .map(|item| item.product.price * item.quantity as f64)
            .sum()
    }
    
    pub fn total(&self) -> f64 {
        let subtotal = self.subtotal();
        self.discount_calculator.apply_discount(subtotal, self.items.len())
    }
    
    pub fn display(&self) {
        println!("=== 购物车 ===");
        for item in &self.items {
            println!("{} x {} = ¥{:.2}", 
                item.product.name,
                item.quantity,
                item.product.price * item.quantity as f64
            );
        }
        println!("小计: ¥{:.2}", self.subtotal());
        println!("总计: ¥{:.2}", self.total());
    }
}
```

**src/cart/item.rs:**
```rust
use crate::product::Product;

pub struct CartItem {
    pub product: Product,
    pub quantity: u32,
}

impl CartItem {
    pub fn new(product: Product, quantity: u32) -> Self {
        Self { product, quantity }
    }
}
```

这些模块化示例展示了：
- ✅ 清晰的项目结构
- ✅ 模块的可见性控制
- ✅ 代码的组织和重用
- ✅ 实际项目的模块化实践

---

## 3. 语法基础

### 3.1 变量与可变性

```rust
fn main() {
    // 不可变变量（默认）
    let x = 5;
    // x = 6;  // 错误：不能修改不可变变量
    
    // 可变变量
    let mut y = 5;
    y = 6;  // 正确
    
    // 常量
    const MAX_POINTS: u32 = 100_000;
    
    // 遮蔽（Shadowing）
    let z = 5;
    let z = z + 1;  // 创建新变量
    let z = "字符串";  // 可以改变类型
}
```

### 3.2 数据类型转换

```rust
fn main() {
    // 显式类型转换
    let x: i32 = 42;
    let y: i64 = x as i64;
    let z: f64 = x as f64;
    
    // 字符串转换
    let s = "42";
    let num: i32 = s.parse().expect("不是数字");
    let num: i32 = s.parse().unwrap();
    
    // 使用 turbofish 语法
    let num = s.parse::<i32>().unwrap();
}
```

### 3.3 注释

```rust
// 单行注释

/*
 * 多行注释
 */

/// 文档注释（用于函数、结构体等）
/// 
/// # Examples
/// 
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

//! 模块级文档注释
```

---

## 4. 数据类型与常用方法

### 4.1 标量类型

```rust
fn main() {
    // 整数类型：i8, i16, i32, i64, i128, isize
    //          u8, u16, u32, u64, u128, usize
    let a: i32 = 42;
    let b: u64 = 100;
    let c = 0xff;        // 十六进制
    let d = 0o77;        // 八进制
    let e = 0b1111_0000; // 二进制
    let f = b'A';        // 字节（u8）
    
    // 浮点类型：f32, f64
    let x = 2.0;      // f64（默认）
    let y: f32 = 3.0;
    
    // 布尔类型
    let t = true;
    let f: bool = false;
    
    // 字符类型（4字节 Unicode）
    let c = 'z';
    let emoji = '😊';
    let chinese = '中';
}
```

### 4.1.1 整数类型常用方法

```rust
fn integer_methods() {
    let num: i32 = 42;
    
    // 数学运算
    println!("绝对值: {}", num.abs());                    // 42
    println!("幂运算: {}", num.pow(2));                   // 1764
    println!("是否为正: {}", num.is_positive());          // true
    
    // 安全运算（避免溢出）
    let result = num.checked_add(100);                    // Some(142)
    let wrapped = num.wrapping_add(i32::MAX);             // 溢出后环绕
    let saturated = num.saturating_add(i32::MAX);         // 饱和到最大值
    
    // 位操作
    println!("前导零: {}", num.leading_zeros());          // 26
    println!("尾随零: {}", num.trailing_zeros());         // 1
    println!("位数: {}", num.count_ones());               // 3
    
    // 类型转换
    let as_u32: u32 = num as u32;
    let as_f64: f64 = num as f64;
    
    // 字节数组转换
    let bytes = num.to_be_bytes();                        // 大端序
    let from_bytes = i32::from_be_bytes([0, 0, 0, 42]);
    
    // 范围
    println!("最小值: {}", i32::MIN);                     // -2147483648
    println!("最大值: {}", i32::MAX);                     // 2147483647
}
```

### 4.1.2 浮点类型常用方法

```rust
fn float_methods() {
    let x: f64 = 3.14159;
    
    // 舍入方法
    println!("向下取整: {}", x.floor());                  // 3.0
    println!("向上取整: {}", x.ceil());                   // 4.0
    println!("四舍五入: {}", x.round());                  // 3.0
    println!("取整数部分: {}", x.trunc());                // 3.0
    println!("小数部分: {}", x.fract());                  // 0.14159
    
    // 数学函数
    println!("绝对值: {}", x.abs());
    println!("平方根: {}", x.sqrt());
    println!("立方根: {}", x.cbrt());
    println!("指数: {}", x.exp());
    println!("对数: {}", x.ln());
    println!("幂运算: {}", x.powf(2.0));
    
    // 三角函数
    use std::f64::consts::PI;
    let angle = PI / 4.0;
    println!("sin: {}", angle.sin());
    println!("cos: {}", angle.cos());
    println!("tan: {}", angle.tan());
    
    // 检查方法
    println!("是否为 NaN: {}", x.is_nan());
    println!("是否有限: {}", x.is_finite());
    println!("是否无限: {}", x.is_infinite());
    
    // 比较
    let y = 3.14;
    println!("最大值: {}", x.max(y));
    println!("最小值: {}", x.min(y));
    println!("限制范围: {}", x.clamp(0.0, 3.0));
}
```

### 4.1.3 布尔类型方法

```rust
fn bool_methods() {
    let flag = true;
    
    // then 方法（条件执行）
    let result = flag.then(|| "这是真的");
    println!("{:?}", result);  // Some("这是真的")
    
    let result = false.then(|| "这不会执行");
    println!("{:?}", result);  // None
    
    // then_some 方法
    let value = flag.then_some(42);
    println!("{:?}", value);  // Some(42)
}
```

### 4.2 复合类型

```rust
fn main() {
    // 元组
    let tuple: (i32, f64, char) = (500, 6.4, 'x');
    let (x, y, z) = tuple;  // 解构
    let first = tuple.0;     // 索引访问
    
    // 数组（固定长度）
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let first = array[0];
    let zeros = [0; 10];  // [0, 0, 0, ..., 0]
    
    // 切片（动态大小）
    let slice: &[i32] = &array[1..3];  // [2, 3]
}
```

### 4.3 字符串类型与方法

```rust
fn main() {
    // String（可变，堆分配）
    let mut s = String::from("hello");
    s.push_str(", world!");
    s.push('!');
    
    // &str（不可变，字符串切片）
    let slice: &str = &s[0..5];
    let literal: &str = "hello";
    
    // 字符串操作
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 被移动
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    
    // 遍历字符串
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    
    for b in "hello".bytes() {
        println!("{}", b);
    }
}
```

### 4.3.1 String 常用方法大全

```rust
fn string_methods() {
    let mut s = String::from("Hello, Rust!");
    
    // 创建方法
    let s1 = String::new();                                // 空字符串
    let s2 = String::from("hello");                        // 从 &str
    let s3 = "hello".to_string();                          // to_string
    let s4 = String::with_capacity(10);                    // 预分配容量
    
    // 追加方法
    s.push_str(", world");                                 // 追加字符串
    s.push('!');                                           // 追加字符
    
    // 插入方法
    s.insert(0, 'X');                                      // 插入字符
    s.insert_str(1, "YZ");                                 // 插入字符串
    
    // 删除方法
    s.pop();                                               // 删除最后一个字符
    s.remove(0);                                           // 删除指定位置
    s.truncate(5);                                         // 截断到指定长度
    s.clear();                                             // 清空
    
    s = String::from("Hello, World!");
    
    // 替换方法
    let s5 = s.replace("World", "Rust");                   // 替换所有
    let s6 = s.replacen("l", "L", 2);                      // 替换前 n 个
    
    // 查询方法
    println!("长度: {}", s.len());                          // 字节长度
    println!("容量: {}", s.capacity());                     // 容量
    println!("是否为空: {}", s.is_empty());                // 是否为空
    println!("包含: {}", s.contains("World"));             // 是否包含
    println!("开头: {}", s.starts_with("Hello"));          // 是否开头
    println!("结尾: {}", s.ends_with("!"));                // 是否结尾
    
    // 查找方法
    if let Some(index) = s.find("World") {                // 查找位置
        println!("找到位置: {}", index);
    }
    
    // 分割方法
    for word in s.split(',') {                             // 按字符分割
        println!("单词: {}", word.trim());
    }
    
    let parts: Vec<&str> = s.split_whitespace().collect(); // 按空白分割
    let lines: Vec<&str> = "a\nb\nc".lines().collect();   // 按行分割
    
    // 大小写转换
    println!("大写: {}", s.to_uppercase());                 // 转大写
    println!("小写: {}", s.to_lowercase());                 // 转小写
    
    // 修剪
    let s7 = "  Hello  ".trim();                           // 两端空白
    let s8 = "  Hello  ".trim_start();                     // 开头空白
    let s9 = "  Hello  ".trim_end();                       // 结尾空白
    let s10 = "***Hello***".trim_matches('*');             // 指定字符
    
    // 重复
    let repeated = "ab".repeat(3);                         // "ababab"
    
    // 解析
    let number: Result<i32, _> = "42".parse();             // 解析为数字
    
    // 字节操作
    let bytes = s.as_bytes();                              // 转字节数组
    let s_from_bytes = String::from_utf8(bytes.to_vec()); // 从字节创建
    
    // 容量管理
    s.reserve(100);                                        // 预留容量
    s.shrink_to_fit();                                     // 收缩到实际大小
}
```

### 4.3.2 &str 常用方法

```rust
fn str_methods() {
    let s: &str = "Hello, Rust World!";
    
    // 切片操作
    let hello = &s[0..5];                                  // "Hello"
    let world = &s[7..11];                                 // "Rust"
    
    // 字符迭代
    for c in s.chars() {                                   // 字符迭代器
        println!("字符: {}", c);
    }
    
    // 字节迭代
    for b in s.bytes() {                                   // 字节迭代器
        println!("字节: {}", b);
    }
    
    // 字符索引迭代
    for (i, c) in s.char_indices() {                       // 位置和字符
        println!("位置 {}: {}", i, c);
    }
    
    // 分割
    let words: Vec<&str> = s.split_whitespace().collect(); // 按空白分割
    let parts: Vec<&str> = s.split(',').collect();        // 按字符分割
    let lines: Vec<&str> = s.lines().collect();           // 按行分割
    
    // 模式匹配
    let matches: Vec<&str> = s.matches("o").collect();    // 找所有匹配
    let count = s.matches("o").count();                    // 计数
    
    // 检查方法
    println!("是否为空: {}", s.is_empty());
    println!("是否为 ASCII: {}", s.is_ascii());
    println!("包含: {}", s.contains("Rust"));
    
    // 转换
    let owned: String = s.to_owned();                      // 转 String
    let upper = s.to_uppercase();                          // 转大写
    let lower = s.to_lowercase();                          // 转小写
}

### 4.4 集合类型与方法

```rust
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    // Vector（动态数组）
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    
    match v.get(2) {
        Some(third) => println!("第三个元素是 {}", third),
        None => println!("没有第三个元素"),
    }
    
    for i in &v {
        println!("{}", i);
    }
    
    // HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // HashSet
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    
    // VecDeque（双端队列）
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_front(0);
}
```

### 4.4.1 Vec<T> 常用方法详解

```rust
fn vec_methods() {
    // 创建方法
    let mut v1: Vec<i32> = Vec::new();                     // 空 Vec
    let v2 = vec![1, 2, 3];                                // 宏创建
    let v3 = Vec::with_capacity(10);                       // 预分配容量
    let v4 = vec![0; 5];                                   // [0, 0, 0, 0, 0]
    
    // 添加元素
    v1.push(1);                                            // 尾部添加
    v1.push(2);
    v1.extend([3, 4, 5]);                                  // 扩展
    v1.extend_from_slice(&[6, 7, 8]);                     // 从切片扩展
    
    // 插入元素
    v1.insert(0, 100);                                     // 指定位置插入
    
    // 删除元素
    v1.pop();                                              // 删除最后一个
    v1.remove(0);                                          // 删除指定索引
    v1.swap_remove(0);                                     // 快速删除（不保序）
    v1.truncate(5);                                        // 截断到指定长度
    v1.clear();                                            // 清空
    
    v1 = vec![1, 2, 3, 4, 5];
    
    // 访问元素
    let first = &v1[0];                                    // 索引访问
    let second = v1.get(1);                                // 安全访问 Option
    let last = v1.last();                                  // 最后一个元素
    let first_mut = v1.first_mut();                        // 可变引用
    
    // 切片
    let slice = &v1[1..4];                                 // [2, 3, 4]
    let (left, right) = v1.split_at(3);                    // 分割
    
    // 迭代
    for item in &v1 {                                      // 不可变迭代
        println!("{}", item);
    }
    
    for item in &mut v1 {                                  // 可变迭代
        *item *= 2;
    }
    
    for item in v1.clone() {                               // 消耗迭代
        println!("{}", item);
    }
    
    // 容量管理
    println!("长度: {}", v1.len());
    println!("容量: {}", v1.capacity());
    println!("是否为空: {}", v1.is_empty());
    
    v1.reserve(100);                                       // 预留容量
    v1.shrink_to_fit();                                    // 收缩容量
    
    // 排序
    v1.sort();                                             // 升序排序
    v1.sort_by(|a, b| b.cmp(a));                          // 降序排序
    v1.sort_unstable();                                    // 不稳定排序（更快）
    
    // 去重
    v1.dedup();                                            // 删除连续重复
    
    // 反转
    v1.reverse();                                          // 反转
    
    // 旋转
    v1.rotate_left(2);                                     // 左旋
    v1.rotate_right(1);                                    // 右旋
    
    // 分割
    let chunks: Vec<&[i32]> = v1.chunks(2).collect();     // 分块
    let windows: Vec<&[i32]> = v1.windows(3).collect();   // 滑动窗口
    
    // 查找
    println!("包含 3: {}", v1.contains(&3));
    if let Some(pos) = v1.iter().position(|&x| x == 3) {
        println!("找到位置: {}", pos);
    }
    
    // 保留
    v1.retain(|&x| x > 2);                                 // 只保留 > 2
    
    // 填充和交换
    v1.fill(0);                                            // 全部填充为 0
    v1.swap(0, 1);                                         // 交换两个元素
    
    // 转换
    let doubled: Vec<i32> = v1.iter().map(|x| x * 2).collect();
    let filtered: Vec<i32> = v1.iter()
        .filter(|&&x| x > 2)
        .copied()
        .collect();
}
```

### 4.4.2 HashMap<K, V> 常用方法

```rust
use std::collections::HashMap;

fn hashmap_methods() {
    // 创建
    let mut map = HashMap::new();
    map.insert("key1".to_string(), 100);
    map.insert("key2".to_string(), 200);
    
    // 从迭代器创建
    let tuples = vec![("a", 1), ("b", 2)];
    let map2: HashMap<_, _> = tuples.into_iter().collect();
    
    // 预分配容量
    let mut map3 = HashMap::with_capacity(10);
    
    // 插入和更新
    map.insert("key3".to_string(), 300);                   // 插入或更新
    
    // entry API（推荐）
    map.entry("key1".to_string())
        .and_modify(|v| *v += 10)                          // 如果存在则修改
        .or_insert(50);                                    // 否则插入
    
    map.entry("key4".to_string())
        .or_insert(400);                                   // 不存在时插入
    
    map.entry("key5".to_string())
        .or_insert_with(|| expensive_fn());                // 懒加载
    
    // 获取值
    let value = map.get("key1");                           // Option<&V>
    let value_mut = map.get_mut("key1");                   // Option<&mut V>
    let value_or = map.get("nonexistent")
        .unwrap_or(&0);                                    // 提供默认值
    
    // 删除
    map.remove("key1");                                    // 返回 Option<V>
    let (k, v) = map.remove_entry("key2").unwrap();        // 返回键值对
    
    // 检查
    println!("包含键: {}", map.contains_key("key1"));
    println!("长度: {}", map.len());
    println!("是否为空: {}", map.is_empty());
    
    // 迭代
    for (key, value) in &map {                             // 不可变迭代
        println!("{}: {}", key, value);
    }
    
    for key in map.keys() {                                // 遍历键
        println!("键: {}", key);
    }
    
    for value in map.values() {                            // 遍历值
        println!("值: {}", value);
    }
    
    for value in map.values_mut() {                        // 可变遍历值
        *value *= 2;
    }
    
    // 保留
    map.retain(|_k, &mut v| v > 100);                      // 保留满足条件
    
    // 清空
    map.clear();
}

fn expensive_fn() -> i32 {
    500
}
```

### 4.4.3 HashSet<T> 常用方法

```rust
use std::collections::HashSet;

fn hashset_methods() {
    let mut set1: HashSet<i32> = HashSet::new();
    let set2: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    
    // 插入
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    
    // 删除
    set1.remove(&2);
    
    // 检查
    println!("包含 1: {}", set1.contains(&1));
    println!("长度: {}", set1.len());
    println!("是否为空: {}", set1.is_empty());
    
    // 集合操作
    let set3: HashSet<_> = [3, 4, 5, 6].iter().cloned().collect();
    
    // 并集
    let union: HashSet<_> = set1.union(&set3).cloned().collect();
    
    // 交集
    let intersection: HashSet<_> = set1.intersection(&set3)
        .cloned()
        .collect();
    
    // 差集
    let difference: HashSet<_> = set1.difference(&set3)
        .cloned()
        .collect();
    
    // 对称差集
    let symmetric_diff: HashSet<_> = set1.symmetric_difference(&set3)
        .cloned()
        .collect();
    
    // 子集和超集
    println!("是子集: {}", set1.is_subset(&set2));
    println!("是超集: {}", set1.is_superset(&set2));
    println!("不相交: {}", set1.is_disjoint(&set3));
    
    // 迭代
    for item in &set1 {
        println!("{}", item);
    }
    
    // 保留
    set1.retain(|&x| x > 1);
    
    // 清空
    set1.clear();
}
```

### 4.5 结构体

```rust
// 经典结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 单元结构体
struct AlwaysEqual;

impl User {
    // 关联函数（构造器）
    fn new(username: String, email: String) -> Self {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
    
    // 方法
    fn is_active(&self) -> bool {
        self.active
    }
    
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut user1 = User::new(
        String::from("user123"),
        String::from("user@example.com"),
    );
    
    user1.deactivate();
    
    // 结构体更新语法
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

### 4.6 枚举

```rust
// 基础枚举
enum IpAddrKind {
    V4,
    V6,
}

// 带数据的枚举
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 复杂枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入: {}", text),
            Message::ChangeColor(r, g, b) => println!("颜色: ({}, {}, {})", r, g, b),
        }
    }
}

// Option<T> 枚举（标准库）
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Result<T, E> 枚举（标准库）
fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(numerator / denominator)
    }
}
```

---

## 5. 逻辑控制

### 5.1 条件语句

```rust
fn main() {
    let number = 6;
    
    // if 表达式
    if number < 5 {
        println!("条件为真");
    } else if number < 10 {
        println!("第二个条件为真");
    } else {
        println!("条件为假");
    }
    
    // if 作为表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    // match 表达式（模式匹配）
    let number = 3;
    match number {
        1 => println!("一"),
        2 | 3 => println!("二或三"),
        4..=10 => println!("四到十"),
        _ => println!("其他"),
    }
    
    // match 守卫
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于五: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    
    // if let（简化的 match）
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("三");
    }
    
    // while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

### 5.2 循环

```rust
fn main() {
    // loop（无限循环）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // 返回值
        }
    };
    
    // 循环标签
    'outer: loop {
        loop {
            break 'outer;  // 跳出外层循环
        }
    }
    
    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    // for 循环
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("值是: {}", element);
    }
    
    // 范围
    for number in 1..4 {  // 1, 2, 3
        println!("{}", number);
    }
    
    for number in (1..=4).rev() {  // 4, 3, 2, 1
        println!("{}", number);
    }
    
    // 枚举索引
    let v = vec!["a", "b", "c"];
    for (index, value) in v.iter().enumerate() {
        println!("{}: {}", index, value);
    }
}
```

---

## 6. 函数

### 6.1 函数基础

```rust
// 基本函数
fn add(x: i32, y: i32) -> i32 {
    x + y  // 表达式返回值（无分号）
}

// 无返回值（返回 unit 类型 ()）
fn print_sum(x: i32, y: i32) {
    println!("和是: {}", x + y);
}

// 提前返回
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        return None;
    }
    Some(x / y)
}
```

### 6.2 所有权与借用

```rust
// 移动语义
fn take_ownership(s: String) {
    println!("{}", s);
}  // s 在这里被释放

// 不可变借用
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可变借用
fn change(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let s1 = String::from("hello");
    take_ownership(s1);
    // println!("{}", s1);  // 错误：s1 已被移动
    
    let s2 = String::from("hello");
    let len = calculate_length(&s2);
    println!("{} 的长度是 {}", s2, len);  // s2 仍然有效
    
    let mut s3 = String::from("hello");
    change(&mut s3);
    println!("{}", s3);
}
```

### 6.3 生命周期

```rust
// 显式生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意: {}", announcement);
        self.part
    }
}

// 生命周期省略规则
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 'static 生命周期
let s: &'static str = "这个字符串拥有静态生命周期";
```

### 6.4 泛型

```rust
// 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 为特定类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 泛型枚举
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 6.5 Trait（特征）

```rust
// 定义 trait
pub trait Summary {
    fn summarize(&self) -> String;
    
    // 默认实现
    fn default_summary(&self) -> String {
        String::from("(阅读更多...)")
    }
}

// 实现 trait
struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

// trait 作为参数
fn notify(item: &impl Summary) {
    println!("突发新闻！{}", item.summarize());
}

// trait bound
fn notify_bound<T: Summary>(item: &T) {
    println!("突发新闻！{}", item.summarize());
}

// 多个 trait bound
fn notify_multiple<T: Summary + Display>(item: &T) {
    // ...
}

// where 子句
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
    0
}

// 返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {
    Article {
        title: String::from("标题"),
        content: String::from("内容"),
    }
}

// trait 继承
trait OutlinePrint: Display {
    fn outline_print(&self) {
        println!("* {} *", self);
    }
}
```

### 6.6 闭包

```rust
fn main() {
    // 基本闭包
    let add = |x, y| x + y;
    println!("{}", add(1, 2));
    
    // 类型注解
    let add: fn(i32, i32) -> i32 = |x, y| x + y;
    
    // 捕获环境
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("{}", equal_to_x(4));
    
    // 移动闭包
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    
    // FnOnce（获取所有权）
    let consume = || {
        println!("{:?}", x);
    };
    
    // FnMut（可变借用）
    let mut list = vec![1, 2, 3];
    let mut borrows_mutably = || list.push(7);
    
    // Fn（不可变借用）
    let borrows = || println!("{:?}", list);
    
    // 作为参数
    fn apply<F>(f: F) where F: FnOnce() {
        f();
    }
}
```

### 6.7 迭代器

```rust
fn main() {
    let v = vec![1, 2, 3];
    
    // 基本迭代
    let v_iter = v.iter();
    for val in v_iter {
        println!("{}", val);
    }
    
    // map
    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    
    // filter
    let v3: Vec<&i32> = v.iter().filter(|x| **x > 1).collect();
    
    // fold
    let sum: i32 = v.iter().fold(0, |acc, x| acc + x);
    
    // zip
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3: Vec<_> = v1.iter().zip(v2.iter()).collect();
    
    // chain
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3: Vec<_> = v1.iter().chain(v2.iter()).collect();
    
    // enumerate
    for (i, v) in vec![1, 2, 3].iter().enumerate() {
        println!("{}: {}", i, v);
    }
}

// 自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

---

## 7. IO 操作

### 7.1 标准输入输出

```rust
use std::io::{self, Write, BufRead};

fn main() {
    // 打印到标准输出
    println!("Hello, world!");
    print!("不换行 ");
    eprintln!("错误信息");  // 标准错误输出
    
    // 格式化输出
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);
    println!("{0} 和 {1}，再次 {0}", x, y);
    println!("{first} {second}", first = x, second = y);
    
    // 读取标准输入
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取失败");
    println!("你输入的是: {}", input);
    
    // 从缓冲区读取
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
```

### 7.2 文件操作

```rust
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufWriter, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // 读取文件（一次性读取）
    let contents = fs::read_to_string("file.txt")?;
    println!("{}", contents);
    
    // 读取为字节
    let bytes = fs::read("file.txt")?;
    
    // 写入文件
    fs::write("output.txt", "Hello, Rust!")?;
    
    // 使用 File
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // 创建文件并写入
    let mut file = File::create("output.txt")?;
    file.write_all(b"Hello, Rust!")?;
    
    // 追加模式
    let mut file = OpenOptions::new()
        .append(true)
        .open("output.txt")?;
    file.write_all(b"\nNew line")?;
    
    // 缓冲读取（高效）
    let file = File::open("file.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    
    // 缓冲写入
    let file = File::create("output.txt")?;
    let mut writer = BufWriter::new(file);
    writer.write_all(b"Hello")?;
    writer.flush()?;
    
    // 文件元信息
    let metadata = fs::metadata("file.txt")?;
    println!("文件大小: {} bytes", metadata.len());
    println!("是否为目录: {}", metadata.is_dir());
    
    // 路径操作
    let path = Path::new("./some/path.txt");
    println!("文件名: {:?}", path.file_name());
    println!("扩展名: {:?}", path.extension());
    println!("父目录: {:?}", path.parent());
    
    // 目录操作
    fs::create_dir("new_dir")?;
    fs::create_dir_all("a/b/c")?;  // 递归创建
    fs::remove_dir("new_dir")?;
    fs::remove_dir_all("a")?;      // 递归删除
    
    // 遍历目录
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        println!("{:?}", entry.path());
    }
    
    // 复制和移动
    fs::copy("source.txt", "dest.txt")?;
    fs::rename("old.txt", "new.txt")?;
    
    Ok(())
}
```

### 7.3 错误处理

```rust
use std::fs::File;
use std::io::{self, Read};
use std::error::Error;

// Result 类型
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// 链式调用
fn read_username_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 自定义错误类型
use std::fmt;

#[derive(Debug)]
enum MyError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO 错误: {}", e),
            MyError::ParseError(e) => write!(f, "解析错误: {}", e),
            MyError::Custom(s) => write!(f, "自定义错误: {}", s),
        }
    }
}

impl Error for MyError {}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::ParseError(error)
    }
}

// 使用 Box<dyn Error>
fn do_something() -> Result<(), Box<dyn Error>> {
    let file = File::open("file.txt")?;
    let number: i32 = "42".parse()?;
    Ok(())
}
```

---

## 8. HTTP 编程

### 8.1 使用 reqwest（HTTP 客户端）

```rust
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET 请求
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?;
    
    println!("状态码: {}", response.status());
    let body = response.text().await?;
    println!("响应体: {}", body);
    
    // JSON 反序列化
    let post: Post = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?
        .json()
        .await?;
    println!("{:?}", post);
    
    // POST 请求
    let new_post = Post {
        id: 0,
        title: "新标题".to_string(),
        body: "新内容".to_string(),
    };
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?;
    
    println!("状态码: {}", response.status());
    
    // 设置请求头
    let response = client
        .get("https://api.example.com/data")
        .header("Authorization", "Bearer TOKEN")
        .header("User-Agent", "MyApp/1.0")
        .send()
        .await?;
    
    // 查询参数
    let response = client
        .get("https://api.example.com/search")
        .query(&[("q", "rust"), ("limit", "10")])
        .send()
        .await?;
    
    // 超时设置
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    
    Ok(())
}
```

### 8.2 使用 axum（HTTP 服务器）

```rust
use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Clone)]
struct AppState {
    users: Arc<Mutex<Vec<User>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        users: Arc::new(Mutex::new(vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
        ])),
    };
    
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).delete(delete_user))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("服务器运行在 http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.users.lock().await;
    Json(users.clone())
}

async fn get_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.lock().await;
    users
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_user(
    State(state): State<AppState>,
    Json(new_user): Json<User>,
) -> (StatusCode, Json<User>) {
    let mut users = state.users.lock().await;
    users.push(new_user.clone());
    (StatusCode::CREATED, Json(new_user))
}

async fn delete_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> StatusCode {
    let mut users = state.users.lock().await;
    if let Some(pos) = users.iter().position(|u| u.id == id) {
        users.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
```

---

## 9. 异步编程

### 9.1 async/await 基础

```rust
use tokio;

async fn say_hello() {
    println!("Hello");
}

async fn fetch_data() -> String {
    // 模拟异步操作
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    "数据".to_string()
}

#[tokio::main]
async fn main() {
    say_hello().await;
    
    let data = fetch_data().await;
    println!("{}", data);
    
    // 并发执行
    let (result1, result2) = tokio::join!(
        fetch_data(),
        fetch_data()
    );
    
    println!("{}, {}", result1, result2);
}
```

### 9.2 Future Trait

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture {
    count: u32,
}

impl Future for MyFuture {
    type Output = u32;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        if self.count < 3 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}
```

### 9.3 tokio 运行时

```rust
use tokio::time::{sleep, Duration};
use tokio::task;

#[tokio::main]
async fn main() {
    // 生成任务
    let handle = task::spawn(async {
        sleep(Duration::from_secs(1)).await;
        "完成"
    });
    
    let result = handle.await.unwrap();
    println!("{}", result);
    
    // 生成阻塞任务
    let blocking = task::spawn_blocking(|| {
        // 执行 CPU 密集型任务
        std::thread::sleep(Duration::from_secs(1));
        "完成阻塞任务"
    });
    
    println!("{}", blocking.await.unwrap());
    
    // 超时
    let result = tokio::time::timeout(
        Duration::from_secs(2),
        async {
            sleep(Duration::from_secs(3)).await;
            "完成"
        }
    ).await;
    
    match result {
        Ok(msg) => println!("{}", msg),
        Err(_) => println!("超时"),
    }
}
```

### 9.4 异步通道

```rust
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    // mpsc（多生产者单消费者）
    let (tx, mut rx) = mpsc::channel(32);
    
    tokio::spawn(async move {
        tx.send("消息1").await.unwrap();
        tx.send("消息2").await.unwrap();
    });
    
    while let Some(msg) = rx.recv().await {
        println!("收到: {}", msg);
    }
    
    // oneshot（一次性通道）
    let (tx, rx) = oneshot::channel();
    
    tokio::spawn(async move {
        tx.send("单次消息").unwrap();
    });
    
    let msg = rx.await.unwrap();
    println!("{}", msg);
}
```

### 9.5 异步流（Stream）

```rust
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    // 创建流
    let mut stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    while let Some(value) = stream.next().await {
        println!("{}", value);
    }
    
    // map 和 filter
    let stream = stream::iter(vec![1, 2, 3, 4, 5])
        .map(|x| x * 2)
        .filter(|x| *x > 5);
    
    tokio::pin!(stream);
    
    while let Some(value) = stream.next().await {
        println!("{}", value);
    }
}
```

### 9.6 异步互斥锁

```rust
use tokio::sync::{Mutex, RwLock};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Mutex
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let mut num = data.lock().await;
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("结果: {}", *data.lock().await);
    
    // RwLock（读写锁）
    let data = Arc::new(RwLock::new(0));
    
    let data_clone = Arc::clone(&data);
    tokio::spawn(async move {
        let mut write = data_clone.write().await;
        *write += 1;
    });
    
    let read = data.read().await;
    println!("读取: {}", *read);
}
```
