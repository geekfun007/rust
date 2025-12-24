// Rust To 系列转换详解
//
// 本教程涵盖所有 to_ 开头的转换方法
// 包括 to_string(), to_str(), to_owned() 等

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

fn main() {
    println!("=== Rust To 系列转换详解 ===\n");
    
    // 1. To 基础
    demo_to_basics();
    
    // 2. to_string() - 字符串转换
    demo_to_string();
    
    // 3. to_str() - 字符串切片
    demo_to_str();
    
    // 4. to_owned() - 所有权转换
    demo_to_owned();
    
    // 5. to_vec() - 向量转换
    demo_to_vec();
    
    // 6. to_lowercase() / to_uppercase() - 大小写转换
    demo_to_case();
    
    // 7. to_bytes() - 字节转换
    demo_to_bytes();
    
    // 8. to_digit() - 数字转换
    demo_to_digit();
    
    // 9. to_radians() / to_degrees() - 角度转换
    demo_to_radians();
    
    // 10. to_le_bytes() / to_be_bytes() - 字节序转换
    demo_to_bytes_endian();
    
    // 11. to_os_string() / to_path_buf() - 路径转换
    demo_to_os();
    
    // 12. to_socket_addrs() - 网络地址转换
    demo_to_socket_addrs();
    
    // 13. as_ vs to_ vs into_
    demo_conversion_comparison();
    
    // 14. 实战案例
    demo_real_world_examples();
    
    println!("\n✅ 所有示例运行成功！");
}

// ============================================
// 1. To 基础
// ============================================
fn demo_to_basics() {
    println!("--- 1. To 基础 ---\n");
    
    println!("什么是 to_ 方法？");
    println!("  - 创建新的拥有所有权的值");
    println!("  - 通常涉及克隆或分配");
    println!("  - 不消费原值（借用）");
    println!("  - 返回新的独立值\n");
    
    println!("To vs Into vs As:");
    println!("┌────────────┬──────────┬──────────┬──────────┐");
    println!("│  特性       │  to_     │  into_   │  as_     │");
    println!("├────────────┼──────────┼──────────┼──────────┤");
    println!("│  所有权     │  借用    │  转移    │  借用    │");
    println!("│  返回值     │  新值    │  新值    │  引用    │");
    println!("│  开销       │  克隆    │  移动    │  零成本  │");
    println!("│  原值可用   │  是      │  否      │  是      │");
    println!("└────────────┴──────────┴──────────┴──────────┘");
    println!();
    
    println!("常用 to_ 方法:");
    println!("  📌 to_string()       - 转换为 String");
    println!("  📌 to_str()          - 转换为 &str");
    println!("  📌 to_owned()        - 获取所有权副本");
    println!("  📌 to_vec()          - 转换为 Vec");
    println!("  📌 to_lowercase()    - 小写");
    println!("  📌 to_uppercase()    - 大写");
    println!("  📌 to_bytes()        - 字节数组");
    println!("  📌 to_digit()        - 数字");
    println!("  📌 to_radians()      - 弧度");
    println!("  📌 to_le_bytes()     - 小端字节");
    println!();
}

// ============================================
// 2. to_string() - 字符串转换
// ============================================
fn demo_to_string() {
    println!("--- 2. to_string() - 字符串转换 ---\n");
    
    println!("ToString trait:");
    println!("  trait ToString {{");
    println!("      fn to_string(&self) -> String;");
    println!("  }}");
    println!();
    
    // 基本类型
    println!("基本类型:");
    
    let num = 42;
    let s = num.to_string();
    println!("  i32: {} → \"{}\"", num, s);
    
    let float = 3.14159;
    let s = float.to_string();
    println!("  f64: {} → \"{}\"", float, s);
    
    let boolean = true;
    let s = boolean.to_string();
    println!("  bool: {} → \"{}\"", boolean, s);
    
    let ch = 'A';
    let s = ch.to_string();
    println!("  char: '{}' → \"{}\"", ch, s);
    println!();
    
    // 字符串类型
    println!("字符串类型:");
    
    let str_slice = "hello";
    let s = str_slice.to_string();
    println!("  &str: \"{}\" → String", s);
    
    let string = String::from("world");
    let s = string.to_string();
    println!("  String: \"{}\" → String (克隆)", s);
    println!();
    
    // 集合类型
    println!("集合类型:");
    
    let vec = vec![1, 2, 3];
    let s = format!("{:?}", vec);  // 使用 format!，Vec 没有 to_string
    println!("  Vec: {:?} → \"{}\"", vec, s);
    
    let mut map = HashMap::new();
    map.insert("key", "value");
    let s = format!("{:?}", map);
    println!("  HashMap: \"{}\"", s);
    println!();
    
    // Display vs Debug
    println!("Display vs Debug:");
    
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl std::fmt::Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    
    let p = Point { x: 10, y: 20 };
    println!("  Display: {}", p.to_string());
    println!("  Debug:   {:?}", p);
    println!();
}

// ============================================
// 3. to_str() - 字符串切片
// ============================================
fn demo_to_str() {
    println!("--- 3. to_str() - 字符串切片 ---\n");
    
    println!("OsStr::to_str():");
    
    let os_str = OsStr::new("hello");
    match os_str.to_str() {
        Some(s) => println!("  OsStr → &str: \"{}\"", s),
        None => println!("  转换失败（非 UTF-8）"),
    }
    println!();
    
    // Path to_str
    println!("Path::to_str():");
    
    let path = Path::new("/usr/local/bin");
    match path.to_str() {
        Some(s) => println!("  Path → &str: \"{}\"", s),
        None => println!("  转换失败"),
    }
    println!();
    
    // CStr to_str (需要导入)
    println!("CStr::to_str():");
    
    use std::ffi::CStr;
    
    let c_string = std::ffi::CString::new("hello").unwrap();
    let c_str = c_string.as_c_str();
    
    match c_str.to_str() {
        Ok(s) => println!("  CStr → &str: \"{}\"", s),
        Err(e) => println!("  转换失败: {}", e),
    }
    println!();
    
    // to_str vs to_string_lossy
    println!("to_str() vs to_string_lossy():");
    
    let path = Path::new("/usr/bin");
    
    println!("  to_str():          {:?}", path.to_str());
    println!("  to_string_lossy(): {}", path.to_string_lossy());
    println!("  说明: to_string_lossy 会替换无效 UTF-8");
    println!();
}

// ============================================
// 4. to_owned() - 所有权转换
// ============================================
fn demo_to_owned() {
    println!("--- 4. to_owned() - 所有权转换 ---\n");
    
    println!("ToOwned trait:");
    println!("  trait ToOwned {{");
    println!("      type Owned;");
    println!("      fn to_owned(&self) -> Self::Owned;");
    println!("  }}");
    println!();
    
    // &str to String
    println!("&str → String:");
    
    let borrowed = "hello";
    let owned = borrowed.to_owned();
    println!("  原始: \"{}\" (借用)", borrowed);
    println!("  拥有: \"{}\" (独立)", owned);
    println!("  两者都可用");
    println!();
    
    // &[T] to Vec<T>
    println!("&[T] → Vec<T>:");
    
    let slice: &[i32] = &[1, 2, 3, 4, 5];
    let vec = slice.to_owned();
    println!("  切片: {:?}", slice);
    println!("  向量: {:?}", vec);
    println!();
    
    // &Path to PathBuf
    println!("&Path → PathBuf:");
    
    let path = Path::new("/usr/local");
    let path_buf = path.to_owned();
    println!("  Path:    {:?}", path);
    println!("  PathBuf: {:?}", path_buf);
    println!();
    
    // &OsStr to OsString
    println!("&OsStr → OsString:");
    
    let os_str = OsStr::new("test");
    let os_string = os_str.to_owned();
    println!("  OsStr:    {:?}", os_str);
    println!("  OsString: {:?}", os_string);
    println!();
    
    // to_owned vs to_string vs clone
    println!("to_owned() vs to_string() vs clone():");
    println!("┌─────────────┬──────────────┬────────────────┐");
    println!("│  方法        │  适用类型     │  返回类型       │");
    println!("├─────────────┼──────────────┼────────────────┤");
    println!("│  to_owned   │  借用类型     │  拥有所有权     │");
    println!("│  to_string  │  Display     │  String        │");
    println!("│  clone      │  Clone       │  相同类型       │");
    println!("└─────────────┴──────────────┴────────────────┘");
    println!();
}

// ============================================
// 5. to_vec() - 向量转换
// ============================================
fn demo_to_vec() {
    println!("--- 5. to_vec() - 向量转换 ---\n");
    
    // 切片 to Vec
    println!("切片 → Vec:");
    
    let slice = &[1, 2, 3, 4, 5];
    let vec = slice.to_vec();
    println!("  切片: {:?}", slice);
    println!("  Vec:  {:?}", vec);
    println!();
    
    // 数组 to Vec
    println!("数组 → Vec:");
    
    let array = [10, 20, 30];
    let vec = array.to_vec();
    println!("  数组: {:?}", array);
    println!("  Vec:  {:?}", vec);
    println!();
    
    // String bytes to Vec
    println!("字节切片 → Vec:");
    
    let bytes: &[u8] = b"hello";
    let vec = bytes.to_vec();
    println!("  字节: {:?}", bytes);
    println!("  Vec:  {:?}", vec);
    println!();
    
    // 性能考虑
    println!("性能考虑:");
    
    let slice = &[1, 2, 3];
    
    // to_vec() - 总是分配
    let vec1 = slice.to_vec();
    println!("  to_vec():     {:?} (总是分配)", vec1);
    
    // 从迭代器收集 - 可能更高效
    let vec2: Vec<i32> = slice.iter().copied().collect();
    println!("  collect():    {:?} (优化可能)", vec2);
    
    // 从切片 - 显式克隆
    let vec3 = slice.to_owned();
    println!("  to_owned():   {:?} (语义清晰)", vec3);
    println!();
}

// ============================================
// 6. to_lowercase() / to_uppercase() - 大小写转换
// ============================================
fn demo_to_case() {
    println!("--- 6. to_lowercase() / to_uppercase() ---\n");
    
    // 字符串大小写
    println!("字符串大小写:");
    
    let s = "Hello, World!";
    let lower = s.to_lowercase();
    let upper = s.to_uppercase();
    
    println!("  原始:     \"{}\"", s);
    println!("  小写:     \"{}\"", lower);
    println!("  大写:     \"{}\"", upper);
    println!();
    
    // 字符大小写
    println!("字符大小写:");
    
    let ch = 'A';
    let lower = ch.to_lowercase();
    let upper = ch.to_uppercase();
    
    println!("  原始: '{}'", ch);
    println!("  小写: {:?}", lower.collect::<String>());
    println!("  大写: {:?}", upper.collect::<String>());
    println!();
    
    // Unicode 支持
    println!("Unicode 支持:");
    
    let s = "Straße";  // 德语：街道
    println!("  原始: \"{}\"", s);
    println!("  小写: \"{}\"", s.to_lowercase());
    println!("  大写: \"{}\"", s.to_uppercase());
    println!();
    
    let s = "ΣὈΜΕ";  // 希腊语
    println!("  原始: \"{}\"", s);
    println!("  小写: \"{}\"", s.to_lowercase());
    println!();
    
    // ASCII 专用方法
    println!("ASCII 专用方法:");
    
    let s = "Hello";
    
    println!("  to_lowercase():       \"{}\" (Unicode)", s.to_lowercase());
    println!("  to_ascii_lowercase(): \"{}\" (仅 ASCII)", s.to_ascii_lowercase());
    println!("  说明: to_ascii_* 更快但仅支持 ASCII");
    println!();
}

// ============================================
// 7. to_bytes() - 字节转换
// ============================================
fn demo_to_bytes() {
    println!("--- 7. to_bytes() - 字节转换 ---\n");
    
    // CStr to_bytes
    println!("CStr::to_bytes():");
    
    use std::ffi::CString;
    
    let c_string = CString::new("hello").unwrap();
    let bytes = c_string.to_bytes();
    println!("  CString: \"hello\"");
    println!("  字节:    {:?}", bytes);
    println!();
    
    // CStr to_bytes_with_nul
    println!("CStr::to_bytes_with_nul():");
    
    let bytes_with_nul = c_string.to_bytes_with_nul();
    println!("  包含 null: {:?}", bytes_with_nul);
    println!("  最后一位是 0 (null 终止符)");
    println!();
    
    // str as_bytes vs to_bytes
    println!("str::as_bytes() vs String::into_bytes():");
    
    let s = "hello";
    let bytes_ref = s.as_bytes();  // 借用
    println!("  as_bytes():   {:?} (借用)", bytes_ref);
    
    let s2 = String::from("world");
    let bytes_owned = s2.into_bytes();  // 转移所有权
    println!("  into_bytes(): {:?} (所有权)", bytes_owned);
    println!();
}

// ============================================
// 8. to_digit() - 数字转换
// ============================================
fn demo_to_digit() {
    println!("--- 8. to_digit() - 数字转换 ---\n");
    
    println!("char::to_digit():");
    
    // 十进制
    println!("  十进制 (基数 10):");
    for ch in ['0', '5', '9', 'A', 'Z'] {
        match ch.to_digit(10) {
            Some(d) => println!("    '{}' → {}", ch, d),
            None => println!("    '{}' → None (无效)", ch),
        }
    }
    println!();
    
    // 十六进制
    println!("  十六进制 (基数 16):");
    for ch in ['0', '9', 'A', 'F', 'G'] {
        match ch.to_digit(16) {
            Some(d) => println!("    '{}' → {}", ch, d),
            None => println!("    '{}' → None (无效)", ch),
        }
    }
    println!();
    
    // 二进制
    println!("  二进制 (基数 2):");
    for ch in ['0', '1', '2'] {
        match ch.to_digit(2) {
            Some(d) => println!("    '{}' → {}", ch, d),
            None => println!("    '{}' → None (无效)", ch),
        }
    }
    println!();
    
    // 实用示例
    println!("实用示例 - 手动解析数字:");
    
    fn parse_hex(s: &str) -> Option<u32> {
        let mut result = 0;
        for ch in s.chars() {
            result = result * 16 + ch.to_digit(16)?;
        }
        Some(result)
    }
    
    println!("  \"1A3\" → {:?}", parse_hex("1A3"));
    println!("  \"FF\" → {:?}", parse_hex("FF"));
    println!("  \"XYZ\" → {:?}", parse_hex("XYZ"));
    println!();
}

// ============================================
// 9. to_radians() / to_degrees() - 角度转换
// ============================================
fn demo_to_radians() {
    println!("--- 9. to_radians() / to_degrees() ---\n");
    
    use std::f64::consts::PI;
    
    println!("角度 → 弧度:");
    
    let degrees: [f64; 5] = [0.0, 45.0, 90.0, 180.0, 360.0];
    for deg in degrees {
        let rad = deg.to_radians();
        println!("  {}° = {:.4} rad", deg, rad);
    }
    println!();
    
    println!("弧度 → 角度:");
    
    let radians: [f64; 5] = [0.0, PI / 4.0, PI / 2.0, PI, 2.0 * PI];
    for rad in radians {
        let deg = rad.to_degrees();
        println!("  {:.4} rad = {}°", rad, deg);
    }
    println!();
    
    // 实用示例
    println!("实用示例 - 三角函数:");
    
    let angle_deg: f64 = 45.0;
    let angle_rad = angle_deg.to_radians();
    
    println!("  角度: {}°", angle_deg);
    println!("  弧度: {:.4}", angle_rad);
    println!("  sin:  {:.4}", angle_rad.sin());
    println!("  cos:  {:.4}", angle_rad.cos());
    println!("  tan:  {:.4}", angle_rad.tan());
    println!();
}

// ============================================
// 10. to_le_bytes() / to_be_bytes() - 字节序转换
// ============================================
fn demo_to_bytes_endian() {
    println!("--- 10. to_le_bytes() / to_be_bytes() ---\n");
    
    println!("什么是字节序？");
    println!("  Little Endian (LE): 低位字节在前");
    println!("  Big Endian (BE):    高位字节在前");
    println!();
    
    // u32 示例
    println!("u32 字节序:");
    
    let num: u32 = 0x12345678;
    let le_bytes = num.to_le_bytes();
    let be_bytes = num.to_be_bytes();
    let ne_bytes = num.to_ne_bytes();  // Native Endian
    
    println!("  原始值:       0x{:08X}", num);
    println!("  Little Endian: {:02X?}", le_bytes);
    println!("  Big Endian:    {:02X?}", be_bytes);
    println!("  Native:        {:02X?}", ne_bytes);
    println!();
    
    // 从字节恢复
    println!("从字节恢复:");
    
    let restored_le = u32::from_le_bytes(le_bytes);
    let restored_be = u32::from_be_bytes(be_bytes);
    
    println!("  从 LE 恢复: 0x{:08X}", restored_le);
    println!("  从 BE 恢复: 0x{:08X}", restored_be);
    println!();
    
    // i16 示例
    println!("i16 字节序:");
    
    let num: i16 = -1000;
    println!("  原始值: {}", num);
    println!("  LE:     {:02X?}", num.to_le_bytes());
    println!("  BE:     {:02X?}", num.to_be_bytes());
    println!();
    
    // f64 示例
    println!("f64 字节序:");
    
    let num: f64 = 3.14159;
    println!("  原始值: {}", num);
    println!("  LE:     {:02X?}", num.to_le_bytes());
    println!("  BE:     {:02X?}", num.to_be_bytes());
    println!();
    
    // 实际应用
    println!("实际应用 - 网络协议:");
    println!("  网络协议通常使用 Big Endian");
    println!("  文件格式可能使用 Little Endian");
    println!("  序列化时需要指定字节序");
    println!();
}

// ============================================
// 11. to_os_string() / to_path_buf() - 路径转换
// ============================================
fn demo_to_os() {
    println!("--- 11. to_os_string() / to_path_buf() ---\n");
    
    // Path to PathBuf
    println!("Path → PathBuf:");
    
    let path = Path::new("/usr/local/bin");
    let path_buf = path.to_path_buf();
    
    println!("  Path:    {:?}", path);
    println!("  PathBuf: {:?}", path_buf);
    println!();
    
    // OsStr to OsString
    println!("OsStr → OsString:");
    
    let os_str = OsStr::new("test.txt");
    let os_string = os_str.to_os_string();
    
    println!("  OsStr:    {:?}", os_str);
    println!("  OsString: {:?}", os_string);
    println!();
    
    // &str to OsString
    println!("&str → OsString:");
    
    let s = "hello.txt";
    let os_string: OsString = s.into();
    
    println!("  &str:     \"{}\"", s);
    println!("  OsString: {:?}", os_string);
    println!();
    
    // 路径操作
    println!("路径操作:");
    
    let mut path_buf = PathBuf::from("/usr");
    println!("  初始:     {:?}", path_buf);
    
    path_buf.push("local");
    println!("  push:     {:?}", path_buf);
    
    path_buf.push("bin");
    println!("  push:     {:?}", path_buf);
    
    path_buf.set_extension("txt");
    println!("  set_ext:  {:?}", path_buf);
    println!();
}

// ============================================
// 12. to_socket_addrs() - 网络地址转换
// ============================================
fn demo_to_socket_addrs() {
    println!("--- 12. to_socket_addrs() ---\n");
    
    use std::net::ToSocketAddrs;
    
    println!("ToSocketAddrs trait:");
    println!("  将各种类型转换为套接字地址");
    println!();
    
    // 字符串地址
    println!("字符串地址:");
    
    let addr = "127.0.0.1:8080";
    match addr.to_socket_addrs() {
        Ok(addrs) => {
            println!("  \"{}\":", addr);
            for addr in addrs {
                println!("    → {}", addr);
            }
        }
        Err(e) => println!("  错误: {}", e),
    }
    println!();
    
    // 元组地址
    println!("元组地址:");
    
    let tuple = ("localhost", 3000);
    match tuple.to_socket_addrs() {
        Ok(addrs) => {
            println!("  (\"localhost\", 3000):");
            for addr in addrs {
                println!("    → {}", addr);
            }
        }
        Err(e) => println!("  错误: {}", e),
    }
    println!();
    
    // 域名解析
    println!("域名解析:");
    
    let domain = "localhost:8080";
    match domain.to_socket_addrs() {
        Ok(addrs) => {
            println!("  \"{}\":", domain);
            for addr in addrs {
                println!("    → {}", addr);
            }
        }
        Err(e) => println!("  错误: {}", e),
    }
    println!();
}

// ============================================
// 13. as_ vs to_ vs into_
// ============================================
fn demo_conversion_comparison() {
    println!("--- 13. as_ vs to_ vs into_ 对比 ---\n");
    
    println!("三种转换方式:");
    println!("┌──────────────┬──────────┬──────────┬──────────┐");
    println!("│  特性         │  as_     │  to_     │  into_   │");
    println!("├──────────────┼──────────┼──────────┼──────────┤");
    println!("│  所有权       │  借用    │  借用    │  转移    │");
    println!("│  返回         │  引用    │  新值    │  新值    │");
    println!("│  开销         │  零成本  │  克隆    │  移动    │");
    println!("│  原值可用     │  是      │  是      │  否      │");
    println!("│  用途         │  查看    │  复制    │  转换    │");
    println!("└──────────────┴──────────┴──────────┴──────────┘");
    println!();
    
    println!("示例对比:");
    
    let s = String::from("hello");
    
    // as_ - 借用
    let bytes_ref = s.as_bytes();
    println!("  as_bytes():   {:?} (借用)", bytes_ref);
    println!("  s 仍可用:     \"{}\"", s);
    println!();
    
    // to_ - 克隆
    let s2 = "world";
    let owned = s2.to_owned();
    println!("  to_owned():   \"{}\" (克隆)", owned);
    println!("  s2 仍可用:    \"{}\"", s2);
    println!();
    
    // into_ - 转移
    let s3 = String::from("rust");
    let bytes_owned = s3.into_bytes();
    println!("  into_bytes(): {:?} (转移)", bytes_owned);
    println!("  s3 不可用");
    // println!("{}", s3); // 错误！
    println!();
    
    println!("选择指南:");
    println!("  🔍 as_   - 只需要查看，不需要拥有");
    println!("  📋 to_   - 需要独立副本，原值还要用");
    println!("  🔄 into_ - 转换类型，不再需要原值");
    println!();
}

// ============================================
// 14. 实战案例
// ============================================
fn demo_real_world_examples() {
    println!("--- 14. 实战案例 ---\n");
    
    // 案例 1: 配置解析
    println!("案例 1: 配置解析\n");
    config_parser_example();
    
    // 案例 2: 文件路径处理
    println!("\n案例 2: 文件路径处理\n");
    path_handling_example();
    
    // 案例 3: 数据序列化
    println!("\n案例 3: 数据序列化\n");
    serialization_example();
    
    // 案例 4: 字符串处理管道
    println!("\n案例 4: 字符串处理管道\n");
    string_pipeline_example();
}

// 案例 1: 配置解析
fn config_parser_example() {
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
                            .map_err(|_| format!("无效的端口: {}", value))?;
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
    
    let config_str = "\
        host = 127.0.0.1\n\
        port = 3000\n\
        debug = true\n\
    ";
    
    match Config::from_str(config_str) {
        Ok(config) => {
            println!("  配置解析成功:");
            println!("    host:  {}", config.host);
            println!("    port:  {}", config.port);
            println!("    debug: {}", config.debug);
        }
        Err(e) => println!("  错误: {}", e),
    }
}

// 案例 2: 文件路径处理
fn path_handling_example() {
    fn process_file_path(path: &str) -> PathBuf {
        let mut path_buf = PathBuf::from(path);
        
        // 规范化
        if !path_buf.has_root() {
            path_buf = PathBuf::from("/tmp").join(path_buf);
        }
        
        // 添加扩展名
        if path_buf.extension().is_none() {
            path_buf.set_extension("txt");
        }
        
        path_buf
    }
    
    let paths = ["data/file", "/var/log/app.log", "output"];
    
    for path in paths {
        let processed = process_file_path(path);
        println!("  \"{}\" → {:?}", path, processed);
    }
}

// 案例 3: 数据序列化
fn serialization_example() {
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
    
    let num = 12345678u32;
    println!("  原始值: {}", num);
    
    let bytes = serialize_u32(num);
    println!("  序列化: {:02X?}", bytes);
    
    let restored = deserialize_u32(&bytes);
    println!("  反序列化: {:?}", restored);
}

// 案例 4: 字符串处理管道
fn string_pipeline_example() {
    fn process_text(text: &str) -> String {
        text.lines()                          // 分行
            .map(|line| line.trim())          // 去空格
            .filter(|line| !line.is_empty())  // 过滤空行
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
    
    let text = "\
        hello world\n\
        \n\
        RUST PROGRAMMING\n\
        \n\
          data processing  \n\
    ";
    
    println!("  原始文本:");
    println!("{}", text);
    
    println!("  处理后:");
    println!("{}", process_text(text));
}

/*
=== 总结 ===

1. To 系列方法总览:

   字符串:
   - to_string()       - 任意类型 → String
   - to_str()          - OsStr/Path → &str
   - to_owned()        - 借用 → 拥有
   - to_lowercase()    - 小写转换
   - to_uppercase()    - 大写转换
   
   集合:
   - to_vec()          - 切片 → Vec
   
   数字:
   - to_digit()        - char → u32
   - to_radians()      - 角度 → 弧度
   - to_degrees()      - 弧度 → 角度
   
   字节:
   - to_bytes()        - CStr → &[u8]
   - to_le_bytes()     - 数字 → 字节(LE)
   - to_be_bytes()     - 数字 → 字节(BE)
   
   路径:
   - to_path_buf()     - Path → PathBuf
   - to_os_string()    - OsStr → OsString

2. 转换方式对比:

   特性             as_        to_        into_
   ───────────────────────────────────────────
   所有权           借用       借用       转移
   返回值           引用       新值       新值
   开销             零成本     克隆       移动
   原值可用         是         是         否
   
   选择:
   - 只查看         → as_
   - 需要副本       → to_
   - 类型转换       → into_

3. 常见模式:

   配置解析:
   - to_string() 转换配置值
   - to_lowercase() 规范化
   - parse() + to_owned() 组合
   
   路径处理:
   - to_path_buf() 可变路径
   - to_str() 获取字符串
   - to_os_string() 系统字符串
   
   数据序列化:
   - to_le_bytes() 序列化
   - from_le_bytes() 反序列化
   - to_vec() 收集字节

4. 最佳实践:

   DO:
   ✓ 需要副本时用 to_
   ✓ 注意 Unicode 处理
   ✓ 选择合适的字节序
   ✓ 使用类型推导
   
   DON'T:
   ✗ 不必要的克隆
   ✗ 忽略 Option/Result
   ✗ 混淆 as/to/into

运行示例:
  cargo run --bin to_conversions_detailed
*/
