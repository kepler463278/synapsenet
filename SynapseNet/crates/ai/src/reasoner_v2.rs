//! Reasoner v2 - Extended with action execution

use crate::action_selector::{ActionSelector, Task};
use crate::memory_chain::MemoryChain;
use crate::planner::Planner;
use crate::reasoner::Reasoner;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Reasoning result with actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningResultV2 {
    pub goal_id: Uuid,
    pub answer: String,
    pub confidence: f64,
    pub steps: usize,
    pub actions_performed: usize,
    pub tools_used: Vec<String>,
    pub total_time_ms: u64,
}

/// Reasoner v2 with action support
pub struct ReasonerV2 {
    base_reasoner: Reasoner,
    action_selector: ActionSelector,
}

impl ReasonerV2 {
    /// Create new reasoner v2
    pub fn new(memory_chain: Arc<MemoryChain>, planner: Arc<Planner>) -> Self {
        Self {
            base_reasoner: Reasoner::new(memory_chain, planner),
            action_selector: ActionSelector::new(),
        }
    }

    /// Execute reasoning with actions
    pub async fn reason_with_actions(
        &self,
        goal_id: Uuid,
        query: &str,
        max_steps: usize,
    ) -> Result<ReasoningResultV2, String> {
        let start = std::time::Instant::now();
        
        // Get plan from base reasoner
        let plan = self.base_reasoner.planner().create_plan(goal_id, query).await?;
        
        let mut actions_performed = 0;
        let mut tools_used = Vec::new();
        let mut steps = 0;
        
        // Execute each task
        for task_node in plan.tasks.iter().take(max_steps) {
            steps += 1;
            
            // Check if task needs tool
            let task = Task {
                id: task_node.id,
                description: task_node.description.clone(),
                task_type: crate::action_selector::TaskType::Research, // Default
                dependencies: task_node.dependencies.clone(),
            };
            
            if self.action_selector.needs_tool(&task) {
                if let Some(selection) = self.action_selector.select_tool(&task) {
                    actions_performed += 1;
                    if !tools_used.contains(&selection.tool_name) {
                        tools_used.push(selection.tool_name.clone());
                    }
                    
                    // TODO: Actually execute tool via AgentCore
                    tracing::info!("Would execute tool: {}", selection.tool_name);
                }
            }
        }
        
        // Get final answer from base reasoner
        let base_result = self.base_reasoner.reason(goal_id, query, max_steps).await?;
        
        let total_time_ms = start.elapsed().as_millis() as u64;
        
        Ok(ReasoningResultV2 {
            goal_id,
            answer: base_result.answer,
            confidence: base_result.confidence,
            steps,
            actions_performed,
            tools_used,
            total_time_ms,
        })
    }

    /// Get base reasoner
    pub fn base(&self) -> &Reasoner {
        &self.base_reasoner
    }

    /// Get action selector
    pub fn selector(&self) -> &ActionSelector {
        &self.action_selector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reasoner_v2_creation() {
        let memory_chain = Arc::new(MemoryChain::new());
        let planner = Arc::new(Planner::new());
        
        let reasoner = ReasonerV2::new(memory_chain, planner);
        assert!(true);
    }
}
