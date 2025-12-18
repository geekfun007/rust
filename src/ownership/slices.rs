// 切片 (Slices)

/// # 字符串切片
pub fn string_slices_demo() {
    println!("\n=== 字符串切片 ===");
    
    let s = String::from("hello world");
    
    // 创建切片
    let hello = &s[0..5];
    let world = &s[6..11];
    
    println!("完整字符串: {}", s);
    println!("切片: '{}' '{}'", hello, world);
    
    // 切片语法
    let s = String::from("hello");
    
    let _slice = &s[0..2];  // 从 0 到 2（不包括 2）
    let _slice = &s[..2];   // 等同于 &s[0..2]
    
    let _slice = &s[3..5];
    let _slice = &s[3..];   // 从 3 到结尾
    
    let slice = &s[..];    // 整个字符串
    
    println!("切片: {}", slice);
    
    // 字符串字面量就是切片
    let s: &str = "Hello, world!";
    println!("字符串字面量: {}", s);
}

/// # 切片的类型
pub fn slice_types_demo() {
    println!("\n=== 切片的类型 ===");
    
    let s = String::from("hello world");
    
    // &str 是字符串切片类型
    let hello: &str = &s[0..5];
    println!("&str 类型: {}", hello);
    
    // String vs &str
    println!("\nString vs &str:");
    println!("  String: 拥有所有权的字符串");
    println!("  &str: 字符串切片引用");
    
    // &str 可以引用 String 的一部分
    let s = String::from("hello");
    let slice: &str = &s;
    println!("完整 String 的切片: {}", slice);
    
    // 字符串字面量的类型是 &str
    let literal: &str = "hello";
    println!("字符串字面量: {}", literal);
}

/// # 切片作为函数参数
pub fn slices_as_parameters_demo() {
    println!("\n=== 切片作为函数参数 ===");
    
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    // 可以传递 String 的切片
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("第一个单词: {}", word);
    
    // 也可以传递字符串字面量
    let word = first_word("hello world");
    println!("第一个单词: {}", word);
    
    // 更灵活的 API
    fn print_slice(s: &str) {
        println!("  切片内容: {}", s);
    }
    
    let s = String::from("hello");
    print_slice(&s);        // String 的切片
    print_slice(&s[..3]);   // 部分切片
    print_slice("world");   // 字符串字面量
}

/// # 数组切片
pub fn array_slices_demo() {
    println!("\n=== 数组切片 ===");
    
    let a = [1, 2, 3, 4, 5];
    
    // 创建数组切片
    let slice = &a[1..3];
    println!("数组: {:?}", a);
    println!("切片 [1..3]: {:?}", slice);
    
    // &[T] 是数组切片类型
    let slice: &[i32] = &a[..];
    println!("完整切片: {:?}", slice);
    
    // 切片操作
    let slice = &a[..2];   // 前两个元素
    println!("前两个: {:?}", slice);
    
    let slice = &a[2..];   // 从第三个开始
    println!("从第三个: {:?}", slice);
    
    let slice = &a[1..4];  // 中间部分
    println!("中间部分: {:?}", slice);
}

/// # 数组切片方法
pub fn array_slice_methods_demo() {
    println!("\n=== 数组切片方法 ===");
    
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &numbers[..];
    
    // 长度
    println!("长度: {}", slice.len());
    println!("是否为空: {}", slice.is_empty());
    
    // 访问元素
    println!("第一个: {:?}", slice.first());
    println!("最后一个: {:?}", slice.last());
    println!("索引 3: {:?}", slice.get(3));
    
    // 分割
    let (left, right) = slice.split_at(5);
    println!("\nsplit_at(5):");
    println!("  左: {:?}", left);
    println!("  右: {:?}", right);
    
    // 迭代
    println!("\n迭代:");
    for item in slice.iter() {
        print!("{} ", item);
    }
    println!();
    
    // 查找
    println!("\n查找:");
    println!("包含 5: {}", slice.contains(&5));
    println!("5 的位置: {:?}", slice.iter().position(|&x| x == 5));
    
    // windows 和 chunks
    println!("\nwindows(3):");
    for window in slice.windows(3) {
        println!("  {:?}", window);
    }
    
    println!("\nchunks(3):");
    for chunk in slice.chunks(3) {
        println!("  {:?}", chunk);
    }
}

/// # Vec 切片
pub fn vec_slices_demo() {
    println!("\n=== Vec 切片 ===");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // Vec 可以被切片
    let slice = &v[1..4];
    println!("Vec: {:?}", v);
    println!("切片 [1..4]: {:?}", slice);
    
    // 函数可以接受切片
    fn sum_slice(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }
    
    println!("切片求和: {}", sum_slice(&v[1..4]));
    println!("全部求和: {}", sum_slice(&v));
    
    // 可变切片
    let mut v = vec![1, 2, 3, 4, 5];
    let slice = &mut v[1..4];
    
    for item in slice {
        *item *= 2;
    }
    
    println!("修改后: {:?}", v);
}

/// # 切片模式匹配
pub fn slice_pattern_matching_demo() {
    println!("\n=== 切片模式匹配 ===");
    
    fn analyze_slice(slice: &[i32]) {
        match slice {
            [] => println!("  空切片"),
            [x] => println!("  单个元素: {}", x),
            [x, y] => println!("  两个元素: {}, {}", x, y),
            [x, y, z] => println!("  三个元素: {}, {}, {}", x, y, z),
            [first, .., last] => {
                println!("  第一个: {}, 最后一个: {}, 长度: {}", first, last, slice.len())
            }
        }
    }
    
    analyze_slice(&[]);
    analyze_slice(&[1]);
    analyze_slice(&[1, 2]);
    analyze_slice(&[1, 2, 3]);
    analyze_slice(&[1, 2, 3, 4, 5]);
}

/// # 切片与借用检查
pub fn slice_borrow_checking_demo() {
    println!("\n=== 切片与借用检查 ===");
    
    let mut s = String::from("hello world");
    
    let word = first_word(&s);
    // s.clear();  // 错误！不能在有不可变借用时修改
    
    println!("第一个单词: {}", word);
    
    // 现在可以修改了
    s.clear();
    println!("清空后: '{}'", s);
    
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
}

/// # 切片的内部表示
pub fn slice_internals_demo() {
    println!("\n=== 切片的内部表示 ===");
    
    let s = String::from("hello world");
    let slice = &s[0..5];
    
    println!("String:");
    println!("  地址: {:p}", &s);
    println!("  长度: {}", s.len());
    println!("  容量: {}", s.capacity());
    
    println!("\n切片 (&str):");
    println!("  指向的数据: {:p}", slice.as_ptr());
    println!("  长度: {}", slice.len());
    println!("  大小: {} 字节（指针 + 长度）", std::mem::size_of_val(&slice));
    
    println!("\n切片包含:");
    println!("  1. 指向数据的指针");
    println!("  2. 长度");
    println!("  不包含容量（与 String 不同）");
}

/// # 切片的实际应用
pub fn practical_slices_demo() {
    println!("\n=== 切片的实际应用 ===");
    
    // 1. 字符串处理
    fn get_file_extension(filename: &str) -> Option<&str> {
        filename.rfind('.').map(|i| &filename[i + 1..])
    }
    
    println!("1. 文件扩展名:");
    println!("  file.txt -> {:?}", get_file_extension("file.txt"));
    println!("  image.png -> {:?}", get_file_extension("image.png"));
    
    // 2. 分割字符串
    fn split_words(text: &str) -> Vec<&str> {
        text.split_whitespace().collect()
    }
    
    println!("\n2. 分割单词:");
    let words = split_words("hello world rust programming");
    for word in words {
        println!("  {}", word);
    }
    
    // 3. 范围查询
    fn get_range(data: &[i32], start: usize, end: usize) -> &[i32] {
        &data[start..end]
    }
    
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("\n3. 范围查询:");
    println!("  [3..7]: {:?}", get_range(&data, 3, 7));
    
    // 4. 窗口操作
    fn moving_average(data: &[f64], window: usize) -> Vec<f64> {
        data.windows(window)
            .map(|w| w.iter().sum::<f64>() / window as f64)
            .collect()
    }
    
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    println!("\n4. 移动平均（窗口=3）:");
    println!("  {:?}", moving_average(&data, 3));
}

/// # 切片的性能
pub fn slice_performance_demo() {
    println!("\n=== 切片的性能 ===");
    
    println!("切片的优势:");
    println!("  1. 零成本抽象");
    println!("     - 只是指针和长度");
    println!("     - 不复制数据");
    
    println!("\n  2. 灵活性");
    println!("     - 可以引用数组、Vec、String 等");
    println!("     - 统一的接口");
    
    println!("\n  3. 安全性");
    println!("     - 边界检查");
    println!("     - 借用规则保护");
    
    let large_vec = vec![1; 1_000_000];
    
    // 传递切片：只传递指针和长度
    fn process_slice(slice: &[i32]) -> usize {
        slice.len()
    }
    
    let len = process_slice(&large_vec);
    println!("\n处理 {} 个元素的 Vec", len);
    println!("只传递了 {} 字节（切片大小）", std::mem::size_of::<&[i32]>());
}

/// # 切片最佳实践
pub fn slice_best_practices_demo() {
    println!("\n=== 切片最佳实践 ===");
    
    println!("1. 函数参数优先使用切片");
    println!("   - 使用 &str 而不是 &String");
    println!("   - 使用 &[T] 而不是 &Vec<T>");
    println!("   - 更灵活，更通用");
    
    // 不好
    fn bad_function(s: &String) {
        println!("{}", s);
    }
    
    // 好
    fn good_function(s: &str) {
        println!("{}", s);
    }
    
    let s = String::from("hello");
    good_function(&s);          // 可以传递 String
    good_function("world");     // 也可以传递字面量
    
    println!("\n2. 避免不必要的索引");
    println!("   - 使用迭代器而不是索引");
    println!("   - 更安全，更高效");
    
    let numbers = [1, 2, 3, 4, 5];
    
    // 不太好
    // for i in 0..numbers.len() {
    //     println!("{}", numbers[i]);
    // }
    
    // 好
    for &num in &numbers {
        print!("{} ", num);
    }
    println!();
    
    println!("\n3. 使用模式匹配");
    println!("   - 更清晰的代码");
    println!("   - 编译时检查");
}

/// 运行所有切片示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║          Rust 切片详解             ║");
    println!("╚════════════════════════════════════╝");
    
    string_slices_demo();
    slice_types_demo();
    slices_as_parameters_demo();
    array_slices_demo();
    array_slice_methods_demo();
    vec_slices_demo();
    slice_pattern_matching_demo();
    slice_borrow_checking_demo();
    slice_internals_demo();
    practical_slices_demo();
    slice_performance_demo();
    slice_best_practices_demo();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_string_slice() {
        let s = String::from("hello");
        let slice = &s[0..2];
        assert_eq!(slice, "he");
    }
    
    #[test]
    fn test_array_slice() {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
    
    #[test]
    fn test_vec_slice() {
        let v = vec![1, 2, 3, 4, 5];
        let slice = &v[2..];
        assert_eq!(slice, &[3, 4, 5]);
    }
}
