# thrift-proto-hertz-gen

A Rust-based concurrent parser for `.thrift` and `.proto` IDL files with automatic Hertz HTTP server code generation.

## Features

- **Concurrent Parsing**: Utilizes tokio for high-performance concurrent file parsing
- **Git Support**: Clone or pull IDL files directly from Git repositories
- **Recursive Resolution**: Automatically resolves and parses included/imported files
- **Thrift Support**: Full support for Thrift IDL parsing including:
  - Namespaces, includes
  - Structs, enums, exceptions
  - Services with HTTP annotations (api.get, api.post, etc.)
  - Constants and typedefs
- **Proto Support**: Protocol Buffers parsing including:
  - Syntax, package, imports
  - Messages with all field types
  - Enums
  - Services with RPC methods
- **Hertz Generation**: Generate ready-to-use CloudWeGo Hertz HTTP server code

## Installation

```bash
# Build from source
cargo build --release

# Install
cargo install --path .
```

## Usage

### Generate Hertz HTTP Server

```bash
# From local directory
hertz-gen generate -i ./examples -o ./output -m github.com/example/myapp

# From Git repository
hertz-gen generate -i https://github.com/example/idl-repo -o ./output -m github.com/example/myapp

# With multiple inputs and include paths
hertz-gen generate -i ./idl/user.thrift -i ./idl/order.thrift -I ./common -o ./output -m myapp
```

### Parse IDL Files (Output AST as JSON)

```bash
# Parse and output to stdout
hertz-gen parse -i ./examples

# Parse and save to file
hertz-gen parse -i ./examples -o ast.json
```

### Fetch from Git Repository

```bash
hertz-gen fetch -u https://github.com/example/idl-repo -t ./local-idl
```

## CLI Options

### Generate Command

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--input` | `-i` | Input paths (directories, files, or git URLs) | Required |
| `--output` | `-o` | Output directory | `./gen` |
| `--module` | `-m` | Go module name | `github.com/example/hertz-server` |
| `--include` | `-I` | Additional include paths | None |
| `--concurrency` | | Max concurrent file reads | 16 |
| `--verbose` | `-v` | Enable verbose output | false |

## HTTP Annotations

### Thrift

```thrift
service UserService {
    User GetUser(1: i64 id) (api.get = "/api/v1/users/:id")
    void CreateUser(1: User user) (api.post = "/api/v1/users")
    void UpdateUser(1: User user) (api.put = "/api/v1/users/:id")
    void DeleteUser(1: i64 id) (api.delete = "/api/v1/users/:id")
}
```

### Proto

```protobuf
service UserService {
    rpc GetUser(GetUserRequest) returns (User) {
        option (api.get) = "/api/v1/users/:id";
    }
    rpc CreateUser(CreateUserRequest) returns (User) {
        option (api.post) = "/api/v1/users";
    }
}
```

## Generated Code Structure

```
output/
├── go.mod
├── main.go
└── biz/
    ├── handler/
    │   ├── user_service.go
    │   └── order_service.go
    ├── model/
    │   ├── user.go
    │   └── order.go
    └── router/
        └── router.go
```

## Library Usage

```rust
use thrift_proto_hertz_gen::{
    fetcher::{Fetcher, FetcherConfig},
    generator::{Generator, HertzGenerator},
};
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure fetcher with custom settings
    let config = FetcherConfig {
        max_concurrent_reads: 32,
        include_paths: vec![PathBuf::from("./common")],
        allow_git_clone: true,
        ..Default::default()
    };

    let fetcher = Fetcher::with_config(config);

    // Fetch and parse IDL files
    let project = fetcher.fetch_all(vec![
        "./idl/user.thrift".to_string(),
        "https://github.com/example/common-idl".to_string(),
    ]).await?;

    // Generate Hertz server code
    let generator = HertzGenerator::new("github.com/myorg/myapp");
    generator.generate(&project, Path::new("./output"))?;

    Ok(())
}
```

## Requirements

- Rust 1.70+
- Git (for repository cloning)

## License

MIT
