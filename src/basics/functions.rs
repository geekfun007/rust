// 函数与逻辑示例

/// # 基本函数定义
pub fn basic_functions_demo() {
    println!("\n=== 基本函数 ===");
    
    // 调用函数
    greet();
    greet_person("张三");
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    // 具有多个参数的函数
    print_labeled_measurement(5, 'h');
}

fn greet() {
    println!("你好，世界！");
}

fn greet_person(name: &str) {
    println!("你好，{}！", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // 没有分号，这是返回值
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("测量值是: {}{}", value, unit);
}

/// # 函数返回值
pub fn return_values_demo() {
    println!("\n=== 函数返回值 ===");
    
    // 隐式返回（最后一个表达式）
    let x = plus_one(5);
    println!("plus_one(5) = {}", x);
    
    // 显式返回
    let y = early_return(10);
    println!("early_return(10) = {}", y);
    
    // 返回元组
    let (sum, product) = calculate(3, 4);
    println!("和: {}, 积: {}", sum, product);
    
    // 返回 Result
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {}", e),
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn early_return(x: i32) -> i32 {
    if x > 5 {
        return x * 2;  // 提前返回
    }
    x
}

fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

/// # 闭包 (Closures)
pub fn closures_demo() {
    println!("\n=== 闭包 ===");
    
    // 基本闭包
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("add_one(5) = {}", add_one(5));
    
    // 类型推导
    let add_two = |x| x + 2;
    println!("add_two(5) = {}", add_two(5));
    
    // 捕获环境变量
    let y = 10;
    let add_y = |x| x + y;
    println!("add_y(5) = {}", add_y(5));
    
    // 多语句闭包
    let multiply_or_add = |x: i32, y: i32| {
        let product = x * y;
        if product > 10 {
            product
        } else {
            x + y
        }
    };
    println!("multiply_or_add(3, 4) = {}", multiply_or_add(3, 4));
    
    // 捕获可变引用
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("increment() = {}", increment());
    println!("increment() = {}", increment());
    println!("最终 count = {}", count);
    
    // 转移所有权
    let list = vec![1, 2, 3];
    println!("闭包前: {:?}", list);
    
    let consume_list = move || {
        println!("闭包内: {:?}", list);
    };
    consume_list();
    // println!("{:?}", list); // 错误！list 已被移动
}

/// # 高阶函数
pub fn higher_order_functions_demo() {
    println!("\n=== 高阶函数 ===");
    
    // 函数作为参数
    let result = apply_operation(5, 3, add);
    println!("apply_operation(5, 3, add) = {}", result);
    
    let result = apply_operation(5, 3, |a, b| a * b);
    println!("apply_operation(5, 3, multiply) = {}", result);
    
    // 返回闭包
    let adder = create_adder(10);
    println!("adder(5) = {}", adder(5));
    
    // map、filter、fold 等
    let numbers = vec![1, 2, 3, 4, 5];
    
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("加倍后: {:?}", doubled);
    
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
    
    let sum: i32 = numbers.iter().sum();
    println!("求和: {}", sum);
    
    let product: i32 = numbers.iter().product();
    println!("求积: {}", product);
}

fn apply_operation<F>(a: i32, b: i32, op: F) -> i32 
where
    F: Fn(i32, i32) -> i32
{
    op(a, b)
}

fn create_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

/// # 迭代器
pub fn iterators_demo() {
    println!("\n=== 迭代器 ===");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // 基本迭代
    println!("基本迭代:");
    for item in v.iter() {
        print!("{} ", item);
    }
    println!();
    
    // map 变换
    let v2: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("map 加倍: {:?}", v2);
    
    // filter 过滤
    let evens: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter 偶数: {:?}", evens);
    
    // 链式调用
    let result: Vec<i32> = v.iter()
        .filter(|&&x| x > 2)
        .map(|x| x * x)
        .collect();
    println!("链式调用: {:?}", result);
    
    // fold 累积
    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("fold 求和: {}", sum);
    
    // enumerate 带索引
    println!("enumerate:");
    for (i, value) in v.iter().enumerate() {
        println!("  索引 {}: 值 {}", i, value);
    }
    
    // zip 组合两个迭代器
    let names = vec!["Alice", "Bob", "Carol"];
    let ages = vec![25, 30, 35];
    println!("zip:");
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("  {} is {} years old", name, age);
    }
    
    // take 和 skip
    let numbers: Vec<i32> = (1..=10).collect();
    let first_three: Vec<i32> = numbers.iter().take(3).copied().collect();
    let skip_five: Vec<i32> = numbers.iter().skip(5).copied().collect();
    println!("take(3): {:?}", first_three);
    println!("skip(5): {:?}", skip_five);
    
    // any 和 all
    let has_even = v.iter().any(|&x| x % 2 == 0);
    let all_positive = v.iter().all(|&x| x > 0);
    println!("有偶数: {}, 全是正数: {}", has_even, all_positive);
    
    // find 查找
    let found = v.iter().find(|&&x| x > 3);
    println!("找到大于3的数: {:?}", found);
    
    // position 查找位置
    let pos = v.iter().position(|&x| x == 3);
    println!("值3的位置: {:?}", pos);
}

/// # 递归函数
pub fn recursion_demo() {
    println!("\n=== 递归函数 ===");
    
    // 阶乘
    let n = 5;
    println!("{}! = {}", n, factorial(n));
    
    // 斐波那契数列
    for i in 0..10 {
        print!("{} ", fibonacci(i));
    }
    println!();
    
    // 汉诺塔
    println!("汉诺塔移动步骤:");
    hanoi(3, 'A', 'C', 'B');
}

fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn hanoi(n: u32, from: char, to: char, aux: char) {
    if n == 1 {
        println!("  移动盘子从 {} 到 {}", from, to);
    } else {
        hanoi(n - 1, from, aux, to);
        println!("  移动盘子从 {} 到 {}", from, to);
        hanoi(n - 1, aux, to, from);
    }
}

/// # 泛型函数
pub fn generics_demo() {
    println!("\n=== 泛型函数 ===");
    
    // 泛型函数
    println!("largest i32: {}", largest(&[1, 5, 3, 2]));
    println!("largest char: {}", largest(&['a', 'z', 'm', 'b']));
    
    // 多个泛型参数
    let p = Point { x: 5, y: 10 };
    println!("Point: ({}, {})", p.x, p.y);
    
    let p2 = Point { x: 1.0, y: 4.0 };
    println!("Point: ({}, {})", p2.x, p2.y);
    
    // 混合类型
    let p3 = MixedPoint { x: 5, y: 4.0 };
    println!("MixedPoint: ({}, {})", p3.x, p3.y);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

/// 运行所有函数示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║      Rust 函数与逻辑示例           ║");
    println!("╚════════════════════════════════════╝");
    
    basic_functions_demo();
    return_values_demo();
    closures_demo();
    higher_order_functions_demo();
    iterators_demo();
    recursion_demo();
    generics_demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
    }
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(6), 8);
    }
}
