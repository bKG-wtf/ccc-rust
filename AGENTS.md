# Updated AGENTS Documentation

## Task Overview
- This document outlines the agents' architecture and the steps involved in implementing Rust solutions for `ccc-rust`.

## New Features
- Migration of `Claude Code Router` to `Rust Web Framework`.
- Updates to the `cloud-code-router` and `sandbox-mcp` execution planes.
- Development of `mcp_adapter.rs` to enhance communication between CCC Rust and turtle-mcp.
- Node UI refactored into a Rust web application.

## Key Steps for Agents

1. **Preparation**:
    ```bash
    git checkout -b feature/rebrand-refactor-agents
    ```

2. **Rebranding to Rust Web Framework**:
    - Set up the new Rust web framework:
        ```bash
        cargo new --bin rust-web-router
        cd rust-web-router
        ```

3. **Integration of Sandbox-MCP**:
    - Ensure proper directory alignment for integration:
        ```bash
        mkdir -p workspace
        git mv path/to/ccc-rust workspace/ccc-rust
        git mv path/to/sturtle-mcp workspace/turtle-mcp
        ```
    - Configure with unified routing:
        ```rust
        // src/main.rs
        use actix_web::{web, App, HttpServer};

        async fn index() -> &'static str {
            "Welcome to Rust Web Framework!"
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            HttpServer::new(|| App::new().route("/", web::get().to(index)))
                .bind("127.0.0.1:8080")?
                .run()
                .await
        }
        ```

4. **Endpoints Routing**:
    - `/v1/messages/count_tokens`
    - `/v1/mcp/route`

5. **Testing Suite**:
    Use extended Makefile scripts:
    - Build Node and Rust
        ```bash
        make start-rust-router
        ```

## Final Actions
- Documentation:
    - AGENTS.md updated.
    - Include `refactor.instruct.md` for changes targeting `.github` configuration.

- Ready for Pull Request:
    - [x] Compile & Build Successful.
    - [x] Local and CI Smoke Tests: Pass
