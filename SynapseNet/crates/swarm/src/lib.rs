//! Swarm Cognition - Collective intelligence

pub mod com;
pub mod loop_impl;
pub mod rov;
pub mod schema;

pub use com::ConsensusOfMeaning;
pub use loop_impl::SwarmLoop;
pub use rov::{AuthorReward, ReinforcementOfValue, VoterReward};
pub use schema::*;

// Re-export loop as loop_impl to avoid keyword conflict
pub use crate::loop_impl as swarm_loop;
