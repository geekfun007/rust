// Rust 文件操作详解
// 
// 涵盖：std::fs、File、OpenOptions、读写操作、目录操作

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::env;

fn main() {
    println!("=== Rust 文件操作实战 ===\n");
    
    // 创建临时工作目录
    let temp_dir = env::temp_dir().join("rust_file_demo");
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir).unwrap();
    }
    println!("工作目录: {:?}\n", temp_dir);
    
    // 1. 文件读取
    demo_file_reading(&temp_dir);
    
    // 2. 文件写入
    demo_file_writing(&temp_dir);
    
    // 3. OpenOptions 高级用法
    demo_open_options(&temp_dir);
    
    // 4. 缓冲读写
    demo_buffered_io(&temp_dir);
    
    // 5. 文件定位和查找
    demo_file_seeking(&temp_dir);
    
    // 6. 目录操作
    demo_directory_operations(&temp_dir);
    
    // 7. 文件元信息
    demo_file_metadata(&temp_dir);
    
    // 8. 路径操作
    demo_path_operations();
    
    // 9. 实战案例
    demo_real_world_cases(&temp_dir);
    
    // 清理
    println!("\n清理临时文件...");
    let _ = fs::remove_dir_all(&temp_dir);
    println!("完成！");
}

// ============================================
// 1. 文件读取
// ============================================
fn demo_file_reading(temp_dir: &Path) {
    println!("--- 1. 文件读取 ---");
    
    // 准备测试文件
    let test_file = temp_dir.join("read_test.txt");
    fs::write(&test_file, "Hello, Rust!\n这是第二行\n第三行内容").unwrap();
    
    // 方式 1: fs::read_to_string - 读取整个文件为 String（最简单）
    match fs::read_to_string(&test_file) {
        Ok(content) => println!("read_to_string:\n{}", content),
        Err(e) => println!("读取失败: {}", e),
    }
    
    println!();
    
    // 方式 2: fs::read - 读取整个文件为字节数组
    match fs::read(&test_file) {
        Ok(bytes) => println!("read: 读取了 {} 字节", bytes.len()),
        Err(e) => println!("读取失败: {}", e),
    }
    
    // 方式 3: File::open + read_to_string - 更灵活
    match File::open(&test_file) {
        Ok(mut file) => {
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(size) => println!("File::open 读取: {} 字节", size),
                Err(e) => println!("读取失败: {}", e),
            }
        }
        Err(e) => println!("打开文件失败: {}", e),
    }
    
    // 方式 4: 逐行读取（内存友好）
    println!("\n逐行读取:");
    match File::open(&test_file) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for (i, line) in reader.lines().enumerate() {
                match line {
                    Ok(content) => println!("  第 {} 行: {}", i + 1, content),
                    Err(e) => println!("  读取行失败: {}", e),
                }
            }
        }
        Err(e) => println!("打开文件失败: {}", e),
    }
    
    println!();
}

// ============================================
// 2. 文件写入
// ============================================
fn demo_file_writing(temp_dir: &Path) {
    println!("--- 2. 文件写入 ---");
    
    // 方式 1: fs::write - 一次性写入（最简单）
    let file1 = temp_dir.join("write_test1.txt");
    match fs::write(&file1, "使用 fs::write 写入的内容") {
        Ok(_) => println!("fs::write 成功"),
        Err(e) => println!("写入失败: {}", e),
    }
    
    // 方式 2: File::create + write - 更灵活
    let file2 = temp_dir.join("write_test2.txt");
    match File::create(&file2) {
        Ok(mut file) => {
            match file.write_all("使用 File::create 写入\n".as_bytes()) {
                Ok(_) => {
                    // 可以继续写入
                    let _ = file.write_all("第二行内容\n".as_bytes());
                    println!("File::create + write_all 成功");
                }
                Err(e) => println!("写入失败: {}", e),
            }
        }
        Err(e) => println!("创建文件失败: {}", e),
    }
    
    // 方式 3: 使用 write! 宏（格式化写入）
    let file3 = temp_dir.join("write_test3.txt");
    match File::create(&file3) {
        Ok(mut file) => {
            let name = "张三";
            let age = 25;
            match write!(file, "姓名: {}\n年龄: {}\n", name, age) {
                Ok(_) => println!("write! 宏写入成功"),
                Err(e) => println!("写入失败: {}", e),
            }
        }
        Err(e) => println!("创建文件失败: {}", e),
    }
    
    // 方式 4: 追加内容
    let file4 = temp_dir.join("append_test.txt");
    fs::write(&file4, "初始内容\n").unwrap();
    
    match OpenOptions::new()
        .append(true)
        .open(&file4)
    {
        Ok(mut file) => {
            match file.write_all("追加的内容\n".as_bytes()) {
                Ok(_) => println!("追加内容成功"),
                Err(e) => println!("追加失败: {}", e),
            }
        }
        Err(e) => println!("打开文件失败: {}", e),
    }
    
    // 验证追加结果
    if let Ok(content) = fs::read_to_string(&file4) {
        println!("追加后的内容:\n{}", content);
    }
    
    println!();
}

// ============================================
// 3. OpenOptions - 精细控制文件打开方式
// ============================================
fn demo_open_options(temp_dir: &Path) {
    println!("--- 3. OpenOptions 详解 ---");
    
    let file_path = temp_dir.join("open_options_test.txt");
    
    // 示例 1: 创建新文件（如果存在则失败）
    match OpenOptions::new()
        .create_new(true)  // 只创建新文件
        .write(true)       // 写入权限
        .open(&file_path)
    {
        Ok(mut file) => {
            file.write_all("新创建的文件\n".as_bytes()).unwrap();
            println!("✓ create_new: 创建新文件成功");
        }
        Err(e) => println!("✗ create_new 失败: {}", e),
    }
    
    // 示例 2: 读写模式打开（文件必须存在）
    match OpenOptions::new()
        .read(true)        // 读取权限
        .write(true)       // 写入权限
        .open(&file_path)
    {
        Ok(mut file) => {
            // 读取内容
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            println!("✓ read+write: 读取到: {}", content.trim());
            
            // 写入新内容
            file.write_all("修改后的内容\n".as_bytes()).unwrap();
        }
        Err(e) => println!("✗ 打开失败: {}", e),
    }
    
    // 示例 3: 追加模式
    match OpenOptions::new()
        .append(true)      // 追加模式
        .open(&file_path)
    {
        Ok(mut file) => {
            file.write_all("追加的行\n".as_bytes()).unwrap();
            println!("✓ append: 追加内容成功");
        }
        Err(e) => println!("✗ 追加失败: {}", e),
    }
    
    // 示例 4: 截断文件（清空内容）
    match OpenOptions::new()
        .write(true)
        .truncate(true)    // 清空文件
        .open(&file_path)
    {
        Ok(mut file) => {
            file.write_all("文件被截断后的新内容\n".as_bytes()).unwrap();
            println!("✓ truncate: 文件已截断并写入新内容");
        }
        Err(e) => println!("✗ 截断失败: {}", e),
    }
    
    // 示例 5: 创建或打开（常用组合）
    let file_path2 = temp_dir.join("create_or_open.txt");
    match OpenOptions::new()
        .create(true)      // 不存在则创建
        .write(true)       // 写入权限
        .append(true)      // 追加模式
        .open(&file_path2)
    {
        Ok(mut file) => {
            file.write_all("使用 create+append 模式\n".as_bytes()).unwrap();
            println!("✓ create+append: 成功");
        }
        Err(e) => println!("✗ 失败: {}", e),
    }
    
    // Unix 特有：设置权限
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        
        match OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o644)   // rw-r--r--
            .open(temp_dir.join("with_permissions.txt"))
        {
            Ok(_) => println!("✓ Unix permissions: 设置文件权限成功"),
            Err(e) => println!("✗ 设置权限失败: {}", e),
        }
    }
    
    println!();
}

// ============================================
// 4. 缓冲读写 - 提高性能
// ============================================
fn demo_buffered_io(temp_dir: &Path) {
    println!("--- 4. 缓冲读写 ---");
    
    let file_path = temp_dir.join("buffered_test.txt");
    
    // 缓冲写入
    match File::create(&file_path) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            
            // 写入多行
            for i in 1..=100 {
                writeln!(writer, "第 {} 行内容", i).unwrap();
            }
            
            // BufWriter 会在 drop 时自动 flush
            // 也可以手动 flush
            writer.flush().unwrap();
            
            println!("✓ BufWriter: 写入 100 行");
        }
        Err(e) => println!("✗ 创建文件失败: {}", e),
    }
    
    // 缓冲读取
    match File::open(&file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            
            // 逐行读取
            let lines: Vec<String> = reader
                .lines()
                .filter_map(|line| line.ok())
                .collect();
            
            println!("✓ BufReader: 读取了 {} 行", lines.len());
            println!("  前 3 行: {:?}", &lines[0..3]);
        }
        Err(e) => println!("✗ 打开文件失败: {}", e),
    }
    
    // 按块读取
    match File::open(&file_path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut buffer = vec![0u8; 50]; // 50 字节的缓冲区
            
            match reader.read(&mut buffer) {
                Ok(n) => {
                    println!("✓ 按块读取: 读取了 {} 字节", n);
                    let text = String::from_utf8_lossy(&buffer[..n]);
                    // 安全地截取字符串（考虑 UTF-8 边界）
                    let preview: String = text.chars().take(10).collect();
                    println!("  内容: {:?}...", preview);
                }
                Err(e) => println!("✗ 读取失败: {}", e),
            }
        }
        Err(e) => println!("✗ 打开失败: {}", e),
    }
    
    println!();
}

// ============================================
// 5. 文件定位和查找（Seek）
// ============================================
fn demo_file_seeking(temp_dir: &Path) {
    println!("--- 5. 文件定位（Seek） ---");
    
    let file_path = temp_dir.join("seek_test.txt");
    fs::write(&file_path, "0123456789ABCDEFGHIJ").unwrap();
    
    match OpenOptions::new()
        .read(true)
        .write(true)
        .open(&file_path)
    {
        Ok(mut file) => {
            // 1. 从开头定位
            file.seek(SeekFrom::Start(10)).unwrap();
            let mut buffer = [0u8; 5];
            file.read_exact(&mut buffer).unwrap();
            println!("✓ SeekFrom::Start(10): {}", String::from_utf8_lossy(&buffer));
            
            // 2. 从当前位置定位
            file.seek(SeekFrom::Current(-3)).unwrap(); // 回退 3 个字节
            file.read_exact(&mut buffer).unwrap();
            println!("✓ SeekFrom::Current(-3): {}", String::from_utf8_lossy(&buffer));
            
            // 3. 从末尾定位
            file.seek(SeekFrom::End(-5)).unwrap(); // 从末尾往前 5 字节
            file.read_exact(&mut buffer).unwrap();
            println!("✓ SeekFrom::End(-5): {}", String::from_utf8_lossy(&buffer));
            
            // 4. 获取当前位置
            let pos = file.stream_position().unwrap();
            println!("✓ 当前文件位置: {}", pos);
            
            // 5. 在指定位置写入
            file.seek(SeekFrom::Start(5)).unwrap();
            file.write_all("XXX".as_bytes()).unwrap();
            
            // 验证
            file.seek(SeekFrom::Start(0)).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            println!("✓ 修改后的内容: {}", content);
        }
        Err(e) => println!("✗ 打开文件失败: {}", e),
    }
    
    println!();
}

// ============================================
// 6. 目录操作
// ============================================
fn demo_directory_operations(temp_dir: &Path) {
    println!("--- 6. 目录操作 ---");
    
    // 1. 创建目录
    let new_dir = temp_dir.join("test_dir");
    match fs::create_dir(&new_dir) {
        Ok(_) => println!("✓ create_dir: 创建目录成功"),
        Err(e) => println!("✗ 创建目录失败: {}", e),
    }
    
    // 2. 创建多级目录
    let nested_dir = temp_dir.join("a/b/c/d");
    match fs::create_dir_all(&nested_dir) {
        Ok(_) => println!("✓ create_dir_all: 创建多级目录成功"),
        Err(e) => println!("✗ 创建失败: {}", e),
    }
    
    // 3. 列出目录内容
    match fs::read_dir(temp_dir) {
        Ok(entries) => {
            println!("✓ read_dir: 目录内容:");
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let file_type = if path.is_dir() { "DIR " } else { "FILE" };
                    println!("  [{}] {}", file_type, entry.file_name().to_string_lossy());
                }
            }
        }
        Err(e) => println!("✗ 读取目录失败: {}", e),
    }
    
    // 4. 递归遍历目录
    fn visit_dirs(dir: &Path, depth: usize) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                let indent = "  ".repeat(depth);
                println!("{}{}", indent, entry.file_name().to_string_lossy());
                
                if path.is_dir() && depth < 3 {
                    visit_dirs(&path, depth + 1)?;
                }
            }
        }
        Ok(())
    }
    
    println!("\n✓ 递归遍历:");
    let _ = visit_dirs(temp_dir, 0);
    
    // 5. 删除空目录
    match fs::remove_dir(&new_dir) {
        Ok(_) => println!("\n✓ remove_dir: 删除空目录成功"),
        Err(e) => println!("\n✗ 删除目录失败: {}", e),
    }
    
    // 6. 删除目录及其内容
    match fs::remove_dir_all(&nested_dir.parent().unwrap()) {
        Ok(_) => println!("✓ remove_dir_all: 递归删除目录成功"),
        Err(e) => println!("✗ 递归删除失败: {}", e),
    }
    
    println!();
}

// ============================================
// 7. 文件元信息
// ============================================
fn demo_file_metadata(temp_dir: &Path) {
    println!("--- 7. 文件元信息 ---");
    
    let test_file = temp_dir.join("metadata_test.txt");
    fs::write(&test_file, "测试文件元信息").unwrap();
    
    match fs::metadata(&test_file) {
        Ok(metadata) => {
            println!("文件: {}", test_file.display());
            println!("  大小: {} 字节", metadata.len());
            println!("  是否为文件: {}", metadata.is_file());
            println!("  是否为目录: {}", metadata.is_dir());
            println!("  是否为符号链接: {}", metadata.is_symlink());
            println!("  只读: {}", metadata.permissions().readonly());
            
            // 时间信息
            if let Ok(created) = metadata.created() {
                println!("  创建时间: {:?}", created);
            }
            if let Ok(modified) = metadata.modified() {
                println!("  修改时间: {:?}", modified);
            }
            if let Ok(accessed) = metadata.accessed() {
                println!("  访问时间: {:?}", accessed);
            }
            
            // Unix 特有信息
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                println!("  inode: {}", metadata.ino());
                println!("  权限: {:o}", metadata.mode());
            }
        }
        Err(e) => println!("✗ 获取元信息失败: {}", e),
    }
    
    // 检查文件/目录是否存在
    println!("\n路径检查:");
    println!("  文件存在: {}", test_file.exists());
    println!("  是文件: {}", test_file.is_file());
    println!("  是目录: {}", test_file.is_dir());
    
    println!();
}

// ============================================
// 8. 路径操作
// ============================================
fn demo_path_operations() {
    println!("--- 8. 路径操作 ---");
    
    // 创建路径
    let path = Path::new("/home/user/documents/file.txt");
    
    // 路径组成部分
    println!("完整路径: {}", path.display());
    println!("文件名: {:?}", path.file_name());
    println!("扩展名: {:?}", path.extension());
    println!("文件名（不含扩展名）: {:?}", path.file_stem());
    println!("父目录: {:?}", path.parent());
    
    // 路径拼接
    let base = Path::new("/home/user");
    let full = base.join("documents").join("file.txt");
    println!("\n路径拼接: {}", full.display());
    
    // 路径比较
    let path1 = Path::new("/home/user/file.txt");
    let path2 = Path::new("/home/user/file.txt");
    println!("\n路径相等: {}", path1 == path2);
    
    // 相对路径和绝对路径
    let relative = Path::new("../documents/file.txt");
    println!("\n相对路径: {}", relative.display());
    println!("是绝对路径: {}", relative.is_absolute());
    println!("是相对路径: {}", relative.is_relative());
    
    // PathBuf（可变路径）
    let mut path_buf = PathBuf::from("/home");
    path_buf.push("user");
    path_buf.push("documents");
    path_buf.set_extension("md");
    println!("\nPathBuf: {}", path_buf.display());
    
    // 获取当前目录
    if let Ok(current) = env::current_dir() {
        println!("\n当前目录: {}", current.display());
    }
    
    // 规范化路径
    let path = Path::new("/home/user/../user/./documents");
    if let Ok(canonical) = path.canonicalize() {
        println!("规范化路径: {}", canonical.display());
    }
    
    println!();
}

// ============================================
// 9. 实战案例
// ============================================
fn demo_real_world_cases(temp_dir: &Path) {
    println!("--- 9. 实战案例 ---");
    
    // 案例 1: 日志文件轮转
    println!("案例 1: 日志文件轮转");
    log_rotation_example(temp_dir);
    
    // 案例 2: 配置文件读写
    println!("\n案例 2: 配置文件管理");
    config_file_example(temp_dir);
    
    // 案例 3: 文件备份
    println!("\n案例 3: 文件备份");
    backup_file_example(temp_dir);
    
    // 案例 4: CSV 文件处理
    println!("\n案例 4: CSV 文件处理");
    csv_processing_example(temp_dir);
}

// 案例 1: 日志文件轮转
fn log_rotation_example(temp_dir: &Path) {
    struct Logger {
        log_file: PathBuf,
        max_size: u64,
    }
    
    impl Logger {
        fn new(log_file: PathBuf, max_size: u64) -> Self {
            Self { log_file, max_size }
        }
        
        fn log(&self, message: &str) -> io::Result<()> {
            // 检查文件大小
            if self.log_file.exists() {
                let metadata = fs::metadata(&self.log_file)?;
                if metadata.len() > self.max_size {
                    self.rotate()?;
                }
            }
            
            // 追加日志
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.log_file)?;
            
            // 简单的时间戳（不使用 chrono）
            use std::time::SystemTime;
            let now = SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            writeln!(file, "[{}] {}", now, message)?;
            Ok(())
        }
        
        fn rotate(&self) -> io::Result<()> {
            // 重命名当前日志文件
            let backup = self.log_file.with_extension("txt.old");
            fs::rename(&self.log_file, backup)?;
            Ok(())
        }
    }
    
    // 使用日志系统（模拟，不使用 chrono）
    let log_file = temp_dir.join("app.log");
    fs::write(&log_file, "").unwrap(); // 创建空文件
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .unwrap();
    
    for i in 1..=5 {
        writeln!(file, "[LOG] 消息 {}", i).unwrap();
    }
    
    println!("  ✓ 日志写入完成");
}

// 案例 2: 配置文件管理
fn config_file_example(temp_dir: &Path) {
    use std::collections::HashMap;
    
    // 简单的 INI 风格配置
    fn save_config(path: &Path, config: &HashMap<String, String>) -> io::Result<()> {
        let mut file = File::create(path)?;
        
        for (key, value) in config {
            writeln!(file, "{}={}", key, value)?;
        }
        
        Ok(())
    }
    
    fn load_config(path: &Path) -> io::Result<HashMap<String, String>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut config = HashMap::new();
        
        for line in reader.lines() {
            let line = line?;
            if let Some((key, value)) = line.split_once('=') {
                config.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        
        Ok(config)
    }
    
    let config_file = temp_dir.join("config.ini");
    
    // 保存配置
    let mut config = HashMap::new();
    config.insert("host".to_string(), "localhost".to_string());
    config.insert("port".to_string(), "8080".to_string());
    config.insert("debug".to_string(), "true".to_string());
    
    save_config(&config_file, &config).unwrap();
    println!("  ✓ 配置已保存");
    
    // 加载配置
    match load_config(&config_file) {
        Ok(loaded) => {
            println!("  ✓ 配置已加载: {:?}", loaded);
        }
        Err(e) => println!("  ✗ 加载失败: {}", e),
    }
}

// 案例 3: 文件备份
fn backup_file_example(temp_dir: &Path) {
    fn backup_file(source: &Path) -> io::Result<PathBuf> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let backup_name = format!(
            "{}.backup.{}",
            source.file_name().unwrap().to_string_lossy(),
            timestamp
        );
        
        let backup_path = source.with_file_name(backup_name);
        fs::copy(source, &backup_path)?;
        
        Ok(backup_path)
    }
    
    // 创建测试文件
    let original = temp_dir.join("important.txt");
    fs::write(&original, "重要数据").unwrap();
    
    // 备份
    match backup_file(&original) {
        Ok(backup) => {
            println!("  ✓ 备份成功: {}", backup.display());
            
            // 验证备份内容
            if let Ok(content) = fs::read_to_string(&backup) {
                println!("  ✓ 备份内容: {}", content);
            }
        }
        Err(e) => println!("  ✗ 备份失败: {}", e),
    }
}

// 案例 4: CSV 文件处理
fn csv_processing_example(temp_dir: &Path) {
    let csv_file = temp_dir.join("data.csv");
    
    // 写入 CSV
    match File::create(&csv_file) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            
            // 写入表头
            writeln!(writer, "姓名,年龄,城市").unwrap();
            
            // 写入数据
            let data = [
                ("张三", 25, "北京"),
                ("李四", 30, "上海"),
                ("王五", 28, "广州"),
            ];
            
            for (name, age, city) in &data {
                writeln!(writer, "{},{},{}", name, age, city).unwrap();
            }
            
            println!("  ✓ CSV 文件已创建");
        }
        Err(e) => println!("  ✗ 创建失败: {}", e),
    }
    
    // 读取 CSV
    match File::open(&csv_file) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader
                .lines()
                .filter_map(|l| l.ok())
                .collect();
            
            println!("  ✓ CSV 内容:");
            for line in lines {
                println!("    {}", line);
            }
        }
        Err(e) => println!("  ✗ 读取失败: {}", e),
    }
}

// ============================================
// 总结和最佳实践
// ============================================

/*
文件操作最佳实践：

1. 错误处理
   - 始终处理 Result，不要随意 unwrap()
   - 使用 ? 操作符简化错误传播

2. 资源管理
   - File 会在 drop 时自动关闭
   - BufWriter 会在 drop 时自动 flush
   - 但关键操作最好显式处理

3. 性能优化
   - 大文件使用 BufReader/BufWriter
   - 小文件可以直接使用 fs::read/write
   - 流式处理大文件，避免一次性加载到内存

4. 路径处理
   - 使用 Path/PathBuf 而不是字符串
   - 跨平台时注意路径分隔符
   - 使用 join() 而不是字符串拼接

5. 并发安全
   - 多线程访问同一文件需要同步
   - 考虑使用文件锁（需要额外的 crate）

6. 错误恢复
   - 关键操作前备份
   - 使用临时文件 + 原子重命名

7. 安全性
   - 验证用户提供的路径
   - 防止路径遍历攻击
   - 注意权限设置

运行示例：
  cargo run --bin file_operations
*/
