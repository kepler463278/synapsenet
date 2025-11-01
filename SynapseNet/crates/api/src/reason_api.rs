//! REST API v2 for reasoning system

use axum::{extract::Path, http::StatusCode, response::Json, routing::*, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Goal creation request
#[derive(Debug, Deserialize)]
pub struct CreateGoalRequest {
    pub text: String,
    pub priority: Option<u8>,
}

/// Goal response
#[derive(Debug, Serialize)]
pub struct GoalResponse {
    pub id: String,
    pub text: String,
    pub status: String,
    pub created_at: i64,
}

/// Reasoning request
#[derive(Debug, Deserialize)]
pub struct ReasoningRequest {
    pub goal_id: Option<String>,
    pub query: Option<String>,
    pub max_steps: Option<usize>,
}

/// Reasoning response
#[derive(Debug, Serialize)]
pub struct ReasoningResponse {
    pub goal_id: String,
    pub answer: String,
    pub confidence: f64,
    pub steps: usize,
}

/// Trace response
#[derive(Debug, Serialize)]
pub struct TraceResponse {
    pub goal_id: String,
    pub steps: Vec<TraceStepResponse>,
}

#[derive(Debug, Serialize)]
pub struct TraceStepResponse {
    pub step: u32,
    pub task: String,
    pub sources: Vec<String>,
    pub synthesis: String,
    pub confidence: f64,
}

/// Stats response
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total_goals: u64,
    pub active_goals: u64,
    pub total_episodes: u64,
    pub avg_confidence: f64,
    pub p2p_insights_used: u64,
}

/// Create reasoning API router
pub fn create_router() -> Router {
    Router::new()
        .route("/v2/goals", post(create_goal))
        .route("/v2/goals/:id", get(get_goal))
        .route("/v2/reason", post(start_reasoning))
        .route("/v2/trace/:goal_id", get(get_trace))
        .route("/v2/episodes", get(get_episodes))
        .route("/v2/stats", get(get_stats))
}

/// Create new goal
async fn create_goal(
    Json(request): Json<CreateGoalRequest>,
) -> Result<Json<GoalResponse>, StatusCode> {
    let goal_id = Uuid::new_v4();
    
    Ok(Json(GoalResponse {
        id: goal_id.to_string(),
        text: request.text,
        status: "pending".to_string(),
        created_at: chrono::Utc::now().timestamp(),
    }))
}

/// Get goal by ID
async fn get_goal(Path(id): Path<String>) -> Result<Json<GoalResponse>, StatusCode> {
    Ok(Json(GoalResponse {
        id,
        text: "Example goal".to_string(),
        status: "completed".to_string(),
        created_at: chrono::Utc::now().timestamp(),
    }))
}

/// Start reasoning
async fn start_reasoning(
    Json(request): Json<ReasoningRequest>,
) -> Result<Json<ReasoningResponse>, StatusCode> {
    let goal_id = request.goal_id.unwrap_or_else(|| Uuid::new_v4().to_string());
    
    Ok(Json(ReasoningResponse {
        goal_id,
        answer: "Reasoning result".to_string(),
        confidence: 0.85,
        steps: 4,
    }))
}

/// Get reasoning trace
async fn get_trace(Path(goal_id): Path<String>) -> Result<Json<TraceResponse>, StatusCode> {
    let steps = vec![
        TraceStepResponse {
            step: 1,
            task: "Understand goal".to_string(),
            sources: vec!["grain_123".to_string()],
            synthesis: "Goal understood".to_string(),
            confidence: 0.9,
        },
    ];
    
    Ok(Json(TraceResponse { goal_id, steps }))
}

/// Get episodes
async fn get_episodes() -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

/// Get reasoning stats
async fn get_stats() -> Result<Json<StatsResponse>, StatusCode> {
    Ok(Json(StatsResponse {
        total_goals: 42,
        active_goals: 3,
        total_episodes: 156,
        avg_confidence: 0.78,
        p2p_insights_used: 23,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_create_goal() {
        let app = create_router();
        let server = TestServer::new(app).unwrap();
        
        let request = CreateGoalRequest {
            text: "Test goal".to_string(),
            priority: Some(1),
        };
        
        let response = server.post("/v2/goals")
            .json(&request)
            .await;
        
        response.assert_status_ok();
    }
}
