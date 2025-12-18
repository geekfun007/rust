// let 变量类型声明与泛型类型详解

fn main() {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║      Rust 类型标注与泛型详解                           ║");
    println!("╚════════════════════════════════════════════════════════╝\n");
    
    type_inference_demo();
    when_to_annotate_demo();
    generic_type_declaration_demo();
    turbofish_syntax_demo();
    complex_generic_demo();
    practical_examples_demo();
}

/// # 类型推导 - 不需要声明类型的情况
fn type_inference_demo() {
    println!("=== 1. 类型推导 - 编译器自动推断 ===\n");
    
    // ✅ 简单类型 - 编译器可以推断
    let x = 5;              // 推断为 i32
    let y = 3.14;           // 推断为 f64
    let z = true;           // 推断为 bool
    let s = "hello";        // 推断为 &str
    
    println!("x = {} (i32)", x);
    println!("y = {} (f64)", y);
    println!("z = {} (bool)", z);
    println!("s = {} (&str)", s);
    
    // ✅ 从上下文推断
    let mut vec = Vec::new();  // 类型未知
    vec.push(1);               // 现在知道是 Vec<i32>
    vec.push(2);
    println!("\nvec = {:?} (Vec<i32>)", vec);
    
    // ✅ 从函数返回值推断
    let s = String::from("hello");  // 明确知道是 String
    println!("s = {} (String)", s);
}

/// # 何时需要类型标注
fn when_to_annotate_demo() {
    println!("\n=== 2. 需要类型标注的情况 ===\n");
    
    // ❌ 情况1：编译器无法推断时
    // let x;  // 错误！没有初始值，无法推断
    
    // ✅ 解决方案：明确声明类型
    let x: i32;
    x = 5;
    println!("1️⃣ 延迟初始化: x = {}", x);
    
    // ❌ 情况2：有多种可能的类型
    // let numbers = [1, 2, 3].iter().collect();  // 错误！collect 可以收集到多种类型
    
    // ✅ 解决方案：指定具体类型
    let numbers: Vec<i32> = [1, 2, 3].iter().copied().collect();
    println!("\n2️⃣ 歧义类型: numbers = {:?}", numbers);
    
    // ❌ 情况3：泛型容器需要类型参数
    // let v = Vec::new();  // 如果不使用，编译器无法推断 T
    
    // ✅ 解决方案：声明类型
    let _v: Vec<String> = Vec::new();
    println!("\n3️⃣ 空容器: Vec<String> (空)");
    
    // ❌ 情况4：parse 等方法有多种返回类型
    let s = "42";
    // let num = s.parse().unwrap();  // 错误！parse 可以解析为多种类型
    
    // ✅ 解决方案A：标注变量类型
    let num: i32 = s.parse().unwrap();
    println!("\n4️⃣ 解析字符串 (变量标注): num = {}", num);
    
    // ✅ 解决方案B：使用 turbofish 语法
    let num = s.parse::<i32>().unwrap();
    println!("   解析字符串 (turbofish):  num = {}", num);
    
    // ❌ 情况5：多个 trait 实现
    let data = "5.5";
    let f: f64 = data.parse().unwrap();  // 需要指定是 f64 还是 f32
    println!("\n5️⃣ 多个实现: f = {} (f64)", f);
}

/// # 泛型类型声明
fn generic_type_declaration_demo() {
    println!("\n=== 3. 泛型类型声明 ===\n");
    
    // 结构体泛型声明
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // ✅ 使用时指定类型
    let int_point: Point<i32> = Point { x: 5, y: 10 };
    let float_point: Point<f64> = Point { x: 1.0, y: 4.0 };
    
    println!("1️⃣ 结构体泛型:");
    println!("   Point<i32>: ({}, {})", int_point.x, int_point.y);
    println!("   Point<f64>: ({}, {})", float_point.x, float_point.y);
    
    // 多个泛型参数
    struct Pair<T, U> {
        first: T,
        second: U,
    }
    
    let pair: Pair<i32, String> = Pair {
        first: 42,
        second: String::from("hello"),
    };
    
    println!("\n2️⃣ 多个泛型参数:");
    println!("   Pair<i32, String>: ({}, {})", pair.first, pair.second);
    
    // 枚举泛型声明
    enum MyOption<T> {
        Some(T),
        None,
    }
    
    let some_num: MyOption<i32> = MyOption::Some(5);
    let _no_val: MyOption<i32> = MyOption::None;
    
    match some_num {
        MyOption::Some(x) => println!("\n3️⃣ 枚举泛型: Some({})", x),
        MyOption::None => println!("None"),
    }
    
    // 函数泛型声明
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    let numbers = vec![34, 50, 25, 100, 65];
    println!("\n4️⃣ 函数泛型: 最大值 = {}", largest(&numbers));
}

/// # Turbofish 语法 ::<T>
fn turbofish_syntax_demo() {
    println!("\n=== 4. Turbofish 语法 ::<T> ===\n");
    
    println!("Turbofish 用于在方法调用时显式指定泛型类型参数\n");
    
    // 示例1：collect 方法
    let v1 = vec![1, 2, 3];
    
    // ✅ 方式A：变量类型标注
    let v2: Vec<i32> = v1.iter().copied().collect();
    println!("1️⃣ 变量标注: {:?}", v2);
    
    // ✅ 方式B：Turbofish
    let v3 = v1.iter().copied().collect::<Vec<i32>>();
    println!("   Turbofish:  {:?}", v3);
    
    // 示例2：parse 方法
    let s = "42";
    
    // ✅ 方式A：变量类型标注
    let n: i32 = s.parse().unwrap();
    println!("\n2️⃣ 变量标注: {}", n);
    
    // ✅ 方式B：Turbofish
    let n = s.parse::<i32>().unwrap();
    println!("   Turbofish:  {}", n);
    
    // 示例3：from_iter
    let v = vec![1, 2, 3];
    let set = std::collections::HashSet::<i32>::from_iter(v.iter().copied());
    println!("\n3️⃣ from_iter: {:?}", set);
    
    // 示例4：default
    let value = i32::default();
    println!("\n4️⃣ default: {}", value);
    
    // 示例5：复杂情况
    let numbers: Vec<String> = vec!["1", "2", "3"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    println!("\n5️⃣ 复杂情况: {:?}", numbers);
}

/// # 复杂泛型示例
fn complex_generic_demo() {
    println!("\n=== 5. 复杂泛型示例 ===\n");
    
    // 1. 带生命周期的泛型
    struct Wrapper<'a, T> {
        value: &'a T,
    }
    
    let num = 42;
    let wrapper: Wrapper<i32> = Wrapper { value: &num };
    println!("1️⃣ 带生命周期: Wrapper {{ value: {} }}", wrapper.value);
    
    // 2. 带 trait 约束的泛型
    fn print_debug<T: std::fmt::Debug>(value: T) {
        println!("   Debug: {:?}", value);
    }
    
    print_debug(vec![1, 2, 3]);
    print_debug("hello");
    
    // 3. 多个 trait 约束
    fn compare<T>(a: T, b: T) -> bool
    where
        T: PartialEq + std::fmt::Debug,
    {
        println!("\n2️⃣ 比较 {:?} 和 {:?}", a, b);
        a == b
    }
    
    println!("   结果: {}", compare(5, 5));
    
    // 4. 关联类型
    use std::ops::Add;
    
    fn add_values<T: Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    
    println!("\n3️⃣ 关联类型:");
    println!("   5 + 3 = {}", add_values(5, 3));
    println!("   1.5 + 2.5 = {}", add_values(1.5, 2.5));
    
    // 5. 嵌套泛型
    let nested: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
    println!("\n4️⃣ 嵌套泛型: {:?}", nested);
    
    let map: std::collections::HashMap<String, Vec<i32>> = 
        std::collections::HashMap::new();
    println!("   HashMap<String, Vec<i32>>: {:?}", map);
}

/// # 实战示例
fn practical_examples_demo() {
    println!("\n=== 6. 实战示例 ===\n");
    
    // 示例1：Option 链式调用
    println!("1️⃣ Option 处理:");
    let maybe_num: Option<i32> = Some(5);
    let doubled = maybe_num
        .map(|x| x * 2)
        .filter(|x| x > &5)
        .unwrap_or(0);
    println!("   结果: {}", doubled);
    
    // 示例2：Result 错误处理
    println!("\n2️⃣ Result 处理:");
    let result: Result<i32, String> = Ok(42);
    match result {
        Ok(val) => println!("   成功: {}", val),
        Err(e) => println!("   错误: {}", e),
    }
    
    // 示例3：迭代器转换
    println!("\n3️⃣ 迭代器转换:");
    let strings = vec!["1", "2", "3"];
    let numbers: Vec<i32> = strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("   {:?}", numbers);
    
    // 示例4：自定义泛型容器
    println!("\n4️⃣ 自定义泛型容器:");
    struct Container<T> {
        items: Vec<T>,
    }
    
    impl<T> Container<T> {
        fn new() -> Self {
            Container { items: Vec::new() }
        }
        
        fn add(&mut self, item: T) {
            self.items.push(item);
        }
        
        fn count(&self) -> usize {
            self.items.len()
        }
    }
    
    let mut int_container: Container<i32> = Container::new();
    int_container.add(1);
    int_container.add(2);
    println!("   Container<i32>: {} 个元素", int_container.count());
    
    let mut string_container: Container<String> = Container::new();
    string_container.add(String::from("hello"));
    println!("   Container<String>: {} 个元素", string_container.count());
    
    // 示例5：Box、Rc、Arc
    use std::rc::Rc;
    use std::sync::Arc;
    
    println!("\n5️⃣ 智能指针:");
    let boxed: Box<i32> = Box::new(5);
    println!("   Box<i32>: {}", boxed);
    
    let rc: Rc<String> = Rc::new(String::from("shared"));
    let rc2 = Rc::clone(&rc);
    println!("   Rc<String>: {} (引用计数: {})", rc, Rc::strong_count(&rc));
    drop(rc2);
    
    let arc: Arc<i32> = Arc::new(42);
    println!("   Arc<i32>: {}", arc);
}
