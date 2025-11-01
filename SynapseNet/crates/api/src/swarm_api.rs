//! REST API for swarm consensus

use axum::{extract::{Path, Query}, http::StatusCode, response::Json, routing::*, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Start swarm consensus request
#[derive(Debug, Deserialize)]
pub struct StartSwarmRequest {
    pub goal_id: String,
    pub max_rounds: Option<u32>,
}

/// Swarm status response
#[derive(Debug, Serialize)]
pub struct SwarmStatusResponse {
    pub goal_id: String,
    pub current_round: u32,
    pub hypotheses_count: usize,
    pub votes_count: usize,
    pub commits_count: usize,
    pub status: String, // "running", "converged", "completed"
}

/// Swarm result response
#[derive(Debug, Serialize)]
pub struct SwarmResultResponse {
    pub goal_id: String,
    pub best_hypothesis: Option<HypothesisResponse>,
    pub final_weight: Option<f32>,
    pub rounds: u32,
    pub converged: bool,
    pub total_hypotheses: usize,
    pub total_votes: usize,
    pub explanation: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HypothesisResponse {
    pub id: String,
    pub content: String,
    pub author: String,
    pub timestamp: i64,
}

/// Hypothesis list response
#[derive(Debug, Serialize)]
pub struct HypothesesResponse {
    pub goal_id: String,
    pub hypotheses: Vec<HypothesisResponse>,
}

/// Query parameters for status
#[derive(Debug, Deserialize)]
pub struct StatusQuery {
    pub goal_id: String,
}

/// Query parameters for result
#[derive(Debug, Deserialize)]
pub struct ResultQuery {
    pub goal_id: String,
    pub explain: Option<bool>,
}

/// Create swarm API router
pub fn create_router() -> Router {
    Router::new()
        .route("/swarm/start", post(start_swarm))
        .route("/swarm/status", get(get_status))
        .route("/swarm/result", get(get_result))
        .route("/swarm/hypotheses/:goal_id", get(get_hypotheses))
        .route("/swarm/votes/:goal_id", get(get_votes))
}

/// Start swarm consensus
async fn start_swarm(
    Json(request): Json<StartSwarmRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::info!("Starting swarm consensus for goal: {}", request.goal_id);

    // TODO: Start swarm loop

    Ok(Json(serde_json::json!({
        "goal_id": request.goal_id,
        "status": "started",
        "max_rounds": request.max_rounds.unwrap_or(3)
    })))
}

/// Get swarm status
async fn get_status(
    Query(query): Query<StatusQuery>,
) -> Result<Json<SwarmStatusResponse>, StatusCode> {
    tracing::debug!("Getting swarm status for goal: {}", query.goal_id);

    // TODO: Get actual status from SwarmHandler

    Ok(Json(SwarmStatusResponse {
        goal_id: query.goal_id,
        current_round: 2,
        hypotheses_count: 8,
        votes_count: 35,
        commits_count: 2,
        status: "running".to_string(),
    }))
}

/// Get swarm result
async fn get_result(
    Query(query): Query<ResultQuery>,
) -> Result<Json<SwarmResultResponse>, StatusCode> {
    tracing::debug!("Getting swarm result for goal: {}", query.goal_id);

    // TODO: Get actual result from SwarmStore

    let explanation = if query.explain.unwrap_or(false) {
        Some("Consensus reached after 3 rounds with 85% agreement".to_string())
    } else {
        None
    };

    Ok(Json(SwarmResultResponse {
        goal_id: query.goal_id,
        best_hypothesis: Some(HypothesisResponse {
            id: "hyp_123".to_string(),
            content: "The answer is based on collective understanding".to_string(),
            author: "node_456".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        }),
        final_weight: Some(0.85),
        rounds: 3,
        converged: true,
        total_hypotheses: 8,
        total_votes: 35,
        explanation,
    }))
}

/// Get hypotheses for goal
async fn get_hypotheses(
    Path(goal_id): Path<String>,
) -> Result<Json<HypothesesResponse>, StatusCode> {
    tracing::debug!("Getting hypotheses for goal: {}", goal_id);

    // TODO: Get from SwarmStore

    Ok(Json(HypothesesResponse {
        goal_id: goal_id.clone(),
        hypotheses: vec![
            HypothesisResponse {
                id: "h1".to_string(),
                content: "First hypothesis".to_string(),
                author: "node1".to_string(),
                timestamp: chrono::Utc::now().timestamp(),
            },
            HypothesisResponse {
                id: "h2".to_string(),
                content: "Second hypothesis".to_string(),
                author: "node2".to_string(),
                timestamp: chrono::Utc::now().timestamp(),
            },
        ],
    }))
}

/// Get votes for goal
async fn get_votes(
    Path(goal_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::debug!("Getting votes for goal: {}", goal_id);

    // TODO: Get from SwarmStore

    Ok(Json(serde_json::json!({
        "goal_id": goal_id,
        "votes": []
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_start_swarm() {
        let app = create_router();
        let server = TestServer::new(app).unwrap();

        let request = StartSwarmRequest {
            goal_id: Uuid::new_v4().to_string(),
            max_rounds: Some(3),
        };

        let response = server.post("/swarm/start")
            .json(&request)
            .await;

        response.assert_status_ok();
    }

    #[tokio::test]
    async fn test_get_status() {
        let app = create_router();
        let server = TestServer::new(app).unwrap();

        let goal_id = Uuid::new_v4().to_string();
        let response = server.get(&format!("/swarm/status?goal_id={}", goal_id))
            .await;

        response.assert_status_ok();
    }

    #[tokio::test]
    async fn test_get_result() {
        let app = create_router();
        let server = TestServer::new(app).unwrap();

        let goal_id = Uuid::new_v4().to_string();
        let response = server.get(&format!("/swarm/result?goal_id={}", goal_id))
            .await;

        response.assert_status_ok();
    }
}
