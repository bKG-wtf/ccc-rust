Task: Merge cloud-code-router und sandbox-mcp in neuen Branch feature/ccc-rust-mcp. Ziel: ccc rust → Token Preflight + MCP Turtle Executor.

Nur Fokus auf:

Rust Implementation von ccr (cloud-code-router)

Sandbox-MCP als Execution Plane (turtle-mcp)

Adapter Layer (mcp_adapter.rs) für Kommunikation zwischen ccc rust → turtle-mcp

Smoke Tests lokal

Docs: AGENTS.md + master-orc.md updated

Schritt-für-Schritt (für Agent):

Branch vorbereiten

git checkout -b feature/ccc-rust-turtle-mcp


Sandbox-MCP einbinden

# falls separate remote
git remote add turtle-mcp https://github.com/ghostwriternr/sandbox-mcp.git || true
git fetch turtle-mcp
git subtree add --prefix=turtle-mcp turtle main --squash


oder, wenn alles im Monorepo:

mkdir -p workspace
git mv path/to/ccc-rust workspace/ccc-rust
git mv path/to/sturtle-mcp workspace/turtle-mcp


Adapter Layer implementieren

Rust: workspace/ccc-rust/src/mcp_adapter.rs

Async JSON-RPC Client zu sandbox-mcp

Send task, poll status, optional webhook callback

CCC Rust Preflight bleibt

/v1/messages/count_tokens

/v1/mcp/route → akzeptiert Task + ContextReport, routed an MCP Sandbox

Docs & Rebrand

Update AGENTS.md: CCC = Token Preflight, sandbox-mcp = Execution Plane

Update master-orc.md: Hinweis MAID Decision Layer, CCC + MCP combo

Smoke Tests

Rust: cargo build && cargo test

Sandbox-MCP: npm ci && npm run build

Makefile / scripts für make start-ccc, make start-sandbox, make smoke

PR Checklist

Compile & build: ✅

Smoke test: ✅

AGENTS.md / master-orc.md updated: ✅

Branch ready for merge: ✅

Key Message für Orca:

„Feature-Branch feature/ccc-rust-mcp starten, CCC Rust Token-Preflight behalten, sandbox-mcp als Executor einbinden, mcp_adapter.rs implementieren, AGENTS.md + master-orc.md updaten, Smoke Tests lokal laufen lassen – nur Setup & Test, keine anderen Agents sichtbar.“
