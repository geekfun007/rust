// Derive 宏详解 - 完全指南
//
// Derive 宏是 Rust 中自动生成 trait 实现的机制
// 本教程涵盖所有常用的 derive 宏和实战技巧

use std::fmt;
use std::cmp::Ordering;

fn main() {
    println!("=== Derive 宏详解 ===\n");
    
    // 1. Derive 基础
    demo_derive_basics();
    
    // 2. Debug - 调试输出
    demo_debug();
    
    // 3. Clone 和 Copy
    demo_clone_copy();
    
    // 4. PartialEq 和 Eq
    demo_equality();
    
    // 5. PartialOrd 和 Ord
    demo_ordering();
    
    // 6. Hash
    demo_hash();
    
    // 7. Default
    demo_default();
    
    // 8. 组合使用
    demo_combinations();
    
    // 9. 手动实现 vs Derive
    demo_manual_vs_derive();
    
    // 10. 实战案例
    demo_real_world_examples();
}

// ============================================
// 1. Derive 基础
// ============================================
fn demo_derive_basics() {
    println!("--- 1. Derive 基础 ---\n");
    
    println!("什么是 Derive？");
    println!("  - 编译器自动生成 trait 实现");
    println!("  - 减少样板代码");
    println!("  - 使用 #[derive(...)] 属性\n");
    
    println!("语法:");
    println!("  #[derive(Debug, Clone, PartialEq)]");
    println!("  struct MyStruct {{ ... }}\n");
    
    println!("常用 Derive 宏:");
    println!("  📌 Debug      - 调试输出 {{:?}}");
    println!("  📌 Clone      - 深拷贝");
    println!("  📌 Copy       - 按位复制（栈上）");
    println!("  📌 PartialEq  - 相等性比较 ==");
    println!("  📌 Eq         - 完全相等");
    println!("  📌 PartialOrd - 排序比较 <, >");
    println!("  📌 Ord        - 完全排序");
    println!("  📌 Hash       - 哈希（用于 HashMap）");
    println!("  📌 Default    - 默认值\n");
}

// ============================================
// 2. Debug - 调试输出
// ============================================
fn demo_debug() {
    println!("--- 2. Debug - 调试输出 ---\n");
    
    println!("作用: 实现 fmt::Debug trait，支持 {{:?}} 格式化\n");
    
    // 自动派生
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let point = Point { x: 10, y: 20 };
    
    println!("标准输出:");
    println!("  {{:?}}  = {:?}", point);
    println!("  {{:#?}} = {:#?}", point);
    println!();
    
    // 嵌套结构
    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }
    
    let rect = Rectangle {
        top_left: Point { x: 0, y: 0 },
        bottom_right: Point { x: 100, y: 50 },
    };
    
    println!("嵌套结构:");
    println!("{:#?}", rect);
    println!();
    
    // 手动实现 Debug
    struct Custom {
        value: i32,
    }
    
    impl fmt::Debug for Custom {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Custom[{}]", self.value)
        }
    }
    
    let custom = Custom { value: 42 };
    println!("自定义 Debug: {:?}", custom);
    println!();
}

// ============================================
// 3. Clone 和 Copy
// ============================================
fn demo_clone_copy() {
    println!("--- 3. Clone 和 Copy ---\n");
    
    // Clone - 深拷贝
    println!("Clone: 显式深拷贝");
    
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let p1 = Person {
        name: "Alice".to_string(),
        age: 25,
    };
    
    let p2 = p1.clone();
    println!("  原始: {:?}", p1);
    println!("  克隆: {:?}", p2);
    println!("  说明: 需要显式调用 .clone()");
    println!();
    
    // Copy - 自动复制
    println!("Copy: 自动按位复制");
    
    #[derive(Debug, Clone, Copy)]
    struct Coord {
        x: i32,
        y: i32,
    }
    
    let c1 = Coord { x: 1, y: 2 };
    let c2 = c1; // 自动复制
    
    println!("  c1: {:?}", c1);
    println!("  c2: {:?}", c2);
    println!("  说明: c1 仍然有效（自动复制）");
    println!();
    
    // Copy 的限制
    println!("Copy 的要求:");
    println!("  ✓ 所有字段都必须实现 Copy");
    println!("  ✓ 不能包含堆分配（如 String, Vec）");
    println!("  ✓ 必须同时 derive Clone");
    println!();
    
    println!("对比:");
    println!("┌──────────┬────────────┬────────────┐");
    println!("│  特性     │  Clone     │  Copy      │");
    println!("├──────────┼────────────┼────────────┤");
    println!("│  调用     │  显式      │  自动      │");
    println!("│  开销     │  可能大    │  小        │");
    println!("│  堆分配   │  允许      │  不允许    │");
    println!("│  使用后   │  可继续用  │  可继续用  │");
    println!("└──────────┴────────────┴────────────┘");
    println!();
}

// ============================================
// 4. PartialEq 和 Eq
// ============================================
fn demo_equality() {
    println!("--- 4. PartialEq 和 Eq ---\n");
    
    // PartialEq - 部分相等
    println!("PartialEq: 支持 == 和 != 操作");
    
    #[derive(Debug, PartialEq)]
    struct User {
        id: u32,
        name: String,
    }
    
    let u1 = User {
        id: 1,
        name: "Alice".to_string(),
    };
    
    let u2 = User {
        id: 1,
        name: "Alice".to_string(),
    };
    
    let u3 = User {
        id: 2,
        name: "Bob".to_string(),
    };
    
    println!("  u1 == u2: {}", u1 == u2);
    println!("  u1 == u3: {}", u1 == u3);
    println!("  u1 != u3: {}", u1 != u3);
    println!();
    
    // Eq - 完全相等
    println!("Eq: PartialEq 的增强版本");
    
    #[derive(Debug, PartialEq, Eq)]
    struct Id(u32);
    
    println!("  说明: Eq 表示相等关系具有自反性");
    println!("  要求: PartialEq 必须先实现");
    println!();
    
    // 自定义相等逻辑
    println!("自定义相等逻辑:");
    
    struct CaseInsensitive {
        text: String,
    }
    
    impl PartialEq for CaseInsensitive {
        fn eq(&self, other: &Self) -> bool {
            self.text.to_lowercase() == other.text.to_lowercase()
        }
    }
    
    let s1 = CaseInsensitive {
        text: "Hello".to_string(),
    };
    let s2 = CaseInsensitive {
        text: "HELLO".to_string(),
    };
    
    println!("  'Hello' == 'HELLO': {}", s1 == s2);
    println!();
}

// ============================================
// 5. PartialOrd 和 Ord
// ============================================
fn demo_ordering() {
    println!("--- 5. PartialOrd 和 Ord ---\n");
    
    // PartialOrd - 部分排序
    println!("PartialOrd: 支持 <, <=, >, >= 操作");
    
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Score {
        points: u32,
    }
    
    let s1 = Score { points: 100 };
    let s2 = Score { points: 200 };
    
    println!("  Score{{100}} < Score{{200}}: {}", s1 < s2);
    println!("  Score{{100}} > Score{{200}}: {}", s1 > s2);
    println!();
    
    // Ord - 完全排序
    println!("Ord: 完全排序，可用于 sort()");
    
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Priority {
        level: u32,
        name: String,
    }
    
    let mut items = vec![
        Priority {
            level: 2,
            name: "B".to_string(),
        },
        Priority {
            level: 1,
            name: "A".to_string(),
        },
        Priority {
            level: 3,
            name: "C".to_string(),
        },
    ];
    
    items.sort();
    
    println!("  排序后:");
    for item in &items {
        println!("    {:?}", item);
    }
    println!();
    
    println!("比较顺序:");
    println!("  按字段声明顺序比较（字典序）");
    println!("  level 先于 name");
    println!();
}

// ============================================
// 6. Hash
// ============================================
fn demo_hash() {
    println!("--- 6. Hash ---\n");
    
    println!("作用: 用作 HashMap/HashSet 的键");
    
    use std::collections::HashMap;
    
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct ProductId {
        category: String,
        sku: u32,
    }
    
    let mut inventory = HashMap::new();
    
    inventory.insert(
        ProductId {
            category: "Electronics".to_string(),
            sku: 12345,
        },
        100,
    );
    
    inventory.insert(
        ProductId {
            category: "Books".to_string(),
            sku: 67890,
        },
        50,
    );
    
    println!("  库存:");
    for (id, count) in &inventory {
        println!("    {:?}: {} 件", id, count);
    }
    println!();
    
    println!("要求:");
    println!("  ✓ 必须同时实现 Eq");
    println!("  ✓ 相等的值必须有相同的哈希");
    println!();
}

// ============================================
// 7. Default
// ============================================
fn demo_default() {
    println!("--- 7. Default ---\n");
    
    println!("作用: 提供类型的默认值");
    
    #[derive(Debug, Default)]
    struct Config {
        host: String,      // ""
        port: u16,         // 0
        debug: bool,       // false
        timeout: u32,      // 0
    }
    
    let config = Config::default();
    println!("  默认配置: {:#?}", config);
    println!();
    
    // 部分字段自定义
    println!("结构体更新语法:");
    let custom_config = Config {
        port: 8080,
        debug: true,
        ..Default::default()
    };
    println!("  自定义配置: {:#?}", custom_config);
    println!();
    
    // 自定义 Default
    println!("自定义 Default:");
    
    struct Server {
        name: String,
        workers: usize,
    }
    
    impl Default for Server {
        fn default() -> Self {
            Server {
                name: "default-server".to_string(),
                workers: 4, // 自定义默认值
            }
        }
    }
    
    let server = Server::default();
    println!("  服务器: {}, {} workers", server.name, server.workers);
    println!();
}

// ============================================
// 8. 组合使用
// ============================================
fn demo_combinations() {
    println!("--- 8. 常用组合 ---\n");
    
    println!("1. 基础组合（最常用）:");
    println!("   #[derive(Debug, Clone, PartialEq)]");
    
    #[derive(Debug, Clone, PartialEq)]
    struct Basic {
        value: i32,
    }
    
    let b1 = Basic { value: 42 };
    let b2 = b1.clone();
    println!("   {:?} == {:?}: {}", b1, b2, b1 == b2);
    println!();
    
    println!("2. 可排序的类型:");
    println!("   #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]");
    
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Sortable {
        priority: u32,
    }
    
    let mut items = vec![
        Sortable { priority: 3 },
        Sortable { priority: 1 },
        Sortable { priority: 2 },
    ];
    items.sort();
    println!("   排序: {:?}", items);
    println!();
    
    println!("3. HashMap 键:");
    println!("   #[derive(Debug, Clone, PartialEq, Eq, Hash)]");
    
    use std::collections::HashMap;
    
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Key {
        id: u32,
    }
    
    let mut map = HashMap::new();
    map.insert(Key { id: 1 }, "value");
    println!("   Map: {:?}", map);
    println!();
    
    println!("4. 简单值类型（Copy）:");
    println!("   #[derive(Debug, Clone, Copy, PartialEq)]");
    
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 1, y: 2 };
    let _ = p; // 自动复制
    println!("   Point: {:?}", p); // 仍然有效
    println!();
    
    println!("5. 完整功能:");
    println!("   #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]");
    println!();
}

// ============================================
// 9. 手动实现 vs Derive
// ============================================
fn demo_manual_vs_derive() {
    println!("--- 9. 手动实现 vs Derive ---\n");
    
    // Derive 版本
    #[derive(Debug, Clone, PartialEq)]
    struct Auto {
        x: i32,
        y: i32,
    }
    
    // 手动实现版本
    struct Manual {
        x: i32,
        y: i32,
    }
    
    impl fmt::Debug for Manual {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Manual")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }
    
    impl Clone for Manual {
        fn clone(&self) -> Self {
            Manual {
                x: self.x,
                y: self.y,
            }
        }
    }
    
    impl PartialEq for Manual {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
    
    println!("Derive 版本:");
    println!("  #[derive(Debug, Clone, PartialEq)]");
    println!("  struct Auto {{ x: i32, y: i32 }}");
    println!();
    
    println!("手动实现版本: ~20 行代码");
    println!();
    
    println!("何时手动实现？");
    println!("  ✓ 需要自定义行为");
    println!("  ✓ 特殊的相等/排序逻辑");
    println!("  ✓ 优化性能");
    println!("  ✓ 忽略某些字段");
    println!();
    
    // 自定义相等（忽略某字段）
    println!("示例: 忽略时间戳的相等比较");
    
    #[derive(Debug)]
    struct Record {
        id: u32,
        data: String,
        timestamp: u64,
    }
    
    impl PartialEq for Record {
        fn eq(&self, other: &Self) -> bool {
            // 只比较 id 和 data，忽略 timestamp
            self.id == other.id && self.data == other.data
        }
    }
    
    let r1 = Record {
        id: 1,
        data: "test".to_string(),
        timestamp: 100,
    };
    
    let r2 = Record {
        id: 1,
        data: "test".to_string(),
        timestamp: 200, // 不同的时间戳
    };
    
    println!("  r1 == r2: {} (忽略了时间戳)", r1 == r2);
    println!();
}

// ============================================
// 10. 实战案例
// ============================================
fn demo_real_world_examples() {
    println!("--- 10. 实战案例 ---\n");
    
    // 案例 1: 数据模型
    println!("案例 1: RESTful API 数据模型\n");
    api_model_example();
    
    // 案例 2: 游戏实体
    println!("\n案例 2: 游戏实体系统\n");
    game_entity_example();
    
    // 案例 3: 配置系统
    println!("\n案例 3: 分层配置系统\n");
    config_system_example();
}

// 案例 1: API 数据模型
fn api_model_example() {
    #[derive(Debug, Clone, PartialEq)]
    struct User {
        id: u32,
        username: String,
        email: String,
        role: UserRole,
    }
    
    #[derive(Debug, Clone, PartialEq)]
    enum UserRole {
        Admin,
        User,
        Guest,
    }
    
    #[derive(Debug, Clone, PartialEq)]
    struct Post {
        id: u32,
        author_id: u32,
        title: String,
        content: String,
        tags: Vec<String>,
    }
    
    #[derive(Debug, Clone, PartialEq)]
    struct ApiResponse<T> {
        success: bool,
        data: Option<T>,
        error: Option<String>,
    }
    
    let user = User {
        id: 1,
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        role: UserRole::Admin,
    };
    
    let response = ApiResponse {
        success: true,
        data: Some(user),
        error: None,
    };
    
    println!("  API 响应: {:#?}", response);
    
    println!("\n  使用的 Derive:");
    println!("    - Debug: 调试输出");
    println!("    - Clone: 复制数据");
    println!("    - PartialEq: 测试比较");
}

// 案例 2: 游戏实体
fn game_entity_example() {
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }
    
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }
    
    #[derive(Debug, Clone, PartialEq)]
    struct Entity {
        id: u32,
        name: String,
        position: Position,
        velocity: Velocity,
        health: u32,
    }
    
    impl Entity {
        fn update(&mut self, delta_time: f32) {
            self.position.x += self.velocity.dx * delta_time;
            self.position.y += self.velocity.dy * delta_time;
        }
        
        fn is_alive(&self) -> bool {
            self.health > 0
        }
    }
    
    let mut player = Entity {
        id: 1,
        name: "Player".to_string(),
        position: Position { x: 0.0, y: 0.0 },
        velocity: Velocity { dx: 1.0, dy: 0.5 },
        health: 100,
    };
    
    println!("  初始位置: {:?}", player.position);
    
    player.update(1.0);
    println!("  更新后位置: {:?}", player.position);
    println!("  存活: {}", player.is_alive());
    
    println!("\n  使用的 Derive:");
    println!("    - Copy: Position 和 Velocity（小型值类型）");
    println!("    - Clone: Entity（包含 String）");
    println!("    - PartialEq: 游戏状态比较");
}

// 案例 3: 配置系统
fn config_system_example() {
    #[derive(Debug, Clone, PartialEq, Default)]
    struct DatabaseConfig {
        host: String,
        port: u16,
        max_connections: usize,
    }
    
    #[derive(Debug, Clone, PartialEq, Default)]
    struct ServerConfig {
        bind_address: String,
        worker_threads: usize,
    }
    
    #[derive(Debug, Clone, PartialEq, Default)]
    struct AppConfig {
        database: DatabaseConfig,
        server: ServerConfig,
        debug_mode: bool,
    }
    
    // 使用默认配置
    let mut config = AppConfig::default();
    
    // 覆盖部分设置
    config.database.host = "localhost".to_string();
    config.database.port = 5432;
    config.server.bind_address = "0.0.0.0:8080".to_string();
    config.server.worker_threads = 4;
    
    println!("  应用配置:");
    println!("    数据库: {}:{}", config.database.host, config.database.port);
    println!("    服务器: {}", config.server.bind_address);
    println!("    Workers: {}", config.server.worker_threads);
    
    // 创建配置副本
    let backup = config.clone();
    
    println!("\n  配置已备份");
    println!("  原始 == 备份: {}", config == backup);
    
    println!("\n  使用的 Derive:");
    println!("    - Default: 提供默认配置");
    println!("    - Clone: 复制配置");
    println!("    - PartialEq: 比较配置");
}

/*
=== 总结 ===

1. 常用 Derive 组合:

   基础:
   #[derive(Debug, Clone, PartialEq)]
   
   可排序:
   #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
   
   HashMap 键:
   #[derive(Debug, Clone, PartialEq, Eq, Hash)]
   
   简单值:
   #[derive(Debug, Clone, Copy, PartialEq)]
   
   完整:
   #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]

2. Derive 依赖关系:
   - Ord 需要 Eq + PartialOrd
   - Eq 需要 PartialEq
   - Copy 需要 Clone
   - Hash 通常需要 Eq

3. 选择指南:
   - Debug: 几乎总是需要
   - Clone: 大多数情况需要
   - Copy: 仅简单类型（无堆分配）
   - PartialEq: 需要比较时
   - Hash: 用作 HashMap 键时
   - Default: 需要默认值时

4. 何时手动实现:
   - 自定义行为
   - 忽略某些字段
   - 性能优化
   - 特殊逻辑

运行示例:
  cargo run --bin derive_macros_detailed
*/
