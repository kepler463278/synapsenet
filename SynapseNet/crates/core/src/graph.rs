use crate::{Grain, Link};
use std::collections::HashMap;

/// Local semantic graph (DAG)
#[derive(Debug, Default)]
pub struct Graph {
    /// Grains indexed by ID
    grains: HashMap<[u8; 32], Grain>,
    /// Links indexed by source grain ID
    links: HashMap<[u8; 32], Vec<Link>>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add grain to graph
    pub fn add_grain(&mut self, grain: Grain) {
        self.grains.insert(grain.id, grain);
    }

    /// Add link to graph
    pub fn add_link(&mut self, link: Link) {
        self.links.entry(link.from).or_default().push(link);
    }

    /// Get grain by ID
    pub fn get_grain(&self, id: &[u8; 32]) -> Option<&Grain> {
        self.grains.get(id)
    }

    /// Get outgoing links from grain
    pub fn get_links(&self, from: &[u8; 32]) -> Option<&Vec<Link>> {
        self.links.get(from)
    }

    /// Get all grains
    pub fn grains(&self) -> impl Iterator<Item = &Grain> {
        self.grains.values()
    }

    /// Count grains
    pub fn grain_count(&self) -> usize {
        self.grains.len()
    }

    /// Count links
    pub fn link_count(&self) -> usize {
        self.links.values().map(|v| v.len()).sum()
    }

    /// Compute connectivity metrics
    pub fn connectivity_metrics(&self) -> ConnectivityMetrics {
        let total_grains = self.grain_count();
        let total_links = self.link_count();

        let avg_degree = if total_grains > 0 {
            total_links as f32 / total_grains as f32
        } else {
            0.0
        };

        ConnectivityMetrics {
            total_grains,
            total_links,
            avg_degree,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectivityMetrics {
    pub total_grains: usize,
    pub total_links: usize,
    pub avg_degree: f32,
}
