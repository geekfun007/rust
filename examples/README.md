# Example IDL Files

This directory contains example IDL files demonstrating the recursive include resolution feature.

## File Structure

```
base.idl (entry point)
├── types.idl
│   └── common.idl
└── interfaces.idl
    └── types.idl (already resolved)
```

## Files Description

- **base.idl**: Main entry point that includes types and interfaces
- **types.idl**: Type definitions that include common definitions
- **interfaces.idl**: Interface definitions that depend on types
- **common.idl**: Common definitions used across multiple files

## Running the Example

From the project root:

```bash
# Basic usage
cargo run -- examples/base.idl

# With dependency graph
cargo run -- examples/base.idl --graph

# Custom output directory
cargo run -- examples/base.idl --output ./my_output

# Verbose mode
cargo run -- examples/base.idl --verbose --graph
```

## Expected Output

The tool will:
1. Parse `base.idl` and find 2 includes
2. Recursively fetch `types.idl` and `interfaces.idl`
3. Continue to fetch `common.idl` from `types.idl`
4. Detect that `types.idl` is already fetched when processing `interfaces.idl`
5. Save all 4 files to the output directory
6. Display a summary and dependency graph
