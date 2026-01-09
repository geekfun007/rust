use pyo3::prelude::*;

/// Add two integers.
#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// A simple stateful counter.
#[pyclass]
struct Counter {
    value: i64,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(value: i64) -> Self {
        Self { value }
    }

    fn inc(&mut self, delta: i64) -> i64 {
        self.value += delta;
        self.value
    }

    #[getter]
    fn value(&self) -> i64 {
        self.value
    }
}

#[pymodule]
fn pyo3_demo(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_class::<Counter>()?;
    Ok(())
}

