// 枚举与方法

/// # 基本枚举
pub fn basic_enum_demo() {
    println!("\n=== 基本枚举 ===");
    
    #[derive(Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    
    let dir = Direction::Up;
    println!("方向: {:?}", dir);
    
    // 使用 match 匹配枚举
    match dir {
        Direction::Up => println!("向上"),
        Direction::Down => println!("向下"),
        Direction::Left => println!("向左"),
        Direction::Right => println!("向右"),
    }
    
    // 枚举作为函数参数
    fn move_player(dir: Direction) {
        match dir {
            Direction::Up => println!("玩家向上移动"),
            Direction::Down => println!("玩家向下移动"),
            Direction::Left => println!("玩家向左移动"),
            Direction::Right => println!("玩家向右移动"),
        }
    }
    
    move_player(Direction::Right);
}

/// # 带数据的枚举
pub fn enum_with_data_demo() {
    println!("\n=== 带数据的枚举 ===");
    
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("IPv4: {:?}", home);
    println!("IPv6: {:?}", loopback);
    
    // 不同变体可以有不同类型和数量的数据
    #[derive(Debug)]
    enum Message {
        Quit,                       // 无数据
        Move { x: i32, y: i32 },   // 命名字段
        Write(String),              // 单个字符串
        ChangeColor(i32, i32, i32),// 三个整数
    }
    
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);
    
    println!("\n消息类型:");
    println!("{:?}", msg1);
    println!("{:?}", msg2);
    println!("{:?}", msg3);
    println!("{:?}", msg4);
}

/// # 枚举方法
pub fn enum_methods_demo() {
    println!("\n=== 枚举方法 ===");
    
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    impl Message {
        // 创建方法
        fn new_move(x: i32, y: i32) -> Message {
            Message::Move { x, y }
        }
        
        // 普通方法
        fn call(&self) {
            match self {
                Message::Quit => println!("退出消息"),
                Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
                Message::Write(text) => println!("写入: {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!("改变颜色为 RGB({}, {}, {})", r, g, b)
                }
            }
        }
        
        // 判断类型的方法
        fn is_quit(&self) -> bool {
            matches!(self, Message::Quit)
        }
        
        fn is_move(&self) -> bool {
            matches!(self, Message::Move { .. })
        }
    }
    
    let msg1 = Message::new_move(10, 20);
    let msg2 = Message::Write(String::from("Hello"));
    let msg3 = Message::Quit;
    
    msg1.call();
    msg2.call();
    msg3.call();
    
    println!("\nmsg3 是退出消息: {}", msg3.is_quit());
    println!("msg1 是移动消息: {}", msg1.is_move());
}

/// # Option 枚举
pub fn option_demo() {
    println!("\n=== Option 枚举 ===");
    
    // Option<T> 是 Rust 标准库中的枚举
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);
    
    // 使用 match 处理 Option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("\nplus_one(Some(5)): {:?}", six);
    println!("plus_one(None): {:?}", none);
    
    // if let 简洁匹配
    if let Some(value) = some_number {
        println!("值是: {}", value);
    }
    
    // unwrap - 获取值或 panic
    let x = Some(5);
    println!("unwrap: {}", x.unwrap());
    
    // unwrap_or - 提供默认值
    let x: Option<i32> = None;
    println!("unwrap_or(0): {}", x.unwrap_or(0));
    
    // unwrap_or_else - 提供默认值计算函数
    println!("unwrap_or_else: {}", x.unwrap_or_else(|| 42));
    
    // map - 变换 Option
    let maybe_num = Some(3);
    let maybe_doubled = maybe_num.map(|n| n * 2);
    println!("map: {:?}", maybe_doubled);
    
    // and_then - 链式操作
    let result = Some(2)
        .and_then(|n| Some(n * 2))
        .and_then(|n| Some(n + 1));
    println!("and_then: {:?}", result);
}

/// # Result 枚举
pub fn result_demo() {
    println!("\n=== Result 枚举 ===");
    
    // Result<T, E> 用于错误处理
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("除数不能为零"))
        } else {
            Ok(a / b)
        }
    }
    
    // 使用 match 处理 Result
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    // unwrap - 获取值或 panic
    let result = divide(10.0, 2.0);
    println!("unwrap: {}", result.unwrap());
    
    // expect - 带自定义错误消息的 unwrap
    let result = divide(10.0, 2.0);
    println!("expect: {}", result.expect("除法失败"));
    
    // unwrap_or - 提供默认值
    let result = divide(10.0, 0.0);
    println!("unwrap_or(0.0): {}", result.unwrap_or(0.0));
    
    // ? 操作符 - 传播错误
    fn calculate() -> Result<f64, String> {
        let x = divide(10.0, 2.0)?;
        let y = divide(x, 2.0)?;
        Ok(y)
    }
    
    match calculate() {
        Ok(result) => println!("calculate: {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    // map 和 map_err
    let result = divide(10.0, 2.0)
        .map(|x| x * 2.0)
        .map_err(|e| format!("计算错误: {}", e));
    println!("map: {:?}", result);
    
    // and_then
    let result = divide(10.0, 2.0)
        .and_then(|x| divide(x, 2.0));
    println!("and_then: {:?}", result);
}

/// # 模式匹配高级用法
pub fn pattern_matching_demo() {
    println!("\n=== 模式匹配高级用法 ===");
    
    #[derive(Debug)]
    enum Color {
        Rgb(u8, u8, u8),
        Hsv(u8, u8, u8),
        Hsl(u8, u8, u8),
    }
    
    let color = Color::Rgb(255, 0, 0);
    
    // 匹配并提取值
    match color {
        Color::Rgb(r, g, b) => {
            println!("RGB: ({}, {}, {})", r, g, b);
        }
        Color::Hsv(h, s, v) => {
            println!("HSV: ({}, {}, {})", h, s, v);
        }
        Color::Hsl(h, s, l) => {
            println!("HSL: ({}, {}, {})", h, s, l);
        }
    }
    
    // 卫语句
    let color = Color::Rgb(255, 0, 0);
    match color {
        Color::Rgb(r, g, b) if r > 200 => {
            println!("高亮度红色");
        }
        Color::Rgb(r, g, b) => {
            println!("普通 RGB: ({}, {}, {})", r, g, b);
        }
        _ => println!("其他颜色"),
    }
    
    // @ 绑定
    #[derive(Debug)]
    enum Message {
        Hello { id: i32 },
    }
    
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello { id: id_var @ 3..=7 } => {
            println!("id 在范围内: {}", id_var);
        }
        Message::Hello { id } => {
            println!("id: {}", id);
        }
    }
}

/// # 实战示例：状态机
pub fn state_machine_demo() {
    println!("\n=== 实战示例：状态机 ===");
    
    #[derive(Debug)]
    enum State {
        Idle,
        Running { progress: u32 },
        Paused { progress: u32 },
        Completed,
        Failed { error: String },
    }
    
    impl State {
        fn start() -> State {
            State::Running { progress: 0 }
        }
        
        fn pause(self) -> State {
            match self {
                State::Running { progress } => State::Paused { progress },
                other => other,
            }
        }
        
        fn resume(self) -> State {
            match self {
                State::Paused { progress } => State::Running { progress },
                other => other,
            }
        }
        
        fn advance(self, amount: u32) -> State {
            match self {
                State::Running { progress } => {
                    let new_progress = progress + amount;
                    if new_progress >= 100 {
                        State::Completed
                    } else {
                        State::Running { progress: new_progress }
                    }
                }
                other => other,
            }
        }
        
        fn fail(self, error: String) -> State {
            State::Failed { error }
        }
        
        fn display(&self) {
            match self {
                State::Idle => println!("状态: 空闲"),
                State::Running { progress } => println!("状态: 运行中 ({}%)", progress),
                State::Paused { progress } => println!("状态: 已暂停 ({}%)", progress),
                State::Completed => println!("状态: 已完成"),
                State::Failed { error } => println!("状态: 失败 - {}", error),
            }
        }
    }
    
    // 使用状态机
    let mut state = State::Idle;
    state.display();
    
    state = State::start();
    state.display();
    
    state = state.advance(30);
    state.display();
    
    state = state.pause();
    state.display();
    
    state = state.resume();
    state.display();
    
    state = state.advance(80);
    state.display();
}

/// # 实战示例：表达式求值
pub fn expression_eval_demo() {
    println!("\n=== 实战示例：表达式求值 ===");
    
    #[derive(Debug)]
    enum Expr {
        Number(f64),
        Add(Box<Expr>, Box<Expr>),
        Subtract(Box<Expr>, Box<Expr>),
        Multiply(Box<Expr>, Box<Expr>),
        Divide(Box<Expr>, Box<Expr>),
    }
    
    impl Expr {
        fn eval(&self) -> f64 {
            match self {
                Expr::Number(n) => *n,
                Expr::Add(left, right) => left.eval() + right.eval(),
                Expr::Subtract(left, right) => left.eval() - right.eval(),
                Expr::Multiply(left, right) => left.eval() * right.eval(),
                Expr::Divide(left, right) => left.eval() / right.eval(),
            }
        }
        
        fn to_string(&self) -> String {
            match self {
                Expr::Number(n) => n.to_string(),
                Expr::Add(left, right) => {
                    format!("({} + {})", left.to_string(), right.to_string())
                }
                Expr::Subtract(left, right) => {
                    format!("({} - {})", left.to_string(), right.to_string())
                }
                Expr::Multiply(left, right) => {
                    format!("({} * {})", left.to_string(), right.to_string())
                }
                Expr::Divide(left, right) => {
                    format!("({} / {})", left.to_string(), right.to_string())
                }
            }
        }
    }
    
    // (2 + 3) * 4
    let expr = Expr::Multiply(
        Box::new(Expr::Add(
            Box::new(Expr::Number(2.0)),
            Box::new(Expr::Number(3.0)),
        )),
        Box::new(Expr::Number(4.0)),
    );
    
    println!("表达式: {}", expr.to_string());
    println!("结果: {}", expr.eval());
    
    // ((10 - 5) * 2) / 5
    let expr = Expr::Divide(
        Box::new(Expr::Multiply(
            Box::new(Expr::Subtract(
                Box::new(Expr::Number(10.0)),
                Box::new(Expr::Number(5.0)),
            )),
            Box::new(Expr::Number(2.0)),
        )),
        Box::new(Expr::Number(5.0)),
    );
    
    println!("\n表达式: {}", expr.to_string());
    println!("结果: {}", expr.eval());
}

/// # 枚举与泛型
pub fn enum_with_generics_demo() {
    println!("\n=== 枚举与泛型 ===");
    
    #[derive(Debug)]
    enum GenericResult<T, E> {
        Success(T),
        Failure(E),
    }
    
    let success: GenericResult<i32, String> = GenericResult::Success(42);
    let failure: GenericResult<i32, String> = 
        GenericResult::Failure(String::from("错误"));
    
    println!("成功: {:?}", success);
    println!("失败: {:?}", failure);
    
    // 多个泛型参数
    #[derive(Debug)]
    enum Either<L, R> {
        Left(L),
        Right(R),
    }
    
    let left: Either<i32, String> = Either::Left(42);
    let right: Either<i32, String> = Either::Right(String::from("hello"));
    
    println!("左: {:?}", left);
    println!("右: {:?}", right);
}

/// 运行所有枚举示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust 枚举与方法详解         ║");
    println!("╚════════════════════════════════════╝");
    
    basic_enum_demo();
    enum_with_data_demo();
    enum_methods_demo();
    option_demo();
    result_demo();
    pattern_matching_demo();
    state_machine_demo();
    expression_eval_demo();
    enum_with_generics_demo();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_option() {
        let some_value = Some(5);
        assert_eq!(some_value.unwrap(), 5);
        
        let none_value: Option<i32> = None;
        assert_eq!(none_value.unwrap_or(0), 0);
    }
    
    #[test]
    fn test_result() {
        let ok_value: Result<i32, String> = Ok(42);
        assert_eq!(ok_value.unwrap(), 42);
        
        let err_value: Result<i32, String> = Err(String::from("error"));
        assert_eq!(err_value.unwrap_or(0), 0);
    }
}
