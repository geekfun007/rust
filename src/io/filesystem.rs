// 文件系统操作

use std::fs;
use std::path::{Path, PathBuf};
use std::io;

/// # 读取目录
pub fn read_directory_demo() {
    println!("\n=== 读取目录 ===");
    
    // 创建测试目录
    let test_dir = "/tmp/rust_demo";
    let _ = fs::create_dir(test_dir);
    let _ = fs::write(format!("{}/file1.txt", test_dir), "content1");
    let _ = fs::write(format!("{}/file2.txt", test_dir), "content2");
    
    // 读取目录内容
    match fs::read_dir(test_dir) {
        Ok(entries) => {
            println!("目录内容:");
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  {:?}", entry.path());
                }
            }
        }
        Err(e) => println!("读取目录失败: {}", e),
    }
}

/// # 创建和删除目录
pub fn create_remove_directory_demo() {
    println!("\n=== 创建和删除目录 ===");
    
    let dir_path = "/tmp/test_dir";
    
    // 创建目录
    match fs::create_dir(dir_path) {
        Ok(_) => println!("创建目录: {}", dir_path),
        Err(e) => println!("创建目录失败: {}", e),
    }
    
    // 创建多层目录
    let nested_path = "/tmp/test_dir/nested/deep";
    match fs::create_dir_all(nested_path) {
        Ok(_) => println!("创建多层目录: {}", nested_path),
        Err(e) => println!("创建多层目录失败: {}", e),
    }
    
    // 删除空目录
    let _ = fs::remove_dir("/tmp/test_dir/nested/deep");
    println!("删除空目录");
    
    // 递归删除目录及其内容
    match fs::remove_dir_all("/tmp/test_dir") {
        Ok(_) => println!("递归删除目录"),
        Err(e) => println!("删除目录失败: {}", e),
    }
}

/// # 文件元数据
pub fn file_metadata_demo() {
    println!("\n=== 文件元数据 ===");
    
    let file_path = "/tmp/test_metadata.txt";
    fs::write(file_path, "test content").unwrap();
    
    match fs::metadata(file_path) {
        Ok(metadata) => {
            println!("文件: {}", file_path);
            println!("  大小: {} 字节", metadata.len());
            println!("  是文件: {}", metadata.is_file());
            println!("  是目录: {}", metadata.is_dir());
            println!("  是符号链接: {}", metadata.is_symlink());
            println!("  只读: {}", metadata.permissions().readonly());
            
            if let Ok(modified) = metadata.modified() {
                println!("  修改时间: {:?}", modified);
            }
            
            if let Ok(accessed) = metadata.accessed() {
                println!("  访问时间: {:?}", accessed);
            }
            
            if let Ok(created) = metadata.created() {
                println!("  创建时间: {:?}", created);
            }
        }
        Err(e) => println!("获取元数据失败: {}", e),
    }
    
    let _ = fs::remove_file(file_path);
}

/// # 文件权限
pub fn file_permissions_demo() {
    println!("\n=== 文件权限 ===");
    
    let file_path = "/tmp/test_permissions.txt";
    fs::write(file_path, "test").unwrap();
    
    // 获取权限
    let metadata = fs::metadata(file_path).unwrap();
    let mut permissions = metadata.permissions();
    
    println!("当前只读: {}", permissions.readonly());
    
    // 设置为只读
    permissions.set_readonly(true);
    fs::set_permissions(file_path, permissions.clone()).unwrap();
    println!("设置为只读: {}", permissions.readonly());
    
    // 尝试写入（会失败）
    match fs::write(file_path, "new content") {
        Ok(_) => println!("写入成功"),
        Err(e) => println!("写入失败: {}", e),
    }
    
    // 恢复写权限
    permissions.set_readonly(false);
    fs::set_permissions(file_path, permissions).unwrap();
    fs::write(file_path, "new content").unwrap();
    println!("恢复写权限后写入成功");
    
    let _ = fs::remove_file(file_path);
}

/// # Path 和 PathBuf
pub fn path_operations_demo() {
    println!("\n=== Path 和 PathBuf 操作 ===");
    
    // 创建 Path
    let path = Path::new("/tmp/test.txt");
    
    println!("路径: {:?}", path);
    println!("文件名: {:?}", path.file_name());
    println!("扩展名: {:?}", path.extension());
    println!("父目录: {:?}", path.parent());
    println!("是绝对路径: {}", path.is_absolute());
    println!("是相对路径: {}", path.is_relative());
    
    // PathBuf - 可修改的路径
    let mut path_buf = PathBuf::from("/tmp");
    path_buf.push("test");
    path_buf.push("file.txt");
    println!("\nPathBuf: {:?}", path_buf);
    
    path_buf.set_extension("log");
    println!("修改扩展名: {:?}", path_buf);
    
    path_buf.pop();
    println!("弹出最后一个组件: {:?}", path_buf);
    
    // 路径连接
    let base = Path::new("/tmp");
    let full = base.join("test").join("file.txt");
    println!("\n连接路径: {:?}", full);
    
    // 遍历路径组件
    println!("\n路径组件:");
    for component in full.components() {
        println!("  {:?}", component);
    }
}

/// # 文件复制和移动
pub fn copy_move_files_demo() {
    println!("\n=== 文件复制和移动 ===");
    
    let source = "/tmp/source.txt";
    let dest1 = "/tmp/dest1.txt";
    let dest2 = "/tmp/dest2.txt";
    
    // 创建源文件
    fs::write(source, "source content").unwrap();
    
    // 复制文件
    match fs::copy(source, dest1) {
        Ok(bytes) => println!("复制 {} 字节", bytes),
        Err(e) => println!("复制失败: {}", e),
    }
    
    // 移动/重命名文件
    match fs::rename(dest1, dest2) {
        Ok(_) => println!("移动文件成功"),
        Err(e) => println!("移动失败: {}", e),
    }
    
    // 清理
    let _ = fs::remove_file(source);
    let _ = fs::remove_file(dest2);
}

/// # 符号链接
pub fn symlink_demo() {
    println!("\n=== 符号链接 ===");
    
    #[cfg(unix)]
    {
        use std::os::unix::fs as unix_fs;
        
        let target = "/tmp/target.txt";
        let link = "/tmp/link.txt";
        
        fs::write(target, "target content").unwrap();
        
        // 创建符号链接
        match unix_fs::symlink(target, link) {
            Ok(_) => println!("创建符号链接: {} -> {}", link, target),
            Err(e) => println!("创建符号链接失败: {}", e),
        }
        
        // 读取链接
        match fs::read_link(link) {
            Ok(path) => println!("链接指向: {:?}", path),
            Err(e) => println!("读取链接失败: {}", e),
        }
        
        // 清理
        let _ = fs::remove_file(link);
        let _ = fs::remove_file(target);
    }
    
    #[cfg(not(unix))]
    {
        println!("符号链接仅在 Unix 系统上演示");
    }
}

/// # 遍历目录树
pub fn walk_directory_tree_demo() {
    println!("\n=== 遍历目录树 ===");
    
    let test_dir = "/tmp/tree_test";
    
    // 创建测试目录结构
    fs::create_dir_all(format!("{}/dir1/subdir1", test_dir)).ok();
    fs::create_dir_all(format!("{}/dir2/subdir2", test_dir)).ok();
    fs::write(format!("{}/file1.txt", test_dir), "").ok();
    fs::write(format!("{}/dir1/file2.txt", test_dir), "").ok();
    fs::write(format!("{}/dir1/subdir1/file3.txt", test_dir), "").ok();
    
    // 递归遍历
    fn walk_dir(path: &Path, level: usize) -> io::Result<()> {
        let indent = "  ".repeat(level);
        
        if path.is_dir() {
            println!("{}目录: {:?}", indent, path.file_name().unwrap());
            
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                walk_dir(&entry.path(), level + 1)?;
            }
        } else {
            println!("{}文件: {:?}", indent, path.file_name().unwrap());
        }
        
        Ok(())
    }
    
    let _ = walk_dir(Path::new(test_dir), 0);
    
    // 清理
    let _ = fs::remove_dir_all(test_dir);
}

/// # 监视文件变化
pub fn file_watching_demo() {
    println!("\n=== 监视文件变化 ===");
    
    println!("文件监视通常使用 notify 库");
    println!("示例:");
    println!("  use notify::{{Watcher, RecursiveMode}};");
    println!("  let mut watcher = notify::watcher(tx, Duration::from_secs(1))?;");
    println!("  watcher.watch(path, RecursiveMode::Recursive)?;");
}

/// # 临时文件和目录
pub fn temp_files_demo() {
    println!("\n=== 临时文件和目录 ===");
    
    println!("使用 tempfile 库创建临时文件:");
    println!("  use tempfile::{{NamedTempFile, TempDir}};");
    println!("  let temp_file = NamedTempFile::new()?;");
    println!("  let temp_dir = TempDir::new()?;");
    
    // 使用标准库
    use std::env;
    let temp_dir = env::temp_dir();
    println!("\n系统临时目录: {:?}", temp_dir);
}

/// # 实战示例：文件管理器
pub fn file_manager_demo() {
    println!("\n=== 实战示例：文件管理器 ===");
    
    struct FileManager {
        base_path: PathBuf,
    }
    
    impl FileManager {
        fn new(base_path: impl AsRef<Path>) -> io::Result<Self> {
            let base_path = base_path.as_ref().to_path_buf();
            fs::create_dir_all(&base_path)?;
            Ok(FileManager { base_path })
        }
        
        fn list_files(&self) -> io::Result<Vec<PathBuf>> {
            let mut files = Vec::new();
            
            for entry in fs::read_dir(&self.base_path)? {
                let entry = entry?;
                if entry.path().is_file() {
                    files.push(entry.path());
                }
            }
            
            Ok(files)
        }
        
        fn create_file(&self, name: &str, content: &str) -> io::Result<()> {
            let path = self.base_path.join(name);
            fs::write(path, content)
        }
        
        fn delete_file(&self, name: &str) -> io::Result<()> {
            let path = self.base_path.join(name);
            fs::remove_file(path)
        }
        
        fn file_info(&self, name: &str) -> io::Result<String> {
            let path = self.base_path.join(name);
            let metadata = fs::metadata(&path)?;
            
            Ok(format!(
                "文件: {}\n  大小: {} 字节\n  修改时间: {:?}",
                name,
                metadata.len(),
                metadata.modified()?
            ))
        }
    }
    
    let manager = FileManager::new("/tmp/file_manager_demo").unwrap();
    
    manager.create_file("test1.txt", "内容1").unwrap();
    manager.create_file("test2.txt", "内容2").unwrap();
    
    println!("文件列表:");
    for file in manager.list_files().unwrap() {
        println!("  {:?}", file.file_name().unwrap());
    }
    
    if let Ok(info) = manager.file_info("test1.txt") {
        println!("\n{}", info);
    }
    
    let _ = fs::remove_dir_all("/tmp/file_manager_demo");
}

/// 运行所有文件系统示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║      Rust 文件系统操作详解         ║");
    println!("╚════════════════════════════════════╝");
    
    read_directory_demo();
    create_remove_directory_demo();
    file_metadata_demo();
    file_permissions_demo();
    path_operations_demo();
    copy_move_files_demo();
    symlink_demo();
    walk_directory_tree_demo();
    file_watching_demo();
    temp_files_demo();
    file_manager_demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_directory_operations() {
        let dir = "/tmp/test_dir_ops";
        fs::create_dir(dir).ok();
        assert!(Path::new(dir).exists());
        fs::remove_dir(dir).unwrap();
        assert!(!Path::new(dir).exists());
    }
    
    #[test]
    fn test_path_operations() {
        let path = Path::new("/tmp/test.txt");
        assert_eq!(path.file_name().unwrap(), "test.txt");
        assert_eq!(path.extension().unwrap(), "txt");
    }
}
