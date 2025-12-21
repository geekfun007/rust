# Contributing to IDL Fetcher

Thank you for your interest in contributing to IDL Fetcher!

## Development Setup

1. Install Rust (1.83 or later):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```bash
   git clone <repository-url>
   cd idl-fetcher
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run tests:
   ```bash
   cargo test
   ```

## Code Style

- Follow Rust standard formatting: `cargo fmt`
- Ensure no clippy warnings: `cargo clippy`
- Write tests for new features
- Update documentation as needed

## Testing

Run the test suite:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Test with examples:
```bash
cargo run -- examples/base.idl --graph
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Check for issues: `cargo clippy`
7. Commit your changes with clear messages
8. Push to your fork
9. Open a Pull Request

## Reporting Issues

When reporting issues, please include:
- Rust version (`rustc --version`)
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Any error messages

## Feature Requests

We welcome feature requests! Please open an issue describing:
- The problem you're trying to solve
- Your proposed solution
- Any alternatives you've considered

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

Thank you for contributing!
