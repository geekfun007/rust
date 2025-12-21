# Rust 文件系统操作详解 (fs / File / OpenOptions)

## 目录

1. [基础概念](#基础概念)
2. [std::fs 模块](#stdfs-模块)
3. [File 类型详解](#file-类型详解)
4. [OpenOptions 详解](#openoptions-详解)
5. [读写操作](#读写操作)
6. [目录操作](#目录操作)
7. [元数据操作](#元数据操作)
8. [实战示例](#实战示例)
9. [最佳实践](#最佳实践)

---

## 基础概念

### Rust 文件系统 API 层次

```rust
std::fs           // 同步文件系统操作
├── File          // 文件句柄
├── OpenOptions   // 文件打开选项
├── DirBuilder    // 目录创建器
├── DirEntry      // 目录项
├── Metadata      // 文件元数据
└── Permissions   // 文件权限

tokio::fs         // 异步文件系统操作（需要 tokio）
```

### 核心概念

1. **所有权**: `File` 拥有文件句柄的所有权
2. **RAII**: 文件在离开作用域时自动关闭
3. **缓冲**: 使用 `BufReader`/`BufWriter` 提高性能
4. **错误处理**: 所有操作都返回 `Result<T, io::Error>`

---

## std::fs 模块

### 快速操作函数

```rust
use std::fs;

// ============================================================================
// 文件读写（一次性操作）
// ============================================================================

// 读取整个文件为字符串
let content = fs::read_to_string("file.txt")?;

// 读取整个文件为字节
let bytes = fs::read("file.bin")?;

// 写入字符串到文件（覆盖）
fs::write("file.txt", "Hello, World!")?;

// 写入字节到文件（覆盖）
fs::write("file.bin", b"binary data")?;

// ============================================================================
// 文件操作
// ============================================================================

// 复制文件
fs::copy("source.txt", "dest.txt")?;

// 移动/重命名文件
fs::rename("old.txt", "new.txt")?;

// 删除文件
fs::remove_file("file.txt")?;

// 创建硬链接
fs::hard_link("original.txt", "link.txt")?;

// 创建符号链接（软链接）
#[cfg(unix)]
fs::soft_link("original.txt", "symlink.txt")?;

// ============================================================================
// 目录操作
// ============================================================================

// 创建目录
fs::create_dir("my_dir")?;

// 创建目录及所有父目录
fs::create_dir_all("path/to/my_dir")?;

// 删除空目录
fs::remove_dir("my_dir")?;

// 递归删除目录及其内容
fs::remove_dir_all("my_dir")?;

// 读取目录内容
for entry in fs::read_dir(".")? {
    let entry = entry?;
    println!("{:?}", entry.path());
}

// ============================================================================
// 元数据查询
// ============================================================================

// 获取文件元数据（跟随符号链接）
let metadata = fs::metadata("file.txt")?;
println!("文件大小: {} 字节", metadata.len());
println!("是否为文件: {}", metadata.is_file());
println!("是否为目录: {}", metadata.is_dir());

// 获取符号链接元数据（不跟随）
let metadata = fs::symlink_metadata("symlink")?;
println!("是否为符号链接: {}", metadata.is_symlink());

// 规范化路径（解析符号链接、. 和 ..）
let canonical = fs::canonicalize("./file.txt")?;
println!("规范路径: {:?}", canonical);

// ============================================================================
// 权限操作
// ============================================================================

// 获取权限
let permissions = fs::metadata("file.txt")?.permissions();

// 设置权限
#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = fs::metadata("file.txt")?.permissions();
    permissions.set_mode(0o644); // rw-r--r--
    fs::set_permissions("file.txt", permissions)?;
}
```

---

## File 类型详解

### 创建和打开文件

```rust
use std::fs::File;
use std::io::{self, Read, Write};

// ============================================================================
// 基本操作
// ============================================================================

// 打开文件（只读）
let file = File::open("file.txt")?;

// 创建文件（只写，如果存在则截断）
let file = File::create("file.txt")?;

// 使用 OpenOptions 进行高级控制
let file = File::options()
    .read(true)
    .write(true)
    .create(true)
    .open("file.txt")?;
```

### File 的方法

```rust
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};

// ============================================================================
// 读取操作
// ============================================================================

let mut file = File::open("file.txt")?;

// 读取到字节数组
let mut buffer = [0u8; 100];
let bytes_read = file.read(&mut buffer)?;

// 读取到 Vec
let mut buffer = Vec::new();
file.read_to_end(&mut buffer)?;

// 读取到字符串
let mut content = String::new();
file.read_to_string(&mut content)?;

// 精确读取 n 字节
file.read_exact(&mut buffer)?; // 如果文件太小会返回错误

// ============================================================================
// 写入操作
// ============================================================================

let mut file = File::create("file.txt")?;

// 写入字节
file.write(b"Hello")?;

// 写入全部内容
file.write_all(b"Hello, World!")?;

// 格式化写入
write!(file, "数字: {}, 字符串: {}", 42, "test")?;

// 刷新缓冲区
file.flush()?;

// ============================================================================
// 定位操作
// ============================================================================

let mut file = File::open("file.txt")?;

// 从文件开始定位
file.seek(SeekFrom::Start(10))?;

// 相对当前位置定位
file.seek(SeekFrom::Current(5))?;

// 从文件末尾定位
file.seek(SeekFrom::End(-10))?;

// 获取当前位置
let position = file.stream_position()?;

// 回到开始
file.rewind()?;

// ============================================================================
// 元数据和同步
// ============================================================================

// 获取文件元数据
let metadata = file.metadata()?;
println!("文件大小: {}", metadata.len());

// 同步数据到磁盘（数据和元数据）
file.sync_all()?;

// 同步数据到磁盘（仅数据）
file.sync_data()?;

// 设置文件长度
file.set_len(100)?; // 截断或扩展文件

// 尝试获取文件独占锁（Unix）
#[cfg(unix)]
{
    use std::os::unix::fs::FileExt;
    // 在特定位置读取，不改变文件位置
    file.read_at(&mut buffer, 100)?;
    // 在特定位置写入，不改变文件位置
    file.write_at(b"data", 100)?;
}
```

---

## OpenOptions 详解

### 基础用法

```rust
use std::fs::OpenOptions;

// ============================================================================
// 读取模式
// ============================================================================

// 只读模式（默认）
let file = OpenOptions::new()
    .read(true)
    .open("file.txt")?;

// 等价于 File::open()
let file = File::open("file.txt")?;

// ============================================================================
// 写入模式
// ============================================================================

// 只写模式，文件不存在则创建，存在则截断
let file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open("file.txt")?;

// 等价于 File::create()
let file = File::create("file.txt")?;

// ============================================================================
// 追加模式
// ============================================================================

// 追加模式（写入始终在文件末尾）
let file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("file.txt")?;

// ============================================================================
// 读写模式
// ============================================================================

// 读写模式，文件必须存在
let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open("file.txt")?;

// 读写模式，文件不存在则创建
let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("file.txt")?;

// ============================================================================
// 创建选项
// ============================================================================

// create(true): 文件不存在时创建
let file = OpenOptions::new()
    .write(true)
    .create(true)
    .open("file.txt")?;

// create_new(true): 文件不存在时创建，存在则失败（原子操作）
let file = OpenOptions::new()
    .write(true)
    .create_new(true)
    .open("file.txt")?;

// ============================================================================
// 截断选项
// ============================================================================

// truncate(true): 打开时清空文件内容
let file = OpenOptions::new()
    .write(true)
    .truncate(true)
    .open("file.txt")?;

// ============================================================================
// Unix 特定选项
// ============================================================================

#[cfg(unix)]
{
    use std::os::unix::fs::OpenOptionsExt;
    
    // 设置文件权限模式（创建时）
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .mode(0o644) // rw-r--r--
        .open("file.txt")?;
    
    // 自定义标志
    let file = OpenOptions::new()
        .write(true)
        .custom_flags(libc::O_SYNC) // 同步 I/O
        .open("file.txt")?;
}

// ============================================================================
// Windows 特定选项
// ============================================================================

#[cfg(windows)]
{
    use std::os::windows::fs::OpenOptionsExt;
    
    // 设置文件访问模式
    let file = OpenOptions::new()
        .access_mode(0x1 | 0x2) // GENERIC_READ | GENERIC_WRITE
        .open("file.txt")?;
    
    // 设置共享模式
    let file = OpenOptions::new()
        .share_mode(0x1) // FILE_SHARE_READ
        .open("file.txt")?;
}
```

### 完整示例

```rust
use std::fs::OpenOptions;
use std::io::{Write, BufWriter};

// ============================================================================
// 示例 1: 日志文件（追加模式）
// ============================================================================

fn append_log(message: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("app.log")?;
    
    let mut writer = BufWriter::new(file);
    writeln!(writer, "[{}] {}", chrono::Local::now(), message)?;
    writer.flush()?;
    
    Ok(())
}

// ============================================================================
// 示例 2: 配置文件（读写模式）
// ============================================================================

fn update_config(key: &str, value: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("config.toml")?;
    
    // 读取现有内容
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    // 修改内容
    let new_content = content + &format!("{} = \"{}\"\n", key, value);
    
    // 清空文件并写入
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(new_content.as_bytes())?;
    
    Ok(())
}

// ============================================================================
// 示例 3: 临时文件（独占创建）
// ============================================================================

fn create_temp_file() -> io::Result<File> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true) // 文件存在则失败
        .open("temp.txt")?;
    
    Ok(file)
}
```

---

## 读写操作

### 缓冲 I/O

```rust
use std::io::{BufReader, BufWriter, BufRead};
use std::fs::File;

// ============================================================================
// BufReader - 缓冲读取
// ============================================================================

let file = File::open("large_file.txt")?;
let reader = BufReader::new(file);

// 逐行读取
for line in reader.lines() {
    let line = line?;
    println!("{}", line);
}

// 读取到指定分隔符
let mut reader = BufReader::new(File::open("data.csv")?);
let mut line = String::new();
while reader.read_line(&mut line)? > 0 {
    // 处理行
    line.clear();
}

// 读取到字节分隔符
let mut buffer = Vec::new();
reader.read_until(b'\n', &mut buffer)?;

// ============================================================================
// BufWriter - 缓冲写入
// ============================================================================

let file = File::create("output.txt")?;
let mut writer = BufWriter::new(file);

// 写入数据（缓冲）
writeln!(writer, "Line 1")?;
writeln!(writer, "Line 2")?;

// 刷新缓冲区
writer.flush()?;

// 获取内部文件句柄
let file = writer.into_inner()?;

// ============================================================================
// 缓冲区大小
// ============================================================================

// 自定义缓冲区大小
let reader = BufReader::with_capacity(8 * 1024, file); // 8KB
let writer = BufWriter::with_capacity(8 * 1024, file); // 8KB
```

### 标准 I/O

```rust
use std::io::{self, Read, Write, stdin, stdout};

// ============================================================================
// 标准输入
// ============================================================================

// 读取一行
let mut input = String::new();
stdin().read_line(&mut input)?;

// 锁定标准输入（提高性能）
let stdin = stdin();
let mut handle = stdin.lock();
let mut buffer = String::new();
handle.read_line(&mut buffer)?;

// ============================================================================
// 标准输出
// ============================================================================

// 写入标准输出
println!("Hello, World!");
print!("No newline");
stdout().flush()?; // 刷新缓冲区

// 锁定标准输出
let stdout = stdout();
let mut handle = stdout.lock();
writeln!(handle, "Efficient output")?;

// ============================================================================
// 标准错误
// ============================================================================

eprintln!("Error message");
```

---

## 目录操作

### 遍历目录

```rust
use std::fs;
use std::path::Path;

// ============================================================================
// 读取目录
// ============================================================================

// 基本遍历
for entry in fs::read_dir(".")? {
    let entry = entry?;
    let path = entry.path();
    
    if path.is_file() {
        println!("文件: {:?}", path);
    } else if path.is_dir() {
        println!("目录: {:?}", path);
    }
}

// ============================================================================
// 递归遍历
// ============================================================================

fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                visit_dirs(&path)?;
            } else {
                println!("{:?}", path);
            }
        }
    }
    Ok(())
}

// ============================================================================
// 使用 walkdir crate（推荐）
// ============================================================================

use walkdir::WalkDir;

for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
    println!("{}", entry.path().display());
}

// 跳过隐藏文件
for entry in WalkDir::new(".")
    .into_iter()
    .filter_entry(|e| !is_hidden(e))
    .filter_map(|e| e.ok())
{
    println!("{}", entry.path().display());
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
```

### DirEntry 方法

```rust
use std::fs;

for entry in fs::read_dir(".")? {
    let entry = entry?;
    
    // 获取路径
    let path = entry.path();
    
    // 获取文件名
    let name = entry.file_name();
    
    // 获取元数据
    let metadata = entry.metadata()?;
    
    // 获取文件类型（不跟随符号链接）
    let file_type = entry.file_type()?;
    
    println!("名称: {:?}", name);
    println!("路径: {:?}", path);
    println!("大小: {} 字节", metadata.len());
    println!("是文件: {}", file_type.is_file());
    println!("是目录: {}", file_type.is_dir());
    println!("是符号链接: {}", file_type.is_symlink());
}
```

---

## 元数据操作

### Metadata 详解

```rust
use std::fs;
use std::time::SystemTime;

let metadata = fs::metadata("file.txt")?;

// ============================================================================
// 文件类型
// ============================================================================

let is_file = metadata.is_file();
let is_dir = metadata.is_dir();
let is_symlink = metadata.is_symlink();

let file_type = metadata.file_type();
println!("文件类型: {:?}", file_type);

// ============================================================================
// 文件大小
// ============================================================================

let size = metadata.len(); // 字节数
println!("文件大小: {} 字节 ({:.2} KB)", size, size as f64 / 1024.0);

// ============================================================================
// 时间戳
// ============================================================================

// 修改时间
if let Ok(modified) = metadata.modified() {
    let duration = SystemTime::now().duration_since(modified)?;
    println!("上次修改: {:?} 秒前", duration.as_secs());
}

// 访问时间
if let Ok(accessed) = metadata.accessed() {
    println!("上次访问: {:?}", accessed);
}

// 创建时间（不是所有平台都支持）
if let Ok(created) = metadata.created() {
    println!("创建时间: {:?}", created);
}

// ============================================================================
// 权限
// ============================================================================

let permissions = metadata.permissions();
let is_readonly = permissions.readonly();
println!("只读: {}", is_readonly);

// Unix 特定
#[cfg(unix)]
{
    use std::os::unix::fs::MetadataExt;
    
    println!("设备 ID: {}", metadata.dev());
    println!("Inode: {}", metadata.ino());
    println!("模式: {:o}", metadata.mode());
    println!("硬链接数: {}", metadata.nlink());
    println!("UID: {}", metadata.uid());
    println!("GID: {}", metadata.gid());
    println!("块大小: {}", metadata.blksize());
    println!("块数: {}", metadata.blocks());
}

// Windows 特定
#[cfg(windows)]
{
    use std::os::windows::fs::MetadataExt;
    
    println!("文件属性: {}", metadata.file_attributes());
    println!("卷序列号: {}", metadata.volume_serial_number().unwrap_or(0));
}
```

---

## 实战示例

### 示例 1: 文件复制工具

```rust
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

fn copy_file(source: &str, dest: &str) -> io::Result<u64> {
    let mut reader = BufReader::new(File::open(source)?);
    let mut writer = BufWriter::new(File::create(dest)?);
    
    let mut buffer = [0u8; 8192];
    let mut total = 0u64;
    
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        
        writer.write_all(&buffer[..bytes_read])?;
        total += bytes_read as u64;
    }
    
    writer.flush()?;
    Ok(total)
}
```

### 示例 2: 配置文件管理

```rust
use std::collections::HashMap;
use std::fs;
use std::io;

struct Config {
    data: HashMap<String, String>,
}

impl Config {
    fn load(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut data = HashMap::new();
        
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                data.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        
        Ok(Config { data })
    }
    
    fn save(&self, path: &str) -> io::Result<()> {
        let mut content = String::new();
        for (key, value) in &self.data {
            content.push_str(&format!("{}={}\n", key, value));
        }
        fs::write(path, content)
    }
    
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
}
```

### 示例 3: 日志轮转

```rust
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufWriter};

struct Logger {
    file: BufWriter<File>,
    path: String,
    max_size: u64,
}

impl Logger {
    fn new(path: &str, max_size: u64) -> io::Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;
        
        Ok(Logger {
            file: BufWriter::new(file),
            path: path.to_string(),
            max_size,
        })
    }
    
    fn log(&mut self, message: &str) -> io::Result<()> {
        // 检查文件大小
        let metadata = fs::metadata(&self.path)?;
        if metadata.len() > self.max_size {
            self.rotate()?;
        }
        
        // 写入日志
        writeln!(self.file, "[{}] {}", chrono::Local::now(), message)?;
        self.file.flush()?;
        
        Ok(())
    }
    
    fn rotate(&mut self) -> io::Result<()> {
        // 关闭当前文件
        self.file.flush()?;
        
        // 重命名旧文件
        let backup = format!("{}.old", self.path);
        fs::rename(&self.path, backup)?;
        
        // 创建新文件
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.path)?;
        
        self.file = BufWriter::new(file);
        Ok(())
    }
}
```

---

## 最佳实践

### 1. 错误处理

```rust
use std::io;

// ✅ 好的做法 - 使用 ? 传播错误
fn read_file(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

// ✅ 好的做法 - 提供上下文
fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.toml")
        .map_err(|e| format!("无法读取配置文件: {}", e))?;
    Ok(parse_config(&content)?)
}

// ❌ 不好的做法 - 使用 unwrap()
fn bad_read() {
    let content = fs::read_to_string("file.txt").unwrap(); // 可能 panic
}
```

### 2. 资源管理

```rust
// ✅ 好的做法 - RAII 自动关闭
{
    let file = File::open("file.txt")?;
    // 使用文件
} // 文件在这里自动关闭

// ✅ 好的做法 - 显式刷新
let mut file = File::create("file.txt")?;
file.write_all(b"important data")?;
file.sync_all()?; // 确保数据写入磁盘
```

### 3. 性能优化

```rust
// ✅ 好的做法 - 使用缓冲 I/O
let file = File::open("large_file.txt")?;
let reader = BufReader::new(file);
for line in reader.lines() {
    // 处理行
}

// ❌ 不好的做法 - 逐字节读取
let mut file = File::open("large_file.txt")?;
let mut byte = [0u8; 1];
while file.read(&mut byte)? > 0 {
    // 非常慢
}

// ✅ 好的做法 - 批量操作
fs::write("file.txt", &data)?; // 一次性写入

// ❌ 不好的做法 - 多次小写入
let mut file = File::create("file.txt")?;
for byte in data {
    file.write(&[byte])?; // 非常慢
}
```

### 4. 跨平台考虑

```rust
use std::path::PathBuf;

// ✅ 好的做法 - 使用 Path/PathBuf
let mut path = PathBuf::from("dir");
path.push("file.txt");

// ❌ 不好的做法 - 硬编码路径分隔符
let path = "dir/file.txt"; // Windows 可能有问题

// ✅ 好的做法 - 使用 std::env
let home = std::env::var("HOME")?;
let config_path = PathBuf::from(home).join(".config").join("app.toml");
```

### 5. 临时文件

```rust
use tempfile::NamedTempFile;

// ✅ 好的做法 - 使用 tempfile crate
let mut temp = NamedTempFile::new()?;
writeln!(temp, "temporary data")?;
// 文件在离开作用域时自动删除

// 持久化临时文件
let persistent_path = temp.path().to_path_buf();
temp.persist(&persistent_path)?;
```

---

## 总结

### 快速选择指南

| 操作 | 推荐方法 |
|------|---------|
| 读取整个文件 | `fs::read_to_string()` / `fs::read()` |
| 逐行读取 | `BufReader::lines()` |
| 写入整个文件 | `fs::write()` |
| 追加内容 | `OpenOptions::append()` |
| 读写文件 | `OpenOptions::read().write()` |
| 复制文件 | `fs::copy()` |
| 移动文件 | `fs::rename()` |
| 删除文件 | `fs::remove_file()` |
| 创建目录 | `fs::create_dir_all()` |
| 遍历目录 | `fs::read_dir()` / `walkdir::WalkDir` |
| 临时文件 | `tempfile::NamedTempFile` |

### 性能建议

1. **大文件**: 使用 `BufReader`/`BufWriter`
2. **小文件**: 使用 `fs::read_to_string()`/`fs::write()`
3. **频繁 I/O**: 增大缓冲区大小
4. **随机访问**: 使用 `seek()` 和 `read_at()`/`write_at()`

### 安全建议

1. 始终处理 `io::Error`
2. 验证用户输入的路径
3. 使用 `create_new()` 避免竞态条件
4. 敏感数据使用 `sync_all()` 确保写入磁盘
5. 临时文件使用随机名称
