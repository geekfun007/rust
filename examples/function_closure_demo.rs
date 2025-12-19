//! Rust 函数与闭包 - 实战演示
//! 
//! 本示例深入展示：
//! - fn vs ||：函数 vs 闭包
//! - 闭包捕获模式
//! - 闭包 Trait (Fn/FnMut/FnOnce)
//! - 函数指针
//! - 高阶函数
//! - 实战应用场景

use std::cell::RefCell;
use std::rc::Rc;

// ============================================
// 1. 基础对比：fn vs ||
// ============================================

fn basic_comparison_demo() {
    println!("\n========== 1. 基础对比：fn vs || ==========");
    
    // 函数定义
    fn add_function(x: i32, y: i32) -> i32 {
        x + y
    }
    
    // 闭包定义
    let add_closure = |x: i32, y: i32| -> i32 { x + y };
    let add_closure_short = |x, y| x + y;  // 类型推断
    
    println!("函数调用: {}", add_function(3, 4));
    println!("闭包调用: {}", add_closure(3, 4));
    println!("闭包简写: {}", add_closure_short(3, 4));
    
    // 大小对比
    println!("\n大小对比:");
    println!("  fn size: {} 字节", std::mem::size_of::<fn(i32, i32) -> i32>());
    println!("  closure (无捕获): {} 字节", std::mem::size_of_val(&add_closure));
}

// ============================================
// 2. 环境捕获
// ============================================

fn capture_demo() {
    println!("\n========== 2. 环境捕获 ==========");
    
    let x = 10;
    let y = String::from("hello");
    
    // 函数无法捕获
    fn _cannot_capture() {
        // println!("{}", x);  // ❌ 错误
    }
    
    // 闭包可以捕获
    let can_capture = || {
        println!("捕获 x: {}", x);
        println!("捕获 y: {}", y);
    };
    
    can_capture();
    
    // 捕获后的大小
    let z = 20;
    let capture_one = || println!("{}", x);
    let capture_two = || println!("{} {}", x, z);
    
    println!("\n闭包大小:");
    println!("  无捕获: {} 字节", std::mem::size_of_val(&(|| {})));
    println!("  捕获一个 i32: {} 字节", std::mem::size_of_val(&capture_one));
    println!("  捕获两个 i32: {} 字节", std::mem::size_of_val(&capture_two));
}

// ============================================
// 3. 三种捕获模式
// ============================================

fn capture_modes_demo() {
    println!("\n========== 3. 三种捕获模式 ==========");
    
    // 模式 1: 不可变借用
    println!("\n--- 不可变借用 (&T) ---");
    let s = String::from("hello");
    let borrow = || println!("借用: {}", s);
    borrow();
    println!("原值仍可用: {}", s);
    
    // 模式 2: 可变借用
    println!("\n--- 可变借用 (&mut T) ---");
    let mut count = 0;
    {
        let mut increment = || {
            count += 1;
            println!("增加后: {}", count);
        };
        increment();
        increment();
    }  // increment 的作用域结束
    println!("最终值: {}", count);
    
    // 模式 3: 获取所有权 (move)
    println!("\n--- 获取所有权 (move) ---");
    let s2 = String::from("world");
    let consume = move || {
        println!("移动: {}", s2);
        // s2 被移动到闭包内
    };
    consume();
    // println!("{}", s2);  // ❌ s2 已被移动
    println!("s2 已被移动到闭包");
}

// ============================================
// 4. 闭包 Trait: Fn, FnMut, FnOnce
// ============================================

fn closure_traits_demo() {
    println!("\n========== 4. 闭包 Trait ==========");
    
    // Fn: 不修改环境
    println!("\n--- Fn Trait ---");
    let x = 10;
    let fn_closure = || {
        println!("Fn: 读取 x = {}", x);
    };
    call_fn(&fn_closure);
    call_fn(&fn_closure);  // 可以多次调用
    
    // FnMut: 修改环境
    println!("\n--- FnMut Trait ---");
    let mut count = 0;
    let mut fn_mut_closure = || {
        count += 1;
        println!("FnMut: count = {}", count);
    };
    call_fn_mut(&mut fn_mut_closure);
    call_fn_mut(&mut fn_mut_closure);  // 可以多次调用
    
    // FnOnce: 消耗环境
    println!("\n--- FnOnce Trait ---");
    let s = String::from("hello");
    let fn_once_closure = move || {
        println!("FnOnce: 消耗 {}", s);
        drop(s);
    };
    call_fn_once(fn_once_closure);
    // call_fn_once(fn_once_closure);  // ❌ 只能调用一次
    println!("FnOnce 闭包已被消耗");
}

fn call_fn<F>(f: &F)
where
    F: Fn(),
{
    f();
}

fn call_fn_mut<F>(f: &mut F)
where
    F: FnMut(),
{
    f();
}

fn call_fn_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

// ============================================
// 5. 函数指针
// ============================================

fn function_pointer_demo() {
    println!("\n========== 5. 函数指针 ==========");
    
    fn double(x: i32) -> i32 {
        x * 2
    }
    
    fn triple(x: i32) -> i32 {
        x * 3
    }
    
    // 函数指针
    let f: fn(i32) -> i32 = double;
    println!("通过函数指针调用: {}", f(5));
    
    // 函数指针数组
    let operations: [fn(i32) -> i32; 2] = [double, triple];
    println!("\n函数指针数组:");
    for (i, op) in operations.iter().enumerate() {
        println!("  操作 {}: {}", i, op(5));
    }
    
    // 函数实现了 Fn
    println!("\n函数作为 Fn trait:");
    apply_operation(double, 10);
    
    // 闭包无法转换为 fn
    // let c: fn(i32) -> i32 = |x| x * 2;  // ❌ 错误
    println!("注意: 闭包不能转换为 fn 类型");
}

fn apply_operation<F>(f: F, x: i32)
where
    F: Fn(i32) -> i32,
{
    println!("结果: {}", f(x));
}

// ============================================
// 6. 高阶函数：返回函数/闭包
// ============================================

fn higher_order_demo() {
    println!("\n========== 6. 高阶函数 ==========");
    
    // 返回闭包（使用 impl Trait）
    println!("\n--- 返回闭包 ---");
    let add_5 = make_adder(5);
    println!("add_5(10) = {}", add_5(10));
    
    let mul_3 = make_multiplier(3);
    println!("mul_3(10) = {}", mul_3(10));
    
    // 返回 Box<dyn Fn>
    println!("\n--- 返回 trait object ---");
    let add_op = make_operation('+');
    let mul_op = make_operation('*');
    println!("add(3, 4) = {}", add_op(3, 4));
    println!("mul(3, 4) = {}", mul_op(3, 4));
    
    // 函数组合
    println!("\n--- 函数组合 ---");
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    let add_then_double = compose(add_one, double);
    println!("(5 + 1) * 2 = {}", add_then_double(5));
}

fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn make_multiplier(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x * y
}

fn make_operation(op: char) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        '+' => Box::new(|a, b| a + b),
        '-' => Box::new(|a, b| a - b),
        '*' => Box::new(|a, b| a * b),
        '/' => Box::new(|a, b| a / b),
        _ => Box::new(|a, b| a + b),
    }
}

fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

// ============================================
// 7. 实战：迭代器操作
// ============================================

fn iterator_demo() {
    println!("\n========== 7. 实战：迭代器操作 ==========");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 使用闭包
    println!("\n--- 使用闭包 ---");
    let evens: Vec<_> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("偶数: {:?}", evens);
    
    let doubled: Vec<_> = numbers.iter()
        .map(|&x| x * 2)
        .collect();
    println!("翻倍: {:?}", doubled);
    
    let sum: i32 = numbers.iter()
        .fold(0, |acc, &x| acc + x);
    println!("求和: {}", sum);
    
    // 使用函数
    println!("\n--- 使用函数 ---");
    fn is_even(x: &&i32) -> bool {
        **x % 2 == 0
    }
    
    fn double_value(x: &i32) -> i32 {
        x * 2
    }
    
    let evens2: Vec<_> = numbers.iter()
        .filter(is_even)
        .collect();
    println!("偶数（函数）: {:?}", evens2);
    
    let doubled2: Vec<_> = numbers.iter()
        .map(double_value)
        .collect();
    println!("翻倍（函数）: {:?}", doubled2);
    
    // 捕获外部变量
    println!("\n--- 捕获外部变量 ---");
    let threshold = 5;
    let above_threshold: Vec<_> = numbers.iter()
        .filter(|&&x| x > threshold)
        .collect();
    println!("大于 {}: {:?}", threshold, above_threshold);
}

// ============================================
// 8. 实战：回调机制
// ============================================

struct Button {
    label: String,
    on_click: Box<dyn Fn()>,
}

impl Button {
    fn new<F>(label: &str, on_click: F) -> Self
    where
        F: Fn() + 'static,
    {
        Button {
            label: label.to_string(),
            on_click: Box::new(on_click),
        }
    }
    
    fn click(&self) {
        println!("\n[{}] 按钮被点击", self.label);
        (self.on_click)();
    }
}

fn callback_demo() {
    println!("\n========== 8. 实战：回调机制 ==========");
    
    // 简单回调
    let simple_button = Button::new("简单按钮", || {
        println!("  执行简单操作");
    });
    simple_button.click();
    
    // 捕获变量的回调
    let counter = Rc::new(RefCell::new(0));
    let counter_clone = counter.clone();
    
    let counter_button = Button::new("计数器", move || {
        *counter_clone.borrow_mut() += 1;
        println!("  点击次数: {}", counter_clone.borrow());
    });
    
    counter_button.click();
    counter_button.click();
    counter_button.click();
}

// ============================================
// 9. 实战：惰性求值
// ============================================

struct LazyValue<F>
where
    F: FnOnce() -> i32,
{
    init: Option<F>,
    value: Option<i32>,
}

impl<F> LazyValue<F>
where
    F: FnOnce() -> i32,
{
    fn new(init: F) -> Self {
        LazyValue {
            init: Some(init),
            value: None,
        }
    }
    
    fn get(&mut self) -> i32 {
        if self.value.is_none() {
            if let Some(init) = self.init.take() {
                println!("    [计算中...]");
                self.value = Some(init());
            }
        }
        self.value.unwrap()
    }
}

fn lazy_evaluation_demo() {
    println!("\n========== 9. 实战：惰性求值 ==========");
    
    let mut lazy = LazyValue::new(|| {
        // 模拟耗时计算
        std::thread::sleep(std::time::Duration::from_millis(100));
        42
    });
    
    println!("LazyValue 已创建（未计算）");
    println!("第一次获取: {}", lazy.get());
    println!("第二次获取: {}", lazy.get());
    println!("第三次获取: {}", lazy.get());
}

// ============================================
// 10. 实战：策略模式
// ============================================

struct Calculator<F>
where
    F: Fn(i32, i32) -> i32,
{
    strategy: F,
}

impl<F> Calculator<F>
where
    F: Fn(i32, i32) -> i32,
{
    fn new(strategy: F) -> Self {
        Calculator { strategy }
    }
    
    fn calculate(&self, a: i32, b: i32) -> i32 {
        (self.strategy)(a, b)
    }
}

fn strategy_pattern_demo() {
    println!("\n========== 10. 实战：策略模式 ==========");
    
    let add_calc = Calculator::new(|a, b| a + b);
    let mul_calc = Calculator::new(|a, b| a * b);
    let max_calc = Calculator::new(|a, b| a.max(b));
    
    println!("加法: 3 + 4 = {}", add_calc.calculate(3, 4));
    println!("乘法: 3 * 4 = {}", mul_calc.calculate(3, 4));
    println!("最大: max(3, 4) = {}", max_calc.calculate(3, 4));
    
    // 动态策略
    let operations: Vec<Box<dyn Fn(i32, i32) -> i32>> = vec![
        Box::new(|a, b| a + b),
        Box::new(|a, b| a - b),
        Box::new(|a, b| a * b),
    ];
    
    println!("\n动态策略:");
    for (i, op) in operations.iter().enumerate() {
        println!("  操作 {}: {}", i, op(10, 3));
    }
}

// ============================================
// 11. 实战：管道/链式操作
// ============================================

fn pipeline_demo() {
    println!("\n========== 11. 实战：管道操作 ==========");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 链式操作
    let result: Vec<_> = data.iter()
        .filter(|&&x| x % 2 == 0)      // 过滤偶数
        .map(|&x| x * 2)                // 翻倍
        .filter(|&x| x > 5)             // 大于 5
        .take(3)                        // 取前 3 个
        .collect();
    
    println!("管道结果: {:?}", result);
    
    // 自定义管道函数
    fn pipeline<T, F1, F2, R>(value: T, f1: F1, f2: F2) -> R
    where
        F1: Fn(T) -> i32,
        F2: Fn(i32) -> R,
    {
        f2(f1(value))
    }
    
    let result2 = pipeline(
        5,
        |x| x * 2,
        |x| x + 1,
    );
    println!("自定义管道: {}", result2);
}

// ============================================
// 12. 性能对比
// ============================================

fn performance_demo() {
    println!("\n========== 12. 性能对比 ==========");
    
    const SIZE: usize = 100_000;
    let data: Vec<i32> = (1..=SIZE as i32).collect();
    
    // 方法 1: 闭包（内联优化）
    let start = std::time::Instant::now();
    let sum1: i64 = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x as i64 * 2)
        .sum();
    let elapsed1 = start.elapsed();
    
    // 方法 2: 手写循环
    let start = std::time::Instant::now();
    let mut sum2: i64 = 0;
    for &x in &data {
        if x % 2 == 0 {
            sum2 += x as i64 * 2;
        }
    }
    let elapsed2 = start.elapsed();
    
    println!("数据大小: {}", SIZE);
    println!("结果: sum1 = {}, sum2 = {}", sum1, sum2);
    println!("\n性能:");
    println!("  闭包方式: {:?}", elapsed1);
    println!("  循环方式: {:?}", elapsed2);
    println!("  差异: {:.2}%", 
        (elapsed1.as_nanos() as f64 - elapsed2.as_nanos() as f64) 
        / elapsed2.as_nanos() as f64 * 100.0);
}

// ============================================
// 13. 常见陷阱
// ============================================

fn common_pitfalls_demo() {
    println!("\n========== 13. 常见陷阱 ==========");
    
    // 陷阱 1: 闭包类型不匹配
    println!("\n--- 陷阱 1: 闭包类型不匹配 ---");
    let c1 = |x: i32| x;
    let c2 = |x: i32| x;
    
    let _f = c1;
    // f = c2;  // ❌ 错误：每个闭包都是唯一类型
    println!("每个闭包都有唯一的类型");
    
    // 解决：使用 trait object
    let mut _f: Box<dyn Fn(i32) -> i32> = Box::new(|x: i32| x);
    _f = Box::new(c2);
    println!("使用 Box<dyn Fn> 可以统一类型");
    
    // 陷阱 2: 借用冲突
    println!("\n--- 陷阱 2: 借用冲突 ---");
    let mut v = vec![1, 2, 3];
    {
        let _c = || v.len();
        // v.push(4);  // ❌ 错误：v 被闭包借用
    }  // 闭包作用域结束
    v.push(4);
    println!("缩小闭包作用域可以避免借用冲突");
    
    // 陷阱 3: move 的误用
    println!("\n--- 陷阱 3: move 误用 ---");
    let s = String::from("hello");
    let s_clone = s.clone();
    let _c1 = move || println!("{}", s);
    let _c2 = move || println!("{}", s_clone);
    // println!("{}", s);  // ❌ s 已被移动
    println!("使用 clone 可以避免 move 冲突");
}

// ============================================
// 14. 类型推断演示
// ============================================

fn type_inference_demo() {
    println!("\n========== 14. 类型推断 ==========");
    
    // 闭包类型推断
    let add = |x, y| x + y;
    let result1 = add(1, 2);        // 推断为 i32
    // let result2 = add(1.0, 2.0);  // ❌ 类型已固定
    println!("第一次使用推断类型: {}", result1);
    
    // 显式类型标注
    let add_i32 = |x: i32, y: i32| -> i32 { x + y };
    let add_f64 = |x: f64, y: f64| -> f64 { x + y };
    
    println!("i32 加法: {}", add_i32(1, 2));
    println!("f64 加法: {}", add_f64(1.0, 2.0));
    
    // 从上下文推断
    let numbers = vec![1, 2, 3];
    let doubled: Vec<i32> = numbers.iter()
        .map(|&x| x * 2)  // 从返回类型推断
        .collect();
    println!("从上下文推断: {:?}", doubled);
}

// ============================================
// Main 函数
// ============================================

fn main() {
    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║   Rust 函数与闭包 - 完整实战演示             ║");
    println!("╚═══════════════════════════════════════════════╝");
    
    basic_comparison_demo();
    capture_demo();
    capture_modes_demo();
    closure_traits_demo();
    function_pointer_demo();
    higher_order_demo();
    iterator_demo();
    callback_demo();
    lazy_evaluation_demo();
    strategy_pattern_demo();
    pipeline_demo();
    performance_demo();
    common_pitfalls_demo();
    type_inference_demo();
    
    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║   演示完成！                                  ║");
    println!("╚═══════════════════════════════════════════════╝");
}

// ============================================
// 测试
// ============================================

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_basic_function() {
        fn add(x: i32, y: i32) -> i32 {
            x + y
        }
        assert_eq!(add(2, 3), 5);
    }
    
    #[test]
    fn test_basic_closure() {
        let add = |x: i32, y: i32| x + y;
        assert_eq!(add(2, 3), 5);
    }
    
    #[test]
    fn test_closure_capture() {
        let x = 10;
        let add_x = |y| x + y;
        assert_eq!(add_x(5), 15);
    }
    
    #[test]
    fn test_fn_trait() {
        let x = 5;
        let closure = || x;
        
        fn call_fn<F: Fn() -> i32>(f: F) -> i32 {
            f()
        }
        
        assert_eq!(call_fn(closure), 5);
    }
    
    #[test]
    fn test_fn_mut_trait() {
        let mut count = 0;
        let mut closure = || {
            count += 1;
            count
        };
        
        assert_eq!(closure(), 1);
        assert_eq!(closure(), 2);
    }
    
    #[test]
    fn test_fn_once_trait() {
        let s = String::from("hello");
        let closure = move || s;
        
        assert_eq!(closure(), "hello");
    }
    
    #[test]
    fn test_function_pointer() {
        fn double(x: i32) -> i32 {
            x * 2
        }
        
        let f: fn(i32) -> i32 = double;
        assert_eq!(f(5), 10);
    }
    
    #[test]
    fn test_higher_order() {
        fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
            move |y| x + y
        }
        
        let add_5 = make_adder(5);
        assert_eq!(add_5(10), 15);
    }
    
    #[test]
    fn test_iterator_with_closure() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * 2)
            .sum();
        assert_eq!(sum, 12);  // (2 + 4) * 2 = 12
    }
    
    #[test]
    fn test_compose() {
        fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
        where
            F: Fn(A) -> B,
            G: Fn(B) -> C,
        {
            move |x| g(f(x))
        }
        
        let add_one = |x: i32| x + 1;
        let double = |x: i32| x * 2;
        let f = compose(add_one, double);
        
        assert_eq!(f(5), 12);  // (5 + 1) * 2
    }
}
