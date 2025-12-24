// Map 详解 - HashMap、BTreeMap 和 .map() 方法完全指南
//
// 本教程涵盖：
// 1. HashMap 使用
// 2. BTreeMap 使用
// 3. 自定义 Key 类型
// 4. .map() 迭代器方法
// 5. 实现简单的 Map
// 6. 实战案例

use std::collections::{HashMap, BTreeMap};
use std::hash::{Hash, Hasher};

fn main() {
    println!("=== Map 详解 ===\n");
    
    // 1. HashMap 基础
    demo_hashmap_basics();
    
    // 2. HashMap 常用操作
    demo_hashmap_operations();
    
    // 3. BTreeMap
    demo_btreemap();
    
    // 4. 自定义 Key 类型
    demo_custom_keys();
    
    // 5. HashMap vs BTreeMap
    demo_comparison();
    
    // 6. .map() 迭代器方法
    demo_iterator_map();
    
    // 7. 实现简单的 Map
    demo_custom_map();
    
    // 8. 实战案例
    demo_real_world_examples();
}

// ============================================
// 1. HashMap 基础
// ============================================
fn demo_hashmap_basics() {
    println!("--- 1. HashMap 基础 ---\n");
    
    println!("什么是 HashMap？");
    println!("  - 键值对存储结构");
    println!("  - 基于哈希表实现");
    println!("  - O(1) 平均时间复杂度");
    println!("  - 无序存储\n");
    
    // 创建 HashMap
    println!("创建方式:");
    
    // 方式 1: new()
    let mut scores1 = HashMap::new();
    scores1.insert("Alice", 95);
    scores1.insert("Bob", 87);
    println!("  方式 1 - new(): {:?}", scores1);
    
    // 方式 2: collect()
    let teams = vec![("Blue", 10), ("Red", 20)];
    let scores2: HashMap<_, _> = teams.into_iter().collect();
    println!("  方式 2 - collect(): {:?}", scores2);
    
    // 方式 3: from()
    let scores3 = HashMap::from([
        ("Alice", 95),
        ("Bob", 87),
        ("Charlie", 92),
    ]);
    println!("  方式 3 - from(): {:?}", scores3);
    println!();
    
    // 基本操作
    println!("基本操作:");
    let mut map = HashMap::new();
    
    // 插入
    map.insert("key1", "value1");
    println!("  插入后: {:?}", map);
    
    // 获取
    if let Some(value) = map.get("key1") {
        println!("  获取: key1 = {}", value);
    }
    
    // 删除
    map.remove("key1");
    println!("  删除后: {:?}", map);
    println!();
}

// ============================================
// 2. HashMap 常用操作
// ============================================
fn demo_hashmap_operations() {
    println!("--- 2. HashMap 常用操作 ---\n");
    
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 2);
    map.insert("orange", 5);
    
    // 1. 获取值
    println!("1. 获取值:");
    println!("  get(): {:?}", map.get("apple"));
    println!("  get_mut(): 可修改引用");
    if let Some(value) = map.get_mut("apple") {
        *value += 1;
        println!("    修改后: apple = {}", value);
    }
    println!();
    
    // 2. 检查是否存在
    println!("2. 检查键是否存在:");
    println!("  contains_key('apple'): {}", map.contains_key("apple"));
    println!("  contains_key('grape'): {}", map.contains_key("grape"));
    println!();
    
    // 3. entry API（重要！）
    println!("3. Entry API:");
    
    // or_insert - 不存在时插入
    let count = map.entry("grape").or_insert(0);
    *count += 1;
    println!("  or_insert: {:?}", map);
    
    // or_insert_with - 惰性插入
    map.entry("watermelon")
        .or_insert_with(|| {
            println!("    → 计算默认值...");
            10
        });
    println!("  or_insert_with: {:?}", map);
    
    // and_modify - 存在时修改
    map.entry("apple")
        .and_modify(|v| *v += 5)
        .or_insert(0);
    println!("  and_modify: {:?}", map);
    println!();
    
    // 4. 迭代
    println!("4. 迭代:");
    println!("  遍历键值对:");
    for (key, value) in &map {
        println!("    {} = {}", key, value);
    }
    
    println!("  只遍历键:");
    for key in map.keys() {
        print!("    {} ", key);
    }
    println!();
    
    println!("  只遍历值:");
    for value in map.values() {
        print!("    {} ", value);
    }
    println!("\n");
    
    // 5. 更新值
    println!("5. 更新值的模式:");
    let mut word_count = HashMap::new();
    let text = "hello world hello rust world";
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("  词频统计: {:?}", word_count);
    println!();
}

// ============================================
// 3. BTreeMap
// ============================================
fn demo_btreemap() {
    println!("--- 3. BTreeMap ---\n");
    
    println!("什么是 BTreeMap？");
    println!("  - 基于 B 树实现");
    println!("  - 键有序存储");
    println!("  - O(log n) 时间复杂度");
    println!("  - 支持范围查询\n");
    
    let mut map = BTreeMap::new();
    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(5, "five");
    map.insert(2, "two");
    map.insert(4, "four");
    
    println!("BTreeMap (自动排序):");
    for (key, value) in &map {
        println!("  {} = {}", key, value);
    }
    println!();
    
    // 范围查询
    println!("范围查询:");
    println!("  范围 2..=4:");
    for (key, value) in map.range(2..=4) {
        println!("    {} = {}", key, value);
    }
    println!();
    
    // 边界操作
    println!("边界操作:");
    println!("  first: {:?}", map.first_key_value());
    println!("  last: {:?}", map.last_key_value());
    println!();
}

// ============================================
// 4. 自定义 Key 类型
// ============================================
fn demo_custom_keys() {
    println!("--- 4. 自定义 Key 类型 ---\n");
    
    println!("HashMap Key 的要求:");
    println!("  - 必须实现 Eq trait");
    println!("  - 必须实现 Hash trait\n");
    
    // 自定义 Key
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct UserId {
        id: u32,
        region: String,
    }
    
    let mut users = HashMap::new();
    
    users.insert(
        UserId {
            id: 1,
            region: "CN".to_string(),
        },
        "张三",
    );
    
    users.insert(
        UserId {
            id: 2,
            region: "US".to_string(),
        },
        "Alice",
    );
    
    println!("自定义 Key 示例:");
    for (key, name) in &users {
        println!("  {:?} => {}", key, name);
    }
    println!();
    
    // 手动实现 Hash（自定义哈希逻辑）
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct CustomKey {
        value: String,
    }
    
    impl Hash for CustomKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // 自定义哈希逻辑：只哈希小写形式
            self.value.to_lowercase().hash(state);
        }
    }
    
    let mut map = HashMap::new();
    map.insert(CustomKey { value: "Hello".to_string() }, 1);
    
    // "HELLO" 和 "hello" 被视为相同的 key
    let key = CustomKey { value: "HELLO".to_string() };
    println!("自定义 Hash 逻辑:");
    println!("  查找 'HELLO': {:?}", map.get(&key));
    println!();
}

// ============================================
// 5. HashMap vs BTreeMap
// ============================================
fn demo_comparison() {
    println!("--- 5. HashMap vs BTreeMap ---\n");
    
    println!("┌─────────────┬──────────────┬──────────────┐");
    println!("│  特性        │  HashMap     │  BTreeMap    │");
    println!("├─────────────┼──────────────┼──────────────┤");
    println!("│  实现        │  哈希表       │  B 树        │");
    println!("│  顺序        │  无序         │  有序        │");
    println!("│  插入        │  O(1)        │  O(log n)    │");
    println!("│  查找        │  O(1)        │  O(log n)    │");
    println!("│  删除        │  O(1)        │  O(log n)    │");
    println!("│  范围查询    │  ✗           │  ✓           │");
    println!("│  内存        │  更大         │  更小        │");
    println!("│  Key 要求    │  Hash + Eq   │  Ord         │");
    println!("└─────────────┴──────────────┴──────────────┘");
    println!();
    
    println!("选择建议:");
    println!("  HashMap:");
    println!("    ✓ 一般场景（默认选择）");
    println!("    ✓ 需要最快的查找速度");
    println!("    ✓ 不关心键的顺序");
    println!();
    println!("  BTreeMap:");
    println!("    ✓ 需要有序遍历");
    println!("    ✓ 需要范围查询");
    println!("    ✓ 需要找最大/最小键");
    println!();
}

// ============================================
// 6. .map() 迭代器方法
// ============================================
fn demo_iterator_map() {
    println!("--- 6. .map() 迭代器方法 ---\n");
    
    println!("什么是 .map()？");
    println!("  - 迭代器适配器");
    println!("  - 转换每个元素");
    println!("  - 惰性求值（不会立即执行）\n");
    
    // 基础示例
    println!("1. 基础转换:");
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<_> = numbers.iter()
        .map(|x| x * 2)
        .collect();
    println!("  原始: {:?}", numbers);
    println!("  加倍: {:?}", doubled);
    println!();
    
    // 类型转换
    println!("2. 类型转换:");
    let strings = vec!["1", "2", "3", "4"];
    let numbers: Vec<i32> = strings.iter()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("  字符串: {:?}", strings);
    println!("  数字: {:?}", numbers);
    println!();
    
    // 结构转换
    println!("3. 结构转换:");
    
    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
    }
    
    let users = vec![
        User { name: "Alice".to_string(), age: 25 },
        User { name: "Bob".to_string(), age: 30 },
    ];
    
    let names: Vec<_> = users.iter()
        .map(|u| &u.name)
        .collect();
    println!("  用户名: {:?}", names);
    println!();
    
    // 链式调用
    println!("4. 链式调用:");
    let result: Vec<_> = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|x| x * 2)        // 乘以 2
        .filter(|x| *x > 5)    // 过滤大于 5
        .map(|x| x.to_string()) // 转为字符串
        .collect();
    println!("  结果: {:?}", result);
    println!();
    
    // map vs for_each
    println!("5. map vs for_each:");
    println!("  map - 返回新迭代器（惰性）");
    println!("  for_each - 立即执行（消费迭代器）");
    
    vec![1, 2, 3].iter().for_each(|x| {
        println!("    处理: {}", x);
    });
    println!();
}

// ============================================
// 7. 实现简单的 Map
// ============================================
fn demo_custom_map() {
    println!("--- 7. 实现简单的 Map ---\n");
    
    // 简单的 Map 实现（使用 Vec）
    struct SimpleMap<K, V> {
        data: Vec<(K, V)>,
    }
    
    impl<K: PartialEq, V> SimpleMap<K, V> {
        fn new() -> Self {
            SimpleMap { data: Vec::new() }
        }
        
        fn insert(&mut self, key: K, value: V) -> Option<V> {
            // 查找是否已存在
            for (k, v) in &mut self.data {
                if k == &key {
                    return Some(std::mem::replace(v, value));
                }
            }
            self.data.push((key, value));
            None
        }
        
        fn get(&self, key: &K) -> Option<&V> {
            self.data.iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v)
        }
        
        fn remove(&mut self, key: &K) -> Option<V> {
            self.data.iter()
                .position(|(k, _)| k == key)
                .map(|i| self.data.remove(i).1)
        }
        
        fn len(&self) -> usize {
            self.data.len()
        }
    }
    
    let mut map = SimpleMap::new();
    
    println!("简单 Map 实现（基于 Vec）:");
    map.insert("apple", 3);
    map.insert("banana", 2);
    map.insert("orange", 5);
    
    println!("  插入后长度: {}", map.len());
    println!("  获取 'apple': {:?}", map.get(&"apple"));
    
    map.remove(&"banana");
    println!("  删除 'banana' 后长度: {}", map.len());
    println!();
    
    println!("性能对比:");
    println!("  SimpleMap (Vec): O(n) 查找");
    println!("  HashMap: O(1) 平均查找");
    println!("  适用场景: 小数据集（< 20 项）");
    println!();
}

// ============================================
// 8. 实战案例
// ============================================
fn demo_real_world_examples() {
    println!("--- 8. 实战案例 ---\n");
    
    // 案例 1: 缓存系统
    println!("案例 1: LRU 缓存\n");
    lru_cache_example();
    
    // 案例 2: 分组统计
    println!("\n案例 2: 数据分组\n");
    grouping_example();
    
    // 案例 3: 配置管理
    println!("\n案例 3: 配置管理器\n");
    config_manager_example();
    
    // 案例 4: 索引构建
    println!("\n案例 4: 倒排索引\n");
    inverted_index_example();
}

// 案例 1: LRU 缓存
fn lru_cache_example() {
    use std::collections::VecDeque;
    
    struct LRUCache<K, V> {
        capacity: usize,
        cache: HashMap<K, V>,
        order: VecDeque<K>,
    }
    
    impl<K: Clone + Eq + Hash + std::fmt::Debug, V> LRUCache<K, V> {
        fn new(capacity: usize) -> Self {
            LRUCache {
                capacity,
                cache: HashMap::new(),
                order: VecDeque::new(),
            }
        }
        
        fn get(&mut self, key: &K) -> Option<&V> {
            if self.cache.contains_key(key) {
                // 更新访问顺序
                self.order.retain(|k| k != key);
                self.order.push_back(key.clone());
                self.cache.get(key)
            } else {
                None
            }
        }
        
        fn put(&mut self, key: K, value: V) {
            if self.cache.contains_key(&key) {
                self.order.retain(|k| k != &key);
            } else if self.cache.len() >= self.capacity {
                // 移除最久未使用的
                if let Some(old_key) = self.order.pop_front() {
                    self.cache.remove(&old_key);
                    println!("  淘汰: {:?}", old_key);
                }
            }
            
            self.cache.insert(key.clone(), value);
            self.order.push_back(key);
        }
    }
    
    let mut cache = LRUCache::new(3);
    
    println!("  容量: 3");
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);
    println!("  添加 a, b, c");
    
    cache.get(&"a");
    println!("  访问 a");
    
    cache.put("d", 4);
    println!("  添加 d (超过容量)");
    
    println!("  缓存状态:");
    println!("    a: {:?}", cache.get(&"a"));
    println!("    b: {:?}", cache.get(&"b"));
    println!("    c: {:?}", cache.get(&"c"));
    println!("    d: {:?}", cache.get(&"d"));
}

// 案例 2: 数据分组
fn grouping_example() {
    #[derive(Debug)]
    struct Student {
        name: String,
        grade: u32,
        score: u32,
    }
    
    let students = vec![
        Student { name: "Alice".to_string(), grade: 1, score: 85 },
        Student { name: "Bob".to_string(), grade: 2, score: 90 },
        Student { name: "Charlie".to_string(), grade: 1, score: 78 },
        Student { name: "David".to_string(), grade: 2, score: 88 },
    ];
    
    // 按年级分组
    let mut by_grade: HashMap<u32, Vec<&Student>> = HashMap::new();
    
    for student in &students {
        by_grade.entry(student.grade)
            .or_insert_with(Vec::new)
            .push(student);
    }
    
    println!("  按年级分组:");
    for (grade, students) in &by_grade {
        println!("    年级 {}:", grade);
        for student in students {
            println!("      {} (分数: {})", student.name, student.score);
        }
    }
    
    // 计算平均分
    println!("\n  平均分:");
    for (grade, students) in &by_grade {
        let avg: f64 = students.iter()
            .map(|s| s.score as f64)
            .sum::<f64>() / students.len() as f64;
        println!("    年级 {}: {:.1}", grade, avg);
    }
}

// 案例 3: 配置管理
fn config_manager_example() {
    struct ConfigManager {
        configs: HashMap<String, String>,
        defaults: HashMap<String, String>,
    }
    
    impl ConfigManager {
        fn new() -> Self {
            let mut defaults = HashMap::new();
            defaults.insert("host".to_string(), "localhost".to_string());
            defaults.insert("port".to_string(), "8080".to_string());
            defaults.insert("timeout".to_string(), "30".to_string());
            
            ConfigManager {
                configs: HashMap::new(),
                defaults,
            }
        }
        
        fn set(&mut self, key: &str, value: &str) {
            self.configs.insert(key.to_string(), value.to_string());
        }
        
        fn get(&self, key: &str) -> Option<&String> {
            self.configs.get(key)
                .or_else(|| self.defaults.get(key))
        }
        
        fn get_or(&self, key: &str, default: &str) -> String {
            self.get(key)
                .cloned()
                .unwrap_or_else(|| default.to_string())
        }
        
        fn list_all(&self) -> HashMap<String, String> {
            let mut all = self.defaults.clone();
            all.extend(self.configs.clone());
            all
        }
    }
    
    let mut config = ConfigManager::new();
    
    println!("  默认配置:");
    println!("    host: {}", config.get("host").unwrap());
    println!("    port: {}", config.get("port").unwrap());
    
    config.set("port", "9000");
    config.set("debug", "true");
    
    println!("\n  修改后:");
    println!("    port: {}", config.get("port").unwrap());
    println!("    debug: {}", config.get("debug").unwrap());
    
    println!("\n  所有配置:");
    for (key, value) in config.list_all() {
        println!("    {} = {}", key, value);
    }
}

// 案例 4: 倒排索引
fn inverted_index_example() {
    struct InvertedIndex {
        index: HashMap<String, Vec<usize>>,
    }
    
    impl InvertedIndex {
        fn new() -> Self {
            InvertedIndex {
                index: HashMap::new(),
            }
        }
        
        fn add_document(&mut self, doc_id: usize, text: &str) {
            for word in text.split_whitespace() {
                let word = word.to_lowercase();
                self.index
                    .entry(word)
                    .or_insert_with(Vec::new)
                    .push(doc_id);
            }
        }
        
        fn search(&self, word: &str) -> Vec<usize> {
            self.index
                .get(&word.to_lowercase())
                .cloned()
                .unwrap_or_default()
        }
    }
    
    let mut index = InvertedIndex::new();
    
    index.add_document(1, "Rust is a systems programming language");
    index.add_document(2, "Rust is fast and safe");
    index.add_document(3, "Programming in Rust is fun");
    
    println!("  文档已索引");
    
    let results = index.search("rust");
    println!("\n  搜索 'rust': 文档 {:?}", results);
    
    let results = index.search("programming");
    println!("  搜索 'programming': 文档 {:?}", results);
}

/*
=== 总结 ===

1. HashMap 基础:
   let mut map = HashMap::new();
   map.insert(key, value);
   map.get(&key);
   map.remove(&key);

2. Entry API:
   map.entry(key).or_insert(default);
   map.entry(key).or_insert_with(|| compute());
   map.entry(key).and_modify(|v| *v += 1).or_insert(0);

3. 迭代:
   for (k, v) in &map { }
   for k in map.keys() { }
   for v in map.values() { }

4. BTreeMap:
   - 有序存储
   - 支持范围查询
   - map.range(start..end)

5. 自定义 Key:
   - 必须实现 Eq + Hash (HashMap)
   - 必须实现 Ord (BTreeMap)

6. .map() 方法:
   iter.map(|x| transform(x)).collect()

7. 选择指南:
   - 一般场景: HashMap
   - 需要顺序: BTreeMap
   - 小数据集: Vec<(K, V)>

运行示例:
  cargo run --bin map_detailed
*/
