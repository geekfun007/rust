// 借用检查器深度演示

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║              借用检查器底层原理演示                      ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
    
    basic_borrowing_demo();
    nll_demo();
    borrowing_scopes_demo();
    mutable_borrowing_rules_demo();
    interior_mutability_demo();
    split_borrowing_demo();
}

/// 1. 基本借用规则
fn basic_borrowing_demo() {
    println!("═══ 1. 基本借用规则 ═══\n");
    
    let mut s = String::from("hello");
    
    println!("✓ 允许：多个不可变引用");
    {
        let r1 = &s;
        let r2 = &s;
        println!("  r1: {}, r2: {}", r1, r2);
        // r1 和 r2 作用域结束
    }
    
    println!("\n✓ 允许：单个可变引用");
    {
        let r3 = &mut s;
        r3.push_str(" world");
        println!("  r3: {}", r3);
        // r3 作用域结束
    }
    
    println!("\n编译器保证：");
    println!("  1. 不可变引用期间数据不会改变");
    println!("  2. 可变引用期间独占访问");
    println!("  3. 没有数据竞争");
}

/// 2. 非词法作用域生命周期 (NLL)
fn nll_demo() {
    println!("\n═══ 2. 非词法作用域生命周期 (NLL) ═══\n");
    
    let mut s = String::from("hello");
    
    println!("旧版 Rust (Pre-2018):");
    println!("  let r = &s;");
    println!("  println!(\"{{}}\", r);  // r 最后使用");
    println!("  let mr = &mut s;        // ❌ 错误：r 还在作用域内");
    
    println!("\n新版 Rust (2018+) - NLL:");
    println!("  let r = &s;");
    println!("  println!(\"{{}}\", r);  // r 最后使用，生命周期结束");
    println!("  let mr = &mut s;        // ✓ OK：r 生命周期已结束");
    
    // 实际演示
    let r = &s;
    println!("\n实际运行:");
    println!("  不可变引用: {}", r);
    // r 最后一次使用 ↑，生命周期结束
    
    let mr = &mut s;  // ✓ OK
    mr.push_str("!");
    println!("  可变引用: {}", mr);
    
    println!("\n关键：生命周期由最后使用决定，不是作用域！");
}

/// 3. 借用作用域可视化
fn borrowing_scopes_demo() {
    println!("\n═══ 3. 借用作用域可视化 ═══\n");
    
    println!("生命周期分析：");
    println!();
    println!("  let mut data = vec![1, 2, 3];");
    println!("  ├─ data 的所有权开始");
    println!("  │");
    println!("  let r1 = &data;");
    println!("  ├─ r1 借用开始 ──┐");
    println!("  let r2 = &data;  │");
    println!("  ├─ r2 借用开始 ──┼──┐");
    println!("  println!(r1, r2);│  │");
    println!("  └─ r1 最后使用 ──┘  │");
    println!("    └─ r2 最后使用 ───┘");
    println!("  ");
    println!("  let r3 = &mut data;");
    println!("  ├─ r3 可变借用 ─────┐");
    println!("  r3.push(4);         │");
    println!("  └─ r3 最后使用 ─────┘");
    
    // 实际执行
    let mut data = vec![1, 2, 3];
    let r1 = &data;
    let r2 = &data;
    println!("\n实际输出:");
    println!("  r1: {:?}, r2: {:?}", r1, r2);
    
    let r3 = &mut data;
    r3.push(4);
    println!("  r3: {:?}", r3);
}

/// 4. 可变借用规则
fn mutable_borrowing_rules_demo() {
    println!("\n═══ 4. 可变借用规则详解 ═══\n");
    
    let mut value = 10;
    
    println!("场景1：修改后立即使用");
    {
        let r = &mut value;
        *r += 1;
        println!("  修改后: {}", r);
    }
    println!("  原值: {}", value);
    
    println!("\n场景2：多个可变引用（不重叠）");
    {
        let r1 = &mut value;
        *r1 += 1;
        println!("  第一次修改: {}", r1);
    }  // r1 结束
    {
        let r2 = &mut value;
        *r2 += 1;
        println!("  第二次修改: {}", r2);
    }  // r2 结束
    
    println!("\n场景3：函数参数中的可变借用");
    fn increment(x: &mut i32) {
        *x += 1;
    }
    increment(&mut value);
    println!("  函数修改后: {}", value);
    
    println!("\n借用规则总结：");
    println!("  ✓ 同一时刻只能有一个 &mut");
    println!("  ✓ &mut 和 & 不能同时存在");
    println!("  ✓ 多个 & 可以共存");
}

/// 5. 内部可变性
fn interior_mutability_demo() {
    println!("\n═══ 5. 内部可变性 (RefCell) ═══\n");
    
    use std::cell::RefCell;
    
    println!("RefCell 提供运行时借用检查：");
    
    let data = RefCell::new(vec![1, 2, 3]);
    
    println!("\n✓ 正常借用：");
    {
        let r1 = data.borrow();
        let r2 = data.borrow();
        println!("  不可变借用 r1: {:?}", r1);
        println!("  不可变借用 r2: {:?}", r2);
    }  // r1, r2 结束
    
    {
        let mut r3 = data.borrow_mut();
        r3.push(4);
        println!("  可变借用 r3: {:?}", r3);
    }  // r3 结束
    
    println!("\n❌ 违反规则会在运行时 panic：");
    println!("  let r1 = data.borrow();");
    println!("  let r2 = data.borrow_mut();  // panic: already borrowed");
    
    println!("\nRefCell 内存布局：");
    println!("  ┌──────────────┐");
    println!("  │ borrow: isize│ ← 借用计数");
    println!("  │   0: 未借用  │");
    println!("  │   n>0: n个&  │");
    println!("  │   -1: 一个&mut│");
    println!("  ├──────────────┤");
    println!("  │ value: T     │ ← 实际数据");
    println!("  └──────────────┘");
    
    println!("\n开销：");
    println!("  ✓ 编译时检查 → 运行时检查");
    println!("  ✗ 有性能损耗（原子操作）");
    println!("  ✓ 允许在不可变环境中修改数据");
}

/// 6. 分割借用
fn split_borrowing_demo() {
    println!("\n═══ 6. 分割借用 ═══\n");
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    println!("✓ 允许：同时可变借用不同部分");
    let (first_half, second_half) = data.split_at_mut(2);
    println!("  第一部分: {:?}", first_half);
    println!("  第二部分: {:?}", second_half);
    
    first_half[0] = 10;
    second_half[0] = 30;
    println!("  修改后: {:?}", data);
    
    println!("\n结构体字段分割借用：");
    struct Point {
        x: i32,
        y: i32,
    }
    
    let mut p = Point { x: 1, y: 2 };
    let rx = &mut p.x;
    let ry = &mut p.y;
    
    *rx += 1;
    *ry += 1;
    println!("  Point {{ x: {}, y: {} }}", p.x, p.y);
    
    println!("\n原理：不同字段/元素不会互相干扰");
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    
    #[test]
    fn test_borrowing() {
        let s = String::from("test");
        let r1 = &s;
        let r2 = &s;
        assert_eq!(r1, r2);
    }
    
    #[test]
    fn test_mutable_borrowing() {
        let mut v = vec![1, 2, 3];
        let r = &mut v;
        r.push(4);
        assert_eq!(r.len(), 4);
    }
    
    #[test]
    fn test_interior_mutability() {
        let data = RefCell::new(5);
        *data.borrow_mut() += 1;
        assert_eq!(*data.borrow(), 6);
    }
    
    #[test]
    #[should_panic(expected = "already borrowed")]
    fn test_refcell_panic() {
        let data = RefCell::new(5);
        let _r1 = data.borrow();
        let _r2 = data.borrow_mut();  // 应该 panic
    }
}
