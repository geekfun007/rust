# IDL Fetcher - Project Summary

## Overview

IDL Fetcher is a Rust command-line tool that recursively fetches IDL (Interface Definition Language) files and all their included dependencies to the local filesystem. It supports both local file paths and remote HTTP/HTTPS URLs.

## Project Structure

```
idl-fetcher/
├── src/
│   ├── main.rs          # CLI entry point and argument parsing
│   ├── parser.rs        # IDL parsing and include extraction
│   ├── fetcher.rs       # HTTP and filesystem fetching
│   └── resolver.rs      # Recursive dependency resolution
├── examples/
│   ├── base.idl         # Example entry point
│   ├── types.idl        # Example type definitions
│   ├── interfaces.idl   # Example interfaces
│   ├── common.idl       # Example common definitions
│   └── README.md        # Examples documentation
├── Cargo.toml           # Project dependencies
├── README.md            # Main documentation
├── USAGE.md             # Detailed usage guide
├── CONTRIBUTING.md      # Contribution guidelines
├── LICENSE              # MIT License
└── .gitignore           # Git ignore patterns
```

## Key Features

### 1. Recursive Include Resolution
- Automatically follows `#include` and `import` directives
- Prevents infinite loops from circular dependencies
- Tracks visited files to avoid duplicate downloads

### 2. Multiple Source Types
- Local filesystem paths
- HTTP/HTTPS URLs
- Automatic source type detection

### 3. Smart Path Resolution
- Resolves relative paths based on parent file location
- Handles both filesystem and URL path resolution
- Supports absolute paths

### 4. Dependency Tracking
- Creates dependency graphs
- Shows include relationships
- Provides detailed summaries

### 5. Flexible Output
- Configurable output directory
- Preserves original filenames
- Creates directories as needed

## Architecture

### Module Breakdown

#### `parser.rs`
- **Purpose**: Parse IDL content and extract include directives
- **Key Component**: `IdlParser` struct with regex-based parsing
- **Pattern Matching**: Supports `#include "file"`, `#include <file>`, and `import "file"`

#### `fetcher.rs`
- **Purpose**: Fetch content from various sources
- **Key Component**: `Fetcher` struct with HTTP client
- **Features**: 
  - HTTP/HTTPS fetching with timeout
  - Local file reading
  - Path resolution (relative, absolute, URL)

#### `resolver.rs`
- **Purpose**: Orchestrate recursive fetching
- **Key Component**: `IdlResolver` struct
- **Algorithm**:
  1. Use breadth-first search with queue
  2. Track visited files in HashSet
  3. Resolve include paths relative to parent
  4. Save files to local filesystem
  5. Generate dependency graph

#### `main.rs`
- **Purpose**: CLI interface
- **Key Component**: Clap-based argument parsing
- **Features**: Help text, verbose mode, graph display

## Technical Details

### Dependencies

```toml
reqwest = "0.11"          # HTTP client with rustls-tls
regex = "1.10"            # Pattern matching for includes
anyhow = "1.0"            # Error handling
thiserror = "1.0"         # Custom error types
tokio = "1.35"            # Async runtime
url = "2.5"               # URL parsing
clap = "4.4"              # CLI argument parsing
```

### Supported Include Formats

```idl
#include "filename.idl"     // Standard C-style include
#include <filename.idl>     // System include style
import "filename.idl"       // Import statement
```

### Path Resolution Examples

**Relative Path (Filesystem):**
```
Parent: /base/dir/file.idl
Include: "subdir/other.idl"
Result: /base/dir/subdir/other.idl
```

**Relative Path (URL):**
```
Parent: https://example.com/api/base.idl
Include: "types/common.idl"
Result: https://example.com/api/types/common.idl
```

**Absolute Path:**
```
Include: "/usr/local/include/system.idl"
Result: /usr/local/include/system.idl (used as-is)
```

## Testing

### Test Coverage

- **Parser Tests**: Include extraction, pattern matching
- **Fetcher Tests**: Path resolution for filesystem and URLs
- **Resolver Tests**: Local path generation

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run release tests
cargo test --release
```

### Example Test Run

```bash
cargo run -- examples/base.idl --graph
```

Expected output:
- Fetches 4 files (base.idl, types.idl, interfaces.idl, common.idl)
- Shows dependency graph
- Saves all files to `./idl_output/`

## Performance Characteristics

- **Time Complexity**: O(n) where n is the number of unique IDL files
- **Space Complexity**: O(n) for tracking visited files
- **Network**: Sequential HTTP requests (no parallel fetching)
- **I/O**: Efficient file operations with minimal buffering

## Error Handling

- **Network Errors**: Graceful handling with context
- **File Errors**: Clear error messages for missing files
- **Parse Errors**: Continues processing on regex failures
- **Path Errors**: Fallback to original path on resolution failure

## Build Information

### Requirements
- Rust 1.82+ (tested with 1.92)
- Cargo
- No system dependencies (uses rustls instead of OpenSSL)

### Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run directly
cargo run -- <source> [options]

# Install globally
cargo install --path .
```

## Usage Examples

### Basic Usage
```bash
idl-fetcher examples/base.idl
```

### With Options
```bash
idl-fetcher examples/base.idl \
  --output ./my_output \
  --graph \
  --verbose
```

### From URL
```bash
idl-fetcher https://example.com/api/service.idl --graph
```

## Future Enhancements (Potential)

1. **Parallel Fetching**: Download multiple files concurrently
2. **Caching**: Cache downloaded files to avoid re-fetching
3. **Authentication**: Support for authenticated HTTP requests
4. **Format Support**: Support additional IDL formats (Protocol Buffers, Thrift, etc.)
5. **Validation**: Validate IDL syntax after fetching
6. **Watch Mode**: Monitor files for changes and re-fetch
7. **Compression**: Support for compressed IDL files
8. **Proxy Support**: HTTP proxy configuration

## Known Limitations

1. **Sequential Processing**: Files are fetched one at a time
2. **No Authentication**: HTTP requests don't support auth headers
3. **Simple Parsing**: Uses regex instead of full IDL parser
4. **Filename Conflicts**: Multiple files with same name will overwrite
5. **No Validation**: Doesn't validate IDL syntax

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## License

MIT License - See [LICENSE](LICENSE) file for details.

## Resources

- **Main Documentation**: [README.md](README.md)
- **Usage Guide**: [USAGE.md](USAGE.md)
- **Examples**: [examples/README.md](examples/README.md)
- **Contributing**: [CONTRIBUTING.md](CONTRIBUTING.md)

## Version History

### v0.1.0 (Initial Release)
- Basic recursive IDL fetching
- Support for local files and HTTP/HTTPS URLs
- Dependency graph generation
- CLI interface with clap
- Comprehensive test coverage
- Full documentation

## Contact & Support

For issues, questions, or contributions, please refer to the project repository.
