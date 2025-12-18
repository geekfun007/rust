// 基础数据类型概览

/// # 复合类型概览
pub fn compound_types_demo() {
    println!("\n=== 复合类型概览 ===");
    
    // 元组
    let tuple: (i32, f64, char) = (500, 6.4, 'x');
    println!("元组: {:?}", tuple);
    println!("访问元素: {}, {}, {}", tuple.0, tuple.1, tuple.2);
    
    // 解构
    let (x, y, z) = tuple;
    println!("解构: x={}, y={}, z={}", x, y, z);
    
    // 数组 - 固定长度
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组: {:?}", array);
    println!("数组长度: {}", array.len());
    
    // 相同值初始化
    let zeros = [0; 10];
    println!("零数组: {:?}", zeros);
    
    // 访问元素
    println!("第一个元素: {}", array[0]);
}

/// 运行所有数据类型示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║      Rust 基础数据类型概览         ║");
    println!("╚════════════════════════════════════╝");
    
    compound_types_demo();
}
