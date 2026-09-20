use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: u32,
    pub to: u32,
    pub weight: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphInput {
    pub vertices: u32,
    pub edges: Vec<Edge>,
    pub source: u32,
}

impl GraphInput {
    pub fn validate(&self) -> Result<(), crate::error::ComputeError> {
        use crate::error::ComputeError;
        if self.vertices == 0 {
            return Err(ComputeError::InvalidGraph("vertices == 0".into()));
        }
        if self.source < 1 || self.source > self.vertices {
            return Err(ComputeError::InvalidGraph("bad source".into()));
        }
        for e in &self.edges {
            if e.from < 1 || e.from > self.vertices || e.to < 1 || e.to > self.vertices {
                return Err(ComputeError::InvalidGraph(format!("bad edge {e:?}")));
            }
            if e.weight < 0 {
                return Err(ComputeError::InvalidGraph("negative weight".into()));
            }
        }
        Ok(())
    }

    pub fn adj_list(&self) -> Vec<Vec<(u32, i64)>> {
        let mut adj = vec![Vec::new(); self.vertices as usize + 1];
        for e in &self.edges {
            adj[e.from as usize].push((e.to, e.weight));
        }
        adj
    }
}
