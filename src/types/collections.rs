// 集合类型：tuple, array, Vec, HashMap, HashSet

use std::collections::{HashMap, HashSet, VecDeque, BTreeMap, BTreeSet, LinkedList};

/// # 元组 (Tuple)
pub fn tuple_demo() {
    println!("\n=== 元组 ===");
    
    // 创建元组
    let tuple: (i32, f64, char) = (500, 6.4, 'x');
    println!("元组: {:?}", tuple);
    
    // 访问元素
    println!("第0个元素: {}", tuple.0);
    println!("第1个元素: {}", tuple.1);
    println!("第2个元素: {}", tuple.2);
    
    // 解构
    let (x, y, z) = tuple;
    println!("解构: x={}, y={}, z={}", x, y, z);
    
    // 单元素元组（需要逗号）
    let single = (5,);
    println!("单元素元组: {:?}", single);
    
    // 空元组（unit type）
    let unit = ();
    println!("空元组: {:?}", unit);
    
    // 嵌套元组
    let nested = ((1, 2), (3, 4));
    println!("嵌套元组: {:?}", nested);
    println!("访问嵌套: {}", (nested.0).0);
    
    // 元组作为函数返回值
    fn swap(pair: (i32, i32)) -> (i32, i32) {
        let (a, b) = pair;
        (b, a)
    }
    
    let pair = (1, 2);
    println!("交换前: {:?}", pair);
    let swapped = swap(pair);
    println!("交换后: {:?}", swapped);
}

/// # 数组 (Array)
pub fn array_demo() {
    println!("\n=== 数组 ===");
    
    // 创建数组
    let arr = [1, 2, 3, 4, 5];
    println!("数组: {:?}", arr);
    println!("长度: {}", arr.len());
    
    // 类型标注
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    // 相同值初始化
    let arr = [0; 10];
    println!("初始化数组: {:?}", arr);
    
    // 访问元素
    let first = arr[0];
    let last = arr[arr.len() - 1];
    println!("第一个: {}, 最后一个: {}", first, last);
    
    // 切片
    let slice = &arr[1..4];
    println!("切片 [1..4]: {:?}", slice);
    
    // 遍历
    println!("遍历:");
    for element in arr.iter() {
        print!("{} ", element);
    }
    println!();
    
    // 带索引遍历
    println!("带索引遍历:");
    for (i, element) in arr.iter().enumerate() {
        println!("  arr[{}] = {}", i, element);
    }
    
    // 数组方法
    let arr = [3, 1, 4, 1, 5, 9, 2, 6];
    println!("\n数组方法:");
    println!("contains(5): {}", arr.contains(&5));
    println!("iter().sum(): {}", arr.iter().sum::<i32>());
    println!("iter().max(): {:?}", arr.iter().max());
    println!("iter().min(): {:?}", arr.iter().min());
}

/// # 动态数组 (Vec)
pub fn vec_demo() {
    println!("\n=== Vec 动态数组 ===");
    
    // 创建 Vec
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    println!("Vec: {:?}", v2);
    
    // 添加元素
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("push后: {:?}", v);
    
    // 预分配容量
    let mut v = Vec::with_capacity(10);
    println!("初始容量: {}", v.capacity());
    v.push(1);
    println!("添加后容量: {} 长度: {}", v.capacity(), v.len());
    
    // 访问元素
    let v = vec![1, 2, 3, 4, 5];
    let third = v[2];
    println!("第3个元素: {}", third);
    
    // 安全访问（返回 Option）
    match v.get(2) {
        Some(third) => println!("get(2): {}", third),
        None => println!("没有第3个元素"),
    }
    
    match v.get(10) {
        Some(element) => println!("get(10): {}", element),
        None => println!("get(10): 索引越界"),
    }
    
    // 遍历
    println!("\n遍历:");
    for i in &v {
        print!("{} ", i);
    }
    println!();
    
    // 可变遍历
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i *= 2;
    }
    println!("乘以2后: {:?}", v);
}

/// # Vec 方法
pub fn vec_methods_demo() {
    println!("\n=== Vec 方法 ===");
    
    let mut v = vec![1, 2, 3, 4, 5];
    
    // 删除元素
    let last = v.pop();
    println!("pop: {:?}, 剩余: {:?}", last, v);
    
    v.remove(1);  // 删除索引1的元素
    println!("remove(1): {:?}", v);
    
    // 插入元素
    v.insert(1, 10);
    println!("insert(1, 10): {:?}", v);
    
    // 追加
    v.append(&mut vec![6, 7, 8]);
    println!("append: {:?}", v);
    
    // 清空
    let mut v = vec![1, 2, 3];
    v.clear();
    println!("clear: {:?}", v);
    
    // 切片
    let v = vec![1, 2, 3, 4, 5];
    let slice = &v[1..4];
    println!("切片 [1..4]: {:?}", slice);
    
    // 排序
    let mut v = vec![5, 3, 1, 4, 2];
    v.sort();
    println!("sort: {:?}", v);
    
    v.reverse();
    println!("reverse: {:?}", v);
    
    // 去重
    let mut v = vec![1, 2, 2, 3, 3, 3];
    v.dedup();
    println!("dedup: {:?}", v);
    
    // 连接
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = [v1, v2].concat();
    println!("concat: {:?}", v3);
    
    // 分割
    let v = vec![1, 2, 3, 4, 5];
    let (left, right) = v.split_at(2);
    println!("split_at(2): {:?} {:?}", left, right);
    
    // 查找
    let v = vec![1, 2, 3, 4, 5];
    println!("contains(&3): {}", v.contains(&3));
    
    match v.iter().position(|&x| x == 3) {
        Some(index) => println!("position(3): {}", index),
        None => println!("未找到"),
    }
    
    // 过滤
    let v = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter偶数: {:?}", evens);
}

/// # HashMap
pub fn hashmap_demo() {
    println!("\n=== HashMap ===");
    
    // 创建 HashMap
    let mut scores = HashMap::new();
    
    // 插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("HashMap: {:?}", scores);
    
    // 访问值
    let team = String::from("Blue");
    let score = scores.get(&team);
    println!("Blue 的分数: {:?}", score);
    
    match scores.get(&team) {
        Some(score) => println!("找到分数: {}", score),
        None => println!("未找到"),
    }
    
    // 遍历
    println!("\n遍历:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }
    
    // 更新值
    scores.insert(String::from("Blue"), 25);
    println!("更新后: {:?}", scores);
    
    // 只在键不存在时插入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(100);  // Blue已存在，不会插入
    println!("entry/or_insert: {:?}", scores);
    
    // 基于旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("\n单词计数: {:?}", map);
}

/// # HashMap 方法
pub fn hashmap_methods_demo() {
    println!("\n=== HashMap 方法 ===");
    
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    // 长度
    println!("len: {}", map.len());
    println!("is_empty: {}", map.is_empty());
    
    // 包含
    println!("contains_key(\"a\"): {}", map.contains_key("a"));
    println!("contains_key(\"d\"): {}", map.contains_key("d"));
    
    // 删除
    let removed = map.remove("b");
    println!("remove(\"b\"): {:?}, 剩余: {:?}", removed, map);
    
    // 清空
    map.clear();
    println!("clear后: {:?}", map);
    
    // 从Vec创建
    let teams = vec![String::from("Blue"), String::from("Red")];
    let scores = vec![10, 50];
    let map: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("\n从Vec创建: {:?}", map);
    
    // 获取所有键
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    println!("\n所有键: {:?}", map.keys());
    println!("所有值: {:?}", map.values());
}

/// # HashSet
pub fn hashset_demo() {
    println!("\n=== HashSet ===");
    
    // 创建 HashSet
    let mut set = HashSet::new();
    
    // 插入元素
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(2);  // 重复，不会插入
    println!("HashSet: {:?}", set);
    println!("长度: {}", set.len());
    
    // 包含
    println!("contains(2): {}", set.contains(&2));
    println!("contains(4): {}", set.contains(&4));
    
    // 删除
    set.remove(&2);
    println!("remove(2): {:?}", set);
    
    // 遍历
    println!("遍历:");
    for value in &set {
        print!("{} ", value);
    }
    println!();
    
    // 集合操作
    let set1: HashSet<_> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<_> = [3, 4, 5, 6].iter().cloned().collect();
    
    println!("\nset1: {:?}", set1);
    println!("set2: {:?}", set2);
    
    // 并集
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("并集: {:?}", union);
    
    // 交集
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("交集: {:?}", intersection);
    
    // 差集
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("差集: {:?}", difference);
    
    // 对称差集
    let symmetric_difference: HashSet<_> = 
        set1.symmetric_difference(&set2).cloned().collect();
    println!("对称差集: {:?}", symmetric_difference);
    
    // 子集和超集
    let set3: HashSet<_> = [1, 2].iter().cloned().collect();
    println!("\nset3 是 set1 的子集: {}", set3.is_subset(&set1));
    println!("set1 是 set3 的超集: {}", set1.is_superset(&set3));
}

/// # 其他集合类型
pub fn other_collections_demo() {
    println!("\n=== 其他集合类型 ===");
    
    // VecDeque - 双端队列
    println!("VecDeque:");
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("  {:?}", deque);
    
    let front = deque.pop_front();
    let back = deque.pop_back();
    println!("  pop_front: {:?}, pop_back: {:?}", front, back);
    println!("  剩余: {:?}", deque);
    
    // BTreeMap - 有序映射
    println!("\nBTreeMap:");
    let mut btree = BTreeMap::new();
    btree.insert(3, "three");
    btree.insert(1, "one");
    btree.insert(2, "two");
    println!("  {:?}", btree);  // 按键排序
    
    // BTreeSet - 有序集合
    println!("\nBTreeSet:");
    let mut btree_set = BTreeSet::new();
    btree_set.insert(3);
    btree_set.insert(1);
    btree_set.insert(2);
    println!("  {:?}", btree_set);  // 有序
    
    // LinkedList - 链表
    println!("\nLinkedList:");
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    println!("  {:?}", list);
}

/// # 数组 vs Vec 对比
pub fn array_vs_vec_demo() {
    println!("\n=== Array vs Vec ===");
    
    // Array - 固定大小
    let arr = [1, 2, 3, 4, 5];
    println!("Array:");
    println!("  固定大小: 编译时确定");
    println!("  存储: 栈上");
    println!("  类型: [i32; 5]");
    println!("  大小: {} 字节", std::mem::size_of_val(&arr));
    
    // Vec - 动态大小
    let vec = vec![1, 2, 3, 4, 5];
    println!("\nVec:");
    println!("  动态大小: 运行时可变");
    println!("  存储: 堆上");
    println!("  类型: Vec<i32>");
    println!("  指针大小: {} 字节", std::mem::size_of_val(&vec));
    println!("  容量: {}", vec.capacity());
    println!("  长度: {}", vec.len());
    
    println!("\n使用场景:");
    println!("  Array: 大小固定、性能敏感、栈分配");
    println!("  Vec: 大小动态、需要增长/收缩");
}

/// # 迭代器适配器
pub fn iterator_adapters_demo() {
    println!("\n=== 迭代器适配器 ===");
    
    let v = vec![1, 2, 3, 4, 5, 6];
    
    // map
    let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
    println!("map: {:?}", doubled);
    
    // filter
    let evens: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter: {:?}", evens);
    
    // take
    let first_three: Vec<_> = v.iter().take(3).collect();
    println!("take(3): {:?}", first_three);
    
    // skip
    let skip_two: Vec<_> = v.iter().skip(2).collect();
    println!("skip(2): {:?}", skip_two);
    
    // chain
    let v2 = vec![7, 8, 9];
    let chained: Vec<_> = v.iter().chain(v2.iter()).collect();
    println!("chain: {:?}", chained);
    
    // zip
    let names = vec!["Alice", "Bob", "Carol"];
    let ages = vec![25, 30, 35];
    let zipped: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("zip: {:?}", zipped);
    
    // enumerate
    let enumerated: Vec<_> = v.iter().enumerate().collect();
    println!("enumerate: {:?}", enumerated);
    
    // fold
    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("fold sum: {}", sum);
    
    // scan
    let factorials: Vec<_> = (1..=5).scan(1, |state, x| {
        *state *= x;
        Some(*state)
    }).collect();
    println!("scan factorials: {:?}", factorials);
}

/// 运行所有集合类型示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust 集合类型详解           ║");
    println!("╚════════════════════════════════════╝");
    
    tuple_demo();
    array_demo();
    vec_demo();
    vec_methods_demo();
    hashmap_demo();
    hashmap_methods_demo();
    hashset_demo();
    other_collections_demo();
    array_vs_vec_demo();
    iterator_adapters_demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec_operations() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v.pop(), Some(4));
    }
    
    #[test]
    fn test_hashmap_operations() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        assert_eq!(map.get("key"), Some(&"value"));
        assert_eq!(map.get("missing"), None);
    }
    
    #[test]
    fn test_hashset_operations() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(1);  // 重复
        assert_eq!(set.len(), 2);
    }
}
