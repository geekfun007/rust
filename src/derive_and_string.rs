// #[derive()] 和 String vs &str 详解
//
// 本教程详细讲解：
// 1. derive 宏的各种用法
// 2. 为什么在 struct 中使用 String 而不是 &str
// 3. 生命周期的影响
// 4. 实战案例

use std::fmt;

fn main() {
    println!("=== Derive 宏 和 String vs &str 详解 ===\n");
    
    // 1. derive 宏基础
    demo_derive_basics();
    
    // 2. 常用 derive 宏详解
    demo_common_derives();
    
    // 3. String vs &str 基础对比
    demo_string_vs_str();
    
    // 4. Struct 中为什么用 String
    demo_string_in_struct();
    
    // 5. 生命周期问题
    demo_lifetime_issues();
    
    // 6. 性能考虑
    demo_performance_considerations();
    
    // 7. 实战案例
    demo_real_world_examples();
    
    // 8. 最佳实践
    demo_best_practices();
}

// ============================================
// 1. derive 宏基础
// ============================================
fn demo_derive_basics() {
    println!("--- 1. derive 宏基础 ---\n");
    
    println!("什么是 derive？");
    println!("  - derive 是编译器自动生成代码的机制");
    println!("  - 为 struct/enum 自动实现常用 trait");
    println!("  - 减少样板代码\n");
    
    // 没有 derive 的 struct
    struct PersonNoDerive {
        name: String,
        age: u32,
    }
    
    let person = PersonNoDerive {
        name: "张三".to_string(),
        age: 25,
    };
    
    println!("没有 derive:");
    println!("  struct PersonNoDerive {{ name: String, age: u32 }}");
    println!("  问题:");
    println!("    - 无法打印: println!(\"{{:?}}\", person); // 错误！");
    println!("    - 无法克隆: let p2 = person.clone(); // 错误！");
    println!("    - 无法比较: person == person2 // 错误！");
    println!();
    
    // 使用 derive
    #[derive(Debug, Clone, PartialEq)]
    struct PersonWithDerive {
        name: String,
        age: u32,
    }
    
    let person1 = PersonWithDerive {
        name: "李四".to_string(),
        age: 30,
    };
    
    println!("有 derive:");
    println!("  #[derive(Debug, Clone, PartialEq)]");
    println!("  struct PersonWithDerive {{ name: String, age: u32 }}");
    println!("  现在可以:");
    println!("    - 打印: {:?}", person1);
    let person2 = person1.clone();
    println!("    - 克隆: {:?}", person2);
    println!("    - 比较: person1 == person2 = {}", person1 == person2);
    println!();
}

// ============================================
// 2. 常用 derive 宏详解
// ============================================
fn demo_common_derives() {
    println!("--- 2. 常用 derive 宏详解 ---\n");
    
    // Debug - 格式化输出
    println!("1. Debug - 调试输出");
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 10, y: 20 };
    println!("   {:?}", p);
    println!("   {:#?}", p); // 美化输出
    println!();
    
    // Clone - 深拷贝
    println!("2. Clone - 深拷贝");
    #[derive(Clone, Debug)]
    struct Data {
        values: Vec<i32>,
    }
    let d1 = Data { values: vec![1, 2, 3] };
    let d2 = d1.clone(); // 完整克隆
    println!("   原始: {:?}", d1);
    println!("   克隆: {:?}", d2);
    println!();
    
    // Copy - 自动复制（仅限简单类型）
    println!("3. Copy - 自动复制");
    #[derive(Copy, Clone, Debug)]
    struct Coordinate {
        x: f64,
        y: f64,
    }
    let c1 = Coordinate { x: 1.0, y: 2.0 };
    let c2 = c1; // 自动复制，c1 仍然可用
    println!("   c1: {:?}, c2: {:?}", c1, c2);
    println!("   注意: Copy 只能用于不包含堆分配的类型");
    println!();
    
    // PartialEq / Eq - 相等性比较
    println!("4. PartialEq / Eq - 相等性比较");
    #[derive(PartialEq, Debug)]
    struct User {
        id: u32,
        name: String,
    }
    let u1 = User { id: 1, name: "Alice".to_string() };
    let u2 = User { id: 1, name: "Alice".to_string() };
    let u3 = User { id: 2, name: "Bob".to_string() };
    println!("   u1 == u2: {}", u1 == u2);
    println!("   u1 == u3: {}", u1 == u3);
    println!();
    
    // PartialOrd / Ord - 排序
    println!("5. PartialOrd / Ord - 排序");
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct Score {
        points: u32,
        level: u32,
    }
    let s1 = Score { points: 100, level: 5 };
    let s2 = Score { points: 200, level: 5 };
    println!("   s1 < s2: {}", s1 < s2);
    println!("   比较顺序: 按字段声明顺序（points 先于 level）");
    println!();
    
    // Default - 默认值
    println!("6. Default - 默认值");
    #[derive(Default, Debug)]
    struct Config {
        host: String,      // ""
        port: u16,         // 0
        enabled: bool,     // false
    }
    let config = Config::default();
    println!("   默认配置: {:?}", config);
    println!();
    
    // Hash - 哈希（用于 HashMap 的 key）
    println!("7. Hash - 哈希");
    use std::collections::HashMap;
    #[derive(Hash, PartialEq, Eq, Debug)]
    struct UserId {
        id: u32,
    }
    let mut map = HashMap::new();
    map.insert(UserId { id: 1 }, "Alice");
    map.insert(UserId { id: 2 }, "Bob");
    println!("   用户映射: {:?}", map);
    println!();
    
    // 组合使用
    println!("8. 常用组合");
    println!("   #[derive(Debug, Clone, PartialEq)]        // 基础组合");
    println!("   #[derive(Debug, Clone, PartialEq, Eq)]    // 可比较");
    println!("   #[derive(Debug, Clone, Default)]          // 有默认值");
    println!("   #[derive(Debug, Copy, Clone)]             // 简单类型");
    println!();
}

// ============================================
// 3. String vs &str 基础对比
// ============================================
fn demo_string_vs_str() {
    println!("--- 3. String vs &str 基础对比 ---\n");
    
    println!("核心区别:");
    println!();
    
    println!("String (拥有所有权的字符串):");
    println!("  - 类型: String");
    println!("  - 存储: 堆上分配");
    println!("  - 可变: 可以修改");
    println!("  - 所有权: 拥有数据");
    println!("  - 大小: 运行时确定");
    println!();
    
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("  示例: let mut s = String::from(\"Hello\");");
    println!("        s.push_str(\", World!\");");
    println!("        结果: {}", s);
    println!();
    
    println!("&str (字符串切片/借用):");
    println!("  - 类型: &str");
    println!("  - 存储: 指向现有字符串的引用");
    println!("  - 可变: 不可变");
    println!("  - 所有权: 借用数据");
    println!("  - 大小: 编译时确定（指针 + 长度）");
    println!();
    
    let s: &str = "Hello";
    println!("  示例: let s: &str = \"Hello\";");
    println!("        结果: {}", s);
    println!("        注意: 不能修改！");
    println!();
    
    // 转换
    println!("转换:");
    let string = String::from("test");
    let str_ref: &str = &string;        // String -> &str (廉价)
    let string2: String = str_ref.to_string(); // &str -> String (有开销)
    println!("  String -> &str: &string");
    println!("  &str -> String: str.to_string() 或 str.to_owned()");
    println!();
}

// ============================================
// 4. Struct 中为什么用 String
// ============================================
fn demo_string_in_struct() {
    println!("--- 4. Struct 中为什么用 String ---\n");
    
    println!("问题: 为什么不能用 &str？\n");
    
    // 尝试使用 &str（会遇到生命周期问题）
    println!("❌ 使用 &str 的问题:");
    println!("```rust");
    println!("struct User {{");
    println!("    name: &str,  // 错误！需要生命周期");
    println!("}}");
    println!("```");
    println!("编译错误: missing lifetime specifier");
    println!();
    
    // 正确的做法：使用 String
    println!("✅ 使用 String（推荐）:");
    #[derive(Debug)]
    struct User {
        name: String,  // 拥有数据
        email: String,
    }
    
    let user = User {
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
    };
    println!("{:?}", user);
    println!("优点:");
    println!("  - 不需要生命周期注解");
    println!("  - struct 拥有数据，可以自由移动");
    println!("  - 数据在 struct 存活期间一直有效");
    println!();
    
    // 如果一定要用 &str（需要生命周期）
    println!("⚠️ 使用 &str（需要生命周期）:");
    #[derive(Debug)]
    struct UserRef<'a> {
        name: &'a str,
        email: &'a str,
    }
    
    let name = String::from("李四");
    let email = String::from("lisi@example.com");
    let user_ref = UserRef {
        name: &name,
        email: &email,
    };
    println!("{:?}", user_ref);
    println!("缺点:");
    println!("  - 需要生命周期注解 'a");
    println!("  - 使用复杂，容易出错");
    println!("  - struct 的生命周期受限于引用的数据");
    println!();
    
    println!("结论:");
    println!("  📌 struct 字段使用 String (拥有所有权)");
    println!("  📌 函数参数使用 &str (借用)");
    println!();
}

// ============================================
// 5. 生命周期问题详解
// ============================================
fn demo_lifetime_issues() {
    println!("--- 5. 生命周期问题详解 ---\n");
    
    println!("示例 1: 返回包含 &str 的 struct");
    println!();
    
    // ❌ 错误：返回悬垂引用
    println!("❌ 错误代码:");
    println!("```rust");
    println!("fn create_user() -> UserRef {{");
    println!("    let name = String::from(\"张三\");");
    println!("    UserRef {{ name: &name }}  // 错误！name 在函数结束时被释放");
    println!("}}");
    println!("```");
    println!();
    
    // ✅ 正确：使用 String
    println!("✅ 正确代码:");
    #[derive(Debug)]
    struct User {
        name: String,
    }
    
    fn create_user() -> User {
        User {
            name: String::from("张三"), // 所有权转移出函数
        }
    }
    
    let user = create_user();
    println!("let user = create_user();");
    println!("user: {:?}", user);
    println!();
    
    println!("示例 2: 在集合中存储 struct");
    println!();
    
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let mut people = Vec::new();
    
    // 可以随意添加，不用担心生命周期
    people.push(Person {
        name: "Alice".to_string(),
        age: 25,
    });
    
    {
        let name = String::from("Bob");
        people.push(Person {
            name: name,  // name 的所有权转移到 Vec 中
            age: 30,
        });
        // name 的所有权已转移，这里 name 不再有效
    }
    
    println!("people: {:?}", people);
    println!("✅ Vec 可以长期持有这些 Person，不受作用域限制");
    println!();
    
    println!("示例 3: 多线程场景");
    println!();
    
    use std::thread;
    
    let user = User {
        name: "线程用户".to_string(),
    };
    
    let handle = thread::spawn(move || {
        println!("  线程中: {:?}", user);
    });
    
    handle.join().unwrap();
    println!("✅ String 可以安全地在线程间移动");
    println!();
}

// ============================================
// 6. 性能考虑
// ============================================
fn demo_performance_considerations() {
    println!("--- 6. 性能考虑 ---\n");
    
    println!("String 的开销:");
    println!("  - 堆分配: ~24 字节 (指针 + 容量 + 长度)");
    println!("  - 实际字符串数据在堆上");
    println!("  - 创建/克隆有性能开销");
    println!();
    
    println!("&str 的开销:");
    println!("  - 16 字节 (指针 + 长度)");
    println!("  - 不涉及堆分配");
    println!("  - 复制开销小");
    println!();
    
    println!("何时考虑使用 &str？");
    println!();
    
    // 1. 函数参数
    println!("1. 函数参数（推荐 &str）:");
    fn print_name(name: &str) {
        println!("  姓名: {}", name);
    }
    
    let name = String::from("测试");
    print_name(&name);  // String 可以自动转为 &str
    print_name("直接传递");
    println!();
    
    // 2. 临时数据
    println!("2. 临时数据（可以用 &str）:");
    fn process_data(data: &str) -> usize {
        data.len()
    }
    let len = process_data("临时字符串");
    println!("  长度: {}", len);
    println!();
    
    // 3. struct 字段（推荐 String）
    println!("3. struct 字段（推荐 String）:");
    #[derive(Debug)]
    struct Product {
        name: String,      // 使用 String
        description: String,
    }
    
    let product = Product {
        name: "商品".to_string(),
        description: "描述".to_string(),
    };
    println!("  {:?}", product);
    println!();
    
    println!("性能优化技巧:");
    println!("  1. 函数参数用 &str（避免不必要的克隆）");
    println!("  2. struct 字段用 String（避免生命周期复杂性）");
    println!("  3. 临时处理用 &str（减少分配）");
    println!("  4. 需要修改时用 String");
    println!("  5. 使用 Cow<str> 处理可能需要修改的情况");
    println!();
}

// ============================================
// 7. 实战案例
// ============================================
fn demo_real_world_examples() {
    println!("--- 7. 实战案例 ---\n");
    
    // 案例 1: HTTP 请求/响应
    println!("案例 1: HTTP 请求模型");
    
    #[derive(Debug, Clone, PartialEq)]
    struct HttpRequest {
        method: String,
        path: String,
        headers: Vec<(String, String)>,
        body: String,
    }
    
    let request = HttpRequest {
        method: "GET".to_string(),
        path: "/api/users".to_string(),
        headers: vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Authorization".to_string(), "Bearer token123".to_string()),
        ],
        body: String::new(),
    };
    
    println!("{:#?}", request);
    println!("✅ 所有字段用 String，可以自由移动、克隆、存储");
    println!();
    
    // 案例 2: 配置文件
    println!("案例 2: 配置管理");
    
    #[derive(Debug, Clone, Default)]
    struct AppConfig {
        database_url: String,
        api_key: String,
        log_level: String,
        port: u16,
    }
    
    impl AppConfig {
        fn new(database_url: &str, api_key: &str) -> Self {
            Self {
                database_url: database_url.to_string(), // &str -> String
                api_key: api_key.to_string(),
                log_level: "info".to_string(),
                port: 8080,
            }
        }
    }
    
    let config = AppConfig::new(
        "postgres://localhost/mydb",
        "secret-key-123"
    );
    println!("{:#?}", config);
    println!("✅ 构造函数接受 &str，内部存储为 String");
    println!();
    
    // 案例 3: 用户管理系统
    println!("案例 3: 用户管理系统");
    
    #[derive(Debug, Clone, PartialEq)]
    struct User {
        id: u32,
        username: String,
        email: String,
        full_name: String,
    }
    
    #[derive(Debug)]
    struct UserManager {
        users: Vec<User>,
    }
    
    impl UserManager {
        fn new() -> Self {
            Self { users: Vec::new() }
        }
        
        fn add_user(&mut self, username: &str, email: &str, full_name: &str) -> User {
            let user = User {
                id: self.users.len() as u32 + 1,
                username: username.to_string(),
                email: email.to_string(),
                full_name: full_name.to_string(),
            };
            self.users.push(user.clone());
            user
        }
        
        fn find_by_username(&self, username: &str) -> Option<&User> {
            self.users.iter().find(|u| u.username == username)
        }
    }
    
    let mut manager = UserManager::new();
    manager.add_user("alice", "alice@example.com", "Alice Smith");
    manager.add_user("bob", "bob@example.com", "Bob Johnson");
    
    if let Some(user) = manager.find_by_username("alice") {
        println!("找到用户: {:?}", user);
    }
    println!("✅ 方法参数用 &str（灵活），内部存储用 String（持久）");
    println!();
    
    // 案例 4: JSON 序列化
    println!("案例 4: JSON 数据模型");
    
    use std::collections::HashMap;
    
    #[derive(Debug, Clone)]
    struct JsonObject {
        data: HashMap<String, String>,
    }
    
    impl JsonObject {
        fn new() -> Self {
            Self {
                data: HashMap::new(),
            }
        }
        
        fn set(&mut self, key: &str, value: &str) {
            self.data.insert(key.to_string(), value.to_string());
        }
        
        fn get(&self, key: &str) -> Option<&str> {
            self.data.get(key).map(|s| s.as_str())
        }
    }
    
    let mut obj = JsonObject::new();
    obj.set("name", "张三");
    obj.set("age", "25");
    
    println!("JSON 对象: {:?}", obj);
    println!("获取 name: {:?}", obj.get("name"));
    println!("✅ HashMap<String, String> 是最常用的模式");
    println!();
}

// ============================================
// 8. 最佳实践
// ============================================
fn demo_best_practices() {
    println!("--- 8. 最佳实践总结 ---\n");
    
    println!("📌 Derive 宏最佳实践:");
    println!();
    println!("1. 基础组合:");
    println!("   #[derive(Debug, Clone)]");
    println!("   - 几乎所有 struct 都应该有");
    println!();
    
    println!("2. 可比较的类型:");
    println!("   #[derive(Debug, Clone, PartialEq, Eq)]");
    println!("   - 用于需要比较的类型");
    println!();
    
    println!("3. 可排序的类型:");
    println!("   #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]");
    println!("   - 用于需要排序的类型");
    println!();
    
    println!("4. 作为 HashMap key:");
    println!("   #[derive(Debug, Clone, PartialEq, Eq, Hash)]");
    println!("   - 必须实现 Eq 和 Hash");
    println!();
    
    println!("5. 简单的值类型:");
    println!("   #[derive(Debug, Copy, Clone, PartialEq)]");
    println!("   - 仅限不含堆分配的类型");
    println!();
    
    println!("📌 String vs &str 最佳实践:");
    println!();
    
    println!("✅ 使用 String 的场景:");
    println!("  1. Struct 字段");
    println!("  2. 需要修改的字符串");
    println!("  3. 需要所有权的场景");
    println!("  4. 返回值（避免生命周期问题）");
    println!("  5. 存储在集合中");
    println!();
    
    println!("✅ 使用 &str 的场景:");
    println!("  1. 函数参数（只读）");
    println!("  2. 临时使用");
    println!("  3. 字符串字面量");
    println!("  4. 不需要所有权的场景");
    println!();
    
    println!("📌 常见模式:");
    println!();
    println!("模式 1: 构造函数接受 &str");
    println!("```rust");
    println!("impl User {{");
    println!("    fn new(name: &str) -> Self {{");
    println!("        Self {{ name: name.to_string() }}");
    println!("    }}");
    println!("}}");
    println!("```");
    println!();
    
    println!("模式 2: Getter 返回 &str");
    println!("```rust");
    println!("impl User {{");
    println!("    fn name(&self) -> &str {{");
    println!("        &self.name");
    println!("    }}");
    println!("}}");
    println!("```");
    println!();
    
    println!("模式 3: 同时接受 String 和 &str");
    println!("```rust");
    println!("fn process<S: Into<String>>(text: S) {{");
    println!("    let text = text.into();");
    println!("    // 使用 text");
    println!("}}");
    println!("```");
    println!();
    
    println!("🎯 记忆口诀:");
    println!();
    println!("  \"结构用 String，参数用 &str\"");
    println!("  \"拥有用 String，借用用 &str\"");
    println!("  \"存储用 String，传递用 &str\"");
    println!();
    
    println!("⚠️ 注意事项:");
    println!();
    println!("1. 避免过度使用 .clone()");
    println!("   - 尽可能使用引用");
    println!("   - 必要时才克隆");
    println!();
    
    println!("2. 不要在 struct 中用 &str（除非必要）");
    println!("   - 生命周期复杂");
    println!("   - 使用受限");
    println!();
    
    println!("3. 函数参数优先使用 &str");
    println!("   - 更灵活");
    println!("   - 性能更好");
    println!();
    
    println!("4. derive 按需添加");
    println!("   - 不是所有 trait 都需要");
    println!("   - Debug 和 Clone 几乎总是需要");
    println!();
}

// ============================================
// 额外示例：高级用法
// ============================================

// 使用 Cow (Clone on Write) 处理可能修改的情况
#[allow(dead_code)]
fn example_cow() {
    use std::borrow::Cow;
    
    fn process_text(text: &str) -> Cow<str> {
        if text.contains("bad") {
            // 需要修改，创建新的 String
            Cow::Owned(text.replace("bad", "good"))
        } else {
            // 不需要修改，直接借用
            Cow::Borrowed(text)
        }
    }
    
    let result1 = process_text("this is good");
    let result2 = process_text("this is bad");
    
    println!("Result1: {}", result1); // 借用
    println!("Result2: {}", result2); // 拥有
}

// 自定义 derive（需要 proc-macro crate）
#[allow(dead_code)]
fn example_custom_derive() {
    // 这需要额外的 derive 宏库，如 serde
    // 
    // use serde::{Serialize, Deserialize};
    // 
    // #[derive(Debug, Serialize, Deserialize)]
    // struct User {
    //     name: String,
    //     age: u32,
    // }
}

/*
=== 总结 ===

1. Derive 宏:
   ✅ 常用组合: #[derive(Debug, Clone, PartialEq)]
   ✅ 减少样板代码
   ✅ 提高开发效率

2. String vs &str:
   ✅ Struct 字段用 String (拥有所有权)
   ✅ 函数参数用 &str (借用)
   ✅ 记住: "存储用 String，传递用 &str"

3. 为什么 struct 中用 String:
   ✅ 避免生命周期问题
   ✅ struct 可以自由移动
   ✅ 不受外部数据生命周期限制

4. 性能考虑:
   ✅ String 有堆分配开销
   ✅ &str 几乎零开销
   ✅ 权衡所有权和性能

5. 最佳实践:
   ✅ 构造函数: fn new(name: &str) -> Self
   ✅ Getter: fn name(&self) -> &str
   ✅ 存储: name: String
   ✅ 传递: process(name: &str)

运行示例:
  cargo run --bin derive_and_string
*/
