// 控制流示例

/// # if 表达式
pub fn if_expressions_demo() {
    println!("\n=== if 表达式 ===");
    
    let number = 6;
    
    // 基本 if-else
    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }
    
    // if-else if-else
    if number % 4 == 0 {
        println!("数字能被 4 整除");
    } else if number % 3 == 0 {
        println!("数字能被 3 整除");
    } else if number % 2 == 0 {
        println!("数字能被 2 整除");
    } else {
        println!("数字不能被 4、3、2 整除");
    }
    
    // if 作为表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);
    
    // 注意：两个分支的类型必须相同
    // let number = if condition { 5 } else { "six" }; // 错误！
}

/// # loop 循环
pub fn loop_demo() {
    println!("\n=== loop 循环 ===");
    
    // 无限循环需要 break 退出
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2; // loop 可以返回值
        }
    };
    println!("loop 返回值: {}", result);
    
    // 循环标签 - 用于多层循环
    let mut count = 0;
    'outer: loop {
        println!("外层循环 count = {}", count);
        let mut remaining = 10;
        
        loop {
            println!("  内层循环 remaining = {}", remaining);
            if remaining == 9 {
                break; // 退出内层循环
            }
            if count == 2 {
                break 'outer; // 退出外层循环
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("外层循环结束");
}

/// # while 条件循环
pub fn while_demo() {
    println!("\n=== while 循环 ===");
    
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("发射！");
    
    // 使用 while 遍历集合（不推荐）
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    
    while index < 5 {
        println!("值为: {}", a[index]);
        index += 1;
    }
}

/// # for 循环
pub fn for_demo() {
    println!("\n=== for 循环 ===");
    
    // 遍历数组
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("值为: {}", element);
    }
    
    // 遍历范围
    for number in 1..4 {
        println!("数字: {}", number); // 1, 2, 3
    }
    
    // 包含结束值的范围
    for number in 1..=5 {
        println!("数字: {}", number); // 1, 2, 3, 4, 5
    }
    
    // 反向遍历
    for number in (1..4).rev() {
        println!("倒计时: {}", number);
    }
    println!("发射！");
    
    // 带索引遍历
    for (index, value) in a.iter().enumerate() {
        println!("索引 {} 的值为 {}", index, value);
    }
}

/// # match 模式匹配
pub fn match_demo() {
    println!("\n=== match 模式匹配 ===");
    
    // 基本匹配
    let number = 13;
    
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 | 11 => println!("这是一个质数"),
        13..=19 => println!("十几"),
        _ => println!("其他"),
    }
    
    // 匹配并绑定值
    let x = 5;
    match x {
        1..=5 => println!("1 到 5"),
        n @ 6..=10 => println!("6 到 10，值为 {}", n),
        _ => println!("其他"),
    }
    
    // 匹配 Option
    let some_number = Some(7);
    match some_number {
        Some(n) => println!("找到数字: {}", n),
        None => println!("没有数字"),
    }
    
    // 匹配 Result
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(value) => println!("成功: {}", value),
        Err(e) => println!("错误: {}", e),
    }
    
    // match 作为表达式
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("二进制: {}", binary);
}

/// # if let 简洁匹配
pub fn if_let_demo() {
    println!("\n=== if let ===");
    
    let some_value = Some(3);
    
    // 使用 match
    match some_value {
        Some(3) => println!("三"),
        _ => (),
    }
    
    // 使用 if let - 更简洁
    if let Some(3) = some_value {
        println!("三");
    }
    
    // 带 else
    let some_value = Some(7);
    if let Some(3) = some_value {
        println!("三");
    } else {
        println!("不是三");
    }
    
    // 实际应用场景
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("最大值配置为 {}", max);
    }
}

/// # while let 条件循环
pub fn while_let_demo() {
    println!("\n=== while let ===");
    
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // 持续弹出直到栈为空
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
}

/// # 解构与模式
pub fn destructuring_demo() {
    println!("\n=== 解构与模式 ===");
    
    // 解构元组
    let tuple = (1, "hello", 4.5);
    let (a, b, c) = tuple;
    println!("解构元组: {}, {}, {}", a, b, c);
    
    // 解构数组
    let array = [1, 2, 3];
    let [x, y, z] = array;
    println!("解构数组: {}, {}, {}", x, y, z);
    
    // 忽略某些值
    let (first, _, third) = (1, 2, 3);
    println!("忽略中间值: {}, {}", first, third);
    
    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("解构结构体: x = {}, y = {}", x, y);
    
    // 简写
    let Point { x, .. } = p;
    println!("只取 x: {}", x);
    
    // 使用 @ 绑定并测试
    let msg = Some(5);
    match msg {
        Some(n @ 3..=7) => println!("找到范围内的数字: {}", n),
        Some(n) => println!("其他数字: {}", n),
        None => println!("没有数字"),
    }
}

/// # 卫语句 (Guard)
pub fn guard_demo() {
    println!("\n=== 卫语句 ===");
    
    let num = Some(4);
    
    match num {
        Some(x) if x < 5 => println!("小于 5: {}", x),
        Some(x) => println!("大于等于 5: {}", x),
        None => println!("没有值"),
    }
    
    let x = 4;
    let y = false;
    
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

/// 运行所有控制流示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust 控制流示例             ║");
    println!("╚════════════════════════════════════╝");
    
    if_expressions_demo();
    loop_demo();
    while_demo();
    for_demo();
    match_demo();
    if_let_demo();
    while_let_demo();
    destructuring_demo();
    guard_demo();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_if_expression() {
        let condition = true;
        let number = if condition { 5 } else { 6 };
        assert_eq!(number, 5);
    }
    
    #[test]
    fn test_loop_return() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        assert_eq!(result, 20);
    }
    
    #[test]
    fn test_match() {
        let x = 5;
        let result = match x {
            1..=5 => "small",
            _ => "large",
        };
        assert_eq!(result, "small");
    }
}
