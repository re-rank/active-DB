# Contributing to ActiveDB

Thank you for your interest in contributing to ActiveDB! This document provides guidelines for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/re-rank/activedb-engine.git`
3. Create a branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Run tests: `cargo test --workspace`
6. Run clippy: `cargo clippy --workspace -- -D warnings`
7. Run fmt: `cargo fmt --all`
8. Commit and push
9. Open a Pull Request

## Development Setup

### Prerequisites
- Rust 1.83+ (stable)
- Docker or Podman (for integration tests)

### Build
```bash
cargo build --workspace
```

### Test
```bash
cargo test --workspace
```

## Code Style

- Follow existing code patterns and conventions
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Write tests for new functionality

## Pull Request Guidelines

- Keep PRs focused on a single change
- Write a clear description of what changed and why
- Reference any related issues
- Ensure CI passes before requesting review

## Reporting Issues

Please report bugs and feature requests via [GitHub Issues](https://github.com/ActiveDB/activedb-engine/issues).

## License

By contributing, you agree that your contributions will be licensed under the AGPL-3.0 license.
