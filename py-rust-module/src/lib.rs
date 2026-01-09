//! PyO3 Python 模块实战
//! 
//! 这个模块演示了如何使用 PyO3 开发 Python 扩展：
//! - 导出 Rust 函数到 Python
//! - 定义 Python 类 (PyClass)
//! - 处理 Python 异常
//! - 类型转换
//! - 集合操作

use pyo3::exceptions::{PyValueError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// 第一部分: 基础函数导出
// ============================================================================

/// 计算两个数的和
/// 
/// Args:
///     a: 第一个数
///     b: 第二个数
/// 
/// Returns:
///     两数之和
#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// 计算阶乘 (演示递归)
/// 
/// Args:
///     n: 非负整数
/// 
/// Returns:
///     n 的阶乘
/// 
/// Raises:
///     ValueError: 如果 n 为负数
#[pyfunction]
fn factorial(n: u64) -> PyResult<u64> {
    if n > 20 {
        return Err(PyValueError::new_err("n is too large (max 20)"));
    }
    
    Ok(match n {
        0 | 1 => 1,
        _ => (2..=n).product(),
    })
}

/// 计算斐波那契数列的第 n 项 (高性能版本)
/// 
/// Args:
///     n: 项数索引
/// 
/// Returns:
///     斐波那契数列的第 n 项
#[pyfunction]
fn fibonacci(n: u64) -> PyResult<u64> {
    if n > 93 {
        return Err(PyValueError::new_err("n is too large (max 93 for u64)"));
    }
    
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp.saturating_add(b);
    }
    
    Ok(a)
}

/// 字符串反转
/// 
/// Args:
///     s: 输入字符串
/// 
/// Returns:
///     反转后的字符串
#[pyfunction]
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// 判断是否为回文
/// 
/// Args:
///     s: 输入字符串
/// 
/// Returns:
///     是否为回文
#[pyfunction]
fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    
    cleaned.chars().eq(cleaned.chars().rev())
}

/// 统计单词频率
/// 
/// Args:
///     text: 输入文本
/// 
/// Returns:
///     单词频率字典
#[pyfunction]
fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    
    for word in text.split_whitespace() {
        let word = word.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        
        if !word.is_empty() {
            *freq.entry(word).or_insert(0) += 1;
        }
    }
    
    freq
}

/// 快速排序实现
/// 
/// Args:
///     arr: 输入数组
/// 
/// Returns:
///     排序后的数组
#[pyfunction]
fn quicksort(arr: Vec<i64>) -> Vec<i64> {
    if arr.len() <= 1 {
        return arr;
    }
    
    let pivot = arr[arr.len() / 2];
    let less: Vec<i64> = arr.iter().copied().filter(|&x| x < pivot).collect();
    let equal: Vec<i64> = arr.iter().copied().filter(|&x| x == pivot).collect();
    let greater: Vec<i64> = arr.iter().copied().filter(|&x| x > pivot).collect();
    
    let mut result = quicksort(less);
    result.extend(equal);
    result.extend(quicksort(greater));
    result
}

/// 二分查找
/// 
/// Args:
///     arr: 已排序数组
///     target: 目标值
/// 
/// Returns:
///     目标值的索引，如果不存在则返回 None
#[pyfunction]
fn binary_search(arr: Vec<i64>, target: i64) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    
    None
}

// ============================================================================
// 第二部分: Python 类定义
// ============================================================================

/// 2D 向量类
/// 
/// 演示基本的 Python 类定义，包括:
/// - 构造函数
/// - 属性
/// - 方法
/// - 特殊方法 (__repr__, __str__, __add__ 等)
#[pyclass]
#[derive(Clone)]
struct Vector2D {
    #[pyo3(get, set)]
    x: f64,
    #[pyo3(get, set)]
    y: f64,
}

#[pymethods]
impl Vector2D {
    /// 创建新的 2D 向量
    #[new]
    fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }
    
    /// 计算向量长度
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    /// 单位化向量
    fn normalize(&self) -> PyResult<Vector2D> {
        let len = self.length();
        if len == 0.0 {
            return Err(PyValueError::new_err("Cannot normalize zero vector"));
        }
        Ok(Vector2D {
            x: self.x / len,
            y: self.y / len,
        })
    }
    
    /// 计算点积
    fn dot(&self, other: &Vector2D) -> f64 {
        self.x * other.x + self.y * other.y
    }
    
    /// 向量加法
    fn __add__(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    
    /// 向量减法
    fn __sub__(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    
    /// 标量乘法
    fn __mul__(&self, scalar: f64) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
    
    /// 相等比较
    fn __eq__(&self, other: &Vector2D) -> bool {
        (self.x - other.x).abs() < 1e-10 && (self.y - other.y).abs() < 1e-10
    }
    
    /// 字符串表示
    fn __repr__(&self) -> String {
        format!("Vector2D({}, {})", self.x, self.y)
    }
    
    fn __str__(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

/// 矩阵类 (演示更复杂的数据结构)
#[pyclass]
struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

#[pymethods]
impl Matrix {
    /// 创建矩阵
    #[new]
    fn new(data: Vec<Vec<f64>>) -> PyResult<Self> {
        if data.is_empty() {
            return Err(PyValueError::new_err("Matrix cannot be empty"));
        }
        
        let rows = data.len();
        let cols = data[0].len();
        
        // 验证所有行长度一致
        if data.iter().any(|row| row.len() != cols) {
            return Err(PyValueError::new_err("All rows must have the same length"));
        }
        
        Ok(Matrix { data, rows, cols })
    }
    
    /// 创建零矩阵
    #[staticmethod]
    fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }
    
    /// 创建单位矩阵
    #[staticmethod]
    fn identity(size: usize) -> Matrix {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Matrix {
            data,
            rows: size,
            cols: size,
        }
    }
    
    /// 获取矩阵形状
    fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
    
    /// 获取元素
    fn get(&self, row: usize, col: usize) -> PyResult<f64> {
        if row >= self.rows || col >= self.cols {
            return Err(PyValueError::new_err("Index out of bounds"));
        }
        Ok(self.data[row][col])
    }
    
    /// 设置元素
    fn set(&mut self, row: usize, col: usize, value: f64) -> PyResult<()> {
        if row >= self.rows || col >= self.cols {
            return Err(PyValueError::new_err("Index out of bounds"));
        }
        self.data[row][col] = value;
        Ok(())
    }
    
    /// 矩阵转置
    fn transpose(&self) -> Matrix {
        let mut transposed = vec![vec![0.0; self.rows]; self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                transposed[j][i] = self.data[i][j];
            }
        }
        Matrix {
            data: transposed,
            rows: self.cols,
            cols: self.rows,
        }
    }
    
    /// 矩阵乘法
    fn matmul(&self, other: &Matrix) -> PyResult<Matrix> {
        if self.cols != other.rows {
            return Err(PyValueError::new_err(format!(
                "Matrix dimensions mismatch: ({}, {}) x ({}, {})",
                self.rows, self.cols, other.rows, other.cols
            )));
        }
        
        let mut result = vec![vec![0.0; other.cols]; self.rows];
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        
        Ok(Matrix {
            data: result,
            rows: self.rows,
            cols: other.cols,
        })
    }
    
    /// 转换为 Python 列表
    fn to_list(&self) -> Vec<Vec<f64>> {
        self.data.clone()
    }
    
    fn __repr__(&self) -> String {
        format!("Matrix({}x{})", self.rows, self.cols)
    }
}

// ============================================================================
// 第三部分: JSON 处理和数据类
// ============================================================================

/// 用户数据类 (演示 serde 集成)
#[derive(Serialize, Deserialize, Clone)]
#[pyclass]
struct User {
    #[pyo3(get, set)]
    id: u64,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    email: String,
    #[pyo3(get, set)]
    age: Option<u8>,
}

#[pymethods]
impl User {
    #[new]
    #[pyo3(signature = (id, name, email, age=None))]
    fn new(id: u64, name: String, email: String, age: Option<u8>) -> Self {
        User { id, name, email, age }
    }
    
    /// 转换为 JSON 字符串
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self)
            .map_err(|e| PyValueError::new_err(format!("JSON serialization error: {}", e)))
    }
    
    /// 从 JSON 字符串创建 User
    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<User> {
        serde_json::from_str(json_str)
            .map_err(|e| PyValueError::new_err(format!("JSON parsing error: {}", e)))
    }
    
    /// 转换为字典
    fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("id", self.id)?;
        dict.set_item("name", &self.name)?;
        dict.set_item("email", &self.email)?;
        dict.set_item("age", self.age)?;
        Ok(dict.into())
    }
    
    fn __repr__(&self) -> String {
        format!("User(id={}, name='{}', email='{}')", self.id, self.name, self.email)
    }
}

/// JSON 工具函数
#[pyfunction]
fn parse_json(json_str: &str) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let value: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| PyValueError::new_err(format!("JSON parsing error: {}", e)))?;
        
        json_to_py(py, &value)
    })
}

/// 将 JSON Value 转换为 Python 对象
fn json_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok((*b).into_pyobject(py)?.to_owned().into_any().unbind()),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.to_owned().into_any().unbind())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.to_owned().into_any().unbind())
            } else {
                Err(PyTypeError::new_err("Invalid number"))
            }
        }
        serde_json::Value::String(s) => Ok(s.into_pyobject(py)?.to_owned().into_any().unbind()),
        serde_json::Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_to_py(py, item)?)?;
            }
            Ok(list.into())
        }
        serde_json::Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (k, v) in obj {
                dict.set_item(k, json_to_py(py, v)?)?;
            }
            Ok(dict.into())
        }
    }
}

/// 将 Python 对象转换为 JSON 字符串
#[pyfunction]
fn to_json(py: Python<'_>, obj: &Bound<'_, pyo3::PyAny>) -> PyResult<String> {
    let value = py_to_json(py, obj)?;
    serde_json::to_string_pretty(&value)
        .map_err(|e| PyValueError::new_err(format!("JSON serialization error: {}", e)))
}

/// 将 Python 对象转换为 JSON Value
fn py_to_json(_py: Python<'_>, obj: &Bound<'_, pyo3::PyAny>) -> PyResult<serde_json::Value> {
    if obj.is_none() {
        return Ok(serde_json::Value::Null);
    }
    
    if let Ok(b) = obj.extract::<bool>() {
        return Ok(serde_json::Value::Bool(b));
    }
    
    if let Ok(i) = obj.extract::<i64>() {
        return Ok(serde_json::Value::Number(i.into()));
    }
    
    if let Ok(f) = obj.extract::<f64>() {
        return Ok(serde_json::json!(f));
    }
    
    if let Ok(s) = obj.extract::<String>() {
        return Ok(serde_json::Value::String(s));
    }
    
    if let Ok(list) = obj.downcast::<PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            arr.push(py_to_json(_py, &item)?);
        }
        return Ok(serde_json::Value::Array(arr));
    }
    
    if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (k, v) in dict.iter() {
            let key: String = k.extract()?;
            map.insert(key, py_to_json(_py, &v)?);
        }
        return Ok(serde_json::Value::Object(map));
    }
    
    Err(PyTypeError::new_err("Unsupported type for JSON conversion"))
}

// ============================================================================
// 第四部分: 数据处理类
// ============================================================================

/// 简单的统计计算器
#[pyclass]
struct Statistics {
    data: Vec<f64>,
}

#[pymethods]
impl Statistics {
    #[new]
    fn new(data: Vec<f64>) -> PyResult<Self> {
        if data.is_empty() {
            return Err(PyValueError::new_err("Data cannot be empty"));
        }
        Ok(Statistics { data })
    }
    
    /// 计算均值
    fn mean(&self) -> f64 {
        self.data.iter().sum::<f64>() / self.data.len() as f64
    }
    
    /// 计算中位数
    fn median(&self) -> f64 {
        let mut sorted = self.data.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        } else {
            sorted[mid]
        }
    }
    
    /// 计算方差
    fn variance(&self) -> f64 {
        let mean = self.mean();
        self.data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / self.data.len() as f64
    }
    
    /// 计算标准差
    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
    
    /// 计算最小值
    fn min(&self) -> f64 {
        self.data.iter().cloned().fold(f64::INFINITY, f64::min)
    }
    
    /// 计算最大值
    fn max(&self) -> f64 {
        self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }
    
    /// 返回汇总统计信息
    fn summary(&self) -> HashMap<String, f64> {
        let mut result = HashMap::new();
        result.insert("count".to_string(), self.data.len() as f64);
        result.insert("mean".to_string(), self.mean());
        result.insert("median".to_string(), self.median());
        result.insert("std_dev".to_string(), self.std_dev());
        result.insert("variance".to_string(), self.variance());
        result.insert("min".to_string(), self.min());
        result.insert("max".to_string(), self.max());
        result
    }
}

// ============================================================================
// 第五部分: 高级示例 - 计数器类
// ============================================================================

/// Python Counter 类似物
#[pyclass]
struct Counter {
    counts: HashMap<String, usize>,
}

#[pymethods]
impl Counter {
    #[new]
    fn new() -> Self {
        Counter {
            counts: HashMap::new(),
        }
    }
    
    /// 从可迭代对象创建 Counter
    #[staticmethod]
    fn from_iterable(items: Vec<String>) -> Counter {
        let mut counts = HashMap::new();
        for item in items {
            *counts.entry(item).or_insert(0) += 1;
        }
        Counter { counts }
    }
    
    /// 更新计数
    fn update(&mut self, items: Vec<String>) {
        for item in items {
            *self.counts.entry(item).or_insert(0) += 1;
        }
    }
    
    /// 获取计数
    fn get(&self, key: &str) -> usize {
        *self.counts.get(key).unwrap_or(&0)
    }
    
    /// 设置计数
    fn set(&mut self, key: String, value: usize) {
        self.counts.insert(key, value);
    }
    
    /// 获取最常见的 n 个元素
    #[pyo3(signature = (n=None))]
    fn most_common(&self, n: Option<usize>) -> Vec<(String, usize)> {
        let mut items: Vec<_> = self.counts.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        
        items.sort_by(|a, b| b.1.cmp(&a.1));
        
        match n {
            Some(n) => items.into_iter().take(n).collect(),
            None => items,
        }
    }
    
    /// 获取所有键
    fn keys(&self) -> Vec<String> {
        self.counts.keys().cloned().collect()
    }
    
    /// 获取所有值
    fn values(&self) -> Vec<usize> {
        self.counts.values().cloned().collect()
    }
    
    /// 转换为字典
    fn to_dict(&self) -> HashMap<String, usize> {
        self.counts.clone()
    }
    
    /// 元素总数
    fn total(&self) -> usize {
        self.counts.values().sum()
    }
    
    fn __len__(&self) -> usize {
        self.counts.len()
    }
    
    fn __repr__(&self) -> String {
        format!("Counter({:?})", self.counts)
    }
}

// ============================================================================
// 模块定义
// ============================================================================

/// Python 模块入口点
#[pymodule]
fn py_rust_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // 添加基础函数
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(factorial, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(reverse_string, m)?)?;
    m.add_function(wrap_pyfunction!(is_palindrome, m)?)?;
    m.add_function(wrap_pyfunction!(word_frequency, m)?)?;
    m.add_function(wrap_pyfunction!(quicksort, m)?)?;
    m.add_function(wrap_pyfunction!(binary_search, m)?)?;
    
    // 添加 JSON 函数
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    m.add_function(wrap_pyfunction!(to_json, m)?)?;
    
    // 添加类
    m.add_class::<Vector2D>()?;
    m.add_class::<Matrix>()?;
    m.add_class::<User>()?;
    m.add_class::<Statistics>()?;
    m.add_class::<Counter>()?;
    
    // 添加模块版本
    m.add("__version__", "0.1.0")?;
    
    Ok(())
}
