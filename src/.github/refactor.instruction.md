# Refactoring Instructions for Rust Code

Last Updated: 2026-01-01 15:22:39 UTC

## Purpose
This document outlines the instructions and best practices for refactoring Rust code in the `ccc-rust` repository. Following these guidelines ensures that the code remains clean, maintainable, and adheres to project standards.

---

## Instructions

### 1. Plan Your Changes
- Identify the area of the code you want to refactor.
- Define the reason for the refactor (e.g., improving readability, enhancing performance, reducing technical debt).
- Ensure your changes align with the overall architecture of the project.

### 2. Adhere to Rust Best Practices
Follow Rust guidelines and idiomatic patterns. Useful references include:
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)

### 3. Write Tests
- Write comprehensive unit and integration tests for the changes.
- Ensure that the existing test suite passes before and after the refactor.
- Use the `cargo test` command to validate your changes.

### 4. Run Tools
- Format the code using `cargo fmt`.
- Check for warnings and errors using `cargo check`.
- Use `cargo clippy` for linter suggestions.

### 5. Code Reviews
- Open a pull request with a detailed description of your changes.
- Request feedback from team members to ensure code quality and correctness.
- Address all comments before merging.

### 6. Commit Messages
- Write clear and descriptive commit messages.
- Follow the convention: `Refactor: [short description]`

---

## Additional Tips
- Avoid over-engineering. Focus on solving the problem without unnecessary complexity.
- Keep refactors small and incremental whenever possible.
- Document the reason for significant architectural changes in the code or accompanying documentation.

---

By following these instructions, we'll maintain a high standard of code quality and streamlined development processes for the `ccc-rust` project.

If you have any questions about refactoring practices, please reach out to the team.