use super::{Algorithm, MintyResult, MintyStep};
use crate::graph::GraphInput;

pub struct MintyAlgorithm;

impl Algorithm for MintyAlgorithm {
    fn name(&self) -> &'static str {
        "minty"
    }

    fn execute(
        &self,
        g: &GraphInput,
        include_steps: bool,
    ) -> Result<(MintyResult, Vec<MintyStep>), crate::error::ComputeError> {
        g.validate()?;
        let n = g.vertices;
        let adj = g.adj_list();
        let mut dist: Vec<Option<u64>> = vec![None; n as usize + 1];
        let mut prev: Vec<Option<u32>> = vec![None; n as usize + 1];
        let mut visited = vec![false; n as usize + 1];
        dist[g.source as usize] = Some(0);
        let mut steps = Vec::new();

        for _ in 0..n {
            let u = (1..=n)
                .filter(|&v| !visited[v as usize] && dist[v as usize].is_some())
                .min_by_key(|&v| dist[v as usize])
                .unwrap_or(0);
            if u == 0 {
                break; // rest unreachable
            }
            visited[u as usize] = true;
            for &(v, w) in &adj[u as usize] {
                let nd = dist[u as usize].unwrap() + w as u64;
                let better = match dist[v as usize] {
                    None => true,
                    Some(d) => nd < d,
                };
                if better {
                    dist[v as usize] = Some(nd);
                    prev[v as usize] = Some(u);
                }
            }
            if include_steps {
                steps.push(MintyStep {
                    current: Some(u),
                    visited: (1..=n).filter(|&v| visited[v as usize]).collect(),
                    distances: (1..=n).map(|v| (v, dist[v as usize])).collect(),
                });
            }
        }

        let mut distances = std::collections::BTreeMap::new();
        let mut paths = std::collections::BTreeMap::new();
        for v in 1..=n {
            distances.insert(v, dist[v as usize]);
            let mut p = vec![];
            if dist[v as usize].is_some() {
                let mut cur = v;
                while cur != g.source {
                    p.push(cur);
                    cur = prev[cur as usize].unwrap();
                }
                p.push(g.source);
                p.reverse();
            }
            paths.insert(v, p);
        }
        Ok((
            MintyResult {
                source: g.source,
                distances,
                paths,
            },
            steps,
        ))
    }
}
