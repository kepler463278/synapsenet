//! REST API for agent actions

use axum::{extract::Path, http::StatusCode, response::Json, routing::*, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Execute action request
#[derive(Debug, Deserialize)]
pub struct ExecuteActionRequest {
    pub goal_id: String,
    pub max_steps: Option<usize>,
}

/// Action execution response
#[derive(Debug, Serialize)]
pub struct ActionExecutionResponse {
    pub goal_id: String,
    pub actions_performed: usize,
    pub tools_used: Vec<String>,
    pub total_time_ms: u64,
    pub success: bool,
}

/// Action trace response
#[derive(Debug, Serialize)]
pub struct ActionTraceResponse {
    pub goal_id: String,
    pub actions: Vec<ActionLogResponse>,
}

#[derive(Debug, Serialize)]
pub struct ActionLogResponse {
    pub id: String,
    pub tool_name: String,
    pub timestamp: i64,
    pub execution_time_ms: u64,
    pub success: bool,
}

/// Tool list response
#[derive(Debug, Serialize)]
pub struct ToolListResponse {
    pub tools: Vec<ToolInfoResponse>,
}

#[derive(Debug, Serialize)]
pub struct ToolInfoResponse {
    pub name: String,
    pub description: String,
    pub version: String,
    pub enabled: bool,
}

/// Tool enable/disable request
#[derive(Debug, Deserialize)]
pub struct ToolToggleRequest {
    pub enabled: bool,
}

/// Create agent API router
pub fn create_router() -> Router {
    Router::new()
        .route("/v2/act", post(execute_action))
        .route("/v2/act/trace/:goal_id", get(get_action_trace))
        .route("/v2/tools", get(list_tools))
        .route("/v2/tools/:name", get(get_tool_info))
        .route("/v2/tools/:name/enable", post(enable_tool))
        .route("/v2/tools/:name/disable", post(disable_tool))
}

/// Execute action for goal
async fn execute_action(
    Json(request): Json<ExecuteActionRequest>,
) -> Result<Json<ActionExecutionResponse>, StatusCode> {
    // TODO: Integrate with AgentCore
    
    Ok(Json(ActionExecutionResponse {
        goal_id: request.goal_id,
        actions_performed: 3,
        tools_used: vec!["web_fetch".to_string(), "math_eval".to_string()],
        total_time_ms: 1500,
        success: true,
    }))
}

/// Get action trace for goal
async fn get_action_trace(
    Path(goal_id): Path<String>,
) -> Result<Json<ActionTraceResponse>, StatusCode> {
    // TODO: Load from database
    
    Ok(Json(ActionTraceResponse {
        goal_id,
        actions: vec![
            ActionLogResponse {
                id: Uuid::new_v4().to_string(),
                tool_name: "web_fetch".to_string(),
                timestamp: chrono::Utc::now().timestamp(),
                execution_time_ms: 500,
                success: true,
            },
        ],
    }))
}

/// List all tools
async fn list_tools() -> Result<Json<ToolListResponse>, StatusCode> {
    // TODO: Get from ToolRegistry
    
    Ok(Json(ToolListResponse {
        tools: vec![
            ToolInfoResponse {
                name: "web_fetch".to_string(),
                description: "Fetch web content".to_string(),
                version: "1.0.0".to_string(),
                enabled: true,
            },
            ToolInfoResponse {
                name: "file_ops".to_string(),
                description: "File operations".to_string(),
                version: "1.0.0".to_string(),
                enabled: true,
            },
            ToolInfoResponse {
                name: "math_eval".to_string(),
                description: "Math evaluation".to_string(),
                version: "1.0.0".to_string(),
                enabled: true,
            },
            ToolInfoResponse {
                name: "code_exec".to_string(),
                description: "Code execution".to_string(),
                version: "1.0.0".to_string(),
                enabled: true,
            },
        ],
    }))
}

/// Get tool info
async fn get_tool_info(
    Path(name): Path<String>,
) -> Result<Json<ToolInfoResponse>, StatusCode> {
    // TODO: Get from ToolRegistry
    
    Ok(Json(ToolInfoResponse {
        name: name.clone(),
        description: format!("{} tool", name),
        version: "1.0.0".to_string(),
        enabled: true,
    }))
}

/// Enable tool
async fn enable_tool(
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Update ToolRegistry
    
    Ok(Json(serde_json::json!({
        "tool": name,
        "enabled": true,
        "message": "Tool enabled successfully"
    })))
}

/// Disable tool
async fn disable_tool(
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Update ToolRegistry
    
    Ok(Json(serde_json::json!({
        "tool": name,
        "enabled": false,
        "message": "Tool disabled successfully"
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_list_tools() {
        let app = create_router();
        let server = TestServer::new(app).unwrap();
        
        let response = server.get("/v2/tools").await;
        response.assert_status_ok();
    }

    #[tokio::test]
    async fn test_execute_action() {
        let app = create_router();
        let server = TestServer::new(app).unwrap();
        
        let request = ExecuteActionRequest {
            goal_id: Uuid::new_v4().to_string(),
            max_steps: Some(5),
        };
        
        let response = server.post("/v2/act")
            .json(&request)
            .await;
        
        response.assert_status_ok();
    }
}
