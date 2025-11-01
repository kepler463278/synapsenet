//! Agent crate - Tool execution and action management

pub mod agent_core;
pub mod sandbox;
pub mod tool_api;
pub mod tool_registry;

pub use agent_core::{ActionLog, ActionTrace, AgentCore};
pub use sandbox::{Sandbox, SandboxBuilder, SandboxConfig, SandboxError, SandboxLimits};
pub use tool_api::{
    ExecutionContext, Tool, ToolError, ToolInfo, ToolInput, ToolMetadata, ToolOutput,
    ToolParameter, ToolReturn, ToolSchema, ResourceUsage,
};
pub use tool_registry::{RateLimit, ToolPolicy, ToolRegistry};
