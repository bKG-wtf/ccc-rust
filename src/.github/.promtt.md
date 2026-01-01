# Updated AGENTS Documentation

## Task Overview
- This document outlines the agents' architecture and the steps involved in implementing Rust solutions for ccc-rust.

## New Features
- Implement updated `cloud-code-router` and `sandbox-mcp` as an execution plane.
- Development of `mcp_adapter.rs` for seamless communication between CCC Rust and turtle-mcp.

## Key Steps for Agents
1. **Preparation**:
    ```bash
    git checkout -b feature/updated-agents
    ```

2. **Integration of Sandbox-MCP**:
    - For standalone remotes:
        ```bash
        git remote add turtle-mcp https://github.com/ghostwriternr/sandbox-mcp.git || true
        git fetch turtle-mcp
        git subtree add --prefix=turtle-mcp turtle main --squash
        ```

    - For monorepos:
        ```bash
        mkdir -p workspace
        git mv path/to/ccc-rust workspace/ccc-rust
        git mv path/to/sturtle-mcp workspace/turtle-mcp
        ```

3. **Implement Adapter Layer**:
    - Path: `workspace/ccc-rust/src/mcp_adapter.rs`
    - Purpose: Provides an Async JSON-RPC client to communicate tasks, poll status, and handle optional webhook callbacks.

4. **Endpoints Routing**:
    - `/v1/messages/count_tokens`: Tracks token counts.
    - `/v1/mcp/route`: Routes tasks and context reports to sandbox.

5. **Testing Suite**:
    ```bash
    # Rust
    cargo build && cargo test

    # Sandbox-MCP
    npm ci && npm run build
    ```
    Add Makefile Scripts:
    - `make start-ccc`
    - `make start-sandbox`
    - `make smoke`

## Final Actions
- Documentation:
    - `AGENTS.md` updated.
    - Master documentation as `master-orc.md`.

- Ready for PR:
    - [x] Compile & Build Successful.
    - [x] Local Smoke Tests: Pass