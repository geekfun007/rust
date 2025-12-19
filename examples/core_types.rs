// Rust 核心类型详解示例

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║           Rust 核心类型与 Trait 详解                     ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
    
    box_demo();
    option_demo();
    result_demo();
    display_demo();
    other_traits_demo();
}

/// 1. Box<T> 智能指针演示
fn box_demo() {
    println!("═══ 1. Box<T> - 智能指针 ═══\n");
    
    // 基本使用
    println!("基本使用：");
    let b = Box::new(5);
    println!("  Box::new(5) = {}", b);
    println!("  解引用: *b = {}", *b);
    
    // 内存大小
    println!("\n内存大小：");
    println!("  Box<i32>: {} 字节（仅指针）", std::mem::size_of::<Box<i32>>());
    println!("  i32: {} 字节", std::mem::size_of::<i32>());
    
    // 递归类型
    println!("\n递归类型（链表）：");
    
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("  链表: {:?}", list);
    
    // 大型数据
    println!("\n大型数据：");
    struct LargeData([u8; 10000]);
    let large = Box::new(LargeData([0; 10000]));
    println!("  分配了 10KB 数据在堆上");
    println!("  Box 本身: {} 字节", std::mem::size_of_val(&large));
    
    // Trait 对象
    println!("\nTrait 对象：");
    trait Draw {
        fn draw(&self) -> &str;
    }
    
    struct Circle;
    struct Square;
    
    impl Draw for Circle {
        fn draw(&self) -> &str { "○" }
    }
    
    impl Draw for Square {
        fn draw(&self) -> &str { "□" }
    }
    
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle),
        Box::new(Square),
        Box::new(Circle),
    ];
    
    print!("  形状: ");
    for shape in &shapes {
        print!("{} ", shape.draw());
    }
    println!();
    
    println!("  Box<dyn Draw> 大小: {} 字节（胖指针）", 
             std::mem::size_of::<Box<dyn Draw>>());
}

/// 2. Option<T> 演示
fn option_demo() {
    println!("\n═══ 2. Option<T> - 可选值 ═══\n");
    
    // 创建
    println!("创建 Option：");
    let some_number = Some(42);
    let no_number: Option<i32> = None;
    println!("  Some(42): {:?}", some_number);
    println!("  None: {:?}", no_number);
    
    // 检查
    println!("\n检查方法：");
    println!("  some_number.is_some(): {}", some_number.is_some());
    println!("  no_number.is_none(): {}", no_number.is_none());
    println!("  some_number == Some(42): {}", some_number == Some(42));
    
    // 提取值
    println!("\n提取值：");
    println!("  unwrap_or(0): {}", no_number.unwrap_or(0));
    println!("  unwrap_or_default(): {}", no_number.unwrap_or_default());
    
    // map 转换
    println!("\nmap 转换：");
    let doubled = some_number.map(|x| x * 2);
    println!("  Some(42).map(|x| x * 2) = {:?}", doubled);
    
    let none_doubled = no_number.map(|x| x * 2);
    println!("  None.map(|x| x * 2) = {:?}", none_doubled);
    
    // and_then 链式
    println!("\nand_then 链式：");
    let result = some_number.and_then(|x| {
        if x > 40 {
            Some(x * 2)
        } else {
            None
        }
    });
    println!("  Some(42).and_then(检查并翻倍) = {:?}", result);
    
    // filter 过滤
    println!("\nfilter 过滤：");
    let filtered = some_number.filter(|&x| x > 40);
    println!("  Some(42).filter(|x| x > 40) = {:?}", filtered);
    
    // 组合
    println!("\n组合操作：");
    let x = Some(2);
    let y: Option<i32> = None;
    println!("  Some(2).and(Some(3)) = {:?}", x.and(Some(3)));
    println!("  Some(2).or(Some(3)) = {:?}", x.or(Some(3)));
    println!("  None.or(Some(3)) = {:?}", y.or(Some(3)));
    
    // 实战：安全除法
    println!("\n实战示例 - 安全除法：");
    fn safe_divide(a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }
    
    println!("  10 / 2 = {:?}", safe_divide(10, 2));
    println!("  10 / 0 = {:?}", safe_divide(10, 0));
    
    // ? 操作符
    println!("\n? 操作符示例：");
    fn try_parse(s: &str) -> Option<i32> {
        let n: i32 = s.parse().ok()?;
        Some(n * 2)
    }
    
    println!("  try_parse(\"42\") = {:?}", try_parse("42"));
    println!("  try_parse(\"abc\") = {:?}", try_parse("abc"));
    
    // 内存优化
    println!("\n内存优化：");
    println!("  Option<i32>: {} 字节", std::mem::size_of::<Option<i32>>());
    println!("  Option<&i32>: {} 字节（空指针优化）", std::mem::size_of::<Option<&i32>>());
    println!("  &i32: {} 字节", std::mem::size_of::<&i32>());
}

/// 3. Result<T, E> 演示
fn result_demo() {
    println!("\n═══ 3. Result<T, E> - 错误处理 ═══\n");
    
    // 创建
    println!("创建 Result：");
    let success: Result<i32, String> = Ok(42);
    let failure: Result<i32, String> = Err("出错了".to_string());
    println!("  Ok(42): {:?}", success);
    println!("  Err(\"出错了\"): {:?}", failure);
    
    // 检查
    println!("\n检查方法：");
    println!("  success.is_ok(): {}", success.is_ok());
    println!("  failure.is_err(): {}", failure.is_err());
    
    // 提取值
    println!("\n提取值：");
    println!("  success.unwrap_or(0): {}", success.clone().unwrap_or(0));
    println!("  failure.unwrap_or(0): {}", failure.clone().unwrap_or(0));
    
    // map 转换
    println!("\nmap 转换：");
    let doubled = success.clone().map(|x| x * 2);
    println!("  Ok(42).map(|x| x * 2) = {:?}", doubled);
    
    let error_mapped = failure.clone().map_err(|e| format!("错误: {}", e));
    println!("  Err(...).map_err(...) = {:?}", error_mapped);
    
    // and_then 链式
    println!("\nand_then 链式：");
    let result = success.and_then(|x| {
        if x > 0 {
            Ok(x * 2)
        } else {
            Err("值必须为正".to_string())
        }
    });
    println!("  Ok(42).and_then(检查并翻倍) = {:?}", result);
    
    // or_else 错误恢复
    println!("\nor_else 错误恢复：");
    let recovered: Result<i32, String> = failure.or_else(|_| Ok(0));
    println!("  Err(...).or_else(|_| Ok(0)) = {:?}", recovered);
    
    // 实战：除法
    println!("\n实战示例 - 除法：");
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    println!("  10 / 2 = {:?}", divide(10, 2));
    println!("  10 / 0 = {:?}", divide(10, 0));
    
    // ? 操作符
    println!("\n? 操作符示例：");
    fn try_divide(a: i32, b: i32, c: i32) -> Result<i32, String> {
        let result1 = divide(a, b)?;
        let result2 = divide(result1, c)?;
        Ok(result2)
    }
    
    println!("  try_divide(100, 10, 2) = {:?}", try_divide(100, 10, 2));
    println!("  try_divide(100, 0, 2) = {:?}", try_divide(100, 0, 2));
    
    // Result 与 Option 转换
    println!("\nResult 与 Option 转换：");
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("无值");
    println!("  Some(42).ok_or(\"无值\") = {:?}", res);
    
    let res: Result<i32, &str> = Ok(42);
    let opt: Option<i32> = res.ok();
    println!("  Ok(42).ok() = {:?}", opt);
}

/// 4. fmt::Display 演示
fn display_demo() {
    println!("\n═══ 4. fmt::Display - 格式化输出 ═══\n");
    
    use std::fmt;
    
    // 简单实现
    println!("简单实现：");
    
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    
    let p = Point { x: 10, y: 20 };
    println!("  Display: {}", p);
    println!("  Debug:   {:?}", p);
    
    // 复杂实现
    println!("\n复杂实现：");
    
    struct Person {
        name: String,
        age: u32,
    }
    
    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} ({}岁)", self.name, self.age)
        }
    }
    
    let person = Person {
        name: "张三".to_string(),
        age: 25,
    };
    println!("  {}", person);
    
    // 枚举实现
    println!("\n枚举实现：");
    
    enum Status {
        Active,
        Inactive,
        Pending,
    }
    
    impl fmt::Display for Status {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Status::Active => write!(f, "✓ 激活"),
                Status::Inactive => write!(f, "✗ 停用"),
                Status::Pending => write!(f, "⏳ 待定"),
            }
        }
    }
    
    println!("  Active:   {}", Status::Active);
    println!("  Inactive: {}", Status::Inactive);
    println!("  Pending:  {}", Status::Pending);
    
    // 格式化参数
    println!("\n格式化参数：");
    let n = 42;
    println!("  默认:     {}", n);
    println!("  宽度5:    {:5}", n);
    println!("  左对齐:   {:<5}", n);
    println!("  右对齐:   {:>5}", n);
    println!("  居中:     {:^5}", n);
    println!("  填充零:   {:05}", n);
    
    // 浮点数格式
    let f = 3.14159;
    println!("\n浮点数格式：");
    println!("  默认:     {}", f);
    println!("  2位小数:  {:.2}", f);
    println!("  科学计数: {:e}", f);
    
    // format! 宏家族
    println!("\nformat! 宏家族：");
    let s = format!("x = {}, y = {}", 10, 20);
    println!("  format!: {}", s);
    
    // 其他格式化 trait
    println!("\n其他格式化 trait：");
    let num = 42;
    println!("  二进制: {:b}", num);
    println!("  八进制: {:o}", num);
    println!("  十六进制: {:x}", num);
    println!("  十六进制(大写): {:X}", num);
}

/// 5. 其他核心 Trait 演示
fn other_traits_demo() {
    println!("\n═══ 5. 其他核心 Trait ═══\n");
    
    // From & Into
    println!("From & Into：");
    
    #[derive(Debug)]
    struct MyInt(i32);
    
    impl From<i32> for MyInt {
        fn from(value: i32) -> Self {
            MyInt(value)
        }
    }
    
    let x: MyInt = MyInt::from(42);
    println!("  From: {:?}", x);
    
    let y: MyInt = 24.into();
    println!("  Into: {:?}", y);
    
    // Default
    println!("\nDefault：");
    
    #[derive(Debug, Default)]
    struct Config {
        timeout: u32,
        retries: u32,
    }
    
    let config = Config::default();
    println!("  默认配置: {:?}", config);
    
    // Clone
    println!("\nClone：");
    
    #[derive(Debug, Clone)]
    struct Data {
        value: String,
    }
    
    let d1 = Data { value: "hello".to_string() };
    let d2 = d1.clone();
    println!("  原始: {:?}", d1);
    println!("  克隆: {:?}", d2);
    
    // Copy
    println!("\nCopy：");
    
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;  // 复制
    println!("  p1: {:?} (仍然有效)", p1);
    println!("  p2: {:?}", p2);
    
    // Drop
    println!("\nDrop：");
    
    struct Guard {
        name: String,
    }
    
    impl Drop for Guard {
        fn drop(&mut self) {
            println!("  销毁 {}", self.name);
        }
    }
    
    {
        let _g1 = Guard { name: "Guard 1".to_string() };
        let _g2 = Guard { name: "Guard 2".to_string() };
        println!("  Guard 创建完成");
    }
    println!("  Guard 作用域结束");
    
    // Iterator
    println!("\nIterator：");
    
    struct Counter {
        count: u32,
        max: u32,
    }
    
    impl Iterator for Counter {
        type Item = u32;
        
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count <= self.max {
                Some(self.count)
            } else {
                None
            }
        }
    }
    
    let counter = Counter { count: 0, max: 5 };
    print!("  计数: ");
    for n in counter {
        print!("{} ", n);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_box() {
        let b = Box::new(42);
        assert_eq!(*b, 42);
    }
    
    #[test]
    fn test_option() {
        let x = Some(42);
        assert_eq!(x.unwrap(), 42);
        assert_eq!(x.map(|v| v * 2), Some(84));
        
        let y: Option<i32> = None;
        assert_eq!(y.unwrap_or(0), 0);
    }
    
    #[test]
    fn test_result() {
        let x: Result<i32, &str> = Ok(42);
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 42);
        
        let y: Result<i32, &str> = Err("error");
        assert!(y.is_err());
        assert_eq!(y.unwrap_or(0), 0);
    }
    
    #[test]
    fn test_display() {
        use std::fmt;
        
        struct Point { x: i32, y: i32 }
        
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }
        
        let p = Point { x: 1, y: 2 };
        assert_eq!(format!("{}", p), "(1, 2)");
    }
    
    #[test]
    fn test_from_into() {
        struct MyInt(i32);
        
        impl From<i32> for MyInt {
            fn from(value: i32) -> Self {
                MyInt(value)
            }
        }
        
        let x: MyInt = 42.into();
        assert_eq!(x.0, 42);
    }
}
