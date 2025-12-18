// Rust 内存布局与底层原理演示

use std::mem;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║         Rust 内存布局与底层原理演示                      ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
    
    size_and_alignment_demo();
    ownership_memory_demo();
    reference_memory_demo();
    slice_memory_demo();
    lifetime_compile_time_demo();
    zero_cost_abstraction_demo();
    performance_comparison_demo();
}

/// 1. 类型大小与对齐
fn size_and_alignment_demo() {
    println!("═══ 1. 类型大小与内存对齐 ═══\n");
    
    println!("基本类型大小：");
    println!("  i8:    {} 字节，对齐: {} 字节", mem::size_of::<i8>(), mem::align_of::<i8>());
    println!("  i32:   {} 字节，对齐: {} 字节", mem::size_of::<i32>(), mem::align_of::<i32>());
    println!("  i64:   {} 字节，对齐: {} 字节", mem::size_of::<i64>(), mem::align_of::<i64>());
    println!("  f32:   {} 字节，对齐: {} 字节", mem::size_of::<f32>(), mem::align_of::<f32>());
    println!("  f64:   {} 字节，对齐: {} 字节", mem::size_of::<f64>(), mem::align_of::<f64>());
    println!("  bool:  {} 字节，对齐: {} 字节", mem::size_of::<bool>(), mem::align_of::<bool>());
    println!("  char:  {} 字节，对齐: {} 字节", mem::size_of::<char>(), mem::align_of::<char>());
    
    println!("\n复合类型大小：");
    println!("  &T (瘦指针):        {} 字节", mem::size_of::<&i32>());
    println!("  &str (胖指针):      {} 字节 (ptr + len)", mem::size_of::<&str>());
    println!("  &[T] (胖指针):      {} 字节 (ptr + len)", mem::size_of::<&[i32]>());
    println!("  String:             {} 字节 (ptr + len + cap)", mem::size_of::<String>());
    println!("  Vec<T>:             {} 字节 (ptr + len + cap)", mem::size_of::<Vec<i32>>());
    println!("  Option<&T>:         {} 字节 (优化后)", mem::size_of::<Option<&i32>>());
    println!("  Result<(), ()>:     {} 字节", mem::size_of::<Result<(), ()>>());
    
    // 结构体对齐示例
    #[repr(C)]
    struct BadLayout {
        a: u8,      // 1 字节
        b: u64,     // 8 字节（需要对齐）
        c: u8,      // 1 字节
    }
    
    #[repr(C)]
    struct GoodLayout {
        b: u64,     // 8 字节
        a: u8,      // 1 字节
        c: u8,      // 1 字节
    }
    
    println!("\n结构体对齐影响：");
    println!("  BadLayout:  {} 字节（有填充）", mem::size_of::<BadLayout>());
    println!("  GoodLayout: {} 字节（优化后）", mem::size_of::<GoodLayout>());
}

/// 2. 所有权的内存表现
fn ownership_memory_demo() {
    println!("\n═══ 2. 所有权的内存表现 ═══\n");
    
    // 栈上的数据
    println!("栈上数据（Copy）：");
    let x = 5;
    let y = x;  // Copy
    println!("  x = {}, y = {} (两者都有效)", x, y);
    println!("  原因：i32 实现了 Copy，栈上直接复制");
    
    // 堆上的数据
    println!("\n堆上数据（Move）：");
    let s1 = String::from("hello");
    println!("  s1 地址: {:p}", s1.as_ptr());
    println!("  s1 长度: {}", s1.len());
    println!("  s1 容量: {}", s1.capacity());
    
    let s2 = s1;  // Move，s1 失效
    println!("  s2 地址: {:p} (与 s1 相同)", s2.as_ptr());
    println!("  s2 继承了 s1 的堆数据");
    // println!("{}", s1);  // ❌ 编译错误：s1 已被移动
    
    // Clone
    println!("\n深拷贝（Clone）：");
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("  s3 地址: {:p}", s3.as_ptr());
    println!("  s4 地址: {:p} (不同地址)", s4.as_ptr());
    println!("  s3 和 s4 都有效，独立的堆内存");
}

/// 3. 引用的内存表现
fn reference_memory_demo() {
    println!("\n═══ 3. 引用的内存表现 ═══\n");
    
    let s = String::from("hello");
    
    // 不可变引用
    println!("不可变引用：");
    let r1 = &s;
    let r2 = &s;
    println!("  原始 String: {:p} (堆数据地址)", s.as_ptr());
    println!("  引用 r1:     {:p} (指向栈上的 String)", r1 as *const String);
    println!("  引用 r2:     {:p} (指向栈上的 String)", r2 as *const String);
    println!("  r1 指向的数据: {:p}", r1.as_ptr());
    println!("  r2 指向的数据: {:p}", r2.as_ptr());
    println!("  引用大小: {} 字节（仅一个指针）", mem::size_of::<&String>());
    
    // 可变引用
    println!("\n可变引用：");
    let mut s2 = String::from("test");
    println!("  修改前地址: {:p}", s2.as_ptr());
    let r3 = &mut s2;
    r3.push_str(" data");
    println!("  修改后地址: {:p} (可能相同，可能重新分配)", r3.as_ptr());
}

/// 4. 切片的内存表现
fn slice_memory_demo() {
    println!("\n═══ 4. 切片的内存表现 ═══\n");
    
    // 字符串切片
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    
    println!("字符串切片（胖指针）：");
    println!("  String 数据: {:p}", s.as_ptr());
    println!("  切片 hello:  {:p}, 长度: {}", hello.as_ptr(), hello.len());
    println!("  切片 world:  {:p}, 长度: {}", world.as_ptr(), world.len());
    println!("  切片大小:    {} 字节 (ptr + len)", mem::size_of::<&str>());
    
    // 数组切片
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    
    println!("\n数组切片：");
    println!("  数组地址:   {:p}", &arr);
    println!("  切片起始:   {:p}", slice.as_ptr());
    println!("  切片长度:   {}", slice.len());
    println!("  元素: {:?}", slice);
    
    // 切片的零成本抽象
    println!("\n切片访问性能：");
    println!("  ✓ 边界检查：O(1)");
    println!("  ✓ 元素访问：O(1)");
    println!("  ✓ 迭代：    零开销（内联后等同于指针操作）");
}

/// 5. 生命周期是编译时概念
fn lifetime_compile_time_demo() {
    println!("\n═══ 5. 生命周期（编译时分析）═══\n");
    
    println!("生命周期标注只在编译时存在：");
    
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    
    let s1 = String::from("long string");
    let s2 = "short";
    let result = longest(&s1, &s2);
    
    println!("  函数签名: longest<'a>(x: &'a str, y: &'a str) -> &'a str");
    println!("  运行时没有生命周期标记");
    println!("  返回的引用: \"{}\"", result);
    println!("  引用地址: {:p}", result.as_ptr());
    println!("  ✓ 编译器已验证引用有效性");
    
    println!("\n生命周期开销：");
    println!("  编译时: 类型检查和借用分析");
    println!("  运行时: 零开销！");
}

/// 6. 零成本抽象验证
fn zero_cost_abstraction_demo() {
    println!("\n═══ 6. 零成本抽象 ═══\n");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    println!("高级迭代器代码：");
    println!("  data.iter().filter(|&&x| x > 5).map(|&x| x * 2).sum()");
    
    let sum1: i32 = data.iter()
        .filter(|&&x| x > 5)
        .map(|&x| x * 2)
        .sum();
    
    println!("\n等价的低级代码：");
    println!("  let mut sum = 0;");
    println!("  for &x in &data {{");
    println!("      if x > 5 {{ sum += x * 2; }}");
    println!("  }}");
    
    let mut sum2 = 0;
    for &x in &data {
        if x > 5 {
            sum2 += x * 2;
        }
    }
    
    println!("\n结果：");
    println!("  高级版本: {}", sum1);
    println!("  低级版本: {}", sum2);
    println!("  ✓ 编译后性能完全相同！");
    println!("  ✓ 可能使用 SIMD 指令优化");
}

/// 7. 性能对比
fn performance_comparison_demo() {
    println!("\n═══ 7. 性能对比 ═══\n");
    
    use std::time::Instant;
    
    const SIZE: usize = 100_000;  // 减小以避免溢出
    let data: Vec<i32> = (0..SIZE as i32).collect();
    
    // 通过引用（零成本）
    let start = Instant::now();
    process_by_ref(&data);
    let duration = start.elapsed();
    println!("通过引用传递:   {:?}", duration);
    
    // 克隆（昂贵）
    let start = Instant::now();
    process_by_clone(data.clone());
    let duration = start.elapsed();
    println!("克隆后传递:     {:?} (更慢)", duration);
    
    // 迭代器链
    let start = Instant::now();
    let _sum: i64 = data.iter().map(|&x| x as i64).filter(|&x| x % 2 == 0).sum();
    let duration = start.elapsed();
    println!("迭代器链:       {:?} (零成本抽象)", duration);
    
    println!("\n内存使用：");
    println!("  引用:   {} 字节（仅指针）", mem::size_of::<&Vec<i32>>());
    println!("  Vec:    {} 字节（栈）+ {} 字节（堆）", 
             mem::size_of::<Vec<i32>>(), 
             SIZE * mem::size_of::<i32>());
    
    fn process_by_ref(data: &Vec<i32>) {
        let _: i64 = data.iter().map(|&x| x as i64).sum();
    }
    
    fn process_by_clone(data: Vec<i32>) {
        let _: i64 = data.iter().map(|&x| x as i64).sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_sizes() {
        assert_eq!(mem::size_of::<&i32>(), 8);  // 64位系统
        assert_eq!(mem::size_of::<&str>(), 16); // 胖指针
        assert_eq!(mem::size_of::<String>(), 24);
    }
    
    #[test]
    fn test_ownership_move() {
        let s1 = String::from("test");
        let s2 = s1;  // Move
        assert_eq!(s2, "test");
        // s1 不再可用
    }
    
    #[test]
    fn test_reference() {
        let s = String::from("test");
        let r = &s;
        assert_eq!(s, "test");
        assert_eq!(r, "test");
        // s 和 r 都有效
    }
}
