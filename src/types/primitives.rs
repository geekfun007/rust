// 基本类型详解：int, float, bool

/// # 整数类型详解
/// 
/// Rust 提供了多种整数类型：
/// - 有符号: i8, i16, i32, i64, i128, isize
/// - 无符号: u8, u16, u32, u64, u128, usize
pub fn integer_types_demo() {
    println!("\n=== 整数类型 ===");
    
    // 不同大小的整数
    let a: i8 = 127;                    // -128 到 127
    let b: i16 = 32_767;                // -32,768 到 32,767
    let c: i32 = 2_147_483_647;         // 默认整数类型
    let d: i64 = 9_223_372_036_854_775_807;
    let e: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    
    println!("i8:   {}", a);
    println!("i16:  {}", b);
    println!("i32:  {}", c);
    println!("i64:  {}", d);
    println!("i128: {}", e);
    
    // 无符号整数
    let u1: u8 = 255;                   // 0 到 255
    let u2: u16 = 65_535;
    let u3: u32 = 4_294_967_295;
    let u4: u64 = 18_446_744_073_709_551_615;
    
    println!("\nu8:  {}", u1);
    println!("u16: {}", u2);
    println!("u32: {}", u3);
    println!("u64: {}", u4);
    
    // 架构相关的整数类型
    let ptr_size: usize = 100;  // 64位系统上是u64，32位系统上是u32
    let idx: isize = -50;       // 64位系统上是i64，32位系统上是i32
    println!("\nusize: {} (字节: {})", ptr_size, std::mem::size_of::<usize>());
    println!("isize: {} (字节: {})", idx, std::mem::size_of::<isize>());
}

/// # 整数方法
pub fn integer_methods_demo() {
    println!("\n=== 整数方法 ===");
    
    let x: i32 = 42;
    let y: i32 = -10;
    
    // 绝对值
    println!("abs({}) = {}", y, y.abs());
    
    // 幂运算
    println!("pow(2, 3) = {}", 2_i32.pow(3));
    
    // 检查溢出的运算
    let (result, overflowed) = 255_u8.overflowing_add(10);
    println!("255u8 + 10 = {} (溢出: {})", result, overflowed);
    
    // 饱和运算（不会溢出）
    let saturated = 255_u8.saturating_add(10);
    println!("255u8 saturating_add 10 = {}", saturated);
    
    // 安全运算（返回 Option）
    let checked = 255_u8.checked_add(10);
    println!("255u8 checked_add 10 = {:?}", checked);
    
    // 位运算
    println!("\n位运算:");
    let a = 0b1100;
    let b = 0b1010;
    println!("{:04b} & {:04b} = {:04b}", a, b, a & b);  // AND
    println!("{:04b} | {:04b} = {:04b}", a, b, a | b);  // OR
    println!("{:04b} ^ {:04b} = {:04b}", a, b, a ^ b);  // XOR
    println!("!{:04b} = {:04b}", a, !a);                // NOT
    println!("{:04b} << 2 = {:04b}", a, a << 2);        // 左移
    println!("{:04b} >> 2 = {:04b}", a, a >> 2);        // 右移
    
    // 类型转换
    println!("\n类型转换:");
    let x: i32 = 42;
    let y: i64 = x as i64;
    let z: f64 = x as f64;
    println!("i32 -> i64: {}", y);
    println!("i32 -> f64: {}", z);
    
    // 最大值和最小值
    println!("\n边界值:");
    println!("i32::MIN = {}", i32::MIN);
    println!("i32::MAX = {}", i32::MAX);
    println!("u32::MIN = {}", u32::MIN);
    println!("u32::MAX = {}", u32::MAX);
}

/// # 浮点数类型详解
pub fn float_types_demo() {
    println!("\n=== 浮点数类型 ===");
    
    // f32 和 f64
    let x: f32 = 3.14159;      // 单精度
    let y: f64 = 2.71828;      // 双精度，默认类型
    
    println!("f32: {} (字节: {})", x, std::mem::size_of::<f32>());
    println!("f64: {} (字节: {})", y, std::mem::size_of::<f64>());
    
    // 科学计数法
    let large = 1e6;           // 1,000,000
    let small = 1e-6;          // 0.000001
    println!("科学计数: {} {}", large, small);
    
    // 特殊值
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    let nan = f64::NAN;
    
    println!("\n特殊值:");
    println!("无穷大: {}", inf);
    println!("负无穷大: {}", neg_inf);
    println!("非数字: {}", nan);
    println!("NaN == NaN: {}", nan == nan);  // false!
    println!("NaN is_nan(): {}", nan.is_nan());
}

/// # 浮点数方法
pub fn float_methods_demo() {
    println!("\n=== 浮点数方法 ===");
    
    let x: f64 = 3.14159;
    let y: f64 = -2.5;
    
    // 基本运算
    println!("abs({}) = {}", y, y.abs());
    println!("floor({}) = {}", x, x.floor());
    println!("ceil({}) = {}", x, x.ceil());
    println!("round({}) = {}", x, x.round());
    println!("trunc({}) = {}", x, x.trunc());
    
    // 数学函数
    println!("\n数学函数:");
    println!("sqrt(16.0) = {}", 16.0_f64.sqrt());
    println!("cbrt(27.0) = {}", 27.0_f64.cbrt());
    println!("pow(2.0, 3.0) = {}", 2.0_f64.powf(3.0));
    println!("exp(1.0) = {}", 1.0_f64.exp());
    println!("ln(e) = {}", std::f64::consts::E.ln());
    println!("log10(100.0) = {}", 100.0_f64.log10());
    
    // 三角函数
    println!("\n三角函数:");
    let angle = std::f64::consts::PI / 4.0;  // 45度
    println!("sin(π/4) = {}", angle.sin());
    println!("cos(π/4) = {}", angle.cos());
    println!("tan(π/4) = {}", angle.tan());
    
    // 判断函数
    println!("\n判断函数:");
    println!("is_finite(3.14) = {}", 3.14_f64.is_finite());
    println!("is_infinite(∞) = {}", f64::INFINITY.is_infinite());
    println!("is_nan(NaN) = {}", f64::NAN.is_nan());
    println!("is_sign_positive(3.14) = {}", 3.14_f64.is_sign_positive());
    println!("is_sign_negative(-2.5) = {}", (-2.5_f64).is_sign_negative());
    
    // 常量
    println!("\n数学常量:");
    println!("π = {}", std::f64::consts::PI);
    println!("e = {}", std::f64::consts::E);
    println!("√2 = {}", std::f64::consts::SQRT_2);
    println!("ln(2) = {}", std::f64::consts::LN_2);
}

/// # 浮点数精度问题
pub fn float_precision_demo() {
    println!("\n=== 浮点数精度问题 ===");
    
    // 浮点数不精确
    let x = 0.1 + 0.2;
    println!("0.1 + 0.2 = {}", x);
    println!("0.1 + 0.2 == 0.3: {}", x == 0.3);  // false!
    
    // 比较浮点数应该使用误差范围
    let epsilon = 1e-10;
    let is_close = (x - 0.3).abs() < epsilon;
    println!("是否接近 0.3: {}", is_close);
    
    // 大数和小数运算
    let big = 1e20;
    let small = 1.0;
    let result = big + small - big;
    println!("\n大数精度损失:");
    println!("1e20 + 1.0 - 1e20 = {}", result);  // 可能不是 1.0
    
    // 使用 f32 vs f64 的精度差异
    let x32: f32 = 1.0 / 3.0;
    let x64: f64 = 1.0 / 3.0;
    println!("\n精度比较:");
    println!("f32: {:.20}", x32);
    println!("f64: {:.20}", x64);
}

/// # 布尔类型详解
pub fn bool_type_demo() {
    println!("\n=== 布尔类型 ===");
    
    let t = true;
    let f: bool = false;
    
    println!("true: {}, false: {}", t, f);
    println!("大小: {} 字节", std::mem::size_of::<bool>());
    
    // 逻辑运算
    println!("\n逻辑运算:");
    println!("!true = {}", !t);
    println!("true && false = {}", t && f);
    println!("true || false = {}", t || f);
    println!("true ^ false = {}", t ^ f);  // XOR
    
    // 比较运算产生布尔值
    println!("\n比较运算:");
    println!("5 > 3 = {}", 5 > 3);
    println!("5 < 3 = {}", 5 < 3);
    println!("5 == 5 = {}", 5 == 5);
    println!("5 != 3 = {}", 5 != 3);
    println!("5 >= 5 = {}", 5 >= 5);
    println!("5 <= 3 = {}", 5 <= 3);
}

/// # 布尔值转换
pub fn bool_conversion_demo() {
    println!("\n=== 布尔值转换 ===");
    
    // 布尔值转整数
    let t = true as i32;
    let f = false as i32;
    println!("true as i32 = {}", t);
    println!("false as i32 = {}", f);
    
    // 整数转布尔（需要比较）
    let x = 5;
    let is_positive = x > 0;
    let is_zero = x == 0;
    println!("{} > 0: {}", x, is_positive);
    println!("{} == 0: {}", x, is_zero);
    
    // 使用 Option<bool>
    let maybe_true: Option<bool> = Some(true);
    let maybe_false: Option<bool> = None;
    
    println!("\nOption<bool>:");
    println!("{:?}", maybe_true);
    println!("{:?}", maybe_false);
    
    // unwrap_or 提供默认值
    println!("unwrap_or(false): {}", maybe_false.unwrap_or(false));
}

/// # 字符类型
pub fn char_type_demo() {
    println!("\n=== 字符类型 ===");
    
    let c: char = 'z';
    let emoji: char = '😀';
    let chinese: char = '中';
    let escape: char = '\n';
    
    println!("字符: {}, {}, {}", c, emoji, chinese);
    println!("大小: {} 字节", std::mem::size_of::<char>());
    
    // 字符是 Unicode 标量值
    println!("\nUnicode 值:");
    println!("'z' = U+{:04X}", 'z' as u32);
    println!("'😀' = U+{:04X}", '😀' as u32);
    println!("'中' = U+{:04X}", '中' as u32);
    
    // 字符方法
    println!("\n字符方法:");
    println!("is_alphabetic('a') = {}", 'a'.is_alphabetic());
    println!("is_numeric('5') = {}", '5'.is_numeric());
    println!("is_alphanumeric('5') = {}", '5'.is_alphanumeric());
    println!("is_lowercase('a') = {}", 'a'.is_lowercase());
    println!("is_uppercase('A') = {}", 'A'.is_uppercase());
    println!("is_whitespace(' ') = {}", ' '.is_whitespace());
    
    // 大小写转换
    println!("\n大小写转换:");
    println!("'a'.to_uppercase() = {}", 'a'.to_uppercase());
    println!("'A'.to_lowercase() = {}", 'A'.to_lowercase());
    
    // 转义字符
    println!("\n转义字符:");
    println!("换行: \\n");
    println!("制表符:\t\\t");
    println!("回车: \\r");
    println!("反斜杠: \\\\");
    println!("单引号: \\'");
    println!("双引号: \\\"");
}

/// # 类型转换
pub fn type_conversion_demo() {
    println!("\n=== 类型转换 ===");
    
    // as 转换
    let x: i32 = 42;
    let y: i64 = x as i64;
    let z: f64 = x as f64;
    let c: char = 65 as char;  // 'A'
    
    println!("i32 -> i64: {}", y);
    println!("i32 -> f64: {}", z);
    println!("u8 -> char: {}", c);
    
    // 可能丢失信息的转换
    let large: i64 = 300;
    let small: i8 = large as i8;  // 截断
    println!("i64(300) -> i8: {}", small);
    
    let float = 3.9;
    let truncated = float as i32;  // 截断小数部分
    println!("f64(3.9) -> i32: {}", truncated);
    
    // 字符串转数字
    println!("\n字符串转数字:");
    let s = "42";
    match s.parse::<i32>() {
        Ok(num) => println!("\"{}\" -> {}", s, num),
        Err(e) => println!("解析错误: {}", e),
    }
    
    // 数字转字符串
    let num = 42;
    let s = num.to_string();
    println!("i32({}) -> String(\"{}\")", num, s);
}

/// 运行所有基本类型示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust 基本类型详解           ║");
    println!("╚════════════════════════════════════╝");
    
    integer_types_demo();
    integer_methods_demo();
    float_types_demo();
    float_methods_demo();
    float_precision_demo();
    bool_type_demo();
    bool_conversion_demo();
    char_type_demo();
    type_conversion_demo();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_integer_overflow() {
        let (result, overflowed) = 255_u8.overflowing_add(1);
        assert_eq!(result, 0);
        assert!(overflowed);
    }
    
    #[test]
    fn test_float_precision() {
        let x = 0.1 + 0.2;
        assert!((x - 0.3).abs() < 1e-10);
    }
    
    #[test]
    fn test_bool_conversion() {
        assert_eq!(true as i32, 1);
        assert_eq!(false as i32, 0);
    }
}
