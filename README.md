# Rust 核心概念详解与实战 🦀

这是一个全面的 Rust 教程项目，涵盖了 Rust 开发中最常用和最重要的概念，配有详细的代码示例和实战案例。

## 📚 目录

- [类型转换方法](#1-类型转换方法)
- [错误处理](#2-错误处理)
- [HTTP 客户端](#3-http-客户端)
- [HTTP 服务器](#4-http-服务器)
- [文件系统操作](#5-文件系统操作)
- [Arc vs Mutex](#6-arc-vs-mutex)
- [Derive 宏详解](#7-derive-宏详解)
- [Anyhow 错误处理](#8-anyhow-错误处理)
- [快速开始](#快速开始)
- [项目结构](#项目结构)

---

## 1. 类型转换方法

**文件**: `src/conversions.rs`

深入讲解 Rust 中的各种类型转换方法和模式。

### 涵盖内容

#### `.ok()` 方法
- 将 `Result<T, E>` 转换为 `Option<T>`
- 当你不关心错误的具体信息，只关心是否成功

```rust
let result: Result<i32, &str> = Ok(42);
let option = result.ok(); // Some(42)
```

#### `.ok_or()` 和 `.ok_or_else()`
- 将 `Option<T>` 转换为 `Result<T, E>`
- 为 `None` 提供错误值

```rust
let some_value: Option<i32> = Some(100);
let result = some_value.ok_or("没有值"); // Ok(100)

let none_value: Option<i32> = None;
let result = none_value.ok_or("没有值"); // Err("没有值")
```

#### `.to_*()` 系列
- **创建新的拥有所有权的副本**
- `.to_string()` - 转换为 `String`
- `.to_owned()` - 从借用创建拥有所有权的副本
- `.to_vec()` - 从切片创建 `Vec`
- `.to_lowercase()` / `.to_uppercase()` - 大小写转换

```rust
let s: &str = "hello";
let owned: String = s.to_string(); // 创建新的 String

let slice: &[i32] = &[1, 2, 3];
let vec: Vec<i32> = slice.to_vec(); // 创建新的 Vec
```

#### `.as_*()` 系列
- **引用转换，不创建新值（零成本抽象）**
- `.as_str()` - `String` → `&str`
- `.as_bytes()` - 字符串 → 字节切片
- `.as_ref()` - 通用引用转换
- `.as_mut()` - 可变引用转换

```rust
let string = String::from("hello");
let str_ref: &str = string.as_str(); // 零成本

let opt: Option<String> = Some("test".to_string());
let opt_ref: Option<&String> = opt.as_ref(); // 不移动所有权
```

#### `.into()` 方法
- **消费原值并转换（移动所有权）**
- 自动类型推导

```rust
let s: &str = "hello";
let string: String = s.into(); // 移动所有权

fn accept_string(s: String) { }
accept_string("hello".into()); // 自动推导
```

### 运行示例

```bash
cargo run --bin conversions
```

---

## 2. 错误处理

**文件**: `src/error_handling.rs`

完整的 Rust 错误处理指南，从基础到高级。

### 涵盖内容

#### Result 和 Option
- `Result<T, E>` - 可能成功或失败的操作
- `Option<T>` - 可能有值或没有值

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}
```

#### `?` 操作符
- 简化错误传播
- 自动类型转换（需要实现 `From` trait）

```rust
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?; // 失败时自动返回
    Ok(n * 2)
}
```

#### unwrap 和 expect
- `unwrap()` - 取值或 panic
- `expect()` - 带自定义消息的 unwrap
- `unwrap_or()` - 提供默认值
- `unwrap_or_else()` - 惰性求值的默认值

```rust
let value = result.unwrap_or(0); // 安全的替代方案
```

#### 自定义错误类型
手动实现 `Error` trait：

```rust
#[derive(Debug)]
enum MyError {
    IoError(io::Error),
    ParseError(ParseIntError),
    ValidationError(String),
}

impl std::error::Error for MyError {}
impl fmt::Display for MyError { /* ... */ }
```

#### thiserror 库
简化自定义错误定义：

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO 错误: {0}")]
    Io(#[from] io::Error),
    
    #[error("验证失败: {message}")]
    Validation { message: String },
}
```

#### anyhow 库
快速原型开发，处理通用错误：

```rust
use anyhow::{Context, Result};

fn process() -> Result<i32> {
    let num: i32 = input
        .parse()
        .context("无法解析输入")?;
    Ok(num)
}
```

### 运行示例

```bash
cargo run --bin error_handling
```

---

## 3. HTTP 客户端

**文件**: `src/http_client.rs`

使用 `reqwest` 库实现各种 HTTP 客户端功能。

### 涵盖内容

#### 基础请求
```rust
// 简单的 GET 请求
let response = reqwest::get("https://api.example.com/data").await?;
let body = response.text().await?;

// POST JSON 数据
let client = Client::new();
let response = client
    .post("https://api.example.com/users")
    .json(&new_user)
    .send()
    .await?;
```

#### 查询参数
```rust
let params = [("userId", "1"), ("_limit", "10")];
let response = client
    .get("https://api.example.com/posts")
    .query(&params)
    .send()
    .await?;
```

#### 自定义请求头
```rust
let response = client
    .get(url)
    .header("Authorization", "Bearer token")
    .header("User-Agent", "My-App/1.0")
    .send()
    .await?;
```

#### 高级配置
```rust
let client = Client::builder()
    .timeout(Duration::from_secs(10))
    .user_agent("My-Client/1.0")
    .gzip(true)
    .build()?;
```

#### 并发请求
```rust
let tasks: Vec<_> = urls.iter()
    .map(|url| tokio::spawn(client.get(url).send()))
    .collect();

for task in tasks {
    let response = task.await??;
    // 处理响应
}
```

#### 文件下载
```rust
let bytes = client
    .get(url)
    .send()
    .await?
    .bytes()
    .await?;

std::fs::write("file.dat", bytes)?;
```

### 运行示例

```bash
cargo run --bin http_client
```

---

## 4. HTTP 服务器

**文件**: `src/http_server.rs`

使用 `Axum` 框架构建高性能的 HTTP 服务器。

### 涵盖内容

#### 路由定义
```rust
let app = Router::new()
    .route("/", get(root))
    .route("/api/users", get(get_users).post(create_user))
    .route("/api/users/:id", get(get_user).delete(delete_user));
```

#### 提取器（Extractors）
```rust
// 路径参数
async fn get_user(Path(id): Path<u32>) -> impl IntoResponse {
    // ...
}

// JSON 请求体
async fn create_user(Json(payload): Json<CreateUserRequest>) -> impl IntoResponse {
    // ...
}

// 查询参数
async fn search(Query(params): Query<SearchParams>) -> impl IntoResponse {
    // ...
}

// 共享状态
async fn handler(State(state): State<AppState>) -> impl IntoResponse {
    // ...
}
```

#### JSON 响应
```rust
async fn get_users() -> impl IntoResponse {
    let users = vec![...];
    Json(users)
}
```

#### 错误处理
```rust
async fn handler() -> Result<Json<Data>, (StatusCode, Json<Error>)> {
    if error_condition {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { message: "错误" })
        ));
    }
    Ok(Json(data))
}
```

#### 中间件
```rust
let app = Router::new()
    .route("/api/users", get(get_users))
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http());
```

#### 共享状态
```rust
#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Database>>,
}

let app = Router::new()
    .route("/", get(handler))
    .with_state(state);
```

### 运行示例

```bash
cargo run --bin http_server
```

启动后访问 `http://localhost:3000` 查看交互式文档。

### 测试端点

```bash
# 获取所有用户
curl http://localhost:3000/api/users

# 创建用户
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"张三","email":"zhangsan@example.com","age":25}'

# 获取单个用户
curl http://localhost:3000/api/users/1

# 删除用户
curl -X DELETE http://localhost:3000/api/users/1

# 带查询参数
curl "http://localhost:3000/api/search?page=1&limit=5"
```

---

## 5. 文件系统操作

**文件**: `src/file_operations.rs`

全面的文件和目录操作指南。

### 涵盖内容

#### 文件读取
```rust
// 方式 1: 一次性读取整个文件
let content = fs::read_to_string("file.txt")?;

// 方式 2: 读取字节
let bytes = fs::read("file.dat")?;

// 方式 3: 逐行读取（内存友好）
let file = File::open("file.txt")?;
let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line?;
    println!("{}", line);
}
```

#### 文件写入
```rust
// 方式 1: 一次性写入
fs::write("file.txt", "内容")?;

// 方式 2: 追加内容
let mut file = OpenOptions::new()
    .append(true)
    .open("file.txt")?;
file.write_all(b"追加的内容\n")?;

// 方式 3: 格式化写入
let mut file = File::create("file.txt")?;
write!(file, "姓名: {}, 年龄: {}", name, age)?;
```

#### OpenOptions 详解
```rust
// 创建新文件（如果存在则失败）
let file = OpenOptions::new()
    .create_new(true)
    .write(true)
    .open("file.txt")?;

// 读写模式
let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open("file.txt")?;

// 追加模式
let file = OpenOptions::new()
    .append(true)
    .open("file.txt")?;

// 截断文件（清空内容）
let file = OpenOptions::new()
    .write(true)
    .truncate(true)
    .open("file.txt")?;

// 创建或打开 + 追加
let file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("file.txt")?;
```

#### 缓冲读写
```rust
// 缓冲写入（提高性能）
let file = File::create("file.txt")?;
let mut writer = BufWriter::new(file);
for i in 1..=1000 {
    writeln!(writer, "第 {} 行", i)?;
}
writer.flush()?;

// 缓冲读取
let file = File::open("file.txt")?;
let reader = BufReader::new(file);
let lines: Vec<_> = reader.lines().collect();
```

#### 文件定位（Seek）
```rust
let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .open("file.txt")?;

// 从开头定位
file.seek(SeekFrom::Start(10))?;

// 从当前位置定位
file.seek(SeekFrom::Current(-5))?;

// 从末尾定位
file.seek(SeekFrom::End(-10))?;

// 获取当前位置
let pos = file.stream_position()?;
```

#### 目录操作
```rust
// 创建目录
fs::create_dir("my_dir")?;

// 创建多级目录
fs::create_dir_all("a/b/c/d")?;

// 列出目录内容
for entry in fs::read_dir(".")? {
    let entry = entry?;
    println!("{:?}", entry.path());
}

// 删除空目录
fs::remove_dir("my_dir")?;

// 递归删除目录
fs::remove_dir_all("my_dir")?;
```

#### 文件元信息
```rust
let metadata = fs::metadata("file.txt")?;

println!("大小: {} 字节", metadata.len());
println!("是文件: {}", metadata.is_file());
println!("是目录: {}", metadata.is_dir());
println!("只读: {}", metadata.permissions().readonly());

if let Ok(modified) = metadata.modified() {
    println!("修改时间: {:?}", modified);
}
```

#### 路径操作
```rust
let path = Path::new("/home/user/file.txt");

// 路径组成部分
println!("文件名: {:?}", path.file_name());
println!("扩展名: {:?}", path.extension());
println!("父目录: {:?}", path.parent());

// 路径拼接
let full = Path::new("/home").join("user").join("file.txt");

// PathBuf（可变路径）
let mut path = PathBuf::from("/home");
path.push("user");
path.set_extension("txt");
```

### 运行示例

```bash
cargo run --bin file_operations
```

---

## 6. Arc vs Mutex

**文件**: `src/arc_vs_mutex.rs` | **详细文档**: `ARC_VS_MUTEX.md`

深入理解 Rust 并发编程中最核心的两个概念。

### 核心区别

#### Arc (Atomic Reference Counted)
- **作用**: 允许多个所有者共享同一份数据
- **解决**: 所有权问题
- **数据可变**: ❌ 否（只读）
- **适用**: 跨线程共享不可变数据

```rust
let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data); // 引用计数 +1
// 多个线程可以同时读取
```

#### Mutex (Mutual Exclusion)
- **作用**: 保护数据，同一时刻只有一个线程可以访问
- **解决**: 数据竞争问题
- **数据可变**: ✅ 是（内部可变性）
- **适用**: 需要修改的共享数据

```rust
let data = Mutex::new(0);
let mut num = data.lock().unwrap();
*num += 1; // 可以修改
```

### 常见组合模式

```rust
// 1. Arc<T> - 多线程只读
let config = Arc::new(Config { ... });

// 2. Mutex<T> - 单线程/单所有者可变
let buffer = Mutex::new(Vec::new());

// 3. Arc<Mutex<T>> - 多线程可变（最常见）
let counter = Arc::new(Mutex::new(0));

// 4. Arc<RwLock<T>> - 读多写少
let cache = Arc::new(RwLock::new(HashMap::new()));

// 5. Arc<AtomicXxx> - 简单类型高性能
let flag = Arc::new(AtomicBool::new(false));
```

### 选择指南

| 场景 | 方案 | 原因 |
|------|------|------|
| 多线程只读 | `Arc<T>` | 无锁，性能最好 |
| 多线程可变 | `Arc<Mutex<T>>` | 标准方案 |
| 读多写少 | `Arc<RwLock<T>>` | 读操作可并发 |
| 简单计数器 | `Arc<AtomicUsize>` | 无锁，最快 |
| 单线程可变 | `Mutex<T>` | 不需要 Arc |

### 常见陷阱

```rust
// ❌ 忘记释放锁（死锁）
let guard = data.lock().unwrap();
let guard2 = data.lock().unwrap(); // 死锁！

// ✅ 使用作用域自动释放
{
    let guard = data.lock().unwrap();
    // 使用数据
} // guard 自动 drop

// ❌ 锁内耗时操作
let mut data = counter.lock().unwrap();
expensive_operation(); // 锁被长时间持有

// ✅ 快速进出锁
{
    let mut data = counter.lock().unwrap();
    *data += 1;
} // 先释放锁
expensive_operation(); // 在锁外执行
```

### 运行示例

```bash
cargo run --bin arc_vs_mutex
```

**查看详细文档**: [ARC_VS_MUTEX.md](./ARC_VS_MUTEX.md)

---

## 7. Derive 宏详解

**文件**: `src/derive_macros_detailed.rs` | **详细文档**: `DERIVE_GUIDE.md`

全面深入的 Rust Derive 宏教程，涵盖所有常用派生宏的使用和最佳实践。

### 什么是 Derive

Derive 宏是 Rust 提供的自动生成 trait 实现的机制，可以大幅减少样板代码。

```rust
// 使用 derive 一行搞定
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 等价于手动实现约 30+ 行代码
```

### 常用 Derive 宏

| Derive | 用途 | 示例 |
|--------|------|------|
| **Debug** | 调试输出 `{:?}` | `println!("{:?}", value)` |
| **Clone** | 深拷贝 | `let copy = value.clone()` |
| **Copy** | 自动按位复制 | `let b = a; // a 仍有效` |
| **PartialEq** | 相等比较 `==` | `if a == b { ... }` |
| **Eq** | 完全相等 | 用于 HashMap |
| **PartialOrd** | 大小比较 `<`, `>` | `if a < b { ... }` |
| **Ord** | 完全排序 | `vec.sort()` |
| **Hash** | 哈希计算 | HashMap 键 |
| **Default** | 默认值 | `T::default()` |

### 核心概念

#### 1. Debug - 调试必备

```rust
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

let user = User { id: 1, name: "Alice".to_string() };
println!("{:?}", user);    // User { id: 1, name: "Alice" }
println!("{:#?}", user);   // 格式化输出
```

#### 2. Clone vs Copy

```rust
// Clone: 显式深拷贝，允许堆分配
#[derive(Debug, Clone)]
struct Person {
    name: String,  // 堆分配
}

let p1 = Person { name: "Bob".to_string() };
let p2 = p1.clone();  // 显式调用

// Copy: 自动复制，仅栈上数据
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,  // 仅栈数据
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // 自动复制，p1 仍有效
```

| 特性 | Clone | Copy |
|------|-------|------|
| 调用方式 | 显式 `.clone()` | 自动 |
| 堆分配 | ✅ 允许 | ❌ 不允许 |
| 开销 | 可能大 | 小（按位） |

#### 3. PartialEq 和 Eq

```rust
#[derive(PartialEq, Eq)]
struct Id(u32);

assert!(Id(1) == Id(1));
assert!(Id(1) != Id(2));
```

**区别**:
- `PartialEq`: 部分相等（如 `f64`，`NaN != NaN`）
- `Eq`: 完全相等（保证 `a == a`）

#### 4. 排序相关

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Priority {
    level: u32,
    name: String,
}

let mut items = vec![
    Priority { level: 2, name: "B".to_string() },
    Priority { level: 1, name: "A".to_string() },
];

items.sort();  // 按 level 排序，再按 name
```

#### 5. Hash - HashMap 键

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct ProductId {
    category: String,
    sku: u32,
}

let mut map = HashMap::new();
map.insert(
    ProductId { 
        category: "Books".to_string(), 
        sku: 123 
    },
    "Rust 编程"
);
```

**要求**: 必须同时实现 `Eq`

#### 6. Default - 默认值

```rust
#[derive(Debug, Default)]
struct Config {
    host: String,       // ""
    port: u16,          // 0
    debug: bool,        // false
}

let config = Config::default();

// 结构体更新语法
let custom = Config {
    port: 8080,
    ..Default::default()
};
```

### 常用组合模式

#### 基础组合（最常用）
```rust
#[derive(Debug, Clone, PartialEq)]
struct MyStruct { ... }
```
适用于 90% 的结构体

#### 可排序类型
```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Sortable { ... }
```

#### HashMap 键
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key { ... }
```

#### 简单值类型（Copy）
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point { x: i32, y: i32 }
```

#### 配置类型
```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct AppConfig { ... }
```

### 手动实现 vs Derive

**何时手动实现**：

```rust
// 示例：忽略时间戳的相等比较
#[derive(Debug)]
struct Record {
    id: u32,
    data: String,
    timestamp: u64,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        // 只比较 id 和 data，忽略 timestamp
        self.id == other.id && self.data == other.data
    }
}
```

**适用场景**：
- ✅ 自定义行为
- ✅ 忽略某些字段
- ✅ 性能优化
- ✅ 特殊逻辑（如大小写不敏感比较）

### Derive 依赖关系

```
Ord      → 需要 Eq + PartialOrd
Eq       → 需要 PartialEq
Copy     → 需要 Clone
Hash     → 通常需要 Eq
```

### 最佳实践

1. **总是 derive Debug**
   ```rust
   #[derive(Debug)]  // 几乎所有类型都应该有
   struct MyType { ... }
   ```

2. **Copy 仅用于小型栈值**
   ```rust
   // ✅ 好
   #[derive(Copy, Clone)]
   struct Point { x: i32, y: i32 }
   
   // ❌ 坏 - 包含堆分配
   #[derive(Copy, Clone)]  // 编译错误！
   struct User { name: String }
   ```

3. **遵循标准顺序**
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
   ```

4. **枚举也可以 derive**
   ```rust
   #[derive(Debug, Clone, PartialEq)]
   enum Status {
       Pending,
       Active { id: u32 },
       Completed,
   }
   ```

### 实战案例

#### API 数据模型
```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u32,
    username: String,
    email: String,
}

#[derive(Debug, Clone, PartialEq)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}
```

#### 游戏实体
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Position { x: f32, y: f32 }

#[derive(Debug, Clone, PartialEq)]
struct Entity {
    id: u32,
    name: String,
    position: Position,
    health: u32,
}
```

#### 配置系统
```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct DatabaseConfig {
    host: String,
    port: u16,
    max_connections: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct AppConfig {
    database: DatabaseConfig,
    debug_mode: bool,
}

let config = AppConfig::default();
```

### 决策树

```
需要调试？ → derive(Debug)
需要复制？
  ├─ 有堆分配？ → derive(Clone)
  └─ 无堆分配？ → derive(Clone, Copy)
需要比较？ → derive(PartialEq)
  └─ 自反性？ → derive(Eq)
需要排序？ → derive(PartialOrd, Ord)
HashMap键？ → derive(Hash, Eq, PartialEq)
需要默认值？ → derive(Default)
```

### 常见问题

**Q: Copy 和 Clone 的区别？**
- Copy: 自动复制，仅栈数据，开销小
- Clone: 显式 `.clone()`，可含堆数据，开销可能大

**Q: 为什么 Ord 需要 Eq？**
- 完全排序要求：如果 `a <= b` 且 `b <= a`，则 `a == b`

**Q: 什么时候不应该 derive？**
- 需要自定义逻辑
- 忽略某些字段
- 性能优化
- 保护敏感信息

**Q: Derive 的性能如何？**
- 编译时零开销
- 生成代码与手写相当
- Debug 输出可能较慢（字符串格式化）

### 运行示例

```bash
cargo run --bin derive_macros_detailed
```

**查看详细文档**: [DERIVE_GUIDE.md](./DERIVE_GUIDE.md)

---

## 8. Anyhow 错误处理

**文件**: `src/anyhow_detailed.rs` | **详细文档**: `ANYHOW_GUIDE.md`

深入探索 Rust 生态中最流行的应用层错误处理库 - Anyhow。

### 什么是 Anyhow

Anyhow 是一个专为**应用程序**设计的错误处理库，极大地简化了错误处理代码。

```rust
// 传统方式
fn traditional() -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("file.txt")?;
    let num: i32 = content.parse()?;
    Ok(format!("{}", num))
}

// Anyhow - 简洁统一
use anyhow::Result;

fn with_anyhow() -> Result<String> {
    let content = fs::read_to_string("file.txt")?;
    let num: i32 = content.parse()?;
    Ok(format!("{}", num))
}
```

### 核心优势

| 特性 | 说明 |
|------|------|
| **统一错误类型** | `anyhow::Error` 可包装任何错误 |
| **自动转换** | 任何 `impl Error` 的类型自动转换 |
| **丰富上下文** | 轻松添加错误发生的背景信息 |
| **错误链** | 追踪错误的完整传播路径 |
| **零成本** | 成功路径无运行时开销 |

### 核心 API

#### 1. Result<T> - 简化的结果类型

```rust
use anyhow::Result;

// 简化前
fn old_way() -> Result<String, Box<dyn std::error::Error>> {
    // ...
}

// 简化后
fn new_way() -> Result<String> {
    // 所有错误自动转换为 anyhow::Error
    let content = fs::read_to_string("file.txt")?;
    let num: i32 = content.parse()?;
    Ok(format!("{}", num))
}
```

#### 2. bail! - 立即返回错误

```rust
use anyhow::{bail, Result};

fn validate_age(age: i32) -> Result<()> {
    if age < 0 {
        bail!("年龄不能为负数");
    }
    if age > 150 {
        bail!("年龄 {} 超出合理范围", age);
    }
    Ok(())
}
```

#### 3. ensure! - 条件断言

```rust
use anyhow::{ensure, Result};

fn divide(a: f64, b: f64) -> Result<f64> {
    ensure!(b != 0.0, "除数不能为零");
    Ok(a / b)
}

fn validate_username(name: &str) -> Result<()> {
    ensure!(!name.is_empty(), "用户名不能为空");
    ensure!(name.len() >= 3, "用户名至少 3 个字符");
    ensure!(name.len() <= 20, "用户名最多 20 个字符");
    Ok(())
}
```

#### 4. Context - 添加错误上下文

```rust
use anyhow::{Context, Result};

// ❌ 不好 - 错误信息模糊
fn bad() -> Result<Config> {
    let content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
// 错误: No such file or directory

// ✅ 好 - 清晰的错误上下文
fn good() -> Result<Config> {
    let content = fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;
    
    Ok(config)
}
// 错误: Failed to read config file: No such file or directory
```

### context() vs with_context()

| 方法 | 求值 | 性能 | 适用场景 |
|------|------|------|----------|
| `context()` | 立即 | 总是分配 | 静态字符串 |
| `with_context()` | 惰性 | 仅错误时分配 | 动态消息 |

```rust
// context - 立即求值
.context("Static message")

// with_context - 惰性求值（推荐）
.with_context(|| format!("Dynamic: {}", variable))
```

### 错误链

多层 `context` 形成错误链，完整记录错误传播路径：

```rust
use anyhow::{Context, Result};

fn level_3() -> Result<()> {
    fs::read_to_string("missing.txt")
        .context("Level 3: File read failed")?;
    Ok(())
}

fn level_2() -> Result<()> {
    level_3().context("Level 2: Config loading failed")?;
    Ok(())
}

fn level_1() -> Result<()> {
    level_2().context("Level 1: App initialization failed")?;
    Ok(())
}

fn main() {
    if let Err(e) = level_1() {
        eprintln!("Error: {}", e);
        
        // 打印完整错误链
        for (i, cause) in e.chain().enumerate() {
            eprintln!("  {}: {}", i, cause);
        }
    }
}
```

**输出**：
```
Error: Level 1: App initialization failed

Caused by:
    0: Level 1: App initialization failed
    1: Level 2: Config loading failed
    2: Level 3: File read failed
    3: No such file or directory (os error 2)
```

### 实战案例

#### 案例 1: 配置文件加载

```rust
use anyhow::{Context, Result, ensure};

fn load_config(path: &str) -> Result<Config> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read: {}", path))?;
    
    let mut host = None;
    let mut port = None;
    
    for (line_num, line) in content.lines().enumerate() {
        let parts: Vec<&str> = line.split('=').collect();
        ensure!(
            parts.len() == 2,
            "Invalid format at line {}: {}",
            line_num + 1,
            line
        );
        
        match parts[0].trim() {
            "host" => host = Some(parts[1].trim().to_string()),
            "port" => {
                port = Some(parts[1].trim().parse()
                    .with_context(|| format!("Invalid port at line {}", line_num + 1))?);
            }
            _ => bail!("Unknown key: {}", parts[0]),
        }
    }
    
    let host = host.context("Missing required field: host")?;
    let port = port.context("Missing required field: port")?;
    
    Ok(Config { host, port })
}
```

#### 案例 2: CLI 工具

```rust
use anyhow::{bail, ensure, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    ensure!(args.len() >= 2, "Usage: {} <command>", args[0]);
    
    match args[1].as_str() {
        "add" => cmd_add(&args[2..])?,
        "div" => cmd_div(&args[2..])?,
        _ => bail!("Unknown command: {}", args[1]),
    }
    
    Ok(())
}

fn cmd_div(args: &[String]) -> Result<()> {
    ensure!(args.len() == 2, "div requires 2 arguments");
    
    let a: f64 = args[0].parse()
        .context("First arg must be a number")?;
    let b: f64 = args[1].parse()
        .context("Second arg must be a number")?;
    
    ensure!(b != 0.0, "Division by zero");
    
    println!("Result: {}", a / b);
    Ok(())
}
```

#### 案例 3: 数据处理管道

```rust
use anyhow::{Context, Result, ensure};

fn parse_csv_line(line: &str, line_num: usize) -> Result<Record> {
    let parts: Vec<&str> = line.split(',').collect();
    
    ensure!(
        parts.len() == 3,
        "Line {}: expected 3 columns, got {}",
        line_num,
        parts.len()
    );
    
    let id: u32 = parts[0].trim().parse()
        .with_context(|| format!("Line {}: invalid ID", line_num))?;
    
    let score: f64 = parts[2].trim().parse()
        .with_context(|| format!("Line {}: invalid score", line_num))?;
    
    ensure!(
        (0.0..=100.0).contains(&score),
        "Line {}: score {} out of range",
        line_num,
        score
    );
    
    Ok(Record { id, score })
}
```

### Anyhow vs Thiserror

| 特性 | Anyhow | Thiserror |
|------|--------|-----------|
| **用途** | 应用程序 | 库 |
| **错误类型** | 统一 (`anyhow::Error`) | 自定义枚举 |
| **类型匹配** | ❌ 不支持 | ✅ 支持 |
| **上下文** | ✅ 内置 | 需手动 |
| **公共 API** | ❌ 不推荐 | ✅ 推荐 |

**选择指南**：
- **应用程序**（CLI、服务器） → 使用 `anyhow`
- **库**（供他人使用） → 使用 `thiserror`

### 最佳实践

#### 1. 总是添加 context

```rust
// ❌ 不好
let content = fs::read_to_string(path)?;

// ✅ 好
let content = fs::read_to_string(path)
    .context("Failed to read file")?;
```

#### 2. 使用 with_context 添加动态信息

```rust
// ❌ 总是分配
.context(format!("File: {}", path))

// ✅ 仅错误时分配
.with_context(|| format!("File: {}", path))
```

#### 3. 优先使用 ensure!

```rust
// ❌ 冗长
if x <= 0 {
    bail!("x must be positive");
}

// ✅ 简洁
ensure!(x > 0, "x must be positive");
```

#### 4. main 返回 Result

```rust
use anyhow::Result;

fn main() -> Result<()> {
    // 错误自动打印到 stderr
    run_app()?;
    Ok(())
}
```

#### 5. 构建清晰的错误链

```rust
fn process() -> Result<()> {
    load_data()
        .context("Failed to load data")?;
    
    validate()
        .context("Validation failed")?;
    
    save()
        .context("Failed to save")?;
    
    Ok(())
}
```

### 核心概念总结

```rust
use anyhow::{anyhow, bail, ensure, Context, Result};

// Result<T> - 简化的结果类型
fn my_fn() -> Result<T> { /* ... */ }

// bail! - 立即返回错误
bail!("error message");

// ensure! - 条件断言
ensure!(condition, "error if false");

// anyhow! - 创建错误对象
let err = anyhow!("error message");

// context - 添加上下文（静态）
operation().context("Failed")?;

// with_context - 添加上下文（动态）
operation().with_context(|| format!("Failed: {}", x))?;
```

### 运行示例

```bash
cargo run --bin anyhow_detailed
```

**查看详细文档**: [ANYHOW_GUIDE.md](./ANYHOW_GUIDE.md)

---

## 快速开始

### 环境要求

- Rust 1.70 或更高版本
- Cargo（Rust 包管理器）

### 安装 Rust

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# 下载并运行 https://rustup.rs/
```

### 克隆项目

```bash
git clone <repository-url>
cd rust-core-concepts
```

### 构建项目

```bash
# 构建所有示例
cargo build

# 构建特定示例
cargo build --bin conversions
```

### 运行示例

```bash
# 运行类型转换示例
cargo run --bin conversions

# 运行错误处理示例
cargo run --bin error_handling

# 运行 HTTP 客户端示例
cargo run --bin http_client

# 运行 HTTP 服务器示例
cargo run --bin http_server

# 运行文件操作示例
cargo run --bin file_operations

# 运行 Arc vs Mutex 示例
cargo run --bin arc_vs_mutex

# 运行 Derive 宏示例
cargo run --bin derive_macros_detailed

# 运行 Anyhow 示例
cargo run --bin anyhow_detailed
```

### 检查代码

```bash
# 检查语法错误
cargo check

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查（linter）
cargo clippy
```

---

## 项目结构

```
rust-core-concepts/
├── Cargo.toml                      # 项目配置和依赖
├── README.md                       # 本文件
├── ARC_VS_MUTEX.md                 # Arc vs Mutex 详细文档
├── DERIVE_GUIDE.md                 # Derive 宏完全指南
├── ANYHOW_GUIDE.md                 # Anyhow 错误处理完全指南
└── src/
    ├── conversions.rs              # 类型转换详解
    ├── error_handling.rs           # 错误处理详解
    ├── http_client.rs              # HTTP 客户端示例
    ├── http_server.rs              # HTTP 服务器示例
    ├── file_operations.rs          # 文件系统操作
    ├── arc_vs_mutex.rs             # Arc vs Mutex 详解
    ├── derive_macros_detailed.rs   # Derive 宏详解
    └── anyhow_detailed.rs          # Anyhow 错误处理详解
```

---

## 依赖说明

### 核心依赖

- **tokio** - 异步运行时
  - 特性: `full`（包含所有功能）

- **axum** - Web 框架
  - 高性能、符合人体工程学的 web 框架

- **reqwest** - HTTP 客户端
  - 特性: `json`（JSON 序列化支持）

- **serde** - 序列化/反序列化
  - 特性: `derive`（派生宏支持）

### 错误处理

- **anyhow** - 快速错误处理
  - 适合应用程序开发

- **thiserror** - 自定义错误
  - 适合库开发

### 实用工具

- **tower** - 中间件和服务抽象
- **tower-http** - HTTP 中间件（CORS、跟踪等）
- **tracing** - 结构化日志
- **tracing-subscriber** - 日志订阅器

---

## 学习路径建议

### 初学者

1. **类型转换** (`conversions.rs`)
   - 理解 Rust 的所有权和借用
   - 掌握不同转换方法的使用场景

2. **错误处理** (`error_handling.rs`)
   - 学习 `Result` 和 `Option`
   - 掌握 `?` 操作符
   - 了解何时使用 `unwrap`

3. **文件操作** (`file_operations.rs`)
   - 练习基本的 I/O 操作
   - 理解缓冲和性能优化

### 进阶

4. **HTTP 客户端** (`http_client.rs`)
   - 异步编程基础
   - 网络请求和错误处理
   - 并发请求处理

5. **HTTP 服务器** (`http_server.rs`)
   - Web 框架使用
   - RESTful API 设计
   - 状态管理和中间件

---

## 最佳实践

### 错误处理
- ✅ 使用 `Result` 和 `Option`
- ✅ 使用 `?` 操作符传播错误
- ✅ 为库使用 `thiserror`
- ✅ 为应用使用 `anyhow`
- ❌ 避免随意使用 `unwrap()`
- ❌ 避免使用 `panic!`（除非真的无法恢复）

### 文件操作
- ✅ 大文件使用缓冲 I/O
- ✅ 使用 `Path`/`PathBuf` 而不是字符串
- ✅ 始终处理 `Result`
- ❌ 避免一次性加载大文件到内存

### 异步编程
- ✅ 使用 `async`/`await`
- ✅ 合理使用 `tokio::spawn` 并发
- ✅ 注意生命周期和所有权
- ❌ 避免阻塞异步任务

### 性能
- ✅ 使用 `&str` 而不是 `String`（当不需要所有权时）
- ✅ 使用 `as_ref()` 避免克隆
- ✅ 适当使用 `Arc` 和 `Mutex` 共享数据
- ❌ 避免不必要的克隆和分配

---

## 常见问题

### Q: 为什么我的异步函数不工作？
A: 确保你在 `#[tokio::main]` 函数中运行，或者使用 `tokio::runtime::Runtime::new()` 创建运行时。

### Q: 如何处理多个不同类型的错误？
A: 使用 `anyhow` 或者创建自定义的枚举错误类型，并实现 `From` trait。

### Q: 什么时候使用 `.to_string()` 和 `.as_str()`？
A: `.to_string()` 创建新的 `String`（有开销），`.as_str()` 只是获取引用（零成本）。如果不需要所有权，优先使用 `.as_str()`。

### Q: HTTP 服务器如何处理并发请求？
A: Axum 基于 tokio，自动处理并发。每个请求在独立的异步任务中处理。

### Q: 如何避免文件操作中的竞态条件？
A: 使用适当的文件锁（如 `fs2` crate），或者使用原子操作（临时文件 + 重命名）。

---

## 扩展资源

### 官方文档
- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio 文档](https://tokio.rs/)
- [Axum 文档](https://docs.rs/axum/)
- [Reqwest 文档](https://docs.rs/reqwest/)

### 推荐阅读
- **The Rust Programming Language** (官方书籍)
- **Programming Rust** (O'Reilly)
- **Rust for Rustaceans** (进阶)

### 在线资源
- [Rust Playground](https://play.rust-lang.org/) - 在线运行 Rust 代码
- [Crates.io](https://crates.io/) - Rust 包仓库
- [This Week in Rust](https://this-week-in-rust.org/) - Rust 周报

---

## 贡献指南

欢迎贡献！如果你发现错误或有改进建议：

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

---

## 许可证

本项目采用 MIT 许可证。详见 LICENSE 文件。

---

## 作者

教程示例项目 - 用于学习 Rust 核心概念

---

## 致谢

感谢 Rust 社区的所有贡献者，以及以下优秀项目：

- Tokio - 异步运行时
- Axum - Web 框架
- Reqwest - HTTP 客户端
- Serde - 序列化框架
- Anyhow & Thiserror - 错误处理

---

**Happy Coding! 🦀✨**

如果这个项目对你有帮助，请给个 ⭐️！
