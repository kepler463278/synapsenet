//! Main reasoning loop - Goal → Plan → Think → Reflect → Learn

use crate::episodes::Episode;
use crate::memory_chain::{MemoryChainManager, MemoryConfig};
use crate::planner::{Planner, TaskGraph, TaskStatus};
use crate::reflection::Reflector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Reasoning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonerConfig {
    pub max_steps: usize,
    pub enable_reflection: bool,
    pub enable_argue: bool,
    pub memory_config: MemoryConfig,
}

impl Default for ReasonerConfig {
    fn default() -> Self {
        Self {
            max_steps: 16,
            enable_reflection: true,
            enable_argue: true,
            memory_config: MemoryConfig::default(),
        }
    }
}

/// Reasoning result with trace
#[derive(Debug, Serialize, Deserialize)]
pub struct ReasoningResult {
    pub goal_id: Uuid,
    pub answer: String,
    pub confidence: f64,
    pub trace: Vec<TraceStep>,
    pub episodes: Vec<Episode>,
}

/// Single step in reasoning trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceStep {
    pub step: u32,
    pub task: String,
    pub sources: Vec<String>,
    pub synthesis: String,
    pub confidence: f64,
}

/// Main reasoner
pub struct Reasoner {
    config: ReasonerConfig,
    planner: Planner,
    memory: MemoryChainManager,
    reflector: Reflector,
}

impl Reasoner {
    /// Create new reasoner
    pub fn new(config: ReasonerConfig) -> Self {
        Self {
            planner: Planner::new(),
            memory: MemoryChainManager::new(config.memory_config.clone()),
            reflector: Reflector::new(),
            config,
        }
    }

    /// Execute reasoning cycle for a goal
    pub async fn reason(&mut self, goal_id: Uuid, goal_text: &str) -> Result<ReasoningResult, String> {
        // 1. Plan: decompose goal into tasks
        let plan = self.planner.plan(goal_text)?;
        
        // 2. Think: execute tasks
        let mut episodes = Vec::new();
        let mut trace = Vec::new();
        
        let sorted_tasks = plan.topological_sort()?;
        let task_count = sorted_tasks.len().min(self.config.max_steps);
        
        for (step, task_id) in sorted_tasks.iter().take(task_count).enumerate() {
            let task_node = plan.get_node(task_id).ok_or("Task not found")?;
            
            // Execute task
            let episode = self.execute_task(goal_id, step as u32, &task_node.task).await?;
            
            // Create trace step
            let trace_step = TraceStep {
                step: step as u32,
                task: task_node.task.clone(),
                sources: episode.retrieved_grains.iter().map(|g| g.grain_id.clone()).collect(),
                synthesis: episode.synthesis.clone(),
                confidence: episode.confidence,
            };
            
            trace.push(trace_step);
            episodes.push(episode);
        }
        
        // 3. Reflect: verify consistency
        if self.config.enable_reflection {
            self.reflector.reflect(&episodes)?;
        }
        
        // 4. Synthesize final answer
        let answer = self.synthesize_answer(&episodes);
        let confidence = self.calculate_confidence(&episodes);
        
        Ok(ReasoningResult {
            goal_id,
            answer,
            confidence,
            trace,
            episodes,
        })
    }

    /// Execute single task
    async fn execute_task(&mut self, goal_id: Uuid, step: u32, task: &str) -> Result<Episode, String> {
        let mut episode = Episode::new(goal_id, step, task);
        
        // Generate query embedding (mock)
        let query_vec = vec![0.5; 384]; // TODO: actual embedding
        episode.query_vec = Some(query_vec.clone());
        
        // Retrieve from memory chain
        let grains = self.memory.retrieve(task, &query_vec).await?;
        for grain in grains {
            episode.add_grain(grain);
        }
        
        // Synthesize (mock)
        let synthesis = format!("Answer to: {}", task);
        episode.set_synthesis(synthesis, 0.8);
        
        // Store episode
        self.memory.add_episode(episode.clone());
        
        Ok(episode)
    }

    /// Synthesize final answer from episodes
    fn synthesize_answer(&self, episodes: &[Episode]) -> String {
        if episodes.is_empty() {
            return "No answer generated".to_string();
        }
        
        // Combine episode syntheses
        episodes.iter()
            .map(|e| e.synthesis.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Calculate overall confidence
    fn calculate_confidence(&self, episodes: &[Episode]) -> f64 {
        if episodes.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = episodes.iter().map(|e| e.confidence).sum();
        sum / episodes.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reasoner_creation() {
        let config = ReasonerConfig::default();
        let _reasoner = Reasoner::new(config);
    }

    #[tokio::test]
    async fn test_reasoning_cycle() {
        let config = ReasonerConfig::default();
        let mut reasoner = Reasoner::new(config);
        
        let goal_id = Uuid::new_v4();
        let result = reasoner.reason(goal_id, "Test goal").await;
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.goal_id, goal_id);
        assert!(!result.answer.is_empty());
    }
}
