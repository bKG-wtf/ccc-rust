mod agent;
mod claude;
mod workflows;
mod mcp_server;
mod mcp_adapter;

use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::claude::ClaudeAgent;
use crate::mcp_server::{create_router, McpServerState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ccc_rust_mcp=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting CCC Rust MCP Server v{}", env!("CARGO_PKG_VERSION"));

    // Create server state
    let state = McpServerState {
        claude_agent: Arc::new(ClaudeAgent::new()),
    };

    // Create router
    let app = create_router(state);

    // Get port from environment or use default
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Server listening on {}", addr);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    // Start server
    axum::serve(listener, app)
        .await?;

    Ok(())
}
