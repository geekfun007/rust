// 结构体与方法

/// # 结构体定义
pub fn struct_definition_demo() {
    println!("\n=== 结构体定义 ===");
    
    // 命名结构体
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    // 创建实例
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("用户: {:?}", user1);
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
    
    // 可变实例
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser"),
        active: true,
        sign_in_count: 1,
    };
    
    user2.email = String::from("newemail@example.com");
    println!("修改后邮箱: {}", user2.email);
    
    // 字段初始化简写
    fn build_user(email: String, username: String) -> User {
        User {
            email,      // 简写
            username,   // 简写
            active: true,
            sign_in_count: 1,
        }
    }
    
    let user3 = build_user(
        String::from("test@example.com"),
        String::from("testuser"),
    );
    println!("构建的用户: {:?}", user3);
    
    // 结构体更新语法
    let user4 = User {
        email: String::from("update@example.com"),
        username: String::from("updateuser"),
        ..user1  // 从 user1 复制其他字段
    };
    println!("更新语法: {:?}", user4);
}

/// # 元组结构体
pub fn tuple_struct_demo() {
    println!("\n=== 元组结构体 ===");
    
    // 元组结构体
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("颜色: {:?}", black);
    println!("点: {:?}", origin);
    
    // 访问元素
    println!("黑色的红色分量: {}", black.0);
    println!("原点的 x 坐标: {}", origin.0);
    
    // 解构
    let Color(r, g, b) = black;
    println!("解构颜色: R={}, G={}, B={}", r, g, b);
}

/// # 单元结构体
pub fn unit_struct_demo() {
    println!("\n=== 单元结构体 ===");
    
    // 没有任何字段的结构体
    #[derive(Debug)]
    struct AlwaysEqual;
    
    let subject = AlwaysEqual;
    println!("单元结构体: {:?}", subject);
    
    // 通常用于实现 trait
}

/// # 方法定义
pub fn methods_demo() {
    println!("\n=== 方法定义 ===");
    
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    // impl 块定义方法
    impl Rectangle {
        // 方法 - 第一个参数是 self
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        
        // 可变方法
        fn set_width(&mut self, width: u32) {
            self.width = width;
        }
        
        // 获取所有权的方法（较少使用）
        fn into_square(self) -> Rectangle {
            let size = self.width.min(self.height);
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("矩形: {:?}", rect1);
    println!("面积: {}", rect1.area());
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("rect1 能容纳 rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 能容纳 rect3: {}", rect1.can_hold(&rect3));
    
    // 可变方法
    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };
    rect.set_width(15);
    println!("修改宽度后: {:?}", rect);
}

/// # 关联函数
pub fn associated_functions_demo() {
    println!("\n=== 关联函数 ===");
    
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // 关联函数 - 不接受 self 参数
        // 通常用作构造函数
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
        
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
        
        fn default() -> Rectangle {
            Rectangle {
                width: 1,
                height: 1,
            }
        }
    }
    
    // 使用 :: 调用关联函数
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::square(20);
    let rect3 = Rectangle::default();
    
    println!("new: {:?}", rect1);
    println!("square: {:?}", rect2);
    println!("default: {:?}", rect3);
}

/// # 多个 impl 块
pub fn multiple_impl_blocks_demo() {
    println!("\n=== 多个 impl 块 ===");
    
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    
    // 第一个 impl 块
    impl Point {
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }
        
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }
    }
    
    // 第二个 impl 块
    impl Point {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
        
        fn translate(&mut self, dx: f64, dy: f64) {
            self.x += dx;
            self.y += dy;
        }
    }
    
    let p1 = Point::new(3.0, 4.0);
    println!("点: {:?}", p1);
    println!("到原点距离: {}", p1.distance_from_origin());
    
    let mut p2 = Point::origin();
    p2.translate(3.0, 4.0);
    println!("平移后: {:?}", p2);
}

/// # 泛型结构体
pub fn generic_struct_demo() {
    println!("\n=== 泛型结构体 ===");
    
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Point<T> {
            Point { x, y }
        }
        
        fn x(&self) -> &T {
            &self.x
        }
    }
    
    // 不同类型的点
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    
    println!("整数点: {:?}", integer_point);
    println!("浮点数点: {:?}", float_point);
    
    // 多个泛型参数
    #[derive(Debug)]
    struct MixedPoint<T, U> {
        x: T,
        y: U,
    }
    
    let mixed = MixedPoint { x: 5, y: 4.0 };
    println!("混合点: {:?}", mixed);
}

/// # 生命周期注解的结构体
pub fn lifetime_struct_demo() {
    println!("\n=== 生命周期注解的结构体 ===");
    
    // 包含引用的结构体需要生命周期注解
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("摘录: {:?}", excerpt);
    println!("内容: {}", excerpt.part);
}

/// # 派生 trait
pub fn derived_traits_demo() {
    println!("\n=== 派生 trait ===");
    
    // Debug - 可调试打印
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 1, y: 2 };
    println!("Debug: {:?}", p);
    println!("Pretty Debug: {:#?}", p);
    
    // Clone - 可克隆
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    let person2 = person1.clone();
    println!("原始: {:?}", person1);
    println!("克隆: {:?}", person2);
    
    // Copy - 可复制（仅用于栈上数据）
    #[derive(Debug, Copy, Clone)]
    struct Point2D {
        x: i32,
        y: i32,
    }
    
    let p1 = Point2D { x: 1, y: 2 };
    let p2 = p1;  // 复制而不是移动
    println!("p1: {:?}, p2: {:?}", p1, p2);
    
    // PartialEq, Eq - 可比较相等
    #[derive(Debug, PartialEq, Eq)]
    struct Coordinate {
        x: i32,
        y: i32,
    }
    
    let c1 = Coordinate { x: 1, y: 2 };
    let c2 = Coordinate { x: 1, y: 2 };
    let c3 = Coordinate { x: 2, y: 3 };
    
    println!("c1 == c2: {}", c1 == c2);
    println!("c1 == c3: {}", c1 == c3);
    
    // PartialOrd, Ord - 可排序
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Height(i32);
    
    let h1 = Height(160);
    let h2 = Height(180);
    println!("h1 < h2: {}", h1 < h2);
}

/// # 实战示例：用户系统
pub fn practical_example() {
    println!("\n=== 实战示例：用户系统 ===");
    
    #[derive(Debug, Clone)]
    struct User {
        id: u64,
        username: String,
        email: String,
        active: bool,
    }
    
    impl User {
        fn new(id: u64, username: String, email: String) -> Self {
            User {
                id,
                username,
                email,
                active: true,
            }
        }
        
        fn deactivate(&mut self) {
            self.active = false;
        }
        
        fn activate(&mut self) {
            self.active = true;
        }
        
        fn update_email(&mut self, new_email: String) {
            self.email = new_email;
        }
        
        fn display(&self) {
            println!("用户 #{}: {} ({})", 
                     self.id, 
                     self.username, 
                     if self.active { "活跃" } else { "已停用" }
            );
            println!("  邮箱: {}", self.email);
        }
    }
    
    struct UserDatabase {
        users: Vec<User>,
        next_id: u64,
    }
    
    impl UserDatabase {
        fn new() -> Self {
            UserDatabase {
                users: Vec::new(),
                next_id: 1,
            }
        }
        
        fn add_user(&mut self, username: String, email: String) -> u64 {
            let id = self.next_id;
            let user = User::new(id, username, email);
            self.users.push(user);
            self.next_id += 1;
            id
        }
        
        fn find_user(&self, id: u64) -> Option<&User> {
            self.users.iter().find(|u| u.id == id)
        }
        
        fn find_user_mut(&mut self, id: u64) -> Option<&mut User> {
            self.users.iter_mut().find(|u| u.id == id)
        }
        
        fn list_active_users(&self) {
            println!("\n活跃用户:");
            for user in &self.users {
                if user.active {
                    user.display();
                }
            }
        }
    }
    
    // 使用示例
    let mut db = UserDatabase::new();
    
    let id1 = db.add_user(
        String::from("alice"),
        String::from("alice@example.com")
    );
    
    let id2 = db.add_user(
        String::from("bob"),
        String::from("bob@example.com")
    );
    
    db.list_active_users();
    
    // 停用用户
    if let Some(user) = db.find_user_mut(id1) {
        user.deactivate();
    }
    
    println!("\n停用 alice 后:");
    db.list_active_users();
}

/// 运行所有结构体示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║      Rust 结构体与方法详解         ║");
    println!("╚════════════════════════════════════╝");
    
    struct_definition_demo();
    tuple_struct_demo();
    unit_struct_demo();
    methods_demo();
    associated_functions_demo();
    multiple_impl_blocks_demo();
    generic_struct_demo();
    lifetime_struct_demo();
    derived_traits_demo();
    practical_example();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_struct_creation() {
        struct Point {
            x: i32,
            y: i32,
        }
        
        let p = Point { x: 1, y: 2 };
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
    }
    
    #[test]
    fn test_methods() {
        struct Rectangle {
            width: u32,
            height: u32,
        }
        
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }
        
        let rect = Rectangle { width: 10, height: 20 };
        assert_eq!(rect.area(), 200);
    }
}
