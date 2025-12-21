# IDL Fetcher - Usage Guide

## Table of Contents

- [Basic Usage](#basic-usage)
- [Command-Line Options](#command-line-options)
- [Examples](#examples)
- [Advanced Features](#advanced-features)
- [Troubleshooting](#troubleshooting)

## Basic Usage

### Fetch from Local File

```bash
idl-fetcher path/to/file.idl
```

This will:
1. Read the IDL file from the local filesystem
2. Parse and extract all `#include` and `import` directives
3. Recursively fetch all included files
4. Save everything to `./idl_output/` directory

### Fetch from URL

```bash
idl-fetcher https://example.com/api/base.idl
```

The tool automatically detects URLs and fetches files over HTTP/HTTPS.

## Command-Line Options

### `-o, --output <DIR>`

Specify a custom output directory:

```bash
idl-fetcher file.idl --output ./my_idl_files
```

### `-g, --graph`

Display a dependency graph showing include relationships:

```bash
idl-fetcher file.idl --graph
```

Example output:
```
DEPENDENCY GRAPH:
============================================================

base.idl
  └─> types.idl
  └─> interfaces.idl

types.idl
  └─> common.idl

interfaces.idl
  └─> types.idl

common.idl
  (no includes)
```

### `-v, --verbose`

Enable verbose output with detailed information:

```bash
idl-fetcher file.idl --verbose
```

### `-h, --help`

Show help message:

```bash
idl-fetcher --help
```

## Examples

### Example 1: Simple Local Fetch

```bash
# Fetch a local IDL file and its includes
idl-fetcher examples/base.idl
```

### Example 2: Fetch with Custom Output

```bash
# Save to a specific directory
idl-fetcher examples/base.idl --output ./downloaded_idls
```

### Example 3: Fetch from URL with Graph

```bash
# Fetch from a remote server and show dependencies
idl-fetcher https://example.com/api/service.idl --graph
```

### Example 4: Complete Verbose Mode

```bash
# Get all details about the fetching process
idl-fetcher examples/base.idl --output ./output --graph --verbose
```

## Advanced Features

### Supported Include Formats

The tool recognizes multiple IDL include formats:

```idl
#include "filename.idl"     // Double quotes
#include <filename.idl>     // Angle brackets
import "filename.idl"       // Import statement
```

### Path Resolution

#### Relative Paths

When an IDL file includes a relative path, it's resolved relative to the parent file:

```
Parent: /base/dir/file.idl
Include: "subdir/other.idl"
Resolved: /base/dir/subdir/other.idl
```

#### URL Paths

For URLs, relative paths are joined with the parent URL:

```
Parent: https://example.com/api/base.idl
Include: "types/common.idl"
Resolved: https://example.com/api/types/common.idl
```

#### Absolute Paths

Absolute paths are used as-is:

```
Include: "/usr/local/include/system.idl"
Resolved: /usr/local/include/system.idl
```

### Circular Dependency Handling

The tool automatically detects and prevents infinite loops from circular includes:

```idl
// file_a.idl includes file_b.idl
// file_b.idl includes file_a.idl
// Tool will fetch each file only once
```

### Duplicate Detection

Files are tracked by their source path, so the same file included multiple times is only fetched once:

```
base.idl
├── types.idl
│   └── common.idl
└── interfaces.idl
    └── common.idl  (already fetched, skipped)
```

## Output Format

### Summary Report

After fetching, the tool displays a summary:

```
============================================================
FETCH SUMMARY
============================================================

[1] Source: examples/base.idl
    Local:  ./idl_output/base.idl
    Size:   190 bytes
    Includes:
      - examples/types.idl
      - examples/interfaces.idl

[2] Source: examples/types.idl
    Local:  ./idl_output/types.idl
    Size:   328 bytes
    Includes:
      - examples/common.idl
...
```

### File Organization

Downloaded files are saved with their original filenames:

```
idl_output/
├── base.idl
├── types.idl
├── interfaces.idl
└── common.idl
```

## Troubleshooting

### Issue: "Failed to fetch from URL"

**Cause**: Network connectivity issues or invalid URL

**Solution**:
- Check your internet connection
- Verify the URL is correct and accessible
- Check if the server requires authentication

### Issue: "Failed to read file"

**Cause**: File doesn't exist or insufficient permissions

**Solution**:
- Verify the file path is correct
- Check file permissions
- Use absolute paths if relative paths aren't working

### Issue: "Failed to create directory"

**Cause**: Insufficient permissions for output directory

**Solution**:
- Choose a different output directory with write permissions
- Run with appropriate permissions

### Issue: Slow Performance

**Cause**: Many files or slow network

**Solution**:
- Use local files when possible
- Check network speed for remote fetches
- The tool processes files sequentially to maintain dependency order

## Performance Tips

1. **Use Local Files**: Fetch from local filesystem when possible for faster processing
2. **Organize Includes**: Structure your IDL files to minimize deep nesting
3. **Cache Results**: Keep fetched files to avoid re-downloading

## Integration Examples

### Shell Script

```bash
#!/bin/bash
# Fetch IDL files and process them

OUTPUT_DIR="./generated_idls"
SOURCE_IDL="https://api.example.com/service.idl"

# Fetch IDL files
idl-fetcher "$SOURCE_IDL" --output "$OUTPUT_DIR" --verbose

# Process the fetched files
for idl in "$OUTPUT_DIR"/*.idl; do
    echo "Processing $idl"
    # Your IDL compiler here
done
```

### Makefile

```makefile
.PHONY: fetch-idl clean

IDL_OUTPUT := ./idl_output
IDL_SOURCE := examples/base.idl

fetch-idl:
	idl-fetcher $(IDL_SOURCE) --output $(IDL_OUTPUT) --graph

clean:
	rm -rf $(IDL_OUTPUT)
```

### CI/CD Pipeline

```yaml
# .github/workflows/fetch-idl.yml
name: Fetch IDL Files

on: [push]

jobs:
  fetch:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build IDL Fetcher
        run: cargo build --release
      
      - name: Fetch IDL Files
        run: ./target/release/idl-fetcher ${{ secrets.IDL_URL }} --output ./idls
      
      - name: Upload IDL Files
        uses: actions/upload-artifact@v2
        with:
          name: idl-files
          path: ./idls
```

## Best Practices

1. **Version Control**: Don't commit fetched IDL files if they're generated from remote sources
2. **Documentation**: Document the source URLs in your project README
3. **Regular Updates**: Periodically re-fetch remote IDL files to get updates
4. **Error Handling**: Check exit codes in scripts to handle fetch failures
5. **Output Organization**: Use descriptive output directory names

## Exit Codes

- `0`: Success
- `1`: Error (fetch failure, parse error, I/O error, etc.)

## Getting Help

- Run `idl-fetcher --help` for command-line help
- Check the [README.md](README.md) for general information
- See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines
- Report issues on the project's issue tracker
