// Rust Trait 详解 - 完全指南
//
// Trait 是 Rust 中定义共享行为的方式，类似其他语言的接口
// 本教程涵盖从基础到高级的所有内容

use std::fmt;

fn main() {
    println!("=== Rust Trait 详解 ===\n");
    
    // 1. Trait 基础
    demo_trait_basics();
    
    // 2. 默认实现
    demo_default_implementations();
    
    // 3. Trait 作为参数
    demo_trait_as_parameters();
    
    // 4. Trait Bounds
    demo_trait_bounds();
    
    // 5. 多个 Trait
    demo_multiple_traits();
    
    // 6. 关联类型
    demo_associated_types();
    
    // 7. Trait 对象（动态分发）
    demo_trait_objects();
    
    // 8. 标准库 Trait
    demo_standard_traits();
    
    // 9. 运算符重载
    demo_operator_overloading();
    
    // 10. 实战案例
    demo_real_world_examples();
}

// ============================================
// 1. Trait 基础
// ============================================
fn demo_trait_basics() {
    println!("--- 1. Trait 基础 ---\n");
    
    println!("什么是 Trait？");
    println!("  - 定义共享行为的方式");
    println!("  - 类似接口（Interface）");
    println!("  - 实现多态");
    println!();
    
    // 定义 trait
    trait Describable {
        fn describe(&self) -> String;
    }
    
    // 为类型实现 trait
    struct Person {
        name: String,
        age: u32,
    }
    
    impl Describable for Person {
        fn describe(&self) -> String {
            format!("{}, {} 岁", self.name, self.age)
        }
    }
    
    struct Book {
        title: String,
        author: String,
    }
    
    impl Describable for Book {
        fn describe(&self) -> String {
            format!("《{}》 作者: {}", self.title, self.author)
        }
    }
    
    let person = Person {
        name: "张三".to_string(),
        age: 25,
    };
    
    let book = Book {
        title: "Rust 编程".to_string(),
        author: "李四".to_string(),
    };
    
    println!("Person: {}", person.describe());
    println!("Book: {}", book.describe());
    println!();
    
    println!("语法:");
    println!("  // 定义 trait");
    println!("  trait TraitName {{");
    println!("      fn method_name(&self) -> ReturnType;");
    println!("  }}");
    println!();
    println!("  // 实现 trait");
    println!("  impl TraitName for TypeName {{");
    println!("      fn method_name(&self) -> ReturnType {{");
    println!("          // 实现");
    println!("      }}");
    println!("  }}");
    println!();
}

// ============================================
// 2. 默认实现
// ============================================
fn demo_default_implementations() {
    println!("--- 2. 默认实现 ---\n");
    
    trait Greeter {
        // 必须实现的方法
        fn name(&self) -> &str;
        
        // 有默认实现的方法
        fn greet(&self) -> String {
            format!("你好, {}!", self.name())
        }
        
        // 另一个默认实现，调用其他方法
        fn formal_greet(&self) -> String {
            format!("尊敬的 {}，您好！", self.name())
        }
    }
    
    struct User {
        username: String,
    }
    
    impl Greeter for User {
        fn name(&self) -> &str {
            &self.username
        }
        // greet() 使用默认实现
        // formal_greet() 使用默认实现
    }
    
    struct Admin {
        username: String,
    }
    
    impl Greeter for Admin {
        fn name(&self) -> &str {
            &self.username
        }
        
        // 覆盖默认实现
        fn greet(&self) -> String {
            format!("管理员 {} 您好！", self.name())
        }
    }
    
    let user = User {
        username: "张三".to_string(),
    };
    
    let admin = Admin {
        username: "李四".to_string(),
    };
    
    println!("User:");
    println!("  {}", user.greet());
    println!("  {}", user.formal_greet());
    println!();
    
    println!("Admin (覆盖了 greet):");
    println!("  {}", admin.greet());
    println!("  {}", admin.formal_greet());
    println!();
}

// ============================================
// 3. Trait 作为参数
// ============================================
fn demo_trait_as_parameters() {
    println!("--- 3. Trait 作为参数 ---\n");
    
    trait Printable {
        fn print(&self);
    }
    
    struct Document {
        content: String,
    }
    
    impl Printable for Document {
        fn print(&self) {
            println!("文档内容: {}", self.content);
        }
    }
    
    struct Image {
        path: String,
    }
    
    impl Printable for Image {
        fn print(&self) {
            println!("图片路径: {}", self.path);
        }
    }
    
    // 方式 1: impl Trait 语法（推荐）
    fn print_item(item: &impl Printable) {
        item.print();
    }
    
    // 方式 2: Trait Bound 语法
    fn print_item_verbose<T: Printable>(item: &T) {
        item.print();
    }
    
    // 方式 3: where 子句（复杂约束时使用）
    fn print_item_where<T>(item: &T)
    where
        T: Printable,
    {
        item.print();
    }
    
    let doc = Document {
        content: "Hello, World!".to_string(),
    };
    
    let img = Image {
        path: "/path/to/image.png".to_string(),
    };
    
    println!("使用不同的函数签名:");
    print_item(&doc);
    print_item_verbose(&img);
    print_item_where(&doc);
    println!();
}

// ============================================
// 4. Trait Bounds（约束）
// ============================================
fn demo_trait_bounds() {
    println!("--- 4. Trait Bounds ---\n");
    
    // 单个约束
    fn print_debug<T: fmt::Debug>(item: &T) {
        println!("  Debug: {:?}", item);
    }
    
    // 多个约束
    fn process<T: fmt::Debug + Clone>(item: &T) -> T {
        println!("  Processing: {:?}", item);
        item.clone()
    }
    
    // where 子句（更清晰）
    fn complex_function<T, U>(t: &T, u: &U) -> String
    where
        T: fmt::Debug + Clone,
        U: fmt::Display + Clone,
    {
        format!("T: {:?}, U: {}", t, u)
    }
    
    #[derive(Debug, Clone)]
    struct Data {
        value: i32,
    }
    
    let data = Data { value: 42 };
    
    println!("单个约束:");
    print_debug(&data);
    println!();
    
    println!("多个约束:");
    let cloned = process(&data);
    println!("  Cloned value: {}", cloned.value);
    println!();
    
    println!("Where 子句:");
    let result = complex_function(&data, &"Hello");
    println!("  Result: {}", result);
    println!();
}

// ============================================
// 5. 多个 Trait
// ============================================
fn demo_multiple_traits() {
    println!("--- 5. 多个 Trait ---\n");
    
    trait Drawable {
        fn draw(&self);
    }
    
    trait Clickable {
        fn click(&self);
    }
    
    struct Button {
        label: String,
    }
    
    impl Drawable for Button {
        fn draw(&self) {
            println!("  绘制按钮: {}", self.label);
        }
    }
    
    impl Clickable for Button {
        fn click(&self) {
            println!("  点击按钮: {}", self.label);
        }
    }
    
    // 要求实现多个 trait
    fn interact<T: Drawable + Clickable>(item: &T) {
        item.draw();
        item.click();
    }
    
    let button = Button {
        label: "提交".to_string(),
    };
    
    println!("交互:");
    interact(&button);
    println!();
    
    // Trait 继承
    trait Widget: Drawable + Clickable {
        fn get_id(&self) -> u32;
    }
    
    impl Widget for Button {
        fn get_id(&self) -> u32 {
            1
        }
    }
    
    println!("Widget ID: {}", button.get_id());
    println!();
}

// ============================================
// 6. 关联类型
// ============================================
fn demo_associated_types() {
    println!("--- 6. 关联类型 ---\n");
    
    // 使用关联类型
    trait Container {
        type Item;
        
        fn add(&mut self, item: Self::Item);
        fn get(&self, index: usize) -> Option<&Self::Item>;
        fn len(&self) -> usize;
    }
    
    struct VecContainer<T> {
        items: Vec<T>,
    }
    
    impl<T> Container for VecContainer<T> {
        type Item = T;
        
        fn add(&mut self, item: Self::Item) {
            self.items.push(item);
        }
        
        fn get(&self, index: usize) -> Option<&Self::Item> {
            self.items.get(index)
        }
        
        fn len(&self) -> usize {
            self.items.len()
        }
    }
    
    let mut container = VecContainer { items: Vec::new() };
    container.add(1);
    container.add(2);
    container.add(3);
    
    println!("容器长度: {}", container.len());
    println!("第 1 项: {:?}", container.get(0));
    println!();
    
    // 关联类型 vs 泛型参数
    println!("关联类型 vs 泛型:");
    println!("  关联类型: 每个类型只能有一个实现");
    println!("  泛型参数: 同一类型可以有多个实现");
    println!();
}

// ============================================
// 7. Trait 对象（动态分发）
// ============================================
fn demo_trait_objects() {
    println!("--- 7. Trait 对象（动态分发） ---\n");
    
    trait Shape {
        fn area(&self) -> f64;
        fn name(&self) -> &str;
    }
    
    struct Circle {
        radius: f64,
    }
    
    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
        
        fn name(&self) -> &str {
            "圆形"
        }
    }
    
    struct Rectangle {
        width: f64,
        height: f64,
    }
    
    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
        
        fn name(&self) -> &str {
            "矩形"
        }
    }
    
    // Trait 对象：使用 dyn 关键字
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 10.0,
            height: 20.0,
        }),
        Box::new(Circle { radius: 3.0 }),
    ];
    
    println!("形状列表:");
    for shape in &shapes {
        println!("  {}: 面积 = {:.2}", shape.name(), shape.area());
    }
    println!();
    
    println!("静态分发 vs 动态分发:");
    println!("  静态分发 (泛型): 编译时确定，性能高");
    println!("  动态分发 (trait 对象): 运行时确定，更灵活");
    println!();
}

// ============================================
// 8. 标准库 Trait
// ============================================
fn demo_standard_traits() {
    println!("--- 8. 标准库 Trait ---\n");
    
    // Clone trait
    #[derive(Clone, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    println!("Clone: {:?} -> {:?}", p1, p2);
    println!();
    
    // Display trait
    struct Product {
        name: String,
        price: f64,
    }
    
    impl fmt::Display for Product {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} (¥{:.2})", self.name, self.price)
        }
    }
    
    let product = Product {
        name: "键盘".to_string(),
        price: 299.99,
    };
    
    println!("Display: {}", product);
    println!();
    
    // From/Into trait
    struct Celsius(f64);
    struct Fahrenheit(f64);
    
    impl From<Celsius> for Fahrenheit {
        fn from(c: Celsius) -> Self {
            Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
        }
    }
    
    let c = Celsius(25.0);
    let f: Fahrenheit = c.into(); // Into 自动实现
    println!("From/Into: 25°C = {:.1}°F", f.0);
    println!();
    
    // Iterator trait
    struct Counter {
        count: u32,
        max: u32,
    }
    
    impl Counter {
        fn new(max: u32) -> Self {
            Counter { count: 0, max }
        }
    }
    
    impl Iterator for Counter {
        type Item = u32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    
    print!("Iterator: ");
    for num in Counter::new(5) {
        print!("{} ", num);
    }
    println!("\n");
}

// ============================================
// 9. 运算符重载
// ============================================
fn demo_operator_overloading() {
    println!("--- 9. 运算符重载 ---\n");
    
    use std::ops::{Add, Sub, Mul};
    
    #[derive(Debug, Clone, Copy)]
    struct Vector2D {
        x: f64,
        y: f64,
    }
    
    // 实现加法
    impl Add for Vector2D {
        type Output = Vector2D;
        
        fn add(self, other: Vector2D) -> Vector2D {
            Vector2D {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    
    // 实现减法
    impl Sub for Vector2D {
        type Output = Vector2D;
        
        fn sub(self, other: Vector2D) -> Vector2D {
            Vector2D {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }
    
    // 实现数乘
    impl Mul<f64> for Vector2D {
        type Output = Vector2D;
        
        fn mul(self, scalar: f64) -> Vector2D {
            Vector2D {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }
    }
    
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };
    
    let v3 = v1 + v2;
    let v4 = v2 - v1;
    let v5 = v1 * 2.0;
    
    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
    println!("v1 + v2 = {:?}", v3);
    println!("v2 - v1 = {:?}", v4);
    println!("v1 * 2.0 = {:?}", v5);
    println!();
}

// ============================================
// 10. 实战案例
// ============================================
fn demo_real_world_examples() {
    println!("--- 10. 实战案例 ---\n");
    
    // 案例 1: 插件系统
    println!("案例 1: 插件系统\n");
    plugin_system_example();
    
    // 案例 2: 序列化
    println!("\n案例 2: 序列化系统\n");
    serialization_example();
    
    // 案例 3: 存储抽象
    println!("\n案例 3: 存储抽象层\n");
    storage_example();
}

// 案例 1: 插件系统
fn plugin_system_example() {
    trait Plugin {
        fn name(&self) -> &str;
        fn version(&self) -> &str;
        fn execute(&self, input: &str) -> String;
    }
    
    struct UppercasePlugin;
    
    impl Plugin for UppercasePlugin {
        fn name(&self) -> &str {
            "Uppercase"
        }
        
        fn version(&self) -> &str {
            "1.0.0"
        }
        
        fn execute(&self, input: &str) -> String {
            input.to_uppercase()
        }
    }
    
    struct ReversePlugin;
    
    impl Plugin for ReversePlugin {
        fn name(&self) -> &str {
            "Reverse"
        }
        
        fn version(&self) -> &str {
            "1.0.0"
        }
        
        fn execute(&self, input: &str) -> String {
            input.chars().rev().collect()
        }
    }
    
    struct PluginManager {
        plugins: Vec<Box<dyn Plugin>>,
    }
    
    impl PluginManager {
        fn new() -> Self {
            PluginManager {
                plugins: Vec::new(),
            }
        }
        
        fn register(&mut self, plugin: Box<dyn Plugin>) {
            println!("  注册插件: {} v{}", plugin.name(), plugin.version());
            self.plugins.push(plugin);
        }
        
        fn execute_all(&self, input: &str) -> Vec<String> {
            self.plugins
                .iter()
                .map(|p| {
                    let result = p.execute(input);
                    println!("  {} 处理: '{}' -> '{}'", p.name(), input, result);
                    result
                })
                .collect()
        }
    }
    
    let mut manager = PluginManager::new();
    manager.register(Box::new(UppercasePlugin));
    manager.register(Box::new(ReversePlugin));
    
    println!("\n执行插件:");
    let results = manager.execute_all("hello");
    println!("\n结果: {:?}", results);
}

// 案例 2: 序列化
fn serialization_example() {
    trait Serializable {
        fn serialize(&self) -> String;
        fn deserialize(data: &str) -> Result<Self, String>
        where
            Self: Sized;
    }
    
    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        email: String,
    }
    
    impl Serializable for User {
        fn serialize(&self) -> String {
            format!("{}|{}|{}", self.id, self.name, self.email)
        }
        
        fn deserialize(data: &str) -> Result<Self, String> {
            let parts: Vec<&str> = data.split('|').collect();
            if parts.len() != 3 {
                return Err("格式错误".to_string());
            }
            
            let id = parts[0]
                .parse()
                .map_err(|_| "ID 解析错误")?;
            
            Ok(User {
                id,
                name: parts[1].to_string(),
                email: parts[2].to_string(),
            })
        }
    }
    
    let user = User {
        id: 1,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
    };
    
    println!("  原始用户: {:?}", user);
    
    let serialized = user.serialize();
    println!("  序列化: {}", serialized);
    
    let deserialized = User::deserialize(&serialized).unwrap();
    println!("  反序列化: {:?}", deserialized);
}

// 案例 3: 存储抽象
fn storage_example() {
    trait Storage {
        fn save(&mut self, key: String, value: String) -> Result<(), String>;
        fn load(&self, key: &str) -> Result<Option<String>, String>;
        fn delete(&mut self, key: &str) -> Result<(), String>;
    }
    
    use std::collections::HashMap;
    
    struct MemoryStorage {
        data: HashMap<String, String>,
    }
    
    impl MemoryStorage {
        fn new() -> Self {
            MemoryStorage {
                data: HashMap::new(),
            }
        }
    }
    
    impl Storage for MemoryStorage {
        fn save(&mut self, key: String, value: String) -> Result<(), String> {
            self.data.insert(key, value);
            Ok(())
        }
        
        fn load(&self, key: &str) -> Result<Option<String>, String> {
            Ok(self.data.get(key).cloned())
        }
        
        fn delete(&mut self, key: &str) -> Result<(), String> {
            self.data.remove(key);
            Ok(())
        }
    }
    
    fn demo_storage<S: Storage>(storage: &mut S) {
        println!("  保存数据...");
        storage.save("user:1".to_string(), "Alice".to_string()).unwrap();
        storage.save("user:2".to_string(), "Bob".to_string()).unwrap();
        
        println!("  读取数据...");
        if let Ok(Some(value)) = storage.load("user:1") {
            println!("    user:1 = {}", value);
        }
        
        println!("  删除数据...");
        storage.delete("user:2").unwrap();
        
        println!("  验证删除...");
        if let Ok(None) = storage.load("user:2") {
            println!("    user:2 已删除");
        }
    }
    
    let mut storage = MemoryStorage::new();
    demo_storage(&mut storage);
}

/*
=== 总结 ===

1. Trait 定义:
   trait TraitName {
       fn method(&self) -> Type;
   }

2. Trait 实现:
   impl TraitName for TypeName {
       fn method(&self) -> Type {
           // 实现
       }
   }

3. Trait 作为参数:
   fn function(param: &impl Trait) { }
   fn function<T: Trait>(param: &T) { }

4. Trait Bounds:
   T: Trait1 + Trait2
   where T: Trait1, U: Trait2

5. Trait 对象:
   Box<dyn Trait>
   &dyn Trait

6. 关联类型:
   trait Container {
       type Item;
   }

7. 默认实现:
   trait Trait {
       fn method(&self) {
           // 默认实现
       }
   }

8. 常用标准 Trait:
   - Debug, Display
   - Clone, Copy
   - PartialEq, Eq
   - From, Into
   - Iterator
   - Add, Sub, Mul (运算符)

运行示例:
  cargo run --bin trait_detailed
*/
