# pyo3-demo

This crate builds a Python extension module named `pyo3_demo` using PyO3 + maturin.

## Build & install (dev)

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -U pip maturin
maturin develop
python -c "import pyo3_demo; print(pyo3_demo.add(1,2)); c=pyo3_demo.Counter(10); print(c.inc(5), c.value)"
```

