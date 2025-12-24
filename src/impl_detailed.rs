// impl 详解 - Rust 方法实现全面指南
//
// 本教程涵盖：
// 1. impl 基础语法
// 2. 方法 vs 关联函数
// 3. self / &self / &mut self 详解
// 4. 多个 impl 块
// 5. 泛型实现
// 6. trait 实现
// 7. 实战案例

fn main() {
    println!("=== Rust impl 详解 ===\n");
    
    // 1. impl 基础
    demo_impl_basics();
    
    // 2. 方法的三种 self
    demo_three_kinds_of_self();
    
    // 3. 关联函数（构造函数）
    demo_associated_functions();
    
    // 4. 方法链（Builder 模式）
    demo_method_chaining();
    
    // 5. 多个 impl 块
    demo_multiple_impl_blocks();
    
    // 6. 泛型实现
    demo_generic_impl();
    
    // 7. trait 实现
    demo_trait_impl();
    
    // 8. 实战案例
    demo_real_world_examples();
}

// ============================================
// 1. impl 基础
// ============================================
fn demo_impl_basics() {
    println!("--- 1. impl 基础 ---\n");
    
    println!("什么是 impl？");
    println!("  - impl = implementation (实现)");
    println!("  - 为 struct/enum 添加方法和关联函数");
    println!("  - Rust 的方法定义方式\n");
    
    // 基础示例
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // 方法（需要 self）
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        // 关联函数（不需要 self，类似静态方法）
        fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }
    }
    
    let rect = Rectangle::new(30, 50);
    println!("矩形: {}x{}", rect.width, rect.height);
    println!("面积: {}", rect.area());
    println!();
    
    println!("语法结构:");
    println!("```rust");
    println!("impl StructName {{");
    println!("    // 方法（需要 self）");
    println!("    fn method_name(&self) {{");
    println!("        // ...");
    println!("    }}");
    println!("    ");
    println!("    // 关联函数（不需要 self）");
    println!("    fn associated_function() {{");
    println!("        // ...");
    println!("    }}");
    println!("}}");
    println!("```\n");
}

// ============================================
// 2. 方法的三种 self
// ============================================
fn demo_three_kinds_of_self() {
    println!("--- 2. 方法的三种 self ---\n");
    
    #[derive(Debug, Clone)]
    struct Counter {
        count: i32,
    }
    
    impl Counter {
        // 1. &self - 不可变借用（最常用）
        fn get(&self) -> i32 {
            self.count
        }
        
        // 2. &mut self - 可变借用（需要修改）
        fn increment(&mut self) {
            self.count += 1;
        }
        
        // 3. self - 获取所有权（消费）
        fn consume(self) -> i32 {
            println!("  消费 Counter，返回最终值");
            self.count
        }
        
        fn new(count: i32) -> Self {
            Self { count }
        }
    }
    
    println!("1. &self - 不可变借用");
    let counter = Counter::new(0);
    println!("   值: {}", counter.get());
    println!("   特点: 只读访问，不修改数据");
    println!("   使用: 获取器(getter)、计算、查询\n");
    
    println!("2. &mut self - 可变借用");
    let mut counter = Counter::new(0);
    counter.increment();
    counter.increment();
    println!("   增加后: {}", counter.get());
    println!("   特点: 可以修改数据");
    println!("   使用: 设置器(setter)、修改操作\n");
    
    println!("3. self - 获取所有权");
    let counter = Counter::new(5);
    let final_value = counter.consume();
    println!("   最终值: {}", final_value);
    // println!("   错误: {:?}", counter); // 编译错误！counter 已被消费
    println!("   特点: 消费对象，转移所有权");
    println!("   使用: 转换、构建器最终调用\n");
    
    println!("对比表格:");
    println!("┌─────────────┬──────────┬──────────┬─────────────┐");
    println!("│  方法签名    │  借用     │  修改    │  使用后状态  │");
    println!("├─────────────┼──────────┼──────────┼─────────────┤");
    println!("│  &self      │  不可变   │  否      │  可继续使用  │");
    println!("│  &mut self  │  可变     │  是      │  可继续使用  │");
    println!("│  self       │  所有权   │  是      │  被消费      │");
    println!("└─────────────┴──────────┴──────────┴─────────────┘");
    println!();
}

// ============================================
// 3. 关联函数（构造函数）
// ============================================
fn demo_associated_functions() {
    println!("--- 3. 关联函数（构造函数） ---\n");
    
    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        email: String,
    }
    
    impl User {
        // 标准构造函数
        fn new(id: u32, name: String, email: String) -> Self {
            Self { id, name, email }
        }
        
        // 带默认值的构造函数
        fn default_user() -> Self {
            Self {
                id: 0,
                name: String::from("匿名"),
                email: String::from("unknown@example.com"),
            }
        }
        
        // 从其他类型构造
        fn from_name(name: String) -> Self {
            Self {
                id: 0,
                name,
                email: String::new(),
            }
        }
        
        // 验证构造函数
        fn new_validated(name: String, email: String) -> Result<Self, String> {
            if name.is_empty() {
                return Err("名字不能为空".to_string());
            }
            if !email.contains('@') {
                return Err("邮箱格式无效".to_string());
            }
            Ok(Self {
                id: 0,
                name,
                email,
            })
        }
    }
    
    println!("1. 标准构造函数:");
    let user1 = User::new(1, "张三".to_string(), "zhangsan@example.com".to_string());
    println!("   {:?}\n", user1);
    
    println!("2. 默认构造函数:");
    let user2 = User::default_user();
    println!("   {:?}\n", user2);
    
    println!("3. 专门的构造函数:");
    let user3 = User::from_name("李四".to_string());
    println!("   {:?}\n", user3);
    
    println!("4. 验证构造函数:");
    match User::new_validated("王五".to_string(), "wangwu@example.com".to_string()) {
        Ok(user) => println!("   成功: {:?}", user),
        Err(e) => println!("   失败: {}", e),
    }
    
    match User::new_validated("".to_string(), "invalid".to_string()) {
        Ok(user) => println!("   成功: {:?}", user),
        Err(e) => println!("   失败: {}", e),
    }
    println!();
    
    println!("调用方式:");
    println!("  - 方法: instance.method()");
    println!("  - 关联函数: StructName::function()");
    println!();
}

// ============================================
// 4. 方法链（Builder 模式）
// ============================================
fn demo_method_chaining() {
    println!("--- 4. 方法链（Builder 模式） ---\n");
    
    #[derive(Debug)]
    struct Pizza {
        size: String,
        cheese: bool,
        pepperoni: bool,
        mushrooms: bool,
    }
    
    impl Pizza {
        fn new() -> Self {
            Self {
                size: "中".to_string(),
                cheese: true,
                pepperoni: false,
                mushrooms: false,
            }
        }
        
        // 返回 self，支持链式调用
        fn size(mut self, size: &str) -> Self {
            self.size = size.to_string();
            self
        }
        
        fn add_pepperoni(mut self) -> Self {
            self.pepperoni = true;
            self
        }
        
        fn add_mushrooms(mut self) -> Self {
            self.mushrooms = true;
            self
        }
        
        fn build(self) -> Self {
            self
        }
    }
    
    println!("Builder 模式示例:");
    let pizza = Pizza::new()
        .size("大")
        .add_pepperoni()
        .add_mushrooms()
        .build();
    
    println!("{:#?}", pizza);
    println!();
    
    println!("关键点:");
    println!("  - 方法接受 self (获取所有权)");
    println!("  - 返回 Self (返回修改后的对象)");
    println!("  - 支持链式调用");
    println!();
}

// ============================================
// 5. 多个 impl 块
// ============================================
fn demo_multiple_impl_blocks() {
    println!("--- 5. 多个 impl 块 ---\n");
    
    struct Calculator {
        value: f64,
    }
    
    // 第一个 impl 块 - 基础操作
    impl Calculator {
        fn new(value: f64) -> Self {
            Self { value }
        }
        
        fn add(&mut self, n: f64) {
            self.value += n;
        }
        
        fn subtract(&mut self, n: f64) {
            self.value -= n;
        }
    }
    
    // 第二个 impl 块 - 高级操作
    impl Calculator {
        fn multiply(&mut self, n: f64) {
            self.value *= n;
        }
        
        fn divide(&mut self, n: f64) {
            if n != 0.0 {
                self.value /= n;
            }
        }
        
        fn result(&self) -> f64 {
            self.value
        }
    }
    
    // 第三个 impl 块 - 实用方法
    impl Calculator {
        fn reset(&mut self) {
            self.value = 0.0;
        }
        
        fn is_zero(&self) -> bool {
            self.value == 0.0
        }
    }
    
    let mut calc = Calculator::new(10.0);
    calc.add(5.0);
    calc.multiply(2.0);
    println!("计算结果: {}", calc.result());
    println!();
    
    println!("为什么使用多个 impl 块？");
    println!("  ✓ 组织代码（按功能分组）");
    println!("  ✓ 条件编译（#[cfg(...)]）");
    println!("  ✓ 不同的泛型约束");
    println!("  ✓ 可读性和维护性");
    println!();
}

// ============================================
// 6. 泛型实现
// ============================================
fn demo_generic_impl() {
    println!("--- 6. 泛型实现 ---\n");
    
    // 泛型 struct
    #[derive(Debug)]
    struct Box<T> {
        value: T,
    }
    
    // 为所有类型实现
    impl<T> Box<T> {
        fn new(value: T) -> Self {
            Self { value }
        }
        
        fn get(&self) -> &T {
            &self.value
        }
        
        fn set(&mut self, value: T) {
            self.value = value;
        }
    }
    
    // 仅为特定类型实现
    impl Box<i32> {
        fn double(&mut self) {
            self.value *= 2;
        }
    }
    
    // 带约束的泛型实现
    impl<T: std::fmt::Display> Box<T> {
        fn print(&self) {
            println!("  Box 包含: {}", self.value);
        }
    }
    
    println!("1. 泛型实现（所有类型）:");
    let mut int_box = Box::new(42);
    println!("   整数 Box: {:?}", int_box);
    
    let str_box = Box::new("Hello");
    println!("   字符串 Box: {:?}", str_box);
    println!();
    
    println!("2. 特定类型实现:");
    int_box.double();
    println!("   Double 后: {:?}", int_box);
    // str_box.double(); // 编译错误！只有 Box<i32> 有这个方法
    println!();
    
    println!("3. 带约束的实现:");
    int_box.print();
    str_box.print();
    println!();
}

// ============================================
// 7. trait 实现
// ============================================
fn demo_trait_impl() {
    println!("--- 7. Trait 实现 ---\n");
    
    // 定义 trait
    trait Describable {
        fn describe(&self) -> String;
    }
    
    trait Drawable {
        fn draw(&self);
    }
    
    #[derive(Debug)]
    struct Circle {
        radius: f64,
    }
    
    // 为 Circle 实现自己的方法
    impl Circle {
        fn new(radius: f64) -> Self {
            Self { radius }
        }
        
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    
    // 为 Circle 实现 Describable trait
    impl Describable for Circle {
        fn describe(&self) -> String {
            format!("圆形，半径 {}", self.radius)
        }
    }
    
    // 为 Circle 实现 Drawable trait
    impl Drawable for Circle {
        fn draw(&self) {
            println!("  绘制圆形 (半径: {})", self.radius);
        }
    }
    
    let circle = Circle::new(5.0);
    println!("1. 自己的方法:");
    println!("   {:?}", circle);
    println!("   面积: {:.2}", circle.area());
    println!();
    
    println!("2. Trait 方法:");
    println!("   {}", circle.describe());
    circle.draw();
    println!();
    
    println!("3. 标准库 trait:");
    
    // 实现 Display trait
    use std::fmt;
    
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    
    let point = Point { x: 10, y: 20 };
    println!("   Point: {}", point);
    println!();
}

// ============================================
// 8. 实战案例
// ============================================
fn demo_real_world_examples() {
    println!("--- 8. 实战案例 ---\n");
    
    // 案例 1: 银行账户
    println!("案例 1: 银行账户系统\n");
    bank_account_example();
    
    // 案例 2: 任务管理
    println!("\n案例 2: 任务管理器\n");
    task_manager_example();
    
    // 案例 3: HTTP 客户端
    println!("\n案例 3: HTTP 客户端（简化版）\n");
    http_client_example();
}

// 案例 1: 银行账户
fn bank_account_example() {
    #[derive(Debug)]
    struct BankAccount {
        account_number: String,
        balance: f64,
        owner: String,
    }
    
    impl BankAccount {
        // 构造函数
        fn new(account_number: String, owner: String) -> Self {
            Self {
                account_number,
                balance: 0.0,
                owner,
            }
        }
        
        // 查询余额
        fn balance(&self) -> f64 {
            self.balance
        }
        
        // 存款
        fn deposit(&mut self, amount: f64) -> Result<(), String> {
            if amount <= 0.0 {
                return Err("存款金额必须大于 0".to_string());
            }
            self.balance += amount;
            Ok(())
        }
        
        // 取款
        fn withdraw(&mut self, amount: f64) -> Result<(), String> {
            if amount <= 0.0 {
                return Err("取款金额必须大于 0".to_string());
            }
            if amount > self.balance {
                return Err("余额不足".to_string());
            }
            self.balance -= amount;
            Ok(())
        }
        
        // 转账
        fn transfer(&mut self, to: &mut BankAccount, amount: f64) -> Result<(), String> {
            self.withdraw(amount)?;
            to.deposit(amount)?;
            Ok(())
        }
        
        // 账户信息
        fn info(&self) -> String {
            format!(
                "账号: {}, 户主: {}, 余额: ¥{:.2}",
                self.account_number, self.owner, self.balance
            )
        }
    }
    
    let mut account1 = BankAccount::new("001".to_string(), "张三".to_string());
    let mut account2 = BankAccount::new("002".to_string(), "李四".to_string());
    
    account1.deposit(1000.0).unwrap();
    println!("  存款后: {}", account1.info());
    
    account1.withdraw(200.0).unwrap();
    println!("  取款后: {}", account1.info());
    
    account1.transfer(&mut account2, 300.0).unwrap();
    println!("  转账后:");
    println!("    {}", account1.info());
    println!("    {}", account2.info());
}

// 案例 2: 任务管理
fn task_manager_example() {
    #[derive(Debug, Clone, PartialEq)]
    enum TaskStatus {
        Todo,
        InProgress,
        Done,
    }
    
    #[derive(Debug)]
    struct Task {
        id: u32,
        title: String,
        description: String,
        status: TaskStatus,
    }
    
    impl Task {
        fn new(id: u32, title: String, description: String) -> Self {
            Self {
                id,
                title,
                description,
                status: TaskStatus::Todo,
            }
        }
        
        fn start(&mut self) {
            self.status = TaskStatus::InProgress;
        }
        
        fn complete(&mut self) {
            self.status = TaskStatus::Done;
        }
        
        fn is_done(&self) -> bool {
            self.status == TaskStatus::Done
        }
        
        fn summary(&self) -> String {
            format!("[{}] {} - {:?}", self.id, self.title, self.status)
        }
    }
    
    struct TaskManager {
        tasks: Vec<Task>,
        next_id: u32,
    }
    
    impl TaskManager {
        fn new() -> Self {
            Self {
                tasks: Vec::new(),
                next_id: 1,
            }
        }
        
        fn add_task(&mut self, title: String, description: String) -> &Task {
            let task = Task::new(self.next_id, title, description);
            self.next_id += 1;
            self.tasks.push(task);
            self.tasks.last().unwrap()
        }
        
        fn start_task(&mut self, id: u32) -> Option<()> {
            self.tasks.iter_mut()
                .find(|t| t.id == id)?
                .start();
            Some(())
        }
        
        fn complete_task(&mut self, id: u32) -> Option<()> {
            self.tasks.iter_mut()
                .find(|t| t.id == id)?
                .complete();
            Some(())
        }
        
        fn list_tasks(&self) {
            for task in &self.tasks {
                println!("    {}", task.summary());
            }
        }
        
        fn pending_count(&self) -> usize {
            self.tasks.iter()
                .filter(|t| !t.is_done())
                .count()
        }
    }
    
    let mut manager = TaskManager::new();
    
    manager.add_task("学习 Rust".to_string(), "完成 impl 教程".to_string());
    manager.add_task("写代码".to_string(), "实现示例项目".to_string());
    manager.add_task("测试".to_string(), "运行所有测试".to_string());
    
    println!("  所有任务:");
    manager.list_tasks();
    
    manager.start_task(1);
    manager.complete_task(1);
    
    println!("\n  更新后:");
    manager.list_tasks();
    println!("  待完成: {}", manager.pending_count());
}

// 案例 3: HTTP 客户端
fn http_client_example() {
    struct HttpClient {
        base_url: String,
        timeout: u64,
        headers: Vec<(String, String)>,
    }
    
    impl HttpClient {
        fn new(base_url: &str) -> Self {
            Self {
                base_url: base_url.to_string(),
                timeout: 30,
                headers: Vec::new(),
            }
        }
        
        fn timeout(mut self, seconds: u64) -> Self {
            self.timeout = seconds;
            self
        }
        
        fn header(mut self, key: &str, value: &str) -> Self {
            self.headers.push((key.to_string(), value.to_string()));
            self
        }
        
        fn get(&self, path: &str) -> String {
            format!("GET {}{} (timeout: {}s)", self.base_url, path, self.timeout)
        }
        
        fn post(&self, path: &str, body: &str) -> String {
            format!("POST {}{} with body: {}", self.base_url, path, body)
        }
    }
    
    let client = HttpClient::new("https://api.example.com")
        .timeout(10)
        .header("User-Agent", "MyApp/1.0")
        .header("Accept", "application/json");
    
    println!("  {}", client.get("/users"));
    println!("  {}", client.post("/users", r#"{"name": "张三"}"#));
}

/*
=== 总结 ===

1. impl 的三种主要用法:
   ✅ 为 struct/enum 实现方法
   ✅ 为 struct/enum 实现 trait
   ✅ 泛型实现

2. 方法的 self 类型:
   ✅ &self      - 不可变借用（最常用）
   ✅ &mut self  - 可变借用（修改数据）
   ✅ self       - 获取所有权（转换/消费）

3. 关联函数:
   ✅ 不需要 self
   ✅ 使用 StructName::function() 调用
   ✅ 常用于构造函数

4. 设计模式:
   ✅ Builder 模式 - 链式调用
   ✅ Constructor 模式 - 多种构造方式
   ✅ Facade 模式 - 简化接口

5. 最佳实践:
   ✅ new() 作为主要构造函数
   ✅ getter 用 &self
   ✅ setter 用 &mut self
   ✅ 转换用 self
   ✅ 按功能组织多个 impl 块

6. 常见错误:
   ❌ 忘记 self 参数（变成关联函数）
   ❌ 用错 self 类型（借用 vs 所有权）
   ❌ 方法签名不匹配 trait 要求

运行示例:
  cargo run --bin impl_detailed
*/
