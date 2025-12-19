//! Rust 引用与解引用 - 实战演示
//! 
//! 本示例深入展示：
//! - &v vs v：引用 vs 值
//! - *v：解引用操作
//! - &&v：双重引用
//! - **v：双重解引用
//! - 模式匹配中的引用
//! - 迭代器中的引用
//! - 实战应用场景

use std::collections::HashMap;

// ============================================
// 1. 基础：&v vs v
// ============================================

fn basic_reference_demo() {
    println!("\n========== 1. 基础引用 vs 值 ==========");
    
    // 值
    let x = 5;
    println!("x = {}, 类型: i32", x);
    
    // 不可变引用
    let r = &x;
    println!("r = {}, 类型: &i32", r);
    println!("*r = {}", *r);  // 解引用
    
    // 可变引用
    let mut y = 10;
    let mr = &mut y;
    println!("\n修改前: {}", mr);
    *mr += 5;  // 通过解引用修改
    println!("修改后: {}", mr);
    
    // 内存大小
    println!("\n内存大小:");
    println!("  i32:      {} 字节", std::mem::size_of::<i32>());
    println!("  &i32:     {} 字节", std::mem::size_of::<&i32>());
    println!("  &&i32:    {} 字节", std::mem::size_of::<&&i32>());
    println!("  &mut i32: {} 字节", std::mem::size_of::<&mut i32>());
}

// ============================================
// 2. 解引用操作 *v
// ============================================

fn deref_operations_demo() {
    println!("\n========== 2. 解引用操作 ==========");
    
    let x = 42;
    let r = &x;
    
    // 显式解引用
    let y = *r;
    println!("x = {}, *r = {}, y = {}", x, *r, y);
    
    // 解引用用于算术运算
    let a = &10;
    let b = &20;
    let sum = *a + *b;  // 必须解引用
    println!("\n算术运算: {} + {} = {}", a, b, sum);
    
    // 解引用用于比较
    let c = &5;
    let d = &5;
    println!("\n比较:");
    println!("  c == d (引用比较): {}", c == d);
    println!("  *c == *d (值比较): {}", *c == *d);
    
    // 可变引用的解引用
    let mut z = 100;
    let mr = &mut z;
    println!("\n可变引用解引用:");
    println!("  修改前: {}", mr);
    *mr *= 2;
    println!("  修改后: {}", mr);
    println!("  原值 z: {}", z);
}

// ============================================
// 3. 双重引用 &&v
// ============================================

fn double_reference_demo() {
    println!("\n========== 3. 双重引用 &&v ==========");
    
    let x = 5;
    let r = &x;      // &i32
    let rr = &r;     // &&i32
    
    println!("三层数据:");
    println!("  x   = {} (类型: i32)", x);
    println!("  *r  = {} (类型: &i32)", *r);
    println!("  **rr = {} (类型: &&i32)", **rr);
    
    // 类型检查
    let _: i32 = x;
    let _: &i32 = r;
    let _: &&i32 = rr;
    println!("\n✓ 类型检查通过");
    
    // 三重引用
    let rrr = &rr;   // &&&i32
    println!("\n三重引用:");
    println!("  ***rrr = {}", ***rrr);
    
    // 自动解引用
    println!("\n自动解引用:");
    println!("  rr 打印: {}", rr);  // 自动解引用
}

// ============================================
// 4. 双重解引用 **v
// ============================================

fn double_deref_demo() {
    println!("\n========== 4. 双重解引用 **v ==========");
    
    let x = 100;
    let r = &x;
    let rr = &r;
    
    // 逐层解引用
    println!("逐层解引用:");
    println!("  rr:   {:p} (类型: &&i32)", rr);
    println!("  *rr:  {:p} (类型: &i32)", *rr);
    println!("  **rr: {} (类型: i32)", **rr);
    
    // 可变引用的解引用
    let mut y = 50;
    println!("\n可变引用:");
    println!("  修改前: {}", y);
    
    {
        let mr = &mut y;
        *mr += 50;  // 解引用修改
    }  // mr 的作用域结束
    
    println!("  修改后: {}", y);
    
    // 复杂嵌套
    let a = 10;
    let r1 = &a;
    let r2 = &r1;
    let r3 = &r2;
    println!("\n四层嵌套:");
    println!("  ****(&r3) = {}", ****&r3);
}

// ============================================
// 5. 模式匹配中的引用
// ============================================

fn pattern_matching_demo() {
    println!("\n========== 5. 模式匹配中的引用 ==========");
    
    let x = 5;
    let r = &x;
    
    // 匹配引用
    match r {
        &val => println!("匹配引用，解构值: {}", val),
    }
    
    // 使用 ref
    match x {
        ref val => println!("使用 ref 创建引用: {}", val),
    }
    
    // if let 解引用
    let val = match r {
        &v => v,
    };
    println!("if let 解引用: {}", val);
    
    // Option<&T> 模式
    let opt: Option<&i32> = Some(&42);
    match opt {
        Some(&value) => println!("\nOption<&i32> 解引用: {}", value),
        None => {}
    }
    
    // 元组引用
    let tuple = (1, 2, 3);
    let tuple_ref = &tuple;
    match tuple_ref {
        &(a, b, c) => println!("\n元组引用解构: {}, {}, {}", a, b, c),
    }
}

// ============================================
// 6. 迭代器中的引用
// ============================================

fn iterator_reference_demo() {
    println!("\n========== 6. 迭代器中的引用 ==========");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // iter() 返回 &i32
    println!("iter() - 不可变引用:");
    for item in v.iter() {
        // item: &i32
        println!("  item = {} (类型: &i32)", item);
    }
    
    // filter 闭包参数是 &&i32
    println!("\nfilter() - 双重引用:");
    let even: Vec<_> = v.iter()
        .filter(|&&x| {  // 双重解引用模式
            println!("  检查: {}", x);
            x % 2 == 0
        })
        .collect();
    println!("结果: {:?}", even);
    
    // map 闭包参数是 &i32
    println!("\nmap() - 单引用:");
    let doubled: Vec<_> = v.iter()
        .map(|&x| {  // 单解引用模式
            x * 2
        })
        .collect();
    println!("结果: {:?}", doubled);
    
    // 不同的解引用方式
    println!("\n不同解引用方式:");
    let sum1: i32 = v.iter().map(|&x| x).sum();  // 模式解引用
    let sum2: i32 = v.iter().map(|x| *x).sum();  // 显式解引用
    println!("  模式解引用: {}", sum1);
    println!("  显式解引用: {}", sum2);
}

// ============================================
// 7. 链式方法调用
// ============================================

fn chaining_methods_demo() {
    println!("\n========== 7. 链式方法调用 ==========");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 复杂的引用链
    println!("原始数据: {:?}", data);
    
    let result: Vec<_> = data
        .iter()                      // Iterator<Item = &i32>
        .filter(|&&x| x > 3)         // 双重解引用
        .map(|&x| x * 2)             // 单解引用
        .filter(|&x| x < 15)         // 注意：这里是 &i32，不是 &&i32
        .collect();
    
    println!("处理后: {:?}", result);
    
    // 显式类型标注
    let result2: Vec<_> = data
        .iter()
        .filter(|x: &&i32| **x > 5)  // 显式双重解引用
        .map(|x: &i32| *x * 3)       // 显式单解引用
        .collect();
    
    println!("显式标注: {:?}", result2);
}

// ============================================
// 8. HashMap 中的引用
// ============================================

fn hashmap_reference_demo() {
    println!("\n========== 8. HashMap 中的引用 ==========");
    
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Carol", 92);
    
    // get 返回 Option<&V>
    match scores.get("Alice") {
        Some(&score) => {  // 解引用模式
            println!("Alice 的分数: {}", score);
        }
        None => {}
    }
    
    // 不解引用
    if let Some(score) = scores.get("Bob") {
        // score: &i32
        println!("Bob 的分数 (引用): {}", score);
        println!("Bob 的分数 (解引用): {}", *score);
    }
    
    // 迭代 HashMap
    println!("\n所有分数:");
    for (name, &score) in &scores {  // 解引用 score
        println!("  {}: {}", name, score);
    }
    
    // 修改值
    if let Some(score) = scores.get_mut("Carol") {
        // score: &mut i32
        *score += 5;  // 解引用修改
    }
    println!("\n修改后 Carol 的分数: {}", scores["Carol"]);
}

// ============================================
// 9. 自定义类型的引用
// ============================================

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    
    fn distance(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

fn custom_type_reference_demo() {
    println!("\n========== 9. 自定义类型的引用 ==========");
    
    let p1 = Point::new(3, 4);
    let p_ref = &p1;
    
    println!("点: {:?}", p1);
    println!("引用: {:?}", p_ref);
    println!("距离: {:.2}", p_ref.distance());  // 自动解引用
    
    // 结构体引用的 Vec
    let points = vec![
        Point::new(1, 2),
        Point::new(3, 4),
        Point::new(5, 6),
    ];
    
    println!("\n点的集合:");
    for point in points.iter() {
        // point: &Point
        println!("  {:?}, 距离: {:.2}", point, point.distance());
    }
    
    // filter 和 map
    let far_points: Vec<_> = points
        .iter()
        .filter(|p| p.distance() > 3.0)  // 自动解引用
        .map(|p| p.clone())              // 克隆点
        .collect();
    
    println!("\n远点: {:?}", far_points);
}

// ============================================
// 10. 实战：排序中的引用
// ============================================

fn sorting_reference_demo() {
    println!("\n========== 10. 实战：排序中的引用 ==========");
    
    let mut numbers = vec![5, 2, 8, 1, 9, 3];
    println!("原始: {:?}", numbers);
    
    // sort_by 的闭包参数是 &i32
    numbers.sort_by(|a, b| {
        // a: &i32, b: &i32
        a.cmp(b)  // 直接比较
    });
    println!("排序后: {:?}", numbers);
    
    // 使用解引用
    let mut numbers2 = vec![5, 2, 8, 1, 9, 3];
    numbers2.sort_by(|a, b| (*a).cmp(&(*b)));
    println!("显式解引用排序: {:?}", numbers2);
    
    // 自定义结构体排序
    let mut points = vec![
        Point::new(5, 2),
        Point::new(1, 1),
        Point::new(3, 4),
    ];
    
    println!("\n点排序 (按距离):");
    println!("排序前: {:?}", points);
    points.sort_by(|a, b| {
        // a: &Point, b: &Point (自动解引用)
        a.distance().partial_cmp(&b.distance()).unwrap()
    });
    println!("排序后: {:?}", points);
}

// ============================================
// 11. 实战：Option 和 Result 的引用
// ============================================

fn option_result_reference_demo() {
    println!("\n========== 11. Option 和 Result 的引用 ==========");
    
    // Option<T> vs Option<&T>
    let value = 42;
    let opt_owned: Option<i32> = Some(value);
    let opt_ref: Option<&i32> = Some(&value);
    
    println!("Option<i32>: {:?}", opt_owned);
    println!("Option<&i32>: {:?}", opt_ref);
    
    // as_ref 转换
    if let Some(val) = opt_owned.as_ref() {
        // val: &i32
        println!("as_ref: {}", val);
    }
    
    // map 中的引用
    let doubled = opt_ref.map(|&x| x * 2);  // 解引用
    println!("doubled: {:?}", doubled);
    
    // Result 的引用
    let result: Result<i32, &str> = Ok(100);
    match result.as_ref() {
        Ok(&value) => println!("\nResult 值: {}", value),
        Err(&err) => println!("错误: {}", err),
    }
}

// ============================================
// 12. 实战：闭包捕获
// ============================================

fn closure_capture_demo() {
    println!("\n========== 12. 闭包捕获 ==========");
    
    let x = 5;
    
    // 捕获不可变引用
    let print_x = || println!("x = {}", x);  // 捕获 &x
    print_x();
    println!("x 仍然可用: {}", x);
    
    // 捕获可变引用
    let mut y = 10;
    let mut increment = || {
        y += 1;  // 捕获 &mut y
    };
    increment();
    println!("\ny 增加后: {}", y);
    
    // move 语义
    let z = vec![1, 2, 3];
    let consume = move || {
        println!("z: {:?}", z);  // 捕获 z 的所有权
    };
    consume();
    // println!("{:?}", z);  // ❌ z 已被移动
    
    // 迭代器闭包捕获
    let values = vec![1, 2, 3, 4, 5];
    let threshold = 3;
    
    let filtered: Vec<_> = values
        .iter()
        .filter(|&&x| x > threshold)  // 捕获 &threshold
        .collect();
    
    println!("\n过滤结果: {:?}", filtered);
    println!("threshold 仍可用: {}", threshold);
}

// ============================================
// 13. 性能对比
// ============================================

fn performance_demo() {
    println!("\n========== 13. 性能对比 ==========");
    
    const SIZE: usize = 100_000;
    let data: Vec<i32> = (1..=SIZE as i32).collect();
    
    // 方法 1：使用引用（零开销）
    let start = std::time::Instant::now();
    let sum1: i64 = data.iter().map(|&x| x as i64).sum();
    let elapsed1 = start.elapsed();
    
    // 方法 2：克隆（有开销）
    let start = std::time::Instant::now();
    let sum2: i64 = data.clone().into_iter().map(|x| x as i64).sum();
    let elapsed2 = start.elapsed();
    
    println!("数据大小: {}", SIZE);
    println!("结果: sum1 = {}, sum2 = {}", sum1, sum2);
    println!("\n性能:");
    println!("  引用方式: {:?}", elapsed1);
    println!("  克隆方式: {:?}", elapsed2);
    println!("  速度比: {:.2}x", elapsed2.as_nanos() as f64 / elapsed1.as_nanos() as f64);
}

// ============================================
// 14. 常见陷阱
// ============================================

fn common_pitfalls_demo() {
    println!("\n========== 14. 常见陷阱 ==========");
    
    // 陷阱 1：忘记解引用
    let v = vec![1, 2, 3];
    // let result: Vec<_> = v.iter().filter(|x| x % 2 == 0).collect();  // ❌ 错误
    let result: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).collect();  // ✓ 正确
    println!("陷阱 1 - 过滤偶数: {:?}", result);
    
    // 陷阱 2：引用的引用
    let x = 5;
    let r = &x;
    let rr = &r;
    // println!("{}", *rr + 1);  // ❌ 错误：*rr 是 &i32
    println!("陷阱 2 - 双重解引用: {}", **rr + 1);  // ✓ 正确
    
    // 陷阱 3：可变引用的生命周期
    let mut values = vec![1, 2, 3];
    {
        let first = &values[0];  // 不可变借用
        println!("陷阱 3 - 第一个值: {}", first);
    }  // 不可变借用结束
    values.push(4);  // 现在可以可变借用
    println!("添加后: {:?}", values);
    
    // 陷阱 4：自动解引用的限制
    let x = &10;
    // let y = x + 1;        // ❌ 错误：不能对引用做算术
    let y = *x + 1;          // ✓ 正确：显式解引用
    println!("陷阱 4 - 算术运算: {}", y);
}

// ============================================
// 15. 高级：Deref Trait
// ============================================

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.0
    }
}

fn deref_trait_demo() {
    println!("\n========== 15. Deref Trait ==========");
    
    let x = MyBox::new(5);
    
    // 显式解引用
    println!("*x = {}", *x);  // 等价于 *(x.deref())
    
    // 自动解引用
    fn print_value(v: &i32) {
        println!("值: {}", v);
    }
    
    print_value(&x);  // MyBox<i32> -> &i32 (自动解引用)
    
    // 字符串的 Deref
    let s = String::from("hello");
    let s_ref: &str = &s;  // String -> &str (Deref)
    println!("\nString Deref: {}", s_ref);
    
    // 多层 Deref
    let boxed = Box::new(Box::new(42));
    println!("Box<Box<i32>>: {}", **boxed);
}

// ============================================
// Main 函数
// ============================================

fn main() {
    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║   Rust 引用与解引用 - 完整实战演示           ║");
    println!("╚═══════════════════════════════════════════════╝");
    
    basic_reference_demo();
    deref_operations_demo();
    double_reference_demo();
    double_deref_demo();
    pattern_matching_demo();
    iterator_reference_demo();
    chaining_methods_demo();
    hashmap_reference_demo();
    custom_type_reference_demo();
    sorting_reference_demo();
    option_result_reference_demo();
    closure_capture_demo();
    performance_demo();
    common_pitfalls_demo();
    deref_trait_demo();
    
    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║   演示完成！                                  ║");
    println!("╚═══════════════════════════════════════════════╝");
}

// ============================================
// 测试
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_reference() {
        let x = 5;
        let r = &x;
        assert_eq!(*r, 5);
        assert_eq!(x, *r);
    }
    
    #[test]
    fn test_double_reference() {
        let x = 42;
        let r = &x;
        let rr = &r;
        assert_eq!(**rr, 42);
    }
    
    #[test]
    fn test_mutable_reference() {
        let mut x = 10;
        let mr = &mut x;
        *mr += 5;
        assert_eq!(*mr, 15);
    }
    
    #[test]
    fn test_double_deref() {
        let x = 100;
        let r = &x;
        let rr = &r;
        let y = **rr;
        assert_eq!(y, 100);
    }
    
    #[test]
    fn test_iterator_reference() {
        let v = vec![1, 2, 3, 4, 5];
        let even: Vec<_> = v.iter()
            .filter(|&&x| x % 2 == 0)
            .collect();
        assert_eq!(even, vec![&2, &4]);
    }
    
    #[test]
    fn test_pattern_matching() {
        let x = 5;
        let r = &x;
        match r {
            &val => assert_eq!(val, 5),
        }
    }
    
    #[test]
    fn test_option_reference() {
        let x = 42;
        let opt: Option<&i32> = Some(&x);
        match opt {
            Some(&val) => assert_eq!(val, 42),
            None => panic!("Should be Some"),
        }
    }
    
    #[test]
    fn test_deref_trait() {
        let x = MyBox::new(5);
        assert_eq!(*x, 5);
    }
    
    #[test]
    fn test_reference_size() {
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::size_of::<&i32>(), 8);
        assert_eq!(std::mem::size_of::<&&i32>(), 8);
    }
    
    #[test]
    fn test_chaining() {
        let data = vec![1, 2, 3, 4, 5];
        let result: Vec<_> = data
            .iter()
            .filter(|&&x| x > 2)
            .map(|&x| x * 2)
            .collect();
        assert_eq!(result, vec![6, 8, 10]);
    }
}
