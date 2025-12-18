// Rust 基础语法示例

/// # 变量与常量
/// 
/// Rust 中变量默认是不可变的 (immutable)
pub fn variables_demo() {
    println!("\n=== 变量与常量 ===");
    
    // 不可变变量
    let x = 5;
    println!("不可变变量 x = {}", x);
    
    // 可变变量 - 需要使用 mut 关键字
    let mut y = 10;
    println!("可变变量 y = {}", y);
    y = 15;
    println!("修改后 y = {}", y);
    
    // 常量 - 必须标注类型，且在编译时确定
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);
    
    // 变量遮蔽 (shadowing) - 可以改变类型
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("遮蔽后的 z = {}", z);
    
    // 遮蔽可以改变类型
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces 的长度 = {}", spaces);
}

/// # 类型标注与类型推导
pub fn type_annotations_demo() {
    println!("\n=== 类型标注与推导 ===");
    
    // 显式类型标注
    let x: i32 = 42;
    let y: f64 = 3.14;
    let z: bool = true;
    let c: char = '中';
    
    println!("i32: {}, f64: {}, bool: {}, char: {}", x, y, z, c);
    
    // 类型推导
    let inferred = 42; // 编译器推导为 i32
    let inferred_float = 3.14; // 推导为 f64
    
    println!("推导类型: {}, {}", inferred, inferred_float);
    
    // 需要显式标注的情况
    let parsed: i32 = "42".parse().expect("解析失败");
    println!("解析的数字: {}", parsed);
}

/// # 注释
pub fn comments_demo() {
    println!("\n=== 注释 ===");
    
    // 单行注释
    
    /* 
     * 多行注释
     * 可以跨越多行
     */
    
    /// 文档注释 - 用于函数、结构体等
    /// 支持 Markdown 格式
    
    //! 模块级文档注释
    //! 用于描述整个模块
    
    println!("注释不会影响程序执行");
}

/// # 表达式与语句
pub fn expressions_vs_statements() {
    println!("\n=== 表达式与语句 ===");
    
    // 语句 - 不返回值
    let x = 5;
    
    // 表达式 - 返回值（注意没有分号）
    let y = {
        let x = 3;
        x + 1  // 这是表达式，返回 4
    };
    println!("y = {}", y);
    
    // if 是表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);
}

/// # 打印输出
pub fn printing_demo() {
    println!("\n=== 打印输出 ===");
    
    // println! - 带换行
    println!("这是一行文本");
    
    // print! - 不带换行
    print!("这是");
    print!("连续");
    println!("的文本");
    
    // 格式化输出
    let x = 42;
    let y = 3.14159;
    println!("整数: {}, 浮点数: {}", x, y);
    
    // 位置参数
    println!("{0} {1} {0}", "A", "B");
    
    // 命名参数
    println!("{name} is {age} years old", name="张三", age=25);
    
    // 格式化选项
    println!("二进制: {:b}", x);
    println!("十六进制: {:x}", x);
    println!("八进制: {:o}", x);
    println!("保留2位小数: {:.2}", y);
    println!("宽度为10: {:10}", x);
    println!("左对齐: {:<10}", x);
    println!("右对齐: {:>10}", x);
    println!("居中: {:^10}", x);
    println!("用0填充: {:0>10}", x);
    
    // Debug 输出
    let v = vec![1, 2, 3];
    println!("Debug: {:?}", v);
    println!("Pretty Debug: {:#?}", v);
}

/// # 标量类型示例
pub fn scalar_types_demo() {
    println!("\n=== 标量类型 ===");
    
    // 整数类型
    let int8: i8 = 127;
    let int16: i16 = 32767;
    let int32: i32 = 2147483647;
    let int64: i64 = 9223372036854775807;
    let uint: u32 = 4294967295;
    
    println!("整数类型: i8={}, i16={}, i32={}, i64={}, u32={}", 
             int8, int16, int32, int64, uint);
    
    // 浮点数类型
    let float32: f32 = 3.14;
    let float64: f64 = 2.718281828;
    println!("浮点数: f32={}, f64={}", float32, float64);
    
    // 布尔类型
    let is_true: bool = true;
    let is_false = false;
    println!("布尔值: {}, {}", is_true, is_false);
    
    // 字符类型 - Unicode 标量值
    let c: char = 'z';
    let emoji: char = '😀';
    let chinese: char = '中';
    println!("字符: {}, {}, {}", c, emoji, chinese);
}

/// # 数字字面量
pub fn numeric_literals_demo() {
    println!("\n=== 数字字面量 ===");
    
    // 十进制
    let decimal = 98_222;
    
    // 十六进制
    let hex = 0xff;
    
    // 八进制
    let octal = 0o77;
    
    // 二进制
    let binary = 0b1111_0000;
    
    // 字节 (仅 u8)
    let byte = b'A';
    
    println!("十进制: {}", decimal);
    println!("十六进制: {}", hex);
    println!("八进制: {}", octal);
    println!("二进制: {}", binary);
    println!("字节: {}", byte);
    
    // 类型后缀
    let x = 42u32;
    let y = 3.14f32;
    println!("带类型后缀: {}, {}", x, y);
}

/// 运行所有基础语法示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║     Rust 基础语法示例演示          ║");
    println!("╚════════════════════════════════════╝");
    
    variables_demo();
    type_annotations_demo();
    comments_demo();
    expressions_vs_statements();
    printing_demo();
    scalar_types_demo();
    numeric_literals_demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_variables() {
        let x = 5;
        assert_eq!(x, 5);
        
        let mut y = 10;
        y = 15;
        assert_eq!(y, 15);
    }
    
    #[test]
    fn test_expressions() {
        let y = {
            let x = 3;
            x + 1
        };
        assert_eq!(y, 4);
    }
}
