// 文件系统操作实战示例
// 运行命令: cargo run --example fs_practical

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

// ============================================================================
// 示例 1: 文件基本操作
// ============================================================================

fn example_basic_file_operations() -> io::Result<()> {
    println!("\n=== 示例 1: 文件基本操作 ===\n");
    
    // 创建测试目录
    fs::create_dir_all("test_files")?;
    
    // 写入文件
    println!("1. 写入文件...");
    fs::write("test_files/hello.txt", "Hello, Rust!")?;
    println!("   ✓ 文件创建成功");
    
    // 读取文件
    println!("\n2. 读取文件...");
    let content = fs::read_to_string("test_files/hello.txt")?;
    println!("   文件内容: {}", content);
    
    // 追加内容
    println!("\n3. 追加内容...");
    let mut file = OpenOptions::new()
        .append(true)
        .open("test_files/hello.txt")?;
    writeln!(file, "追加的新行")?;
    println!("   ✓ 内容追加成功");
    
    // 再次读取
    let content = fs::read_to_string("test_files/hello.txt")?;
    println!("   更新后的内容:");
    for line in content.lines() {
        println!("     {}", line);
    }
    
    // 复制文件
    println!("\n4. 复制文件...");
    fs::copy("test_files/hello.txt", "test_files/hello_copy.txt")?;
    println!("   ✓ 文件复制成功");
    
    // 重命名文件
    println!("\n5. 重命名文件...");
    fs::rename("test_files/hello_copy.txt", "test_files/hello_renamed.txt")?;
    println!("   ✓ 文件重命名成功");
    
    // 获取文件元数据
    println!("\n6. 文件元数据:");
    let metadata = fs::metadata("test_files/hello.txt")?;
    println!("   文件大小: {} 字节", metadata.len());
    println!("   是否为文件: {}", metadata.is_file());
    println!("   是否只读: {}", metadata.permissions().readonly());
    
    // 删除文件
    println!("\n7. 删除文件...");
    fs::remove_file("test_files/hello_renamed.txt")?;
    println!("   ✓ 文件删除成功");
    
    Ok(())
}

// ============================================================================
// 示例 2: 使用 BufReader 逐行读取
// ============================================================================

fn example_buffered_reading() -> io::Result<()> {
    println!("\n=== 示例 2: 缓冲读取（逐行） ===\n");
    
    // 创建测试文件
    let test_file = "test_files/lines.txt";
    let lines = vec![
        "第一行：Rust 是一门系统编程语言",
        "第二行：注重安全、并发和性能",
        "第三行：拥有现代化的工具链",
        "第四行：社区活跃且友好",
        "第五行：适合构建可靠高效的软件",
    ];
    
    fs::write(test_file, lines.join("\n"))?;
    println!("测试文件已创建，包含 {} 行\n", lines.len());
    
    // 方法 1: 使用 lines() 迭代器
    println!("方法 1: 使用 lines() 迭代器");
    let file = File::open(test_file)?;
    let reader = BufReader::new(file);
    
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        println!("  [{}] {}", i + 1, line);
    }
    
    // 方法 2: 使用 read_line()
    println!("\n方法 2: 使用 read_line()");
    let file = File::open(test_file)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut line_num = 1;
    
    while reader.read_line(&mut line)? > 0 {
        print!("  [{}] {}", line_num, line);
        line.clear();
        line_num += 1;
    }
    
    Ok(())
}

// ============================================================================
// 示例 3: 使用 BufWriter 高效写入
// ============================================================================

fn example_buffered_writing() -> io::Result<()> {
    println!("\n=== 示例 3: 缓冲写入 ===\n");
    
    let file = File::create("test_files/numbers.txt")?;
    let mut writer = BufWriter::new(file);
    
    println!("写入 1000 个数字...");
    for i in 1..=1000 {
        writeln!(writer, "数字: {}", i)?;
    }
    
    // 刷新缓冲区
    writer.flush()?;
    println!("✓ 写入完成\n");
    
    // 读取并显示前 10 行
    println!("前 10 行内容:");
    let file = File::open("test_files/numbers.txt")?;
    let reader = BufReader::new(file);
    
    for (i, line) in reader.lines().take(10).enumerate() {
        println!("  [{}] {}", i + 1, line?);
    }
    println!("  ...");
    
    Ok(())
}

// ============================================================================
// 示例 4: 文件定位（Seek）操作
// ============================================================================

fn example_seek_operations() -> io::Result<()> {
    println!("\n=== 示例 4: 文件定位操作 ===\n");
    
    // 创建测试文件
    let content = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    fs::write("test_files/seek_test.txt", content)?;
    println!("测试文件内容: {}\n", content);
    
    let mut file = File::open("test_files/seek_test.txt")?;
    
    // 从开始位置定位
    println!("1. 从开始定位到位置 10");
    file.seek(SeekFrom::Start(10))?;
    let mut buffer = [0u8; 5];
    file.read_exact(&mut buffer)?;
    println!("   读取 5 字节: {}", String::from_utf8_lossy(&buffer));
    
    // 相对当前位置定位
    println!("\n2. 相对当前位置前进 5 字节");
    file.seek(SeekFrom::Current(5))?;
    file.read_exact(&mut buffer)?;
    println!("   读取 5 字节: {}", String::from_utf8_lossy(&buffer));
    
    // 从末尾定位
    println!("\n3. 从末尾倒退 10 字节");
    file.seek(SeekFrom::End(-10))?;
    file.read_exact(&mut buffer)?;
    println!("   读取 5 字节: {}", String::from_utf8_lossy(&buffer));
    
    // 获取当前位置
    let position = file.stream_position()?;
    println!("\n4. 当前文件位置: {}", position);
    
    // 回到开始
    println!("\n5. 回到文件开始");
    file.rewind()?;
    file.read_exact(&mut buffer)?;
    println!("   读取 5 字节: {}", String::from_utf8_lossy(&buffer));
    
    Ok(())
}

// ============================================================================
// 示例 5: OpenOptions 高级用法
// ============================================================================

fn example_open_options() -> io::Result<()> {
    println!("\n=== 示例 5: OpenOptions 高级用法 ===\n");
    
    let test_file = "test_files/options_test.txt";
    
    // 1. 创建新文件（如果存在则失败）
    println!("1. 创建新文件（create_new）");
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(test_file)
    {
        Ok(mut file) => {
            writeln!(file, "这是新创建的文件")?;
            println!("   ✓ 文件创建成功");
        }
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            println!("   文件已存在，先删除...");
            fs::remove_file(test_file)?;
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(test_file)?;
            writeln!(file, "这是新创建的文件")?;
            println!("   ✓ 文件重新创建成功");
        }
        Err(e) => return Err(e),
    }
    
    // 2. 追加模式
    println!("\n2. 追加模式（append）");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(test_file)?;
    writeln!(file, "追加的第一行")?;
    writeln!(file, "追加的第二行")?;
    println!("   ✓ 内容追加成功");
    
    // 3. 读写模式
    println!("\n3. 读写模式");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(test_file)?;
    
    // 读取内容
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("   原始内容:");
    for line in content.lines() {
        println!("     {}", line);
    }
    
    // 在开头插入内容
    file.seek(SeekFrom::Start(0))?;
    let original = content.clone();
    file.set_len(0)?;
    writeln!(file, "=== 文件开头 ===")?;
    write!(file, "{}", original)?;
    println!("   ✓ 内容修改成功");
    
    // 显示最终内容
    let final_content = fs::read_to_string(test_file)?;
    println!("\n   最终内容:");
    for line in final_content.lines() {
        println!("     {}", line);
    }
    
    Ok(())
}

// ============================================================================
// 示例 6: 目录操作
// ============================================================================

fn example_directory_operations() -> io::Result<()> {
    println!("\n=== 示例 6: 目录操作 ===\n");
    
    // 创建目录结构
    println!("1. 创建目录结构");
    fs::create_dir_all("test_files/dir1/subdir1")?;
    fs::create_dir_all("test_files/dir1/subdir2")?;
    fs::create_dir_all("test_files/dir2")?;
    println!("   ✓ 目录结构创建成功");
    
    // 在目录中创建文件
    println!("\n2. 创建测试文件");
    fs::write("test_files/dir1/file1.txt", "内容 1")?;
    fs::write("test_files/dir1/file2.txt", "内容 2")?;
    fs::write("test_files/dir1/subdir1/file3.txt", "内容 3")?;
    fs::write("test_files/dir2/file4.txt", "内容 4")?;
    println!("   ✓ 测试文件创建成功");
    
    // 读取目录内容
    println!("\n3. 读取目录内容 (test_files/dir1):");
    for entry in fs::read_dir("test_files/dir1")? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        
        let type_str = if metadata.is_file() {
            "文件"
        } else if metadata.is_dir() {
            "目录"
        } else {
            "其他"
        };
        
        println!("   {} - {} ({} 字节)", 
            type_str,
            path.display(),
            metadata.len()
        );
    }
    
    // 递归遍历目录
    println!("\n4. 递归遍历目录树:");
    visit_dirs(Path::new("test_files/dir1"), 0)?;
    
    // 统计信息
    println!("\n5. 统计信息:");
    let (files, dirs, total_size) = count_dir_stats("test_files")?;
    println!("   文件数: {}", files);
    println!("   目录数: {}", dirs);
    println!("   总大小: {} 字节", total_size);
    
    Ok(())
}

// 递归访问目录
fn visit_dirs(dir: &Path, level: usize) -> io::Result<()> {
    if dir.is_dir() {
        let indent = "  ".repeat(level);
        println!("{}📁 {}", indent, dir.file_name().unwrap_or_default().to_string_lossy());
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                visit_dirs(&path, level + 1)?;
            } else {
                let metadata = entry.metadata()?;
                println!("{}📄 {} ({} 字节)", 
                    "  ".repeat(level + 1),
                    path.file_name().unwrap_or_default().to_string_lossy(),
                    metadata.len()
                );
            }
        }
    }
    Ok(())
}

// 统计目录信息
fn count_dir_stats(dir: &str) -> io::Result<(usize, usize, u64)> {
    let mut files = 0;
    let mut dirs = 0;
    let mut total_size = 0u64;
    
    fn visit(path: &Path, files: &mut usize, dirs: &mut usize, size: &mut u64) -> io::Result<()> {
        if path.is_dir() {
            *dirs += 1;
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                visit(&entry.path(), files, dirs, size)?;
            }
        } else {
            *files += 1;
            *size += fs::metadata(path)?.len();
        }
        Ok(())
    }
    
    visit(Path::new(dir), &mut files, &mut dirs, &mut total_size)?;
    Ok((files, dirs, total_size))
}

// ============================================================================
// 示例 7: 文件元数据
// ============================================================================

fn example_metadata() -> io::Result<()> {
    println!("\n=== 示例 7: 文件元数据 ===\n");
    
    let test_file = "test_files/metadata_test.txt";
    fs::write(test_file, "测试元数据")?;
    
    let metadata = fs::metadata(test_file)?;
    
    println!("文件: {}", test_file);
    println!("\n基本信息:");
    println!("  文件大小: {} 字节", metadata.len());
    println!("  是文件: {}", metadata.is_file());
    println!("  是目录: {}", metadata.is_dir());
    println!("  是符号链接: {}", metadata.is_symlink());
    
    println!("\n权限:");
    let permissions = metadata.permissions();
    println!("  只读: {}", permissions.readonly());
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        println!("  权限模式: {:o}", permissions.mode());
    }
    
    println!("\n时间戳:");
    if let Ok(modified) = metadata.modified() {
        println!("  修改时间: {:?}", modified);
    }
    if let Ok(accessed) = metadata.accessed() {
        println!("  访问时间: {:?}", accessed);
    }
    if let Ok(created) = metadata.created() {
        println!("  创建时间: {:?}", created);
    }
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        println!("\nUnix 特定信息:");
        println!("  Inode: {}", metadata.ino());
        println!("  硬链接数: {}", metadata.nlink());
        println!("  UID: {}", metadata.uid());
        println!("  GID: {}", metadata.gid());
    }
    
    Ok(())
}

// ============================================================================
// 示例 8: 实战 - 简单的日志系统
// ============================================================================

struct SimpleLogger {
    file: BufWriter<File>,
}

impl SimpleLogger {
    fn new(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;
        
        Ok(SimpleLogger {
            file: BufWriter::new(file),
        })
    }
    
    fn log(&mut self, level: &str, message: &str) -> io::Result<()> {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(self.file, "[{}] [{}] {}", timestamp, level, message)?;
        self.file.flush()?;
        Ok(())
    }
    
    fn info(&mut self, message: &str) -> io::Result<()> {
        self.log("INFO", message)
    }
    
    fn warn(&mut self, message: &str) -> io::Result<()> {
        self.log("WARN", message)
    }
    
    fn error(&mut self, message: &str) -> io::Result<()> {
        self.log("ERROR", message)
    }
}

fn example_logger() -> io::Result<()> {
    println!("\n=== 示例 8: 简单日志系统 ===\n");
    
    let mut logger = SimpleLogger::new("test_files/app.log")?;
    
    println!("写入日志...");
    logger.info("应用程序启动")?;
    logger.info("加载配置文件")?;
    logger.warn("配置文件中缺少某些选项，使用默认值")?;
    logger.info("连接到数据库")?;
    logger.error("数据库连接失败: 超时")?;
    logger.info("使用本地缓存")?;
    println!("✓ 日志写入完成\n");
    
    // 读取并显示日志
    println!("日志内容:");
    let content = fs::read_to_string("test_files/app.log")?;
    for line in content.lines() {
        println!("  {}", line);
    }
    
    Ok(())
}

// ============================================================================
// 示例 9: 实战 - 配置文件管理
// ============================================================================

use std::collections::HashMap;

struct ConfigManager {
    path: PathBuf,
    data: HashMap<String, String>,
}

impl ConfigManager {
    fn load(path: &str) -> io::Result<Self> {
        let path_buf = PathBuf::from(path);
        let mut data = HashMap::new();
        
        // 如果文件存在则加载
        if path_buf.exists() {
            let content = fs::read_to_string(&path_buf)?;
            for line in content.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    data.insert(
                        key.trim().to_string(),
                        value.trim().to_string()
                    );
                }
            }
        }
        
        Ok(ConfigManager {
            path: path_buf,
            data,
        })
    }
    
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
    
    fn save(&self) -> io::Result<()> {
        let mut content = String::new();
        for (key, value) in &self.data {
            content.push_str(&format!("{}={}\n", key, value));
        }
        fs::write(&self.path, content)?;
        Ok(())
    }
}

fn example_config_manager() -> io::Result<()> {
    println!("\n=== 示例 9: 配置文件管理 ===\n");
    
    let config_path = "test_files/app.config";
    
    // 创建或加载配置
    println!("1. 创建配置管理器");
    let mut config = ConfigManager::load(config_path)?;
    println!("   ✓ 配置管理器初始化完成");
    
    // 设置配置项
    println!("\n2. 设置配置项");
    config.set("app_name", "Rust 实战示例");
    config.set("version", "1.0.0");
    config.set("debug", "true");
    config.set("max_connections", "100");
    println!("   ✓ 配置项已设置");
    
    // 保存配置
    println!("\n3. 保存配置到文件");
    config.save()?;
    println!("   ✓ 配置已保存");
    
    // 重新加载配置
    println!("\n4. 重新加载配置");
    let config = ConfigManager::load(config_path)?;
    println!("   ✓ 配置已加载");
    
    // 读取配置项
    println!("\n5. 读取配置项:");
    for key in ["app_name", "version", "debug", "max_connections"] {
        if let Some(value) = config.get(key) {
            println!("   {} = {}", key, value);
        }
    }
    
    // 显示原始文件内容
    println!("\n6. 配置文件内容:");
    let content = fs::read_to_string(config_path)?;
    for line in content.lines() {
        println!("   {}", line);
    }
    
    Ok(())
}

// ============================================================================
// 示例 10: 文件清理工具
// ============================================================================

fn example_cleanup() -> io::Result<()> {
    println!("\n=== 示例 10: 清理测试文件 ===\n");
    
    let test_dir = "test_files";
    
    if Path::new(test_dir).exists() {
        println!("清理目录: {}", test_dir);
        
        // 统计信息
        let (files, dirs, size) = count_dir_stats(test_dir)?;
        println!("  文件数: {}", files);
        println!("  目录数: {}", dirs);
        println!("  总大小: {} 字节", size);
        
        // 删除整个目录
        print!("\n确认删除? (将在 2 秒后自动删除)... ");
        io::stdout().flush()?;
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("删除中...");
        
        fs::remove_dir_all(test_dir)?;
        println!("✓ 清理完成");
    } else {
        println!("目录不存在: {}", test_dir);
    }
    
    Ok(())
}

// ============================================================================
// 主函数
// ============================================================================

fn main() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║    Rust 文件系统操作实战示例                ║");
    println!("╚══════════════════════════════════════════════╝");
    
    let examples: Vec<(&str, fn() -> io::Result<()>)> = vec![
        ("文件基本操作", example_basic_file_operations),
        ("缓冲读取（逐行）", example_buffered_reading),
        ("缓冲写入", example_buffered_writing),
        ("文件定位操作", example_seek_operations),
        ("OpenOptions 高级用法", example_open_options),
        ("目录操作", example_directory_operations),
        ("文件元数据", example_metadata),
        ("简单日志系统", example_logger),
        ("配置文件管理", example_config_manager),
    ];
    
    for (name, example_fn) in examples {
        println!("\n{}", "=".repeat(60));
        println!("运行示例: {}", name);
        println!("{}", "=".repeat(60));
        
        if let Err(e) = example_fn() {
            eprintln!("❌ 示例运行失败: {}", e);
        }
        
        // 添加延迟
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    
    // 清理
    println!("\n{}", "=".repeat(60));
    println!("清理测试文件");
    println!("{}", "=".repeat(60));
    
    if let Err(e) = example_cleanup() {
        eprintln!("❌ 清理失败: {}", e);
    }
    
    println!("\n{}", "=".repeat(60));
    println!("所有示例运行完成！");
    println!("{}", "=".repeat(60));
    println!();
}
