//! Action Selector - Determines which tools to use for tasks

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Task that may require tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub task_type: TaskType,
    pub dependencies: Vec<Uuid>,
}

/// Type of task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Research,
    Computation,
    DataProcessing,
    CodeGeneration,
    FileOperation,
    WebQuery,
    Analysis,
}

/// Tool selection for a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSelection {
    pub task_id: Uuid,
    pub tool_name: String,
    pub parameters: serde_json::Value,
    pub confidence: f64,
}

/// Action selector
pub struct ActionSelector;

impl ActionSelector {
    pub fn new() -> Self {
        Self
    }

    /// Determine if task needs tool execution
    pub fn needs_tool(&self, task: &Task) -> bool {
        matches!(
            task.task_type,
            TaskType::WebQuery
                | TaskType::FileOperation
                | TaskType::CodeGeneration
                | TaskType::Computation
        )
    }

    /// Select appropriate tool for task
    pub fn select_tool(&self, task: &Task) -> Option<ToolSelection> {
        let (tool_name, params) = match task.task_type {
            TaskType::WebQuery => {
                // Extract URL from description (simple heuristic)
                if task.description.contains("http") {
                    let url = self.extract_url(&task.description)?;
                    (
                        "web_fetch".to_string(),
                        serde_json::json!({
                            "url": url,
                            "method": "GET"
                        }),
                    )
                } else {
                    return None;
                }
            }
            TaskType::FileOperation => {
                // Determine operation from description
                if task.description.contains("read") {
                    (
                        "file_ops".to_string(),
                        serde_json::json!({
                            "operation": "read",
                            "path": "data.txt"
                        }),
                    )
                } else if task.description.contains("write") {
                    (
                        "file_ops".to_string(),
                        serde_json::json!({
                            "operation": "write",
                            "path": "output.txt",
                            "content": ""
                        }),
                    )
                } else {
                    return None;
                }
            }
            TaskType::Computation => (
                "math_eval".to_string(),
                serde_json::json!({
                    "expression": "2+2"
                }),
            ),
            TaskType::CodeGeneration => (
                "code_exec".to_string(),
                serde_json::json!({
                    "language": "python",
                    "code": "print('hello')"
                }),
            ),
            _ => return None,
        };

        Some(ToolSelection {
            task_id: task.id,
            tool_name,
            parameters: params,
            confidence: 0.8,
        })
    }

    /// Extract URL from text
    fn extract_url(&self, text: &str) -> Option<String> {
        // Simple URL extraction
        for word in text.split_whitespace() {
            if word.starts_with("http://") || word.starts_with("https://") {
                return Some(word.to_string());
            }
        }
        None
    }

    /// Generate tool parameters from task context
    pub fn generate_parameters(
        &self,
        task: &Task,
        context: &serde_json::Value,
    ) -> serde_json::Value {
        // Merge task description with context
        let mut params = serde_json::json!({
            "task_description": task.description,
        });

        if let Some(obj) = params.as_object_mut() {
            if let Some(ctx_obj) = context.as_object() {
                for (k, v) in ctx_obj {
                    obj.insert(k.clone(), v.clone());
                }
            }
        }

        params
    }
}

impl Default for ActionSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_selector_creation() {
        let selector = ActionSelector::new();
        assert!(true);
    }

    #[test]
    fn test_needs_tool() {
        let selector = ActionSelector::new();

        let task = Task {
            id: Uuid::new_v4(),
            description: "Fetch data".to_string(),
            task_type: TaskType::WebQuery,
            dependencies: vec![],
        };

        assert!(selector.needs_tool(&task));

        let task = Task {
            id: Uuid::new_v4(),
            description: "Analyze".to_string(),
            task_type: TaskType::Analysis,
            dependencies: vec![],
        };

        assert!(!selector.needs_tool(&task));
    }

    #[test]
    fn test_select_tool_web_query() {
        let selector = ActionSelector::new();

        let task = Task {
            id: Uuid::new_v4(),
            description: "Fetch https://example.com".to_string(),
            task_type: TaskType::WebQuery,
            dependencies: vec![],
        };

        let selection = selector.select_tool(&task);
        assert!(selection.is_some());

        let selection = selection.unwrap();
        assert_eq!(selection.tool_name, "web_fetch");
    }

    #[test]
    fn test_select_tool_computation() {
        let selector = ActionSelector::new();

        let task = Task {
            id: Uuid::new_v4(),
            description: "Calculate result".to_string(),
            task_type: TaskType::Computation,
            dependencies: vec![],
        };

        let selection = selector.select_tool(&task);
        assert!(selection.is_some());
        assert_eq!(selection.unwrap().tool_name, "math_eval");
    }

    #[test]
    fn test_url_extraction() {
        let selector = ActionSelector::new();

        let url = selector.extract_url("Fetch https://example.com for data");
        assert_eq!(url, Some("https://example.com".to_string()));

        let url = selector.extract_url("No URL here");
        assert_eq!(url, None);
    }
}
