# IDL Fetcher

A Rust tool to fetch IDL (Interface Definition Language) entries and recursively download all included files to your local filesystem.

## Features

- 🔄 **Recursive Include Resolution**: Automatically follows and downloads all `#include` and `import` directives
- 🌐 **Multiple Source Types**: Supports both HTTP/HTTPS URLs and local file paths
- 📁 **Smart Path Resolution**: Correctly resolves relative paths for both URLs and local files
- 🔍 **Dependency Tracking**: Prevents duplicate downloads and tracks file relationships
- 📊 **Dependency Graph**: Optional visualization of include relationships
- 🎯 **Flexible Output**: Configurable output directory for downloaded files

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build from Source

```bash
git clone <repository-url>
cd idl-fetcher
cargo build --release
```

The binary will be available at `target/release/idl-fetcher`.

## Usage

### Basic Usage

Fetch an IDL file and all its includes:

```bash
cargo run -- examples/base.idl
```

Or using the compiled binary:

```bash
./target/release/idl-fetcher examples/base.idl
```

### Specify Output Directory

```bash
cargo run -- examples/base.idl --output ./my_idl_files
```

### Show Dependency Graph

```bash
cargo run -- examples/base.idl --graph
```

### Verbose Output

```bash
cargo run -- examples/base.idl --verbose
```

### Fetch from URL

```bash
cargo run -- https://example.com/api/base.idl
```

## Command-Line Options

```
Usage: idl-fetcher [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>  Source IDL file (URL or local path)

Options:
  -o, --output <DIR>  Output directory for downloaded files [default: ./idl_output]
  -g, --graph         Show dependency graph
  -v, --verbose       Verbose output
  -h, --help          Print help
```

## Quick Start

```bash
# Clone and build
git clone <repository-url>
cd idl-fetcher
cargo build --release

# Run with examples
./target/release/idl-fetcher examples/base.idl --graph
```

## Example

The repository includes example IDL files in the `examples/` directory demonstrating a typical include hierarchy:

```
base.idl
├── types.idl
│   └── common.idl
└── interfaces.idl
    └── types.idl (already fetched)
```

Run the example:

```bash
cargo run -- examples/base.idl --graph
```

Output:
```
Processing: examples/base.idl
Found 2 include(s)
Saved to: idl_output/base.idl

Processing: examples/types.idl
Found 1 include(s)
Saved to: idl_output/types.idl

Processing: examples/interfaces.idl
Found 1 include(s)
Saved to: idl_output/interfaces.idl

Processing: examples/common.idl
Found 0 include(s)
Saved to: idl_output/common.idl

✓ Successfully fetched 4 IDL file(s)

============================================================
FETCH SUMMARY
============================================================

[1] Source: examples/base.idl
    Local:  idl_output/base.idl
    Size:   156 bytes
    Includes:
      - examples/types.idl
      - examples/interfaces.idl

[2] Source: examples/types.idl
    Local:  idl_output/types.idl
    Size:   298 bytes
    Includes:
      - examples/common.idl

[3] Source: examples/interfaces.idl
    Local:  idl_output/interfaces.idl
    Size:   279 bytes
    Includes:
      - examples/types.idl

[4] Source: examples/common.idl
    Local:  idl_output/common.idl
    Size:   182 bytes

============================================================

DEPENDENCY GRAPH:
============================================================

examples/base.idl
  └─> examples/types.idl
  └─> examples/interfaces.idl

examples/types.idl
  └─> examples/common.idl

examples/interfaces.idl
  └─> examples/types.idl

examples/common.idl
  (no includes)
```

## How It Works

1. **Parse**: The tool parses IDL files to extract `#include` and `import` directives using regex patterns
2. **Fetch**: Downloads content from URLs or reads from local filesystem
3. **Resolve**: Resolves relative include paths based on the parent file's location
4. **Recurse**: Processes each included file recursively, tracking visited files to avoid duplicates
5. **Save**: Saves all fetched files to the specified output directory

## Supported Include Formats

The tool recognizes the following include patterns:

```idl
#include "filename.idl"
#include <filename.idl>
import "filename.idl"
```

## Architecture

The project is organized into several modules:

- **`parser.rs`**: IDL parsing and include extraction using regex
- **`fetcher.rs`**: HTTP and filesystem fetching with path resolution
- **`resolver.rs`**: Recursive dependency resolution and file management
- **`main.rs`**: CLI interface using clap

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Dependencies

- `reqwest`: HTTP client for fetching remote files
- `regex`: Pattern matching for include directives
- `anyhow`: Error handling
- `thiserror`: Custom error types
- `tokio`: Async runtime
- `url`: URL parsing and manipulation
- `clap`: Command-line argument parsing

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
