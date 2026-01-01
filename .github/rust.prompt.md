# Rust Prompt for Agents Development

## Purpose
This document serves as a quick reference for agents or contributors working on Rust components within the `ccc-rust` project.

## Key Principles
- **Reliability**: Ensure all endpoints conform to existing standards.
- **Modularity**: Leverage crates for clear separation of concerns.
- **Performance**: Profile and test regularly to minimize latencies.

## Quick Start
- Initialize a new Rust component:
    ```bash
    cargo new --lib <component_name>
    ```
- Implement and test your module:
    ```bash
    cargo build
    cargo test
    ```

## Testing Standards
1. Use `cargo test` for asserting module functionality.
2. Add integration tests alongside unit tests with coverage for:
    - Code edge-cases.
    - Error message clarity.

## Helpful Links
- [Rust Book](https://doc.rust-lang.org/book/)
- [Actix Web Documentation](https://actix.rs/)
- [Standard Library Reference](https://doc.rust-lang.org/std/)

Follow these guidelines to contribute high-quality Rust code efficiently.