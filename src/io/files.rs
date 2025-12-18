// 文件读写操作

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter, Seek, SeekFrom};

/// # 读取整个文件
pub fn read_entire_file_demo() {
    println!("\n=== 读取整个文件 ===");
    
    let file_path = "/tmp/test_read.txt";
    fs::write(file_path, "Hello, Rust!\n这是测试内容。").unwrap();
    
    // 读取为 String
    match fs::read_to_string(file_path) {
        Ok(contents) => println!("文件内容:\n{}", contents),
        Err(e) => println!("读取失败: {}", e),
    }
    
    // 读取为字节
    match fs::read(file_path) {
        Ok(bytes) => println!("读取 {} 字节", bytes.len()),
        Err(e) => println!("读取失败: {}", e),
    }
    
    let _ = fs::remove_file(file_path);
}

/// # 写入整个文件
pub fn write_entire_file_demo() {
    println!("\n=== 写入整个文件 ===");
    
    let file_path = "/tmp/test_write.txt";
    
    // 写入字符串
    match fs::write(file_path, "Hello, Rust!") {
        Ok(_) => println!("写入成功"),
        Err(e) => println!("写入失败: {}", e),
    }
    
    // 验证
    let contents = fs::read_to_string(file_path).unwrap();
    println!("验证内容: {}", contents);
    
    let _ = fs::remove_file(file_path);
}

/// # 使用 File 打开文件
pub fn open_file_demo() {
    println!("\n=== 使用 File 打开文件 ===");
    
    let file_path = "/tmp/test_open.txt";
    fs::write(file_path, "test content").unwrap();
    
    // 打开文件读取
    match File::open(file_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            println!("读取内容: {}", contents);
        }
        Err(e) => println!("打开失败: {}", e),
    }
    
    // 创建新文件
    match File::create(file_path) {
        Ok(mut file) => {
            file.write_all(b"new content").unwrap();
            println!("创建并写入成功");
        }
        Err(e) => println!("创建失败: {}", e),
    }
    
    let _ = fs::remove_file(file_path);
}

/// # OpenOptions 高级选项
pub fn open_options_demo() {
    println!("\n=== OpenOptions 高级选项 ===");
    
    let file_path = "/tmp/test_options.txt";
    
    // 读写模式
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();
    
    file.write_all(b"Hello").unwrap();
    println!("写入: Hello");
    
    // 追加模式
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .unwrap();
    
    file.write_all(b" World").unwrap();
    println!("追加: World");
    
    // 验证
    let contents = fs::read_to_string(file_path).unwrap();
    println!("最终内容: {}", contents);
    
    // 截断模式
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    
    file.write_all(b"New").unwrap();
    println!("截断后: {}", fs::read_to_string(file_path).unwrap());
    
    let _ = fs::remove_file(file_path);
}

/// # 缓冲读写
pub fn buffered_io_demo() {
    println!("\n=== 缓冲读写 ===");
    
    let file_path = "/tmp/test_buffered.txt";
    
    // BufWriter - 缓冲写入
    {
        let file = File::create(file_path).unwrap();
        let mut writer = BufWriter::new(file);
        
        for i in 0..5 {
            writeln!(writer, "Line {}", i).unwrap();
        }
        
        writer.flush().unwrap();
        println!("缓冲写入完成");
    }
    
    // BufReader - 缓冲读取
    {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        
        println!("\n逐行读取:");
        for line in reader.lines() {
            println!("  {}", line.unwrap());
        }
    }
    
    let _ = fs::remove_file(file_path);
}

/// # 逐行读取
pub fn line_by_line_demo() {
    println!("\n=== 逐行读取 ===");
    
    let file_path = "/tmp/test_lines.txt";
    fs::write(file_path, "line 1\nline 2\nline 3\n").unwrap();
    
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    
    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => println!("Line {}: {}", index + 1, content),
            Err(e) => println!("读取错误: {}", e),
        }
    }
    
    let _ = fs::remove_file(file_path);
}

/// # 文件定位 (Seek)
pub fn seek_demo() {
    println!("\n=== 文件定位 ===");
    
    let file_path = "/tmp/test_seek.txt";
    fs::write(file_path, "0123456789").unwrap();
    
    let mut file = File::open(file_path).unwrap();
    
    // 从开头偏移
    file.seek(SeekFrom::Start(5)).unwrap();
    let mut buffer = [0; 5];
    file.read_exact(&mut buffer).unwrap();
    println!("从位置5读取: {}", String::from_utf8_lossy(&buffer));
    
    // 从结尾偏移
    file.seek(SeekFrom::End(-5)).unwrap();
    file.read_exact(&mut buffer).unwrap();
    println!("从末尾-5读取: {}", String::from_utf8_lossy(&buffer));
    
    // 相对当前位置
    file.seek(SeekFrom::Start(0)).unwrap();
    file.seek(SeekFrom::Current(3)).unwrap();
    file.read_exact(&mut buffer).unwrap();
    println!("相对偏移3读取: {}", String::from_utf8_lossy(&buffer));
    
    let _ = fs::remove_file(file_path);
}

/// # 标准输入输出
pub fn stdio_demo() {
    println!("\n=== 标准输入输出 ===");
    
    // 标准输出
    println!("这是 println! 输出");
    print!("这是 print! 输出（无换行）");
    println!();
    
    // 标准错误
    eprintln!("这是错误输出");
    
    // 格式化到字符串
    let s = format!("格式化: {}", 42);
    println!("{}", s);
    
    println!("\n标准输入示例:");
    println!("  use std::io::{{self, BufRead}};");
    println!("  let stdin = io::stdin();");
    println!("  for line in stdin.lock().lines() {{");
    println!("      println!(\"输入: {{}}\", line?);");
    println!("  }}");
}

/// # 二进制文件操作
pub fn binary_file_demo() {
    println!("\n=== 二进制文件操作 ===");
    
    let file_path = "/tmp/test_binary.bin";
    
    // 写入二进制数据
    let data: Vec<u8> = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];  // "Hello"
    fs::write(file_path, &data).unwrap();
    println!("写入二进制数据: {:?}", data);
    
    // 读取二进制数据
    let read_data = fs::read(file_path).unwrap();
    println!("读取二进制数据: {:?}", read_data);
    println!("转换为字符串: {}", String::from_utf8_lossy(&read_data));
    
    // 按块读取
    let mut file = File::open(file_path).unwrap();
    let mut buffer = [0u8; 2];
    
    println!("\n按块读取:");
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        println!("  读取 {} 字节: {:?}", n, &buffer[..n]);
    }
    
    let _ = fs::remove_file(file_path);
}

/// # 内存映射文件
pub fn memory_mapped_file_demo() {
    println!("\n=== 内存映射文件 ===");
    
    println!("内存映射文件通常使用 memmap2 库:");
    println!("  use memmap2::Mmap;");
    println!("  let file = File::open(path)?;");
    println!("  let mmap = unsafe {{ Mmap::map(&file)? }};");
    println!("  let data = &mmap[..];");
    
    println!("\n优势:");
    println!("  - 适合大文件");
    println!("  - 减少系统调用");
    println!("  - 操作系统管理内存");
}

/// # 文件锁
pub fn file_locking_demo() {
    println!("\n=== 文件锁 ===");
    
    println!("文件锁通常使用 fs2 库:");
    println!("  use fs2::FileExt;");
    println!("  let file = File::open(path)?;");
    println!("  file.lock_exclusive()?;  // 独占锁");
    println!("  file.lock_shared()?;     // 共享锁");
    println!("  file.unlock()?;          // 解锁");
}

/// # 实战示例：日志文件
pub fn log_file_demo() {
    println!("\n=== 实战示例：日志文件 ===");
    
    struct Logger {
        file: BufWriter<File>,
    }
    
    impl Logger {
        fn new(path: &str) -> io::Result<Self> {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?;
            
            Ok(Logger {
                file: BufWriter::new(file),
            })
        }
        
        fn log(&mut self, level: &str, message: &str) -> io::Result<()> {
            let timestamp = chrono::Local::now();
            writeln!(
                self.file,
                "[{}] [{}] {}",
                timestamp.format("%Y-%m-%d %H:%M:%S"),
                level,
                message
            )?;
            self.file.flush()
        }
        
        fn info(&mut self, message: &str) -> io::Result<()> {
            self.log("INFO", message)
        }
        
        fn error(&mut self, message: &str) -> io::Result<()> {
            self.log("ERROR", message)
        }
    }
    
    let log_path = "/tmp/app.log";
    let mut logger = Logger::new(log_path).unwrap();
    
    logger.info("应用程序启动").unwrap();
    logger.error("发生错误").unwrap();
    logger.info("应用程序关闭").unwrap();
    
    println!("日志内容:");
    let contents = fs::read_to_string(log_path).unwrap();
    print!("{}", contents);
    
    let _ = fs::remove_file(log_path);
}

/// # 实战示例：CSV 文件处理
pub fn csv_file_demo() {
    println!("\n=== 实战示例：CSV 文件 ===");
    
    let csv_path = "/tmp/data.csv";
    
    // 写入 CSV
    let mut file = File::create(csv_path).unwrap();
    writeln!(file, "名字,年龄,城市").unwrap();
    writeln!(file, "张三,25,北京").unwrap();
    writeln!(file, "李四,30,上海").unwrap();
    writeln!(file, "王五,28,广州").unwrap();
    
    // 读取 CSV
    let file = File::open(csv_path).unwrap();
    let reader = BufReader::new(file);
    
    println!("CSV 内容:");
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if i == 0 {
            println!("  表头: {}", line);
        } else {
            let fields: Vec<&str> = line.split(',').collect();
            println!("  记录: 名字={}, 年龄={}, 城市={}", 
                     fields[0], fields[1], fields[2]);
        }
    }
    
    let _ = fs::remove_file(csv_path);
    
    println!("\n实际项目建议使用 csv 库:");
    println!("  use csv::{{Reader, Writer}};");
}

/// # 实战示例：配置文件
pub fn config_file_demo() {
    println!("\n=== 实战示例：配置文件 ===");
    
    use std::collections::HashMap;
    
    let config_path = "/tmp/config.txt";
    
    // 写入配置
    let mut file = File::create(config_path).unwrap();
    writeln!(file, "host=localhost").unwrap();
    writeln!(file, "port=8080").unwrap();
    writeln!(file, "timeout=30").unwrap();
    
    // 读取配置
    let file = File::open(config_path).unwrap();
    let reader = BufReader::new(file);
    
    let mut config = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some((key, value)) = line.split_once('=') {
            config.insert(key.to_string(), value.to_string());
        }
    }
    
    println!("配置:");
    for (key, value) in &config {
        println!("  {} = {}", key, value);
    }
    
    let _ = fs::remove_file(config_path);
    
    println!("\n实际项目建议使用:");
    println!("  - toml 库处理 TOML 配置");
    println!("  - serde_json 处理 JSON 配置");
    println!("  - serde_yaml 处理 YAML 配置");
}

/// # 文件 I/O 最佳实践
pub fn file_io_best_practices_demo() {
    println!("\n=== 文件 I/O 最佳实践 ===");
    
    println!("1. 选择合适的方法:");
    println!("   - 小文件: fs::read_to_string, fs::write");
    println!("   - 大文件: BufReader, BufWriter");
    println!("   - 逐行处理: BufReader::lines()");
    
    println!("\n2. 错误处理:");
    println!("   - 总是处理 io::Result");
    println!("   - 使用 ? 操作符传播错误");
    println!("   - 提供有意义的错误上下文");
    
    println!("\n3. 资源管理:");
    println!("   - 文件在离开作用域时自动关闭");
    println!("   - 使用 flush() 确保数据写入");
    println!("   - 考虑使用 RAII 模式");
    
    println!("\n4. 性能优化:");
    println!("   - 使用缓冲 I/O");
    println!("   - 批量操作");
    println!("   - 考虑内存映射");
    println!("   - 异步 I/O (tokio::fs)");
    
    println!("\n5. 安全性:");
    println!("   - 验证文件路径");
    println!("   - 检查权限");
    println!("   - 避免路径遍历攻击");
}

/// 运行所有文件操作示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust 文件操作详解           ║");
    println!("╚════════════════════════════════════╝");
    
    read_entire_file_demo();
    write_entire_file_demo();
    open_file_demo();
    open_options_demo();
    buffered_io_demo();
    line_by_line_demo();
    seek_demo();
    stdio_demo();
    binary_file_demo();
    memory_mapped_file_demo();
    file_locking_demo();
    log_file_demo();
    csv_file_demo();
    config_file_demo();
    file_io_best_practices_demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_write() {
        let path = "/tmp/test_rw.txt";
        let content = "test content";
        
        fs::write(path, content).unwrap();
        let read = fs::read_to_string(path).unwrap();
        
        assert_eq!(read, content);
        fs::remove_file(path).unwrap();
    }
    
    #[test]
    fn test_buffered_io() {
        let path = "/tmp/test_buf.txt";
        
        {
            let file = File::create(path).unwrap();
            let mut writer = BufWriter::new(file);
            writeln!(writer, "line 1").unwrap();
            writeln!(writer, "line 2").unwrap();
            writer.flush().unwrap();
        }
        
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        
        assert_eq!(lines.len(), 2);
        fs::remove_file(path).unwrap();
    }
}
