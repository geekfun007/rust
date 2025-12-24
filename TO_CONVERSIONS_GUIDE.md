# Rust To 系列转换完全指南

完整的 `to_` 系列方法详解，包含理论、实践和最佳实践。

## 目录

1. [核心概念](#核心概念)
2. [字符串转换](#字符串转换)
3. [集合转换](#集合转换)
4. [数值转换](#数值转换)
5. [字节转换](#字节转换)
6. [路径转换](#路径转换)
7. [转换对比](#转换对比)
8. [实战案例](#实战案例)
9. [最佳实践](#最佳实践)

---

## 核心概念

### To 方法特点

| 特性 | 说明 |
|------|------|
| **所有权** | 借用原值，返回新值 |
| **开销** | 通常涉及克隆/分配 |
| **原值** | 保持可用 |
| **返回** | 拥有所有权的新值 |

### as_ vs to_ vs into_

```
┌────────────┬──────────┬──────────┬──────────┐
│  特性       │  as_     │  to_     │  into_   │
├────────────┼──────────┼──────────┼──────────┤
│  所有权     │  借用    │  借用    │  转移    │
│  返回值     │  引用    │  新值    │  新值    │
│  开销       │  零成本  │  克隆    │  移动    │
│  原值可用   │  是      │  是      │  否      │
│  用途       │  查看    │  复制    │  转换    │
└────────────┴──────────┴──────────┴──────────┘
```

**选择指南**:
- 🔍 **as_** - 只需要查看，不需要拥有
- 📋 **to_** - 需要独立副本，原值还要用
- 🔄 **into_** - 类型转换，不再需要原值

---

## 字符串转换

### 1. to_string()

将任何实现了 `Display` 的类型转换为 `String`。

```rust
// 基本类型
let num = 42;
let s = num.to_string();  // "42"

let float = 3.14;
let s = float.to_string();  // "3.14"

let boolean = true;
let s = boolean.to_string();  // "true"

// 字符串类型
let str_slice = "hello";
let s = str_slice.to_string();  // String

// 自定义类型
#[derive(Debug)]
struct Point { x: i32, y: i32 }

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 10, y: 20 };
let s = p.to_string();  // "(10, 20)"
```

**关键点**:
- 需要实现 `Display` trait
- 返回新的 `String`
- 不消费原值

### 2. to_str()

将 `OsStr`、`Path`、`CStr` 等转换为 `&str`。

```rust
use std::path::Path;
use std::ffi::OsStr;

// Path to &str
let path = Path::new("/usr/local/bin");
match path.to_str() {
    Some(s) => println!("{}", s),
    None => println!("非 UTF-8 路径"),
}

// OsStr to &str
let os_str = OsStr::new("hello");
if let Some(s) = os_str.to_str() {
    println!("{}", s);
}
```

**关键点**:
- 返回 `Option<&str>` 或 `Result<&str, ...>`
- 可能失败（非 UTF-8）
- 使用 `to_string_lossy()` 作为替代方案

### 3. to_owned()

将借用类型转换为拥有所有权的类型。

```rust
// &str → String
let borrowed = "hello";
let owned = borrowed.to_owned();
// 两者都可用

// &[T] → Vec<T>
let slice: &[i32] = &[1, 2, 3];
let vec = slice.to_owned();

// &Path → PathBuf
use std::path::Path;
let path = Path::new("/usr/local");
let path_buf = path.to_owned();

// &OsStr → OsString
use std::ffi::OsStr;
let os_str = OsStr::new("test");
let os_string = os_str.to_owned();
```

**ToOwned trait**:
```rust
trait ToOwned {
    type Owned;
    fn to_owned(&self) -> Self::Owned;
}
```

### 4. to_lowercase() / to_uppercase()

大小写转换。

```rust
// 字符串
let s = "Hello, World!";
let lower = s.to_lowercase();  // "hello, world!"
let upper = s.to_uppercase();  // "HELLO, WORLD!"

// 字符
let ch = 'A';
let lower = ch.to_lowercase();  // 迭代器
let upper = ch.to_uppercase();  // 迭代器

// Unicode 支持
let s = "Straße";  // 德语
println!("{}", s.to_uppercase());  // "STRASSE"

// ASCII 专用（更快）
let s = "Hello";
let lower = s.to_ascii_lowercase();  // "hello"
let upper = s.to_ascii_uppercase();  // "HELLO"
```

**对比**:
| 方法 | 支持 | 性能 |
|------|------|------|
| `to_lowercase()` | Unicode | 较慢 |
| `to_ascii_lowercase()` | 仅 ASCII | 更快 |

---

## 集合转换

### 1. to_vec()

将切片或数组转换为 `Vec`。

```rust
// 切片 → Vec
let slice = &[1, 2, 3, 4, 5];
let vec = slice.to_vec();

// 数组 → Vec
let array = [10, 20, 30];
let vec = array.to_vec();

// 字节切片 → Vec
let bytes: &[u8] = b"hello";
let vec = bytes.to_vec();
```

**性能考虑**:
```rust
let slice = &[1, 2, 3];

// 总是分配
let vec1 = slice.to_vec();

// 可能更高效
let vec2: Vec<i32> = slice.iter().copied().collect();

// 语义清晰
let vec3 = slice.to_owned();
```

---

## 数值转换

### 1. to_digit()

将字符转换为数字。

```rust
// 十进制
'0'.to_digit(10);  // Some(0)
'9'.to_digit(10);  // Some(9)
'A'.to_digit(10);  // None

// 十六进制
'A'.to_digit(16);  // Some(10)
'F'.to_digit(16);  // Some(15)
'G'.to_digit(16);  // None

// 二进制
'0'.to_digit(2);   // Some(0)
'1'.to_digit(2);   // Some(1)
'2'.to_digit(2);   // None

// 实用：手动解析十六进制
fn parse_hex(s: &str) -> Option<u32> {
    let mut result = 0;
    for ch in s.chars() {
        result = result * 16 + ch.to_digit(16)?;
    }
    Some(result)
}

parse_hex("1A3");  // Some(419)
parse_hex("FF");   // Some(255)
```

### 2. to_radians() / to_degrees()

角度和弧度转换。

```rust
use std::f64::consts::PI;

// 角度 → 弧度
let deg = 180.0;
let rad = deg.to_radians();  // π

// 弧度 → 角度
let rad = PI;
let deg = rad.to_degrees();  // 180.0

// 三角函数示例
let angle_deg = 45.0;
let angle_rad = angle_deg.to_radians();
println!("sin(45°) = {}", angle_rad.sin());  // 0.7071
println!("cos(45°) = {}", angle_rad.cos());  // 0.7071
```

---

## 字节转换

### 1. to_bytes()

C 字符串转字节数组。

```rust
use std::ffi::CString;

let c_string = CString::new("hello").unwrap();

// 不含 null
let bytes = c_string.to_bytes();
// [104, 101, 108, 108, 111]

// 含 null
let bytes_with_nul = c_string.to_bytes_with_nul();
// [104, 101, 108, 108, 111, 0]
```

### 2. to_le_bytes() / to_be_bytes()

数字的字节序转换。

```rust
// Little Endian (小端)
let num: u32 = 0x12345678;
let le = num.to_le_bytes();  // [78, 56, 34, 12]

// Big Endian (大端)
let be = num.to_be_bytes();  // [12, 34, 56, 78]

// Native Endian (本机字节序)
let ne = num.to_ne_bytes();

// 恢复
let restored_le = u32::from_le_bytes(le);
let restored_be = u32::from_be_bytes(be);
```

**应用场景**:
- 网络协议（通常 Big Endian）
- 文件格式（可能 Little Endian）
- 二进制序列化

**支持的类型**:
- 整数：`i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`
- 浮点：`f32`, `f64`

---

## 路径转换

### 1. to_path_buf()

`Path` 转 `PathBuf`。

```rust
use std::path::{Path, PathBuf};

let path = Path::new("/usr/local/bin");
let path_buf = path.to_path_buf();

// PathBuf 可变，可以修改
let mut path_buf = PathBuf::from("/usr");
path_buf.push("local");
path_buf.push("bin");
```

### 2. to_os_string()

`OsStr` 转 `OsString`。

```rust
use std::ffi::{OsStr, OsString};

let os_str = OsStr::new("test.txt");
let os_string = os_str.to_os_string();

// 从 &str
let os_string: OsString = "hello.txt".into();
```

### 3. to_socket_addrs()

网络地址转换。

```rust
use std::net::ToSocketAddrs;

// 字符串地址
let addr = "127.0.0.1:8080";
for addr in addr.to_socket_addrs()? {
    println!("{}", addr);
}

// 元组地址
let tuple = ("localhost", 3000);
for addr in tuple.to_socket_addrs()? {
    println!("{}", addr);
}

// 域名解析
let domain = "example.com:80";
for addr in domain.to_socket_addrs()? {
    println!("{}", addr);
}
```

---

## 转换对比

### to_owned() vs to_string() vs clone()

```rust
┌─────────────┬──────────────┬────────────────┐
│  方法        │  适用类型     │  返回类型       │
├─────────────┼──────────────┼────────────────┤
│  to_owned   │  借用类型     │  拥有所有权     │
│  to_string  │  Display     │  String        │
│  clone      │  Clone       │  相同类型       │
└─────────────┴──────────────┴────────────────┘
```

```rust
// to_owned - &str → String
let s = "hello";
let owned = s.to_owned();

// to_string - Display → String
let num = 42;
let s = num.to_string();

// clone - T → T
let vec = vec![1, 2, 3];
let vec2 = vec.clone();
```

### as_bytes() vs to_bytes() vs into_bytes()

```rust
// as_bytes() - 借用
let s = "hello";
let bytes = s.as_bytes();  // &[u8]
// s 仍可用

// to_bytes() - CStr 特有
use std::ffi::CString;
let c = CString::new("hello").unwrap();
let bytes = c.to_bytes();  // &[u8]

// into_bytes() - 转移所有权
let s = String::from("hello");
let bytes = s.into_bytes();  // Vec<u8>
// s 不可再用
```

---

## 实战案例

### 案例 1: 配置解析

```rust
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

impl Config {
    fn from_str(s: &str) -> Result<Self, String> {
        let mut config = Config {
            host: "localhost".to_string(),
            port: 8080,
            debug: false,
        };
        
        for line in s.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                continue;
            }
            
            let key = parts[0].trim();
            let value = parts[1].trim();
            
            match key {
                "host" => config.host = value.to_string(),
                "port" => {
                    config.port = value.parse()
                        .map_err(|_| format!("无效端口: {}", value))?;
                }
                "debug" => {
                    config.debug = value.to_lowercase() == "true";
                }
                _ => {}
            }
        }
        
        Ok(config)
    }
}
```

### 案例 2: 文件路径处理

```rust
use std::path::PathBuf;

fn normalize_path(path: &str) -> PathBuf {
    let mut path_buf = PathBuf::from(path);
    
    // 添加根目录
    if !path_buf.has_root() {
        path_buf = PathBuf::from("/tmp").join(path_buf);
    }
    
    // 添加扩展名
    if path_buf.extension().is_none() {
        path_buf.set_extension("txt");
    }
    
    path_buf
}

// 使用
normalize_path("data/file");        // "/tmp/data/file.txt"
normalize_path("/var/log/app.log"); // "/var/log/app.log"
```

### 案例 3: 二进制序列化

```rust
fn serialize_u32(num: u32) -> Vec<u8> {
    num.to_le_bytes().to_vec()
}

fn deserialize_u32(bytes: &[u8]) -> Option<u32> {
    if bytes.len() < 4 {
        return None;
    }
    let array: [u8; 4] = bytes[0..4].try_into().ok()?;
    Some(u32::from_le_bytes(array))
}

// 使用
let num = 12345678u32;
let bytes = serialize_u32(num);
let restored = deserialize_u32(&bytes);
```

### 案例 4: 字符串处理管道

```rust
fn process_text(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            // 首字母大写
            let mut chars = line.chars();
            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() 
                        + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
```

---

## 最佳实践

### ✅ 推荐做法

1. **选择合适的转换方法**
   ```rust
   // 好 - 只需查看
   let bytes = s.as_bytes();
   
   // 好 - 需要独立副本
   let owned = s.to_owned();
   
   // 好 - 类型转换
   let bytes = s.into_bytes();
   ```

2. **注意 Unicode 处理**
   ```rust
   // 好 - Unicode 安全
   let lower = s.to_lowercase();
   
   // 好 - ASCII 优化
   if s.is_ascii() {
       let lower = s.to_ascii_lowercase();
   }
   ```

3. **处理可能的失败**
   ```rust
   // 好 - 处理 Option
   match path.to_str() {
       Some(s) => process(s),
       None => use_lossy(path.to_string_lossy()),
   }
   ```

4. **选择正确的字节序**
   ```rust
   // 网络协议
   let bytes = num.to_be_bytes();
   
   // 文件格式（特定平台）
   let bytes = num.to_le_bytes();
   ```

### ❌ 避免做法

1. **不必要的克隆**
   ```rust
   // 差 - 不必要的分配
   fn process(s: &str) -> String {
       let owned = s.to_string();
       owned.to_uppercase()  // 又一次分配
   }
   
   // 好 - 直接转换
   fn process(s: &str) -> String {
       s.to_uppercase()
   }
   ```

2. **忽略错误处理**
   ```rust
   // 差 - 可能 panic
   let s = path.to_str().unwrap();
   
   // 好 - 处理错误
   let s = path.to_str()
       .ok_or("非 UTF-8 路径")?;
   ```

3. **混淆转换方法**
   ```rust
   // 差 - 不必要的转换
   let s = "hello".to_string().to_owned();
   
   // 好 - 直接转换
   let s = "hello".to_string();
   ```

---

## 总结

### To 系列方法分类

**字符串**:
- `to_string()` - 任意类型 → String
- `to_str()` - OsStr/Path → &str
- `to_owned()` - 借用 → 拥有
- `to_lowercase()` / `to_uppercase()` - 大小写

**集合**:
- `to_vec()` - 切片 → Vec

**数字**:
- `to_digit()` - char → u32
- `to_radians()` / `to_degrees()` - 角度转换

**字节**:
- `to_bytes()` - CStr → &[u8]
- `to_le_bytes()` / `to_be_bytes()` - 数字 → 字节

**路径**:
- `to_path_buf()` - Path → PathBuf
- `to_os_string()` - OsStr → OsString

### 快速参考

```rust
// 字符串转换
"hello".to_string()           // &str → String
num.to_string()               // 数字 → String
s.to_owned()                  // &str → String
s.to_lowercase()              // 小写
s.to_uppercase()              // 大写

// 集合转换
slice.to_vec()                // &[T] → Vec<T>
slice.to_owned()              // &[T] → Vec<T>

// 数值转换
'A'.to_digit(16)              // char → Option<u32>
deg.to_radians()              // 角度 → 弧度
rad.to_degrees()              // 弧度 → 角度

// 字节转换
num.to_le_bytes()             // 数字 → [u8; N]
num.to_be_bytes()             // 数字 → [u8; N]

// 路径转换
path.to_path_buf()            // Path → PathBuf
os_str.to_os_string()         // OsStr → OsString
```

---

## 参考资源

- [Rust 标准库文档](https://doc.rust-lang.org/std/)
- [ToString trait](https://doc.rust-lang.org/std/string/trait.ToString.html)
- [ToOwned trait](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html)
- [字符串转换指南](https://doc.rust-lang.org/book/ch08-02-strings.html)

---

**运行示例**:
```bash
cargo run --bin to_conversions_detailed
```
