// Rust 迭代器详解示例

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║              Rust 迭代器与 for 循环详解                  ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
    
    iterator_basics_demo();
    for_loop_internals_demo();
    three_iteration_methods_demo();
    consumer_methods_demo();
    adapter_methods_demo();
    custom_iterator_demo();
    performance_demo();
    practical_patterns_demo();
}

/// 1. 迭代器基础
fn iterator_basics_demo() {
    println!("═══ 1. 迭代器基础 ═══\n");
    
    println!("Iterator trait 核心：");
    println!("  pub trait Iterator {{");
    println!("      type Item;");
    println!("      fn next(&mut self) -> Option<Self::Item>;");
    println!("  }}");
    
    // 手动使用 next
    println!("\n手动调用 next：");
    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    
    println!("  第一次 next: {:?}", iter.next());  // Some(&1)
    println!("  第二次 next: {:?}", iter.next());  // Some(&2)
    println!("  第三次 next: {:?}", iter.next());  // Some(&3)
    println!("  第四次 next: {:?}", iter.next());  // None
    
    // 迭代器是惰性的
    println!("\n迭代器是惰性的：");
    let v = vec![1, 2, 3];
    let doubled = v.iter().map(|x| {
        println!("  映射 {}", x);
        x * 2
    });
    println!("  创建了迭代器，但还没有执行");
    println!("  开始收集...");
    let _result: Vec<_> = doubled.collect();
}

/// 2. for 循环底层原理
fn for_loop_internals_demo() {
    println!("\n═══ 2. for 循环底层原理 ═══\n");
    
    let v = vec![1, 2, 3];
    
    println!("标准 for 循环：");
    println!("  for item in v {{ ... }}");
    println!();
    
    println!("展开后的等价代码：");
    println!("  {{");
    println!("      let mut iter = IntoIterator::into_iter(v);");
    println!("      loop {{");
    println!("          match iter.next() {{");
    println!("              Some(item) => {{ ... }},");
    println!("              None => break,");
    println!("          }}");
    println!("      }}");
    println!("  }}");
    
    // 实际演示
    println!("\n实际执行：");
    {
        let mut iter = v.into_iter();
        loop {
            match iter.next() {
                Some(item) => println!("  处理: {}", item),
                None => break,
            }
        }
    }
}

/// 3. 三种迭代方法
fn three_iteration_methods_demo() {
    println!("\n═══ 3. 三种迭代方法 ═══\n");
    
    // iter() - 不可变借用
    println!("1. iter() - 不可变借用：");
    let v = vec![1, 2, 3];
    for item in v.iter() {
        println!("  &i32: {}", item);
    }
    println!("  v 仍然有效: {:?}", v);
    
    // iter_mut() - 可变借用
    println!("\n2. iter_mut() - 可变借用：");
    let mut v = vec![1, 2, 3];
    println!("  修改前: {:?}", v);
    for item in v.iter_mut() {
        *item *= 2;
    }
    println!("  修改后: {:?}", v);
    
    // into_iter() - 获取所有权
    println!("\n3. into_iter() - 获取所有权：");
    let _v = vec![1, 2, 3];
    for item in _v.into_iter() {
        println!("  i32: {}", item);
    }
    println!("  v 已被移动，不能再使用");
    
    // 自动选择
    println!("\n自动选择迭代方法：");
    let v = vec![1, 2, 3];
    println!("  for x in v  → into_iter() (移动)");
    println!("  for x in &v → iter() (借用)");
    println!("  for x in &mut v → iter_mut() (可变借用)");
}

/// 4. 消费器方法演示
fn consumer_methods_demo() {
    println!("\n═══ 4. 消费器方法 ═══\n");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // 收集
    println!("收集方法：");
    let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
    println!("  collect: {:?}", doubled);
    
    let (even, odd): (Vec<&i32>, Vec<&i32>) = v.iter().partition(|&&x| x % 2 == 0);
    println!("  partition: even={:?}, odd={:?}", even, odd);
    
    // 聚合
    println!("\n聚合方法：");
    let sum: i32 = v.iter().sum();
    println!("  sum: {}", sum);
    
    let product: i32 = v.iter().product();
    println!("  product: {}", product);
    
    let sum_fold = v.iter().fold(0, |acc, x| acc + x);
    println!("  fold: {}", sum_fold);
    
    // 查找
    println!("\n查找方法：");
    let found = v.iter().find(|&&x| x > 3);
    println!("  find(x > 3): {:?}", found);
    
    let pos = v.iter().position(|&x| x == 3);
    println!("  position(x == 3): {:?}", pos);
    
    let any_even = v.iter().any(|&x| x % 2 == 0);
    println!("  any(even): {}", any_even);
    
    let all_positive = v.iter().all(|&x| x > 0);
    println!("  all(positive): {}", all_positive);
    
    // 最值
    println!("\n最值方法：");
    let max = v.iter().max();
    println!("  max: {:?}", max);
    
    let min = v.iter().min();
    println!("  min: {:?}", min);
    
    // 计数
    println!("\n计数方法：");
    let count = v.iter().count();
    println!("  count: {}", count);
    
    let nth = v.iter().nth(2);
    println!("  nth(2): {:?}", nth);
    
    let last = v.iter().last();
    println!("  last: {:?}", last);
}

/// 5. 适配器方法演示
fn adapter_methods_demo() {
    println!("\n═══ 5. 适配器方法 ═══\n");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // 转换
    println!("转换方法：");
    let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
    println!("  map(x * 2): {:?}", doubled);
    
    let parsed: Vec<i32> = vec!["1", "two", "3"]
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("  filter_map(parse): {:?}", parsed);
    
    let nested = vec![vec![1, 2], vec![3, 4]];
    let flattened: Vec<_> = nested.iter().flatten().collect();
    println!("  flatten: {:?}", flattened);
    
    // 过滤
    println!("\n过滤方法：");
    let even: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("  filter(even): {:?}", even);
    
    let first_three: Vec<_> = v.iter().take(3).collect();
    println!("  take(3): {:?}", first_three);
    
    let taken_while: Vec<_> = v.iter().take_while(|&&x| x < 4).collect();
    println!("  take_while(< 4): {:?}", taken_while);
    
    let skipped: Vec<_> = v.iter().skip(2).collect();
    println!("  skip(2): {:?}", skipped);
    
    // 组合
    println!("\n组合方法：");
    let v2 = vec![6, 7, 8];
    let chained: Vec<_> = v.iter().chain(v2.iter()).collect();
    println!("  chain: {:?}", chained);
    
    let zipped: Vec<_> = v.iter().zip(v2.iter()).collect();
    println!("  zip: {:?}", zipped);
    
    let enumerated: Vec<_> = v.iter().enumerate().collect();
    println!("  enumerate: {:?}", enumerated);
    
    // 其他
    println!("\n其他方法：");
    let cycled: Vec<_> = v.iter().cycle().take(8).collect();
    println!("  cycle().take(8): {:?}", cycled);
    
    let reversed: Vec<_> = v.iter().rev().collect();
    println!("  rev: {:?}", reversed);
    
    println!("\ninspect (调试)：");
    let sum: i32 = v.iter()
        .inspect(|x| println!("  处理: {}", x))
        .sum();
    println!("  结果: {}", sum);
}

/// 6. 自定义迭代器演示
fn custom_iterator_demo() {
    println!("\n═══ 6. 自定义迭代器 ═══\n");
    
    // 简单计数器
    println!("简单计数器：");
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
            self.count += 1;
            if self.count <= self.max {
                Some(self.count)
            } else {
                None
            }
        }
    }
    
    print!("  Counter(5): ");
    for n in Counter::new(5) {
        print!("{} ", n);
    }
    println!();
    
    // 斐波那契数列
    println!("\n斐波那契数列：");
    struct Fibonacci {
        curr: u64,
        next: u64,
    }
    
    impl Fibonacci {
        fn new() -> Self {
            Fibonacci { curr: 0, next: 1 }
        }
    }
    
    impl Iterator for Fibonacci {
        type Item = u64;
        
        fn next(&mut self) -> Option<Self::Item> {
            let current = self.curr;
            self.curr = self.next;
            self.next = current + self.next;
            
            // 防止溢出
            if current > u64::MAX / 2 {
                None
            } else {
                Some(current)
            }
        }
    }
    
    print!("  前 10 个斐波那契数: ");
    for n in Fibonacci::new().take(10) {
        print!("{} ", n);
    }
    println!();
    
    // 自定义步长
    println!("\n自定义步长迭代器：");
    struct StepRange {
        current: i32,
        end: i32,
        step: i32,
    }
    
    impl StepRange {
        fn new(start: i32, end: i32, step: i32) -> Self {
            StepRange { current: start, end, step }
        }
    }
    
    impl Iterator for StepRange {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let result = self.current;
                self.current += self.step;
                Some(result)
            } else {
                None
            }
        }
    }
    
    print!("  StepRange(0, 10, 2): ");
    for n in StepRange::new(0, 10, 2) {
        print!("{} ", n);
    }
    println!();
}

/// 7. 性能演示
fn performance_demo() {
    println!("\n═══ 7. 性能分析 ═══\n");
    
    use std::time::Instant;
    
    let data: Vec<i32> = (0..100_000).collect();
    
    // 迭代器链
    let start = Instant::now();
    let sum1: i64 = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x as i64 * 2)
        .sum();
    let duration1 = start.elapsed();
    
    // 手写循环
    let start = Instant::now();
    let mut sum2: i64 = 0;
    for &x in &data {
        if x % 2 == 0 {
            sum2 += x as i64 * 2;
        }
    }
    let duration2 = start.elapsed();
    
    println!("零成本抽象验证：");
    println!("  数据量: {} 个元素", data.len());
    println!("  迭代器链: {:?}, 结果: {}", duration1, sum1);
    println!("  手写循环: {:?}, 结果: {}", duration2, sum2);
    println!("  性能差异: 几乎相同（零成本抽象）");
    
    println!("\n避免不必要的分配：");
    println!("  ❌ 慢：");
    println!("     data.iter().map().collect()");
    println!("       .iter().filter().collect()");
    println!("     // 两次分配");
    println!();
    println!("  ✓ 快：");
    println!("     data.iter().map().filter().collect()");
    println!("     // 一次分配");
}

/// 8. 实战模式演示
fn practical_patterns_demo() {
    println!("\n═══ 8. 实战模式 ═══\n");
    
    // 数据处理管道
    println!("1. 数据处理管道：");
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<_> = data.iter()
        .filter(|&&x| x % 2 == 0)    // 偶数
        .map(|&x| x * x)              // 平方
        .take(3)                      // 前 3 个
        .collect();
    println!("  偶数平方前3个: {:?}", result);
    
    // 分组统计
    println!("\n2. 分组统计：");
    use std::collections::HashMap;
    let words = vec!["hello", "world", "hello", "rust", "world", "hello"];
    let counts: HashMap<&str, usize> = words.iter()
        .fold(HashMap::new(), |mut acc, &word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        });
    println!("  词频统计: {:?}", counts);
    
    // 窗口处理
    println!("\n3. 窗口处理：");
    let data = vec![1, 2, 3, 4, 5];
    let windows: Vec<_> = data.windows(2).collect();
    println!("  滑动窗口(2): {:?}", windows);
    
    let chunks: Vec<_> = data.chunks(2).collect();
    println!("  分块(2): {:?}", chunks);
    
    // 错误处理
    println!("\n4. 错误处理：");
    let results: Vec<Result<i32, &str>> = vec![
        Ok(1), Err("error"), Ok(2), Ok(3)
    ];
    
    let successes: Vec<_> = results.iter()
        .filter_map(|r| r.as_ref().ok())
        .collect();
    println!("  成功结果: {:?}", successes);
    
    // 链式转换
    println!("\n5. 复杂链式转换：");
    let text = "hello world rust programming";
    let result: Vec<_> = text
        .split_whitespace()           // 分词
        .map(|word| word.chars()
            .next()
            .unwrap()
            .to_uppercase()
            .to_string())             // 首字母大写
        .collect();
    println!("  首字母: {:?}", result);
    
    // 笛卡尔积
    println!("\n6. 笛卡尔积：");
    let v1 = vec![1, 2];
    let v2 = vec!['a', 'b', 'c'];
    let cartesian: Vec<_> = v1.iter()
        .flat_map(|&x| v2.iter().map(move |&y| (x, y)))
        .collect();
    println!("  笛卡尔积: {:?}", cartesian);
    
    // 延迟计算
    println!("\n7. 延迟计算（无限序列）：");
    let fibonacci = std::iter::successors(Some((0u64, 1u64)), |&(a, b)| {
        Some((b, a + b))
    }).map(|(a, _)| a);
    
    let first_10: Vec<_> = fibonacci.take(10).collect();
    println!("  前10个斐波那契数: {:?}", first_10);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_iterator_basics() {
        let v = vec![1, 2, 3];
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 6);
    }
    
    #[test]
    fn test_map_filter() {
        let v = vec![1, 2, 3, 4, 5];
        let result: Vec<_> = v.iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * 2)
            .collect();
        assert_eq!(result, vec![4, 8]);
    }
    
    #[test]
    fn test_custom_iterator() {
        struct Counter { count: u32, max: u32 }
        impl Counter {
            fn new(max: u32) -> Self { Counter { count: 0, max } }
        }
        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;
                if self.count <= self.max {
                    Some(self.count)
                } else {
                    None
                }
            }
        }
        
        let sum: u32 = Counter::new(5).sum();
        assert_eq!(sum, 15);
    }
    
    #[test]
    fn test_chain_zip() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6];
        
        let chained: Vec<_> = v1.iter()
            .chain(v2.iter())
            .copied()
            .collect();
        assert_eq!(chained, vec![1, 2, 3, 4, 5, 6]);
        
        let zipped: Vec<_> = v1.iter()
            .zip(v2.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        assert_eq!(zipped, vec![5, 7, 9]);
    }
    
    #[test]
    fn test_fold_reduce() {
        let v = vec![1, 2, 3, 4, 5];
        
        let sum = v.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
        
        let sum = v.iter().copied().reduce(|acc, x| acc + x);
        assert_eq!(sum, Some(15));
    }
}
