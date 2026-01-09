# rust-thrift-ts-pyo3 实战

这个仓库是一个 Rust workspace，包含两个“可跑起来”的实战项目：

- **`crates/thrift-ts-gen`**：解析 Thrift IDL（`.thrift`），生成 TypeScript 类型文件（`.ts`）。
- **`crates/pyo3-demo`**：用 **PyO3** 写一个 Python 扩展模块（Rust 编译成 `.so`），用 **maturin** 安装到虚拟环境里。

## Thrift 解析 + 生成 TS 类型（CLI）

### 功能点

- 解析：`typedef / enum / struct / union / exception / service`
- 生成：
  - `typedef` -> `export type ...`
  - `enum` -> `export enum ...`
  - `struct/exception` -> `export interface ...`
  - `union` -> TS union（带 `kind` 判别字段）
  - `service` -> `export interface ...`（方法返回 `Promise<T>`）
- 类型映射：
  - `bool` -> `boolean`
  - `i8/i16/i32/double/byte` -> `number`
  - `i64` -> 默认 `string`（可选 `bigint`）
  - `string` -> `string`
  - `binary` -> `Uint8Array`
  - `list/set` -> `Array<T>`
  - `map` -> `Record<K, V>`（K 为 `string/number` 时）否则 `Map<K, V>`

### 快速运行

仓库自带了一个示例 IDL：`examples/demo.thrift`。

```bash
cargo run -p thrift-ts-gen -- --input examples/demo.thrift
```

输出到文件：

```bash
cargo run -p thrift-ts-gen -- --input examples/demo.thrift --output examples/demo.ts
```

`i64` 映射成 `bigint`：

```bash
cargo run -p thrift-ts-gen -- --input examples/demo.thrift --i64 bigint
```

把生成内容包在 `namespace` 里（默认使用 Thrift 里的 `namespace js ...`）：

```bash
cargo run -p thrift-ts-gen -- --input examples/demo.thrift --wrap-namespace
```

## Rust 开发 Python 模块（PyO3 + maturin）

模块代码在 `crates/pyo3-demo`，最终 Python 里 `import pyo3_demo`。

### 本地安装（开发模式）

```bash
cd crates/pyo3-demo
python3 -m venv .venv
source .venv/bin/activate
pip install -U pip maturin
maturin develop
python -c "import pyo3_demo; print(pyo3_demo.add(1,2)); c=pyo3_demo.Counter(10); print(c.inc(5), c.value)"
```

## 开发/测试

```bash
cargo test
```

