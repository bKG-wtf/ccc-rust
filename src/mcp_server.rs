use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::agent::Agent;
use crate::claude::ClaudeAgent;

/// MCP Server state
#[derive(Clone)]
pub struct McpServerState {
    pub claude_agent: Arc<ClaudeAgent>,
}

/// Request for token counting
#[derive(Debug, Deserialize)]
pub struct TokenCountRequest {
    pub text: String,
}

/// Response for token counting
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenCountResponse {
    pub count: usize,
}

/// Request for MCP routing
#[derive(Debug, Deserialize)]
pub struct McpRouteRequest {
    pub task: String,
    pub context: Option<Value>,
}

/// Response for MCP routing
#[derive(Debug, Serialize)]
pub struct McpRouteResponse {
    pub status: String,
    pub result: Value,
}

/// Create the MCP server router
pub fn create_router(state: McpServerState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/v1/messages/count_tokens", post(count_tokens))
        .route("/v1/mcp/route", post(mcp_route))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "ccc-rust-mcp",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Count tokens in the provided text
async fn count_tokens(
    Json(payload): Json<TokenCountRequest>,
) -> Json<TokenCountResponse> {
    // Simple token counting (split by whitespace and punctuation)
    let count = payload.text
        .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .filter(|s| !s.is_empty())
        .count();
    
    tracing::debug!("Counted {} tokens", count);
    
    Json(TokenCountResponse { count })
}

/// Route MCP requests to appropriate handlers
async fn mcp_route(
    State(state): State<McpServerState>,
    Json(payload): Json<McpRouteRequest>,
) -> Json<McpRouteResponse> {
    tracing::info!("Routing MCP request for task: {}", payload.task);
    
    let input = json!({
        "task": payload.task,
        "context": payload.context
    });
    
    match state.claude_agent.handle(input).await {
        Ok(result) => Json(McpRouteResponse {
            status: "success".to_string(),
            result,
        }),
        Err(e) => {
            tracing::error!("Error handling MCP route: {}", e);
            Json(McpRouteResponse {
                status: "error".to_string(),
                result: json!({
                    "error": e.to_string()
                }),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_health_check() {
        let state = McpServerState {
            claude_agent: Arc::new(ClaudeAgent::new()),
        };
        let app = create_router(state);

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_count_tokens() {
        let state = McpServerState {
            claude_agent: Arc::new(ClaudeAgent::new()),
        };
        let app = create_router(state);

        let request_body = json!({
            "text": "Hello world, this is a test!"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/messages/count_tokens")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result: TokenCountResponse = serde_json::from_slice(&body).unwrap();
        assert!(result.count > 0);
    }
}
