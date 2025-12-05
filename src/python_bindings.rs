// PyO3 Python 绑定实现
use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Python 用户类
#[pyclass]
#[derive(Clone)]
pub struct PyUser {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub age: u32,
    #[pyo3(get, set)]
    pub email: String,
}

#[pymethods]
impl PyUser {
    /// 创建新用户
    #[new]
    fn new(name: String, age: u32, email: String) -> Self {
        PyUser { name, age, email }
    }

    /// 获取用户描述
    fn description(&self) -> String {
        let user = crate::User::new(self.name.clone(), self.age, self.email.clone());
        user.description()
    }

    /// 检查是否成年
    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    /// 转换为字典
    fn to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("name", &self.name)?;
        dict.set_item("age", self.age)?;
        dict.set_item("email", &self.email)?;
        Ok(dict.into())
    }

    /// 字符串表示
    fn __repr__(&self) -> String {
        format!(
            "PyUser(name='{}', age={}, email='{}')",
            self.name, self.age, self.email
        )
    }

    fn __str__(&self) -> String {
        self.description()
    }
}

/// 计算斐波那契数列
#[pyfunction]
fn fibonacci(n: u32) -> u64 {
    crate::math::fibonacci(n)
}

/// 判断是否为质数
#[pyfunction]
fn is_prime(n: u64) -> bool {
    crate::math::is_prime(n)
}

/// 计算阶乘
#[pyfunction]
fn factorial(n: u32) -> u64 {
    crate::math::factorial(n)
}

/// 反转字符串
#[pyfunction]
fn reverse_string(s: &str) -> String {
    crate::text::reverse(s)
}

/// 统计单词数量
#[pyfunction]
fn word_count(s: &str) -> usize {
    crate::text::word_count(s)
}

/// 首字母大写
#[pyfunction]
fn capitalize_words(s: &str) -> String {
    crate::text::capitalize_words(s)
}

/// 批量计算斐波那契数列
#[pyfunction]
fn fibonacci_batch(numbers: Vec<u32>) -> Vec<u64> {
    numbers.iter().map(|&n| crate::math::fibonacci(n)).collect()
}

/// 过滤质数
#[pyfunction]
fn filter_primes(numbers: Vec<u64>) -> Vec<u64> {
    numbers
        .into_iter()
        .filter(|&n| crate::math::is_prime(n))
        .collect()
}

/// Python 模块定义
#[pymodule]
fn _rust_py_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyUser>()?;
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(is_prime, m)?)?;
    m.add_function(wrap_pyfunction!(factorial, m)?)?;
    m.add_function(wrap_pyfunction!(reverse_string, m)?)?;
    m.add_function(wrap_pyfunction!(word_count, m)?)?;
    m.add_function(wrap_pyfunction!(capitalize_words, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci_batch, m)?)?;
    m.add_function(wrap_pyfunction!(filter_primes, m)?)?;
    Ok(())
}
